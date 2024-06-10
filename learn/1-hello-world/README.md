# Hello World!

Let's get started with Rust by creating a simple *Hello World!* program with Cargo.

> If you are not using the development container, make sure you have Rust and Cargo installed by following the instructions [here](https://www.rust-lang.org/tools/install).

Navigate to the `learn/1-hello-world` directory and run the following command to create a new Rust project:

```bash
cargo new hello-world
```

This will create a new directory called `hello-world` with the following files:

```bash
hello-world
├── Cargo.toml
└── src
    └── main.rs
```

> You may see a warning that the `hello-world` project is not included in the workspace. Add `"hello-world"` update the members in the `Cargo.toml` file located in the root of the project to fix this.
>
> ```toml
> [workspace]
> members = [
>     "hello-world" # Add this
>     ...
> ]

The `Cargo.toml` file is the manifest file for Rust projects. It contains all the metadata for the project, as well as the dependencies.

The `src` directory contains the source code of the project. The `main.rs` file is the entry point of the program. Later, we will see how to structure/create libraries and other binaries.

Open the `hello-world` directory in VSCode and take a look at the `main.rs` file. It should look something like this:

```rust
fn main() {
    println!("Hello, world!");
}
```

From the terminal, you can run the following command to build and run the program:

```bash
cargo run
```

## The Compiler is your Friend

Rust has a powerful compiler that helps you catch errors at compile time. Rather than a more traditional "edit-compile-run" cycle, Rust's compiler is designed to help you write correct code the first time. This may seem frustrating at first, but it's a powerful tool that helps you write more reliable code.

Compiler feedback is rich and informative, providing suggestions and explanations for errors. In other languages, you might get runtime errors or even worse, no errors at all. Rust's compiler helps you catch these errors early.

For example, in JavaScript the following code would run without errors:

```javascript
let spam = ['cat', 'dog', 'mouse']
console.log(spam[6])

// no output, no errors, spam[6] is `undefined`
```

...or in Python, the following code would produce a *runtime* error:

```python
# example.py
spam = ['cat', 'dog', 'mouse']
print(spam[6])
```

```python
$ python example.py

Traceback (most recent call last):
  File "segfaults.py", line 2, in <module>
    print(spam[6])
IndexError: list index out of range
```

In Rust, the following code would produce a *compile-time* error with detailed information about the problem (and in many cases, suggestions on how to fix it):

```rust
fn main() {
    let animals = ["cat", "dog", "mouse"];
    println!("{}", animals[42]);
}
```

```bash
$ cargo build

error: this operation will panic at runtime
 --> main.rs:3:15
  |
3 |     println!("{}", animals[42]);
  |                    ^^^^^^^^^^^ index out of bounds:
        the length is 3 but the index is 42
```

The Rust compiler provides:

- The error itself and where it occurred
- What the value should be, no more than 3
- What the value actually was, 6

Lets go deeper...

Suppose you would like to print out a number for debugging purposes, but forget that, like in C, numbers need a format string to be printed. The following code would produce a *compile-time* error:

```rust
println!(42);
```

```bash
$ cargo build

error: format argument must be a string literal
  --> src/main.rs:13:22
   |
13 |             println!(n);
   |                      ^
   |
help: you might be missing a string literal to format with
   |
13 |             println!("{}", n);
   |                      +++++
```

This same form of compiler help is available throughout the entire Rust ecosystem. The standard library and even third-party libraries and crates all provide detailed error messages to help you write code.
