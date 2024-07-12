# Getting Started

This application is written in Rust. It uses several libraries/frameworks standard in the Rust ecosystem for web backends. Before you can build and run this application, you'll need to install a Rust toolchain using `rustup`. Download and follow the relevant steps at [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started) to install a `stable` toolchain. Installation methods vary by OS. On Windows, `rustup` will additionally install an MSVC C++ toolchain if it is not already present on your machine. This application was built and tested using Rust stable 1.79.0. Note that `rustup` can be finicky with Docker, so I don't recommend using Docker with these steps.

You'll need `cargo` and the Rust stdlib installed - `rustup` should do this by default. You'll also need an internet connection and access to [https://crates.io/](https://crates.io/) for `cargo` to be able to download dependencies.

# Build and Run

To build the application, run `cargo build` in this directory. This may take a while because `cargo` has to download and build all the dependencies. It should look something like this.
```
$ cargo build
   Compiling cfg-if v1.0.0
   Compiling proc-macro2 v1.0.86
   Compiling unicode-ident v1.0.12
   Compiling pin-project-lite v0.2.14
   Compiling once_cell v1.19.0
   / ... many more snipped ... /
   Compiling serve-ex v0.1.0 (C:\------\serve-ex)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 59.28s
```

To run the application, run `cargo run` in this directory. You should see this.
```
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target\debug\serve-ex.exe`
```
You can now make requests to the server at [http://localhost:8080](http://localhost:8080). To terminate the server, interrupt the process using Ctrl+C in the terminal.