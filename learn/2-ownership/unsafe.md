# Manual Memory Management

Previously, we saw that Rust manages memory automatically to provide safety. But sometimes, particularly in low-level code, you might want or need to manage memory manually.

Rust provides a way to do this with the `unsafe` keyword. The `unsafe` keyword allows you to write code that the Rust compiler can't verify is safe. This is useful when you need to interact with code that doesn't follow Rust's safety rules, such as C libraries or hardware interfaces.

When using the `unsafe` keyword, you can do things that are normally disallowed by the Rust compiler, such as:

- [Dereferencing raw pointers](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer)
- [Calling unsafe functions or methods](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#calling-an-unsafe-function-or-method)
- [Accessing or modifying static mutable variables](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#accessing-or-modifying-a-mutable-static-variable)
- [Implementing unsafe traits](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#implementing-an-unsafe-trait)
- [Accessing fields of `union`s](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#accessing-fields-of-a-union)

It's important to understand that `unsafe` doesn't turn off the borrow checker or disable any other of Rust's safety checks. The `unsafe` keyword only gives you access to these five features that are then not checked by the compiler for memory safety.

Despite its name, `unsafe` does not mean the code inside the block is necessarily dangerous or that it will definitely have memory safety problems. As the programmer, you'll ensure the code inside an unsafe block will access memory in a valid way.

By requiring these five unsafe operations to be inside unsafe blocks, memory errors can be localized and minimized. By keeping unsafe blocks small and isolated, it is easier to debug errors when they occur.

<!-- TODO: helpful compiler/linting for unsafe https://youtu.be/8j_FbjiowvE?si=Nfq5Z4nJSnoNMCy0 -->

## Interacting with unsafe code

One of the most fundamental reasons to use `unsafe` is to deal with Rust's raw pointer types `*const T` and `*mut T`. These are analogous to `&T` and `&mut T` references, except that they do not have lifetimes and are not subject to the same validity rules as references.

<!-- TODO -->

### Abstracting unsafe code

Just because a function contains unsafe code doesn't mean it also needs to be marked as unsafe. For example, the standard library contains many functions that use unsafe code internally, but are safe to call because they have been carefully written to ensure that the unsafe code is contained and doesn't leak out.

<!-- TODO -->

### FFI (Foreign Function Interface)

Sometimes, your Rust code might need to interact with code written in another language. For this, Rust has the `extern` keyword that facilitates the creation and use of a *Foreign Function Interface (FFI)*. An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.

<!-- TODO -->

### Using `extern` functions to call external code

<!-- TODO -->
