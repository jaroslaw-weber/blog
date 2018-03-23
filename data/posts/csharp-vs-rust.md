# Option

Rust uses `Option` to elimnate `NullReferenceException`. Rust does not have `null`. Instead, you can express "lack of value" with an `Option` wrapper. It is so much easier to work with options than nulls.
Let's say we have a function in C#:

```
void DoSomething(SomeClass someClass)
{
    var x = someClass.GetX();
}
```

If we accidentally pass null into the method, we get a runtime error. When debugging, you need to find where this error happened and think why it happened. In Rust you can write:

```
fn do_something(some_struct: &SomeStruct)
{
    let x = some_struct.get_x();
}
```
We can guarantee that this won't crash. Rust does not have classes. It uses structs instead. Just like in C#, struct cannot be null. How would we express "lack of value" with Option then?

```
let content : Option<String> = read_file("text.txt");
```
*Disclaimer: There is no `read_file` function, I use simplifications in all examples to keep the concept simple.*

`read_file` function can fail - for example, there is no file to read. But in this case, it will return result wrapped in option - if everything went without problems, we would get `Some(content)` as a result. If something went bad, we would get `None`.

So why `Option`? Why not just use plain nullable type?

Option is a wrapper so it does not allow directly access wrapped value inside it. You cannot try use `split` on `Option<String>`. You need to unwrap first. This way, Rust guarantees that there is no exception unhandled. So, to access the value, you need to unwrap it first. This prevent's from trying to access a field from something that does not exists.

We can use `unwrap()` function to get the value inside, but it is not recommended. It is dirty and quick solution for prototyping. 

```
let content : Option<String> = read_file("text.txt");
let content_unwrapped : String = content.unwrap();
```

`unwrap` tries to take value from the `Option` but panics (crash) if there is no value (`None` value). The error is not very helpful. There is no explicit error message. Default error doesn't even point you to the place where it happened.

`expect` would be better in this case. It allows you to provide custom message.

```
let content : Option<String> = read_file("text.txt");
let content_unwrapped : String = content.expect("failed to read file!")
```

How would we write custom error in C#?

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
Much more verbose and bulky. Rust code is more compact and elegant.

**TLDR**: `Option` is better than `null`. No runtime null exceptions, every possible "lack of value" must be explicitly handled. Writing custom exceptions is much easier and more compact.

# Result & "?" syntax

Crashing an app is not always a good way to handle an error.

So what would you do to avoid crashes in C#? You would probably write null checks in every possible place or wrap things in `try/catch`. Error handling in C# is horrible and very difficult.

To solve this problem, Rust have something called `Result`.

`Result` is like an `Option`, but contains information (error), why some action failed.
Let's see some example code:

```
let content : Result<String, Error> = read_file("text.txt");
```
This looks just like option but with some small addition. `read_file` can five us file or give us error struct. For non-trivial projects, it is better to create custom errors. Handling everything with only strings (like in Option example) is too simple for more complex projects.

Let's say we want to have a function which reads second line of a file. We want to handle all errors. What possible "bad things" can happen? File may not exist. Also it may not have second line (it may be one-line file). We could try something like this:

```
fn get_second_line() -> Result<String,MyError>
{
    let file_result : Result<String, Error> = read_file("some_text.txt");
    let file : String;
    if file_result.is_ok() //check if reading file succeeded
    {
        file = file_result.unwrap(); //safe unwrap because we checked!
    }
    else
    {
        return MyError::new("failed to get file");
    }
    let lines = file.lines();
    let first_line_option = lines.next(); //next means just "take next line from lines"
    let second_line_option = lines.next();
    let second_line : String;
    if second_line_option.is_ok() //checking if value is there
    {
        second_line = second_line_option.unwrap();
    }
    else
    {
        return MyError::new("failed to read second line");
    }
    Ok(second_line) //Ok is creating a Result wrapper. We cannot just return string because we said that we will return value in wrapper in the function signature
}

```
*Disclaimer: Rust promoted `match` instead of `if`, but I used it because of familiar syntax.*

This is a long function. Nobody wants to write it like this. That is why there is a `?` syntax and `try!` macro. `?` helps us write the same thing in just few lines of code.

