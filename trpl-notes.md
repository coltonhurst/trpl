# TRPL Notes

My notes from [The Rust Programming Language](https://doc.rust-lang.org/stable/book/title-page.html). I have attempted to group these notes logically rather than the sequential order they appear in the book.

## General Commands

* To install Rust: `curl https://sh.rustup.rs -sSf | sh`
* To update Rust: `rustup update`
* To uninstall Rust: `rustup self uninstall`
* To check the current version of Rust you're using: `rustc --version`
* Add Rust to the PATH `set PATH ~/.cargo/bin $PATH` (if using [Fish Shell](https://fishshell.com/), open or make your config file at `~/.config/fish/config.fish`)
* View Rust docs locally: `rustup doc`
* Compile a Rust file with `rustc filename.rs`
* You can format your code with `rustfmt filename.rs`

## Cargo

### Commands

* Get the current Cargo version: `cargo --version`
* Create a new project with Cargo: `cargo new project_name`
* Change which VCS cargo uses: `cargo new --vcs=git`
* See help with `cargo new`: `cargo new --help`
* Build a project with `cargo build`
* Build & run a project with `cargo run`
* Build a project for release with: `cargo build --release`
* Check your code to make sure it compiles, without creating the executable (faster than the whole build process): `cargo check`

### Notes

`Cargo.toml` is Cargo's configuration file for your project (written in [TOML](https://github.com/toml-lang/toml)).

## Macros

Rust macros have an exclamation point `!`. In the example below, `println!` is a macro.

```rust
fn main() {
    println!("Hello, world!");
}
```

## Variables & References

### Notes

Variables are immutable by default. To make a mutable variable, add the reserved word `mut`. Example: `let mut foo = 5;`

References are immutable by default.

## Functions

### Notes

#### Associated Functions

`::` signifies a function is an *associated function*. Example:

```rust
let mut guess = String::new();
```

In the above example, `::new` indicates the `new` is an *associated function* of the `String` type. This is implemented on the type `String`, rather than on an instance of the `String` type (also known as a *static method* in other languages).

## Enumerations

### Notes

An Enumeration (or "enum") is a "type that can have a fixed set of values, and those values are called the enum’s *variants*."