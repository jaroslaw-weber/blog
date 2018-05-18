# Why

I have a static website on github pages. I was using small javascript script to write custom "html form to toml formatter". The site is simple, you fill the form and click "save" button and it saves the contents of the form to toml file.

I used "moment.js" for parsing date, but I didn't want to mess with npm just yet. My custom formatter was mostly working, but had some bugs. I was deciding between using npm for something without bugs or trying wasm. I decided it could be interesting experiment to write front-end with Rust. It is great language, and it is much easier to maintain than javascript.

So I rewrote my javascript to Rust.

# SEO & macros formating vs going full rust

//todo: talk about quasar and yew more
I found out about quasar and yew. Great projects. But I didn't want to make a one page application. I would need to load everything before displaying a website which is not good for user experience, especially on small site targeted for both pc and mobile. Also, I wanted to stay SEO friendly as much as possible. So I decided to go the "jQuery way" - create static page and manipulate it later.

For creating static page I used my site generation library - slime. It loads data from Toml files and put it inside handlebar templates.

For dom manipulation I used stdweb;

# stdweb

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

fn get_time() -> String {
    js! (
        return moment().format();
    ).try_into()
        .expect("failed to get time")
}

...
let time_string = get_time();
let time = toml::value::Datetime::from_str(&time_string).expect("failed to parse date");
```
So I was able to find quite easy workaround. Although, I wish I could do it with only Rust.

# File size

The other problem was file size. My website was loading slowly. So I assumed it was because of the size of wasm file (300kb). Well, the problem was, I accidentally copy-pasted "font-awesome". It was not so awesome, it was blocking the loading of my page. Anyway, I did not notice at first so I tried to reduce the wasm size. With some config parameters and changing allocator to something smaller, I got down to 220kb.

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

# Same page, second app

# Mysterious bug, bad mutex and disappearing errors

# It is working!

https://cbt-diary.github.io/

# Thank you github