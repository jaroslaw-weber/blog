# Option

Rust uses `Option` to prevent `NullReferenceException` on runtime. Rust does not have `null`. Instead, you can express "lack of value" with an `Option` wrapper. It is so much easier to work with options than nulls.
Let's say we have a function in C#:

```
void DoSomething(SomeClass someClass)
{
    var x = someClass.GetX();
}
```

If we accidentally pass null into the method, we get a runtime error. When debugging, you need to find where this error happened and think why it happened. In Rust you can write:

```
fn do_something(someStruct: &SomeStruct)
{
    let x = someStruct.get_x();
}
```
We can guarantee that this won't crash. Struct cannot be null. How would we express "lack of value" with Option then?

```
fn foo()
{
    let content : Option<String> = read_file("text.txt");
    content.expect("failed to read the text.txt file!");
}
```
`read_file` can fail. But in this case, it will return `Some(content_as_string)` or `None` and won't crash.
So what can we do with `Option`? Why not just use plain nullable type?
Option is a wrapper so it does not allow directly access wrapped value inside it. To access the value, you need to unwrap it first. This prevent's from calling a function or accessing a field from something that does not exists.

We can use `unwrap()` function to get the value inside, but it is not recommended. It is dirty and quick solution for prototyping.

`unwrap` tries to take value from the `Option` but panics (crash) if there is no value.
Another way would be using `expect()` function to unwrap with custom error, as in the previous example.
As you can see, it is much easier to write a custom error than in C#.
In C# we would need to write it like this:
```
void Foo()
{
    string content = ReadFile("text.txt");
    if(content == null || file == string.Empty)
    {
        new System.Exception("failed to read the file!");
    }
}
```
As you can see, Rust code is more compact and elegant. You could write it even shorter, if you omit the type signature `Option<String>`. 

# Result & Error handling

Crashing an app is not always a good way to handle an error.

So what would you do to avoid crashes in C#? You would probably write null checks in every possible place.
I don't want to see this monster. Rust have instead something called `Result`.

`Result` is like an `Option`, but contains information (error), why some action failed.
Let's see some example code:

```
extern crate failure;
using failure::Error;
fn read_first_line() -> Result<String, Error>
{
    let file: String = read_file("some_text.txt")?;
    let lines = file.lines();
    let first_line: String = lines.next()?;
    Ok(first_line)
}
 ```

So what is going on here? There is some background magic that needs to be explained.
First, `extern` and `using` statements are just importing things from another library.
Then you can see `Result<String, Error>` in return value signature. This means that you will get the "first line of file" wrapped in Result or get an error wrapped in Result.

The first line of this function have `?` (question mark) in the end. It also states that the return value will be String. Why not `Option` or `Result`? Reading file can fail! Actually, `read_file()` returns a `Result<String, Error>`. But we not only also calling a function, we also use a safe unwrapper symbol - `?` after getting the `Result`. `?` is just a shortcut for `try!` macro. If unwrapping (using `?`) fails, Rust will just return error from `read_file()` function early. If you expand this, this would be something like this:

```
let file_result: Result<String, Error> = read_file("some_text.txt");
//if we get error we just return it
if file_result.is_error()
{
    return file_result.get_error();
}
//if not we safely unwrap the value!
let file = file_result.unwrap();
```

Neat, huh? This saves lot of typing and provides incentive to not crashing the application.
Each `?` is just a safe unwrapping with early return. Writing this in C# would be a nightmare.
Believe me I tried.

So what was the last line, with `Ok` in it? We need to return the `Result`. We cannot just return String.
`Ok(value)` is just creating a `Result` wrapper for the value.

Also, did you notice lack of `return` keyword? It can be ommited in Rust. Makes your code little shorter.

# Why use wrappers?

It is all about the control. We, as a library user, can decide that we don't want to crash the application, even if some library (for example, parsing json) fails. Instead of crashing with exception, we can deal with it in more elegant way, and provide a flow without bulky and easy to forget `try/catch` statement. Every library in C# can crash unexpectedly. If you don't wrap the methods in `try/catch` you will have a potential invisible crash waiting there.

Most of libraries in Rust don't crash when something goes wrong. You get a value in a `Result` wrapper. You decide what do you want to do if something goes wrong.

Some people may wonder, why did we imported the external crate to handle error. 
Well there is a reason.

# Failure

`failure` crate (crate means library) is a new library for error handling. Before `failure`, people were using `std::error::Error` from standard library or `error_chain` crate. Why so many choices about error handling? Shouldn't errors be in the standard library? Well, one of Rust greatest strengths is flexibility. Standard library is reduced to necessary minimum. Every additional functionality needs to be imported from external library. 

