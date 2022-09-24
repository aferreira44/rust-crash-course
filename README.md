# Rust Crash Course

Rust Crash Course - https://www.youtube.com/watch?v=zF34dRivLOw

## What is Rust?

https://www.rust-lang.org/

## Why Rust?

- Systems programming
- WebAssembly (WASM)
- Don't have garbage collector
- Performance
- Memory safety

## How to install Rust?

https://www.rust-lang.org/tools/install

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Rustup

`rustup --version`
`rustup update`

### Rust Compiler

`rustc --version`

### Compile a file

`rustc hello.rs`

### Cargo Package Manager

`cargo --version`

### Create a new project

`cargo init`

`cargo run`

Executable created: rust-crash-course/target/debug/rust-crash-course

`cargo clean`

`cargo build`

## Build for production

`cargo build --release`



