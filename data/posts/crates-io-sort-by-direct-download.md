# Problem

I looked at top 30 of "All-time Downloads" and "Recently Downloads" crates.
I found lot of crates that I have never used, heard of and probably won't use. 
Examples: `thread_local`, `aho-corasick`, `num-traits`, `memchr`

# What do I propose

Implement new download counter, counting only "direct dependencies downloads" and allow to sort using it.

# Details

The name of the sort is open to a debate. The main idea is this:
Each time a library downloads a crate, it would only count a download if downloaded crate was mentioned in `cargo.toml` file as a direct dependency. All crates dependent on direct dependencies would not count as a download. For example:

```
[dependencies] 
regex = "*"
```

Downloading `regex` crates also downloads dependencies of `regex`.  After downloading `regex` crate, only `regex` would count as a "direct download". `regex` dependencies download count would not change.

# Why?

This type of sort would really indicate what crates are most popular. This type of sort would help to increase popular high-level crates discoveries. It would move `tokio`, `Rocket` or other similar crates more to the top and decrease the noise of crates most people probably won't use.

# Problems with implementation

This would probably require some additional implementation in the cargo's code.