Still, putting error handling in external library seems odd. But error handling is also a part of a language that can be improved. If we decide to use one version of error handling we are doomed to use it forever. If we try to change it, when it is inside the standard library, we break whole language. Moving important parts to external libraries give Rust an opportunity to "iterate" through different solutions. That is how `failure` was born. First, we had only standard `Error`. But the functionality was not enough, and writing custom errors was bulky. So someone came up with `error_chain` crate for better error handling. Still, after using the `error_chain`, someone came up with better, more elegant solution which would eliminate downsides of `error_chain`.

# Cargo

Cargo is a package manager, something like NuGet for C#. To add a package for C#, we would add it with GUI in Visual Studio. I don't think the package manager should be embedded into IDE. Package manager should be something standalone, like "npm" or "homebrew".

We could probably try to edit nuget.config file. Let's see example packages in nuget.config:
```
<packageSources>
    <add key="nuget.org" value="https://api.nuget.org/v3/index.json" protocolVersion="3" />
    <add key="Contoso" value="https://contoso.com/packages/" />
    <add key="Test Source" value="c:\packages" />
</packageSources>
```

And that is why I don't like xml. It is very verbose. "npm" which is a javascript package manager is using json. Toml is much better for configuration files. What is Toml? It is a language similar to YAML but more simple. Cargo uses Toml for project config file. Example project (yes, whole project) config file in Rust:

```
[package]
name = "blog"
version = "0.1.0"
authors = ["Jaroslaw Weber <jaroslaw.weber@gmail.com>"]

[dependencies]
slime = "0.4.0"
toml = "0.4.5"
easy_fs = {git = "https://github.com/jaroslaw-weber/easy_fs"} 
```

So simple that someone who doesn't even know Toml can read and edit it. Adding a crate (library) is very simple. You just edit the .toml file and add a line with crate name and version. When you build the project, crates will automatically download. When you download someone's repository and want to test it, you don't need to set up anything, you just run `cargo build` from command line and dependencies are automatically downloaded and installed (localy! because different projects are using different version of libraries). If you want to change the library version you just change the number in the config file and rebuild it again. So easy and clean! And don't need to install anything!

# Immutability by default

When declaring a variable in Rust, the variable is immutable by default. If we would try do this:
```
let v = Vec::new();
v.push(1);
```
the compiler would scream. We need to explicitly say:
```
let mut v = Vec::new();
v.push(1);
```
to be able to mutate the variable.

A lot of data we pass around is read-only. If we deserialize some data from csv or json, we usually don't want anyone to play with it. To ensure it, we would need to create properties. Too much mutable state is not very safe. With explicit mutability, we can see where the variable could be changed, and if something is wrong with data, we can quickly pinpoint the potential culprit.

# Macros

Have you ever tried generate code with C#? You can do it with T4 templates or try manually generating files. If you ever make mistake in the template or generating script and generate it, you will need some time to undo it (probably reseting the changes with git command). Also you need to actually generate the code to check if template is valid. Not very pleasant experience.

What else could you do? You could use reflection and make your implementation 3 times slower. Not a great solution also.

Macros are just a way to generate code without creating new files.
If you ever tried C++ macros you may think macros are evil. Overusing macros is bad practice, but no macros creates lot of boilerplate code.

Rust macros are safer. Basically there are 2 types of macros: function-like and attribute-like.
Macros enables you to statically check the correctness. Compiler won't compile your code if something is wrong with macro. You are also not creating any new files, and can insert macros almost anywhere. This prevents you from creating too many files when you are doing a lot of code generation, and let keep your project clean and simple. 

# Full control over memory

Rust does not have garbage collector. GC has a runtime overhead. If you want to have faster implementation of something in C# you would have to write it in another language and import as `.dll` file. Rust's speed is similar to C++ and C. But wouldn't manual control over memory be more dangerous and difficult to write? Not exactly. Rust have something called "lifetimes". Rust automatically release resources from the memory without explicitly calling deallocation functions. It is different concept than "reference counting" (and smart pointers) which you may know from other languages, like Swift or C++. 

```
fn foo()
{
    let x = 3;
    let y = Point::new(1,3);
}
```

`x` and `y` would be released from memory when `foo()` ends. Even without the garbage collector, you don't need to manage memory manually.

# Traits

Traits are like interfaces in C#, but traits can also contain implementation. For example:

```
trait HaveExtension
{
    extension_without_dot: String

    //adds dot before extension name and return it
    fn get_extension_with_dot() -> String
    {
        format!(".{}", &extension_without_dot)
    }
}
```

There are some limitations to it - you can only use fields declared inside trait. You could use abstract class in C#, but you could avoid the inheritance and create mixins-like classes with traits. It is as safe as using an interface, but more flexible.
There is also another great thing about traits.

```
#[derive(Debug,PartialEq,Copy)]
struct Point {
    x: i32,
    y: i32,
}
```
You can "derive" some traits. Deriving is just using macros for generating an implementation. So a lot of times you don't need to even implement the trait, because macro is doing it for you.

