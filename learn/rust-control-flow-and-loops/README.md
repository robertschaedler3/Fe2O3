# Control flow, loops, and expressions

This section covers the Rust `if`, `while`, `for` and `loop` keywords.

In Rust `if` behaves as a control flow statement like many other languages, but it can also be used an expression. This is best illustrated by an example.

Create a new package called `controlflow` by executing `cargo new controlflow` under this folder, open `main.rs` in VSCode, and replace the default content with the following snippet.

```rust
fn main() {
    let x = 42;
    if x < 42 {
        println!("Smaller than the secret of life");
    } else if x == 42 {
        println!("Is equal to the secret of life");
    } else {
        println!("Larger than the secret of life");
    }
    let is_secret_of_life = if x == 42 {true} else {false};
    println!("{}", is_secret_of_life);
}
```

Notice how the `if` in `let is_secret_of_life = if x == 42 {true} else {false};` is used as an expression that assigns a value. This is made possible by a Rust feature called `expression blocks` (more on this later).

The ```while``` keyword can be used to loop while an expression is true.

```rust
fn main() {
    let mut x = 40;
    while x != 42 {
        x += 1;
    }
}
```

The `for` keyword can be used to iterate over ranges. Unlike other languages that require a variable to define the start of a range and increment it until the end, Rust simply uses constants for the beginning and end of the range. The increment for the range is always `1`. Note that the end of the range *isn't inclusive unless* it's prefixed with a `=`. 

```rust
fn main() {
    // Will not print 43; use in 40..=43 to include last element
    for x in 40..43 {
        println!("{}", x);
    }
}
```
The use of constant range in the `for` above is an application of a more general Rust concept called `iterators`. We'll discuss how to comine `for` and `iterators` in a forthcoming chapter.

The Rust `loop` keyword creates an infinite loop that can be terminated by a `break` keyword.

```rust
fn main() {
    let mut x = 40;
     loop {
        if x == 42 {
            break;
        }
        x += 1;
    }
}
```

The `break` keyword can also can include an optional expression that can be used to assign the value of from ```loop```.

```rust
fn main() {
    let mut x = 40;
    // y will be assigned 42 when the loop terminates
    let y = loop {
        if x == 42 {
            break x;
        }
        x += 1;
    };
}
```

The `continue` keyword can be used to return to the top of the `loop`. This can be combined with `loop labels` in nested loops to specify the target of the `break`. Loop labels are prefixed with a ``` ` ``` and must use a `:` suffix.

```rust
fn main() {
    let mut x = 40;
    'outer: loop {
        if x+1 == 42 {
            break;
        }
        loop {
            x += 1;
            if x == 41 {
                // Breaks to 'outer loop
                break 'outer;
            } else {
                // Silly, but continues inner loop
                continue;
            }
        }
    }
}
```

In Rust, most keywords are *expressions* (as opposed to statements in other languages). Expression blocks are simply a sequence of expressions enclosed in `{}`. The general rule is that evaluated value is simply the last expression in the block.

In preceding examples we used expression blocks with `if {true} else {false};` to assign a value, return values while breaking from loops, and so forth.

Expression blocks can also be used to elide return statements from functions. The idea is that the return value of the function is simply the evaluation of last expression.

```rust
fn is_secret_of_life(x: u32) -> bool {
    // Same as if x == 42 {true} else {false}
    x == 42 // Note: ; must be omitted
}

fn main() {
    println!("{}", is_secret_of_life(42));
}
```

Note that the last expression `x == 42` evaluates to a `bool`, which matches the return type of `is_secret_of_life`. If we change the expression to `x == 42;` it will be equivalent to the following snippet:

```rust
fn is_secret_of_life(x: u32) -> bool {
    let _ = x == 42;
}
```
The `_` in the `let` assignment is a placeholder for an anonymous variable that receives the `bool` value from the expression evaluation. However, in the absence of an explicit return statement, the function now returns the `unit type` or `()` (**which will result in an informative compiler error about the mismatched return type**). The Rust `unit type` is roughly the equivalent of the `void` return type in some languages.

Unless specified otherwise, the `()` is default type that's returned from from all functions, including `main()`.