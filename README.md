# 🦀 Learn Rust (Fe<sub>2</sub>O<sub>3</sub>)

Learn Rust and become fully *oxidized*!

This is the starting point for the Learn Innovate Tinker (LIT) project to learn Rust and create your very own cipher decoder. This repository contains a collection of Rust code samples and exercises to help you learn the language and get started.

1. [Getting started](#getting-started)
1. [Learn](#learn)
1. [Innovate](#innovate)
1. [Tinker](#tinker)
1. [Resources](#resources)

## Getting started

Create a fork of this repository and clone it locally:

```bash
git clone https://github.com/<your-username>/Fe2O3.git
```

The easiest way to get started is by using the Visual Studio Code Remote - Containers / Codespaces [development container](.devcontainer/devcontainer.json) included in this repository. This container comes with Rust, Cargo, and several VSCode extensions pre-installed.

- For [Remote - Containers](https://aka.ms/vscode-remote/download/containers), use the **Remote-Containers: Open Repository in Container...** command which creates a Docker volume for better disk I/O on macOS and Windows.
- For Codespaces, install the [GitHub Codespaces](https://marketplace.visualstudio.com/items?itemName=GitHub.codespaces) extension in VSCode, and use the **Codespaces: Create New Codespace command**.

Once your workspace is setup, open a terminal to check everything is working:

```bash
cargo --version
```

```bash
rustc --version
```

## 📖 Learn

Each exercise is a Rust project in the [learn](./learn) folder. Each project contains a `README.md` file with information, instructions, and additional resources.

0. [What is Rust?](./learn/0-what-is-rust/README.md)
1. [Hello World!](./learn/1-hello-world/README.md)
1. [Ownership & Borrowing](./learn/2-ownership/README.md)
1. [The Type System](./learn/3-type-system/README.md)
1. [Traits](./learn/4-traits/README.md)
1. [Error Handling](./learn/5-error-handling/README.md)

<!--

**Advanced (optional)**

6. [Concurrency](./learn/6-concurrency/README.md)
1. [Unsafe Rust & FFI](./learn/7-unsafe-rust/README.md)
1. [Macros](./learn/8-macros/README.md)
1. [Testing](./learn/9-testing/README.md)

-->

## 💡 Innovate

Now that you know the basics, it's time to innovate! The [innovate](./innovate) folder contains a series of lessons to help you build the skeleton for your decoder. The project is setup with the following crates:

- [clap](https://crates.io/crates/clap) for command line argument parsing
- [thiserror](https://crates.io/crates/thiserror) for error handling
- [serde](https://crates.io/crates/serde) for serialization and deserialization
- [miette](https://crates.io/crates/miette) for pretty error reporting

<!-- TODO: how to make this a step-by-step set of instructions so that people can bulid up the skeleton code themselves ? -->

Feel free to add any other [crates](https://crates.io/) you need to customize your project!

## 🔧 Tinker

Run wild and implement your decoders!

For convenience, a complete skeleton is provided for you directly implement your algorithms.

<!-- To run the decoder, use the following command:

```bash
cargo run --bin decode -- <input-file> <output-file>
``` -->

## Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Talks - _NoBoilerplate on Youtube_](https://youtube.com/playlist?list=PLZaoyhMXgBzoM9bfb5pyUOT3zjnaDdSEP&si=E5Ps7IYXtTLxKLP1)
