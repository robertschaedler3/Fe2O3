# What is Rust?

Rust is a systems programming language designed for performance, safety, and concurrency that aims to provide a balance between low-level control and high-level, *zero-cost* abstractions.

*A foundational goal of Rust is to ensure that your programs never have undefined behavior.* That is the meaning of "safety". Undefined behavior is especially dangerous for low-level programs with direct access to memory. The Rust compiler guarantees memory safety without a garbage collector or additional runtime cost.

> ~70% of the vulnerabilities Microsoft assigns a CVE each year continue to be memory safety issues.
>
> [We need a safer systems programming language](https://msrc.microsoft.com/blog/2019/07/we-need-a-safer-systems-programming-language/)

A secondary goal of Rust is to detect and prevent undefined behavior at compile-time. By moving the detection of bugs earlier in the development process, Rust aims to improve the quality of software by preventing bugs from reaching production.

> *Zero Cost Abstractions* - the ability to move certain behaviors to compile time execution or analysis. Rust's high-level abstractions don’t incur runtime overhead.

Rust achieves these goals through its ownership system - a set of rules that the compiler checks and enforces. The ownership system is key to understanding and programming in Rust. It is a discipline for managing memory that ensures memory is used correctly and safely. Later we will examine how the borrow checker enforces these rules and how lifetimes ensure references are valid throughout their usage.

Rust has a rich type system that allows for expressive and concise code. It is syntactically is similar to C++, but with modern features like [pattern matching](https://doc.rust-lang.org/book/ch06-00-enums.html), [traits](https://doc.rust-lang.org/book/ch10-02-traits.html), and [generics](https://doc.rust-lang.org/book/ch10-01-syntax.html).

Similarly, handling concurrent programming safely and efficiently is another of Rust's major goals. The ownership model and string type system help manage memory safety and concurrency problems!

In summary, Rust combines low-level control with high-level abstractions, making it an excellent choice for embedded systems programming all the way up to web development and beyond.

## Cargo

Rust comes with a package manager and build tool called Cargo. Cargo simplifies the process of building, testing, and managing Rust projects. It handles dependencies, compiles code, and manages project configuration.

Cargo is a powerful tool that streamlines the development process and ensures consistent project structure. It is an essential part of the Rust ecosystem and is used by developers to create, build, and share Rust projects.

> Rust libraries are called *crates*. Cargo downloads and compiles crates from the [crates.io](https://crates.io/) registry by default. You can also specify dependencies from other sources, such as internal mirrors or private repositories.

## Tooling

Rust has a rich ecosystem of tools that enhance the development experience. The rustc compiler produces efficient machine code, and the rustdoc tool generates documentation from source code comments. The Rust Language Server (RLS) provides IDE support for code completion, refactoring, and error checking.

[rustup](https://www.rust-lang.org/tools/install) is the easiest way to install and manage Rust and its developer tools:

- [rustc](https://doc.rust-lang.org/rustc/what-is-rustc.html) - the Rust compiler
- [cargo](https://doc.rust-lang.org/cargo/) - the package manager and build tool
- [rustdoc](https://doc.rust-lang.org/stable/rustdoc/) - the documentation generator
- [rustfmt](https://github.com/rust-lang/rustfmt) - the code formatter. It formats Rust code according to the official style guidelines.
- [clippy](https://doc.rust-lang.org/nightly/clippy/) - a collection of lints to catch common mistakes and improve code quality.

> The [Rust Playground](https://play.rust-lang.org/) is an online tool for experimenting with Rust code without installing anything locally.

Other tools like [Rust Analyzer](https://rust-analyzer.github.io/) provide advanced IDE support for Rust development. All of these tools are configured and available in the official [Rust devcontianer](https://github.com/microsoft/vscode-dev-containers/blob/main/containers/rust/README.md).

## Testing

<!-- TODO -->

## Documentation

Rust has excellent documentation that covers the language, standard library, and ecosystem. The official [Rust Book](https://doc.rust-lang.org/book/) is a comprehensive guide to learning Rust from scratch. The [Rust Reference](https://doc.rust-lang.org/reference/) provides detailed information about the language syntax and semantics.

Rust's documentation is generated from source code comments using the `rustdoc` tool. Developers can write detailed explanations and examples using Markdown syntax in their code. Crate documentation is published on [crates.io](https://crates.io/) and can be viewed online or offline using `cargo doc`.

---

### Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Playground](https://play.rust-lang.org/)
