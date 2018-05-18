# Option

Rust does not have `null`. Instead it uses `Option` wrapper to express the "lack of value".

`NullReferenceException` is the most common runtime error in C#, so there are already some plans for solving this problem in future C# versions (non nullable reference types). Many modern languages, like Kotlin or Swift, are using `Option` instead of nullable reference types. It is so much easier to work with options than nulls.

Let's say we have a function in C#:

```csharp
void DoSomething(SomeClass someClass)
{
    var x = someClass.GetX();
}
```

If we accidentally pass null into the method, we get a runtime error (when calling `.GetX()`). 

In Rust you would write it like this:

```rust
fn do_something(some_struct: &SomeStruct)
{
    let x = some_struct.get_x();
}
```
and calling `.get_x()` would be safe here. `some_struct` is a "reference to a struct" and is guaranteed to have a value. Rust does not have classes, it uses structs instead. Just like in C#, struct cannot be null. And the reference (`&`) cannot be null too. 

How would we express "lack of value" with `Option` then?

```rust
let content : Option<String> = read_file("text.txt");
```
*Disclaimer: There is no `read_file` function in standard library, I use simplifications in all examples to keep it simple.*

`read_file` function can fail - for example, there is no file to read. But in this case, it will return result wrapped in `Option` - if everything went without problems, we would get `Some(text)` as a result. If something went bad, we would get `None`.

So why `Option`? Why not just use plain nullable type?

Option is a wrapper so it does not allow directly access wrapped value inside it. You cannot use `split()` function on `Option<String>`. You need to unwrap first explicitly, to access the value. It is like accessing `int?` with `.Value` in C#. This way, Rust guarantees that there is no exception unhandled. This prevents from trying to access something that does not exists.

We can use `unwrap()` function to get the value inside, but it is not recommended. It is dirty and quick solution for prototyping. 

```rust
let content : Option<String> = read_file("text.txt");
let content_unwrapped : String = content.unwrap();
```

`unwrap` tries to take value from the `Option` but panics (crash) if there is no value (`None` value). The error is not very helpful. There is no explicit error message. Default error doesn't even point you to the place where it happened.

`expect` would be better in this case. It allows you to provide custom error message.

```rust
let content : Option<String> = read_file("text.txt");
let content_unwrapped : String = content.expect("failed to read file!");
```

The short version would be:

```rust
let content = read_file("text.txt").expect("failed to read file!");
```

How would we write custom error in C#?

```csharp
string content = ReadFile("text.txt");
if(string.IsNullOrEmpty(content))
    throw new System.Exception("failed to read the file!");
```
Much more verbose and bulky. Rust code is more compact and elegant.

**TLDR**: `Option` is better than `null`. No runtime null exceptions, every possible "lack of value" must be explicitly handled. Writing custom exceptions is much easier and more compact.

# Result & "?" syntax

Crashing an app is not always a good way to handle an error.

So what would you do to avoid crashes in C#? You would probably write null checks in every possible place or wrap things in `try/catch`. Error handling in C# is difficult.

To solve this problem, Rust have something called `Result`.

`Result` is like an `Option`, but contains information (error), why some action failed.
Let's see some example code:

```rust
let content : Result<String, Error> = read_file("text.txt");
```
This looks just like option but with some small addition. `read_file` can five us file or give us error struct.

Let's say we want to have a function which reads second line of a file. We want to handle all errors. What possible "bad things" can happen? File may not exist. Also it may not have second line (it may be one-line file). We could try something like this:

```rust
fn get_second_line() -> Result<String,MyError>
{
    let file_result : Result<String, Error> = read_file("some_text.txt");
    let file : String;
    //check if reading file succeeded
    if file_result.is_ok() 
    {
        //safe unwrap because we checked!
        file = file_result.unwrap(); 
    }
    else
    {
        return MyError::new("failed to get file");
    }
    let lines = file.lines();
    let first_line_option = lines.next(); 
    let second_line_option = lines.next();
    let second_line : String;
    //checking if value is there
    if second_line_option.is_ok() 
    {
        second_line = second_line_option.unwrap();
    }
    else
    {
        return MyError::new("failed to read second line");
    }
    Ok(second_line)
    /*
    Ok is creating a Result wrapper. 
    We cannot just return string because in function signature we stated,
    that we will return value in a Result wrapper
    */
}

```

This is a long function. But we can use `?` syntax to make this simpler. `?` is just a safe unwrapping with early return. 

```rust
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

**TLDR**:  Special `?` syntax and `Result` helps explicitly handle all errors and keep code clean and short. Handling errors in Rust is very easy.

# Failure & language modularity

How would we implement Error with standard library? Let's see the example from the [Rust book](https://doc.rust-lang.org/std/error/trait.Error.html):
```rust

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
This is kind of funny, but the implementation is way too long. Is there a better way to do this?
There is. It is called `chain_error` crate. The description on github is "Error boilerplate for Rust". Let's see if it's really improving the situation. This is the example from documentation:

```rust
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
            description("unknown toolchain version"),
            display("unknown toolchain version: '{}'", v),
        }
    }
}
```
There is lot of stuff going on. It is not just simple implementing error. Still, if you look closely, implementing new errors is just 3 lines of code per error. It is shorter than implementing everything with only stardard library. But can we do better?

There is a new crate, called `failure`. Without much theory, let's see an example:

```rust
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

```xml
<packageSources>
    <add key="nuget.org" value="https://api.nuget.org/v3/index.json" protocolVersion="3" />
    <add key="Contoso" value="https://contoso.com/packages/" />
    <add key="Test Source" value="c:\packages" />
