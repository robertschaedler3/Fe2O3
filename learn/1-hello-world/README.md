# Hello World!

Let's get started with Rust by creating a simple "Hello World!" program with Cargo.

> If you are not using the development container, make sure you have Rust and Cargo installed by following the instructions [here](https://www.rust-lang.org/tools/install).

Navigate to the `learn/1-hello-world` directory and run the following command to create a new Rust project:

```bash
cargo new hello-world
```

> You may see a warning that the `hello-world` project is included in the workspace. Uncomment the `"hello-world"` line in the root `Cargo.toml` file to fix this.

This will create a new directory called `hello-world` with the following files:

```bash
hello-world
├── Cargo.toml
└── src
    └── main.rs
```

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
