# Why

I have a static website on github pages. I was using small javascript script to write custom "html form to toml formatter". The site is simple, you fill the form and click "save" button and it saves the contents of the form to toml file.

I used "moment.js" for parsing date and "filesaver.js" for saving files. I started to use few outside libraries so I was thinking about going from plain javascript to npm (node package manager) project. But I didn't want to mess with npm just yet. My custom formatter was mostly working, but had some bugs. I was deciding between using npm or trying wasm. I decided it could be interesting experiment to write front-end with Rust. It is great language, and it is much easier to maintain than javascript.

So I rewrote my javascript to Rust.

# SEO & macros formating vs going full rust

I found out about quasar and yew. Great projects. But I didn't want to make a one page application. I would need to load everything before displaying a website which is not good for user experience, especially on small site targeted for both pc and mobile. Also, I wanted to stay SEO friendly as much as possible. So I decided to go the "jQuery way" - create static page and manipulate it later.

For creating static page I site generation library I created - slime. It loads data from toml files and put it inside handlebars templates.

For dom manipulation I used stdweb.

# quasar

Quasar is one framework for writing front-end web applications with rust.

[https://github.com/anowell/quasar](https://github.com/anowell/quasar)

Example code:

```rust
#[derive(Default, BartDisplay)]
#[template_string = "<p>Count: {{count}}</p><button>+1</button>"]
struct CounterData { count: u32 }

impl Component for CounterData {
    fn onload(view: &View<Self>) {
        view.on_each(EventType::Click, "button", |mut evt| {
              evt.binding.data_mut().count += 1;
        });
    }
}

fn main() {
    let mut app = quasar::init();
    app.bind("#counter", CounterData::default());
    app.spin();
}
```

It is quite straightforward and looks really nice, but as I already said, I needed to only manipulate dom, not create everything from scratch.

# yew

Yew is another library for doing front-end with Rust.

[https://github.com/DenisKolodin/yew](https://github.com/DenisKolodin/yew)

It is very similar to quasar but little more verbose. It is currently the most popular library for writing front-end in Rust (4000 starts, over 100 forks). It has a lot of examples.

# stdweb

[https://github.com/koute/stdweb](https://github.com/koute/stdweb)

Stdweb is just a wrapper around a plain javascript. It is mostly mapping things like "querySelector" or ".childNodes()" into rust syntax. It also has 'js!' macro which allows you to write javascript inside Rust file. It is very simple but powerful library. It is still lacking the functionality but you can do a lot of things with plain rust, and fill the holes with 'js!' macro.

Macro example:
```rust
let name = "Peter";
let age = 30;
js! {
    var person = @{person};
    console.log( @{name} + " is " + @{age} + " years old." );
};
```

Super easy, right?

Example of querying inside rust:
```rust

let inputs = form.query_selector_all("input").expect("no input");
```

Same as javascript. It was mostly like writing javascript with types.

# Rewriting script in rust

Rewriting everything was quite easy. I was already familiar with toml crate, and querying form was also easy with stdweb.
The problem was mainly with not being able to use all creates (like chrono, unfortunately) and getting everything working (it was my first wasm project so I was not sure how to set it up).
The other problem was app size.

# Datetime problem

So the `chrono` crate did not compile. `chrono` is using `std::time`. It is possible to use chrono with wasm, but I cannot use `time` so I cannot get current time. This was great obstacle for me, as I was planning to add timestamp into my toml files. The easiest way was just using `moment.js` like this:

```rust
using toml::value::DateTime;

fn get_time() -> String {
    js! (
        return moment().format();
    ).try_into()
        .expect("failed to get time")
}

...
let time_string = get_time();
let time = Datetime::from_str(&time_string)
    .expect("failed to parse date");
```
So I was able to find quite easy workaround. Although, I wish I could do it with only Rust.

# File size

The other problem was file size. My website was loading slowly. So I assumed it was because of the size of wasm file (300kb). Well, the problem was, I accidentally copy-pasted "font-awesome" font into header (...) . It was not so awesome, it was blocking the loading of my page. Anyway, I did not notice at first so I tried to reduce the wasm size. With some config parameters and changing allocator to something smaller, I got down to 220kb.

Here was my new config file:
```toml
[dependencies]
stdweb = "0.4.4"
toml = "0.4.6"
wee_alloc = "0.4.0"

[profile.release]
lto = true
opt-level = 'z' # `s` or 'z'
debug = false
```

Not bad, especially that it is loading in the background asynchronously (so it will load before you fill the form!). I was actually able to get it down to about 160kb with wasm-opt.

There is actually a page, where you can read about this stuff:

[Link Here](https://rust-lang-nursery.github.io/rust-wasm/game-of-life/code-size.html)

# It is working! But...

I builded the script, uploaded on Github. 

[Github Pages - here](https://cbt-diary.github.io/)

[Source Code - here](https://github.com/jaroslaw-weber/cbt-diary)

The script is working and site is loading fast (because the wasm script is loaded asynchronously). The whole page size (without wasm) is 65kb. Not bad!
How would I make it even smaller? "moment.js" library is 32kb. I could load it asynchronously.

By the way, the javascript file generated with cargo web is 5kb so it is really small.

Still, we need to wait for the load of wasm file which is about 200kb.

So the site is working! Kind of. I am using adblock plugin (ublock origin) and it blocks files with .wasm extension. It is easy to do a client side workaround (turn off the adblock or fix the adblock rule) but explaining this to user is making the user experience little worse.

# Trust

The other painpoint of wasm is that it is not readable. With javascript, we can see the source inside the browser. Yes, you can check the code on the github, but it is more difficult to confirm "safety" of the code, than pure javascript implementation. 

My website is handling some sensitive data and user don't want the data to be stored anywhere. Before, I could say to him "look at the source code in the browser" to assure that I don't do anything suspicious, but currently I can recommend "turning off the wifi" or checking the source code on Github.

# Build times again...

Unfortunately, my build times got much longer. I was only using plain javascript before so there was no "build step". Yes, we can cache things in GitHub, but first build is painfully long.

# Same page, second app

I wanted to also create script for analysing the toml files and showing the result on the same page by manipulating DOM. Still, I wanted to have it as a separate project from the "form to toml" part. So I created new project.

# Async dom update

I wanted user to be able to upload multiple times at the same time and update the dom as the file is ready. 

I had a list which I wanted update on file upload. So I deleted all nodes each time there was new file uploaded, and then added new list elements (rebuilding list compeletely). 

But asynchronously rebuilding list had a bug. It was creating new elements too slow and list had a lot of duplicates. So I used "replace_node" for replacing the list elements. 
This time I did not get duplicates.

# Mysterious bug, bad mutex and disappearing errors

Working on my script I started getting mysterious error:
```
RuntimeError: unreachable executed
```
Until some time ago everything was working and outputing nice panic messages. But something changed and all errors became one.

So I asked my friend google what is going on and found this:

[https://github.com/jakedeichert/wasm-astar](https://github.com/jakedeichert/wasm-astar)

The problem was... mutex locks. I added mutex when I decided to add asynchronous behaviour inside my app. After fixing all bugs in my code everything was working. So I was able to stop error from appearing but it will probably come back if I create bug somewhere. It was not great to debug, unfortunately.

# Cross-platform GUI solution?

Wasm may be THE way to create cross-platform gui apps with Rust. It is cross-platform, you don't need to pack whole browser to use it (lighter than electron), and it is easy to build (with some wrappers like yew/quasar). You could also post it on github pages for free.

The problem may be some crates not working ("chrono") on wasm yet. Still, you can find some workarounds with javascript libraries.

The overall experience with wasm was great. I see a great potential in it and hope yew/quasar libraries get better.

