# Data Types

Rust has several built-in types that are similar to their analogues in other languages.

|  **Description**  |            **Type**            |          **Example**          |
|:-----------------:|:------------------------------:|:-----------------------------:|
| Signed integers   | i8, i16, i32, i64, i128, isize | -1, 42, 1_00_000, 1_00_000i64 |
| Unsigned integers | u8, u16, u32, u64, u128, usize | 0, 42, 42u32, 42u64           |
| Floating point    | f32, f64                       | 0.0, 0.42                     |
| Unicode           | char                           | 'a', '$'                      |
| Boolean           | bool                           | true, false                   |

Notice how Rust permits arbitrarily use of `_` between numbers for ease of reading.

Rust uses the `let` keyword to assign values to variables. The type of the variable can be optionally specified after a `:` or using a suffix (like `u32`, `u64` etc). If the type isn't explicitly defined, it can often be can be implicitly inteferred by the compiler using `type inference` as shown in the code snippet below.

```bash
cargo new types
```

Open the `types` folder in VSCode and replace the contents of `main.rs` with the following. Notice how the VSCode displays the inferred types.

```rust
fn main() {
    let x : i32 = 42;
    // Type is x1 is implicitly inferred as i32
    let x1 = 42;
    // These two assignments are logically equivalent
    let y : u32 = 42;
    let z = 42u32;
}
```

Note that `type inference` cannot be used in function parameters or function
return values. In Rust, functions must begin with the `fn` keyword (as seen in `fn main()` above).
Function parameters are listed within the paranthesis `()` and must have an explicit type (specified after the parameter name with `:` separator).
Functions may optionally return an explictly specified value that must follow `->` as shown in the code snippet below.

```rust
fn add_one(x: u32) -> u32 {
    // This simply returns the passed in parameter as the return value from the function.
    // We'll revisit the return statement later.
    return x + 1;
}
```

Let us put this together in an example to see how it all comes together. Replace the contents
of `main.rs` with the following:

```rust
fn secret_of_life_u32(x: u32) {
    // We'll discuss formatting in println!() later
    println!("The u32 secret_of_life is {}", x);
}

fn secret_of_life_u8(x: u8) {
    // We'll discuss formatting in println!() later
    println!("The u8 secret_of_life is {}", x);
}

fn main() {
    let a = 42; // The let keyword assigns a value
    let b = 42; // The let keyword assigns a value
                // Notice how the compiler uses the function signature to infer types
                // Comment out the next two lines to see how the type of a and b changes
    secret_of_life_u32(a);
    secret_of_life_u8(b);
}
```
Notice that when the calls to the `secret_of_life_*` are commented out, it automatically changes the inferred type of the variable. Another side effect of commenting
the function calls is that the compiler emits a warning about the unused variables. In Rust,
unused varibles should be prefixed with `_` to avoid this warning (try changing `a` and `b` to `_a` and `_b`).