</packageSources>
```

Let's compare this to Rust's config file:

```toml
[package]
name = "blog"
version = "0.1.0"
authors = ["Jaroslaw Weber <jaroslaw.weber@gmail.com>"]

[dependencies]
slime = "0.4.0"
toml = "0.4.5"
easy_fs = {git = "https://github.com/jaroslaw-weber/easy_fs"} 
```

So easy and clean! Even someone who doesn't even know TOML (language which is used for config files) can read and edit it. One line per dependency (more if configuration is more complicated)

**TLDR**: Simpler config files.

# Immutability by default

When declaring a variable in Rust, the variable is immutable by default. If we would try do this:

```rust
let v = Vec::new();
v.push(1);
```

the compiler would scream and code wouldn't compile. We need to explicitly say:

```rust
let mut v = Vec::new();
v.push(1);
```

Explicit mutability makes code simpler. We produce more "pure functions" (no "side effects"). Pure functions are often easier to debug and reason about. F# is also a language with immutability by default.

# Macros

Have you ever tried generate code with C#? You can do it with T4 templates or try manually generating files. If you ever make mistake in the template or generating script and generate it, you will need some time to undo it (probably reseting the changes with git command). Also you need to actually generate the code to check if template is valid. Not very pleasant experience.

What else could you do? You could use reflection and make your implementation 3 times slower. Not a great solution also.

Macros are just a way to generate code without creating new files.
If you ever tried C++ macros you may think macros are evil. Overusing macros is bad practice, but no macros creates lot of boilerplate code.

Rust macros are safer. Basically there are 2 types of macros: function-like and attribute-like.
Macros enables you to statically check the correctness. Compiler won't compile your code if something is wrong with macro. You are also not creating any new files, and can insert macros almost anywhere. This prevents you from creating too many files when you are doing a lot of code generation, and let keep your project clean and simple. 

**TLDR**: Less boilerplate, better performance than reflection.

# Full control over memory

Rust does not have garbage collector. GC has a runtime overhead. 

If you want to have faster implementation of something in C# you would have to write it in another language and import as `.dll` file. Rust's speed is similar to C++ and C. But wouldn't manual control over memory be more dangerous and difficult to write? Not exactly. Rust have something called "lifetimes". Rust automatically release resources from the memory without explicitly calling deallocation functions. It is different concept than "reference counting" (and smart pointers) which you may know from other languages, like Swift or C++. If you heard about term RAII, you will be probably familiar with the topic.

```rust
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

```rust
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

```rust
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

Functions are more readable because of the snake case. `ThisIsVeryLongFunctionName` would be `this_is_very_long_function_name` in Rust. 

Rust is very minimalistic in all aspects, hiding unnecessary complexity, but also being explicit about important things.
```csharp
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
```rust
fn main ()
{
    println!("hello world!");
}
```
in Rust.

The other nice thing about Rust's syntax is that `return` keyword is not necessary:
```rust
fn double(x:i32)-> i32
{
    x*2
}
```

Rust is using `?` for unwrapping `Result`. This makes writing error handling much easier than in C#.

I like how Rust changed classic type aliases.
`int` became `i32`, `float` is `f32` and `ulong` is `u64`. The syntax is shorter and has all the information needed.

**TLDR**: Nicer syntax (in my opinion)

# Compiler errors

Compiler errors are very informative in Rust. If you write this:
```rust
fn main() {
       let v = Vec::new();
       v.push(1);
}
```
you will get this error:

```rust
error[E0596]: cannot borrow immutable local variable `v` as mutable
 --> src/main.rs:3:8
  |
2 |        let v = Vec::new();
  |            - consider changing this to `mut v`
3 |        v.push(1);
  |        ^ cannot borrow mutably
```

This error is showing not only what happened, but also where it happened (with nice little arrow). It also shows you a context (part of code) and very often a solution! "consider changing this to `mut v`" is explaining how to fix the code!

```rust
fn main() {
       let mut v = Vec::new();
       v.push(1);
}
```

We listened to the compiler and now it is working!

**TLDR**: Nice errors, easier to debug and fix.

# rustfmt

Have you ever argued with coworker on syntax? Have you ever have your PR rejected cause you didn't add a space before "="? Rust have "official guidelines" for formatting. Almost all the code wrote in rust have same syntax. Everyone uses autoformating with same setup. Even things like snake case or camel case are giving warnings on compilation.

Some people argue that formatting should not be defined by someone, and the programmer should decide how to structure his code. Still, I was really happy that I don't need to focus on formatting, setting up my IDE, fix my PRs and could focus on coding.

# Multiplatform support

C# is a Microsoft's baby. It is kind of multiplatform, but everything started on Windows. So there are some things you can do only on Windows.

I tried removing all unused usings from the project. I would need to pay for Resharper and use Windows to do that. 

I tried to install docfx on mac. I couldn't because there was a bug "linux and mac only".

I often hit the "not windows, sorry" wall, those two examples are from last month only.
The tooling in Rust is open source and multiplatform from the very beginning.

# What is better in C#?

Rust is still a young language and still needs some improvement. Personally, I would probably use Rust more, if it had a great GUI library. There are some GUI solutions currently, but nothing really outstanding. We need something like Electron, but with Rust.

Rust may be also more difficult when creating a game. Rust is great for CLI apps. But it is so much easier to work with Unity3D than creating a game in any Rust game engine library. Mainly because of the Editor.

Rust is generally more difficult. You can do all kinds of magic, but some concepts like lifetimes may be confusing at the beginning.

Compile times are long in Rust. You can avoid some waiting with "incremental compilation" which is just partial compilation of only those parts which has changed since last compilation. Still, it is slower than C#.

But I do believe Rust will become a mainstream language some day. The community is great and it is a great modern language with lot of useful and innovative features.