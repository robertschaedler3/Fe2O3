# Variable immutability and shadowing

One of the unique features of Rust is that variables are **immutable** by default. The `mut` keyword can be used to define a **mutable** variable.

Consider the following code snippet (notice the use of `_` before the variable name to avoid the compiler warning about unused variables):

```rust
fn main() {
    let a = 42;
    a = 43;  // Will not compile
}
```
Create a new package called `variables` by executing `cargo new variables` under this folder, open `main.rs` in VSCode, and replace the default content with the above snippet.

Try to execute the program using `cargo run` from the terminal and note that the compiler will not allow the program to compile until `let a = 42` is changed to `let mut a = 42` as shown below.

```rust
fn main() {
    let mut a = 42;
    a = 43;  // OK
}
```
Another unique feature of Rust is variable shadowing, i.e., it is possible to redefine a variable with the same name. Note that this creates a *new* variable that isn't associated with the original.

```rust
fn main() {
    let a = 42;
    let a = 43;  // OK; defines a new immutable variable
    // a = 42;   // This won't compile as before
}
```
Variable shadowing is permitted both within the same scope as well as a nested scope. The compiler automatically tracks the shadowing.

```rust
fn main() {
    let a = 42; // Immutable variable
    {
        let mut a = 43; // Different mutable variable inside a new scope
        a = 42; // Ok within this scope
    }
    // a = 42;   // This won't compile as before
}
```

The key takeway is that immutability is a core concept in Rust and everything is *immutable* by default unless it's explicitly declared as mutable. This might appear strange since it's the exact inverse of many other languages, but the increased predictability about side effects of function calls and other operations on variables actually makes it easier to reason about code.