It is also an "abstraction without overhead" (term borrowed from official Rust blog). What it means is that you usually get some overhead when using generics. But with traits you don't have any overhead (static dispatch). So it is faster than generics (dynamic dispatch). Still, you can use generics in Rust so the solution is very flexible.

# Syntax

It is my personal preference, but I find Rust's syntax more elegant than C#. 

Functions are more readable because of the snake case. `ThisIsVeryLongFunctionName` would be `this_is_very_long_function_name` in Rust. Rust is very minimalistic in all aspects, hiding unnecessary complexity, but also being explicit about important things.
```
using System;
namespace HelloWorld
{
    class Hello 
    {
        static void Main() 
        {
            Console.WriteLine("Hello World!");
        }
    }
}
```
would be
```
fn main ()
{
    println!("hello world!");
}
```
in Rust.

The other nice thing about Rust's syntax is that `return` keyword is not necessary:
```
fn double(x:i32)-> i32
{
    x*2
}
```
Last line (without semicolon) is just returning a value. `return` is really redundant in C#. 

Rust deals with `Option` and `Result` using only `?`. It is very simple, and yet very powerful. It is similar in functionality to `??` ("elvis operator") in C# for nullchecks.

I like how Rust changed `int` to `i32`. "What is the point?", you could say. How about `f32` or `u64`? In C# you would write `float` or `ulong` which is longer.

# Safety

Rust unique features prevent lot of problems which would otherwise appear on runtime. If something compiles, it is probably much safer than same thing written in different language.
Option prevents null exceptions, lifetimes/borrowing prevents from data races.

"If it compiles, it works" is the motto of Rust. This is THE safest language (comparable to haskell) and that is why it's used for multithreading and programming reliable systems.

# Compiler errors

Compiler errors are beautiful in Rust. If you write this:
```
fn main() {
       let v = Vec::new();
       v.push(1);
}
```
you will get this error:

```
error[E0596]: cannot borrow immutable local variable `v` as mutable
 --> src/main.rs:3:8
  |
2 |        let v = Vec::new();
  |            - consider changing this to `mut v`
3 |        v.push(1);
  |        ^ cannot borrow mutably
```

This error is showing not only what happened, but also where it happened (with nice little arrow). It also shows you a context (part of code) and very often a solution! "consider changing this to `mut v`" is explaining how to fix the code!

```
fn main() {
       let mut v = Vec::new();
       v.push(1);
}
```

We listened to the compiler and now it is working!

# Multiplatform support

C# is a Microsoft's baby. It is kind of multiplatform, but everything started on Windows. So there are some things you can do only on Windows.

I tried removing all unused usings from the project. I would need to pay for Resharper and use Windows to do that. 

I tried to install docfx on mac. I couldn't because there was a bug "linux and mac only".
I often hit the "not windows, sorry" wall, those two examples are from last month only.
The tooling in Rust is open source and multiplatform from the very beginning.

# Future of Rust and C#

Rust have a nightly (early development) channel - you can try features before they are stabilised. New features needs to be proposed and implemented, but this process is much faster than in C#. Many people are using nightly channel, and each month there are new, exciting features added to the language. If it's not a breaking change it will probably be added to stable eventually.
Rust provides great environment for experimenting, but also make sure that only well thought and well tested features are added to stable version of the language. Each feature needs to be discussed before the implementation.

C# is going to probably implement a lot of features that Rust already have.  There are already propositions for non-nullable references as default (which would reduce null exceptions on runtime, probably going to be released in 8.0 version of the language). I also saw macros proposition but I think it is a long way from people even thinking about an actual implementation.

Rust currently uses semver for versioning and for did not yet implement any breaking change and is still on version 1.x . So the language is very stable and already used in multiple projects. 
There are some plans to implement "epochs" (although the naming may change). It is a new type of versioning of Rust. "epochs" in Rust would allow language to introduce breaking changes with warning periods, which would allow the language to ditch old, obsolete solutions, but also create a way for a transition to new, better version of Rust. If you ever used python, you've probably heard about problems when moving from python 2 to python 3. Breaking changes are good, but language need a transition system to avoid breakage of ecosystem (like the one that happened to python).

Even with epochs, the Rust team is going to ensure that Rust is stable, and every breaking change is going to be well discussed and have very long transition period.

# What is better in C#?

Rust is still a young language and still needs some improvement. Personally, I would probably use Rust more, if it had a great GUI library. There are some GUI solutions currently, but nothing really outstanding. We need something like Electron but with Rust.

Rust may be also more difficult when creating a game. Rust is great for CLI apps. But it is so much easier to work with Unity3D than creating a game in any Rust game engine library. Mainly because of the Editor.
Rust is generally more difficult. You can do all kinds of magic, but some concepts like lifetimes may be confusing at the beginning.

Compile times are long in Rust. You can avoid some waiting with "incremental compilation" which is just partial compilation of parts which changed. But it is still slower than C#.
But I do believe Rust will become a mainstream language some day. The community is great and idea behind it is promising. 

Rust made me enjoy programming again.