```
fn read_second_line() -> Result<String, Error>
{
    let file: String = read_file("some_text.txt")?;
    let lines = file.lines();
    let first_line: String = lines.next()?;
    let second_line: String = lines.next()?;
    Ok(second_line)
}
```
So much shorter right? And we handled **all** the possible exceptions. This won`t crash.
This helps us handle all the exceptions explicitly and keep code short and clean.

Each `?` is just a safe unwrapping with early return. Writing this in C# would be a nightmare (like the first version I wrote)

Also, did you notice lack of `return` keyword? It can be ommited in Rust. Makes your code little shorter.

**TLDR**:  Special syntax and `Result` helps explicitly handle all errors and keep code clean and short. Handling errors in Rust is very easy.

# Why use wrappers?

It is all about the control. We, as a library user, can decide that we don't want to crash the application, even if some library (for example, parsing json) fails. Instead of crashing with exception, we can deal with it in more elegant way, and provide a flow without bulky and easy to forget `try/catch` statement. Every library in C# can crash unexpectedly. If you don't wrap the methods in `try/catch` you will have a potential crash waiting there.

Most of libraries in Rust don't crash when something goes wrong. You get a value in a `Result` wrapper. You can see how "unsafe" the library is, just by searching `unwrap`, `except` in code. Libraries returns `Result` so you are deciding what do you want to do if something goes wrong.

# Failure

How would we implement Error with standard library? Let's see example from the [Rust book](https://doc.rust-lang.org/std/error/trait.Error.html):
```

#[derive(Debug)]
struct SuperError {
    side: SuperErrorSideKick,
}

impl fmt::Display for SuperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for SuperError {
    fn description(&self) -> &str {
        "I'm the superhero of errors"
    }

    fn cause(&self) -> Option<&Error> {
        Some(&self.side)
    }
}

```
This is kind of funny, but way too long. Is there a better way to do this?
There is. It is called `chain_error` crate. The description on github is "Error boilerplate for Rust". Let`s see if it`s really improving the situation. This is the example from documentation:

```
mod other_error {
    error_chain! {}
}

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
        Another(other_error::Error, other_error::ErrorKind) #[cfg(unix)];
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
    }

    errors {
        InvalidToolchainName(t: String) {
            description("invalid toolchain name")
            display("invalid toolchain name: '{}'", t)
        }

        UnknownToolchainVersion(v: String) {
            description("unknown toolchain version"), // note the ,
            display("unknown toolchain version: '{}'", v), // trailing comma is allowed
        }
    }
}
```
There is lot of stuff going on. It is not just simple implementing error. Still, if you look closely, implementing new errors is just 3 lines of code per error. It is shorter than implementing everything by hand. But can we do better?

New crated appeared lately. It is called `failure`. Without much theory, let's see an example:

```
#[derive(Debug, Fail)]
enum ToolchainError {
    #[fail(display = "invalid toolchain name: {}", name)]
    InvalidToolchainName {
        name: String,
    },
    #[fail(display = "unknown toolchain version: {}", version)]
    UnknownToolchainVersion {
        version: String,
    }
}
```

This looks much simpler. So what was the point of presenting this 3 ways to implement error?

Why so many choices about error handling? Shouldn't errors be in the standard library? Well, one of Rust greatest strengths is flexibility. Standard library is reduced to necessary minimum. Every additional functionality needs to be imported from external library.

Still, putting error handling in external library seems odd. But error handling is also a part of a language that can be improved. If we decide to use one version of error handling we are doomed to use it forever. If we try to change it, when it is inside the standard library, we break whole language. Moving important parts to external libraries give Rust an opportunity to "iterate" through different solutions. That is how `failure` was born. First, we had only standard `Error`. But the functionality was not enough, and writing custom errors was bulky. So someone came up with `error_chain` crate for better error handling. Still, after using the `error_chain`, someone came up with better, more elegant solution which would eliminate downsides of `error_chain`.

**TLDR**: Lot of Rust's functionality is in external libraries. This solves problem of having standard library too big or not being able to depreciate old solutions for the sake of something new and better.

# Package manager

C# uses NuGet as a package manager. Let's see what is inside the config file (nuget.config):

```
<packageSources>
    <add key="nuget.org" value="https://api.nuget.org/v3/index.json" protocolVersion="3" />
    <add key="Contoso" value="https://contoso.com/packages/" />
    <add key="Test Source" value="c:\packages" />
