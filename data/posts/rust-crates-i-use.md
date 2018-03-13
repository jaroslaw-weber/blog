# failure

```
use failure::Error;
fn foo(a: i32) -> Result<String, Error>
{
    let x = foo2(a)?;
    let y = foo3(x)?;
    ensure!(y>2);

    //...
}
```

My favorite crate for error handling. Using standard library is bulky, and I didn't like that I need to use macros for everything in error-chain.

We can use `failure::Error` to group multiple errors. We don't need to use `Box<Error>` everywhere. We can also ensure some conditions with simple macro. 

This is as simple as unwrapping, so we can prototype easily without losing the stability!

# github-gql-rs

```
let mut g = Github::new("token").unwrap();
    let (headers, status, json) = g.query::<Value>(
        &Query::new_raw(r#"

{
  user(login: "SergioBenitez") {
    repository(name: "Rocket") {
      stargazers {
        totalCount
      }
    }
  }
}


"#).unwrap();
```

Github introduced GraphQL as their 4th version of API. What is great about the GraphQL is that you can get a lot of information with only one request (which would be impossible with simple REST). This crate is just a simple wrapper for GraphQL API and uses `&str` requests, the versioning is still on 0.0.1 so it is not very stable, but still it was better experience and more flexible than REST version of the API.

To check and fix the formatting of our request we can use GraphQL explorer:

https://developer.github.com/v4/explorer/


# serde_json

```
#[derive(Serialize, Deserialize)]
struct Car {
    name: String,
    color: String,
    comments: Vec<String>
}

fn get_car() -> Result<Car, Error> {
    let data = r#"{
                    "name": "old car",
                    "color": "red",
                    "comments": [
                      "wow!",
                      "great car!"
                    ]
                  }"#;
    let c: Car = serde_json::from_str(data)?;
    Ok(c)
}
```
Everyone using Rust know this library. This is a library for parsing json. With `serde_derive` you can deserialize anything very fast and easily.

# toml

```
let toml_str = r#"
        name = "Joe"
        age = 32
        favorite_color = "red"
        [[kids]]
        name = "Baby 1"
        [[kids]]
        name = "Baby 2"
    "#;

let decoded: Person = toml::from_str(toml_str).unwrap();
```

To be honest, I don't use so much `serde_json` anymore. I ditched it after I found out about Toml - my favorite language for data serialization. It is more readable and easier to write than json, it is much simpler than YAML. 

More about Toml:

https://github.com/toml-lang/toml

# clap

```
 let matches = App::new("My first app")
                          .version("1.0")
                          .author("Me <me@gmail.com>")
                          .about("Does nothing")
                          .arg(Arg::with_name("name")
                               .short("n")
                               .long("name")
                               .value_name("NAME")
                               .help("Help me knows your name")
                          .get_matches();

    let name = matches.value_of("name").unwrap_or("World);
    println!("Hello {}!", name);
```
Want to make a simple CLI application? `clap` is the way to go.

# handlebars-rust

```
let mut reg = Handlebars::new();
let template = "Hello {{name}}";
let data = json!({"name": "World"});
let rendered = reg.render_template(template, &data).unwrap();

println!("{}", rendered);
```
Want to use templates for your website? I am using this library for almost all my static and dynamic website projects. The ability to break HTML to smaller parts is something I don't want to part with.

# Rocket

```
#[get("/<name>")]
fn hello(name: String,) -> String {
    format!("Hello {}", name)
}

fn main() {
    rocket::ignite().mount("/hello", routes![hello]).launch();
}
```
The easiest way to make a dynamic website with Rust is Rocket. It uses macros and nightly (early development version) features of Rust so you cannot use it with stable version of the language, but it is a still a great library!

# csv

```
#[derive(Debug, Deserialize)]
struct Map {
    data: String,
    template: String,
    output: String,
}

fn foo()-> Result<(), Error>
{
    let mut v = Vec::new();
    let mut rdr = csv::Reader::from_path("map.csv")?;
    for result in rdr.deserialize() {
        let m: Map = result?;
        v.push(m);
    }
    //...
}
```
Very fast and easy to setup csv parsing library. As every serialization library, it is compatible with `serde` and `serde_derive` which means that most of the serialization in Rust is very centralized and it is very easy to use other serialization libraries after you learn one.


# slime

```
let s = Slime::default();
s.initialize()
let data = s.load_toml_data("data")
    .expect("failed to load toml data");
s.generate("index", "index", &data)
    .expect("failed to generate page");
```
This is actually a crate I made, but also a crate I used a lot. It is a flexible site generator. I made this blog with `slime`. If you want to have static page but also want to customize every aspect of it, it is great choice. Current solutions (jekyll, gutenberg) are great but are not very flexible. `slime` has also a binary version, `slime-mini`, which works without writing any Rust code! Great for prototyping! 