</packageSources>
```

This is just a small part of config file.
Overly verbose and complicated. XML is useful but is it realy the best solution for config files? "npm" (javascript's package manager) is using json for config files. I think it is better, but is there something even better?

Actually there is. Rust use "Toml" for config file. What is Toml? It is a new markup language similar to YAML but more simple. Example project (yes, whole project) config file in Rust:

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

So easy and clean! Even someone who doesn't even know Toml can read and edit it. One line per dependency (more if configuration is more complicated)

**TLDR**: Simpler config files.

# Immutability by default

When declaring a variable in Rust, the variable is immutable by default. If we would try do this:
```
let v = Vec::new();
v.push(1);
```
the compiler would scream and code wouldn't compile. We need to explicitly say:
```
let mut v = Vec::new();
v.push(1);
```
to be able to mutate the variable.

Too much mutable state is bad. If something is wrong with values of some field in struct/class, we need to find out who access it. With mutability by default (in C#), we need to check everything related to the field/class that had wrong values. With lot of code it may be difficult to find. By explicitly defining mutability, we need to only check places which were able to mutate the struct. Less debugging.

**TLDR**: Easier debugging because of immutability by default.

# Macros

Have you ever tried generate code with C#? You can do it with T4 templates or try manually generating files. If you ever make mistake in the template or generating script and generate it, you will need some time to undo it (probably reseting the changes with git command). Also you need to actually generate the code to check if template is valid. Not very pleasant experience.

What else could you do? You could use reflection and make your implementation 3 times slower. Not a great solution also.

Macros are just a way to generate code without creating new files.
If you ever tried C++ macros you may think macros are evil. Overusing macros is bad practice, but no macros creates lot of boilerplate code.

Rust macros are safer. Basically there are 2 types of macros: function-like and attribute-like.
Macros enables you to statically check the correctness. Compiler won't compile your code if something is wrong with macro. You are also not creating any new files, and can insert macros almost anywhere. This prevents you from creating too many files when you are doing a lot of code generation, and let keep your project clean and simple. 

**TLDR**: Less boilerplate, better performance than reflection.

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

**TLDR**: Better performance without manually managing memory.

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

**TLDR**: Better performance than simple generics, also more functionality than interfaces. Autoderive saves lot of implementation boilerplate.

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

I like how Rust changed `int` to `i32`. In C# you would write `float` or `ulong` in C# would be `f32` or `u64` in Rust. Much shorter syntax.

**TLDR**: Nicer syntax (in my opinion)

# Safety

Rust unique features prevent lot of problems which would otherwise appear on runtime. If something compiles, it is probably much safer than same thing written in different language.
Option prevents null exceptions, lifetimes/borrowing prevents from data races.

"If it compiles, it works" is the motto of Rust. This is THE safest language (comparable to haskell) and that is why it's used for multithreading and programming reliable systems.

**TLDR**: Less bugs, more stable.

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

**TLDR**: Nice errors, easier to debug and fix.

# rustfmt

Have you ever argued with coworker on syntax? Have you ever have your PR rejected cause you didn't add a space before "="? Rust have "official guidelines" for formatting. Almost all the code wrote in rust have same syntax. Everyone uses autoformating with same setup. Even things like snake case or camel case are giving warnings on compilation.

# Multiplatform support

C# is a Microsoft's baby. It is kind of multiplatform, but everything started on Windows. So there are some things you can do only on Windows.

I tried removing all unused usings from the project. I would need to pay for Resharper and use Windows to do that. 

I tried to install docfx on mac. I couldn't because there was a bug "linux and mac only".

I often hit the "not windows, sorry" wall, those two examples are from last month only.
The tooling in Rust is open source and multiplatform from the very beginning.

# Future of Rust and C#

Rust version iterations are much faster than C#. We are currently on 1.24 version (semver). For the next version of C# you would need to wait months. The experimental channel (nightly) is very accessible - you just type a command to switch between different versions of the compiler.

Ever heard of someone using "experimental features" in C# project? Rust's "nightly" allows you to enable only those features you really need. You need to declare in code "I will use this feature" to use it. Otherwise it is very similar to the stable compiler and you can clearly see what parts of your code may be unstable (by explicitly stating what features will you use).

C# is going to probably implement a lot of features that Rust already have.  There are already propositions for non-nullable references as default (which would reduce null exceptions on runtime, probably going to be released in 8.0 version of the language). I also saw macros proposition but I think it is a long way from people even thinking about an actual implementation.

Rust currently uses semver for versioning and for did not yet implement any breaking change and is still on version 1.x . So the language is very stable and already used in multiple projects. 

There are some plans to implement "editions". It is a new type of versioning of Rust. "editions" in Rust would allow language to introduce breaking changes with very long warning periods, which would allow the language to ditch old, obsolete solutions, but also create a way for a transition to new, better version of Rust. 
If you ever used python, you've probably heard about problems when moving from python 2 to python 3. Breaking changes are good, but language need a transition system to avoid breakage of ecosystem (like the one that happened to python).
Each transition would be easy because of the warnign periods, so compiler would help you upgrade your code (something like autoupdating api in Unity3d).

**TLDR**: Lot of new features without breaking compability and losing stability. Also ability to introduce breaking changes in language with smooth transition to new version.

# What is better in C#?

Rust is still a young language and still needs some improvement. Personally, I would probably use Rust more, if it had a great GUI library. There are some GUI solutions currently, but nothing really outstanding. We need something like Electron but with Rust.

Rust may be also more difficult when creating a game. Rust is great for CLI apps. But it is so much easier to work with Unity3D than creating a game in any Rust game engine library. Mainly because of the Editor.
Rust is generally more difficult. You can do all kinds of magic, but some concepts like lifetimes may be confusing at the beginning.

Compile times are long in Rust. You can avoid some waiting with "incremental compilation" which is just partial compilation of parts which changed. But it is still slower than C#.

But I do believe Rust will become a mainstream language some day. The community is great and idea behind it is promising. 

Rust made me enjoy programming again.