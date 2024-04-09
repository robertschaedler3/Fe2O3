# Advanced

Advanced and extra reading for topic related to ownership/borrowing.

## Manual Memory Management

Previously, we saw that Rust manages memory automatically to provide safety. But sometimes, particularly in low-level code, you might want/need to manage memory manually.

Rust provides a way to do this with the `unsafe` keyword. The `unsafe` keyword allows you to write code that the Rust compiler can't verify is safe. This is useful when you need to interact with code that doesn't follow Rust's safety rules, such as C libraries or hardware interfaces.

<!-- TODO: unsafe doesnt mean the whole program is unsafe -->
<!-- TODO: examples of how Rust STD uses unsafe for linked lists -->
<!-- TODO: helpful compiler/linting for unsafe -->

### Dropping values

When a value is no longer needed, Rust will run a *"destructor"* on that value. The most common way that a value is no longer needed is when it goes out of scope.

This *descructor* consists of 2 components:

- A call to `Drop::drop` for that value, if a special `Drop` trait is implemented for its type
- The automatically generated implementation which recursively calls the destructors of all the fields of a value

As Rust automatically calls the destructors of all contained fields, you don’t have to implement Drop in most cases. But there are some cases where it is useful, for example for types which directly manage a resource. That resource may be memory, it may be a file descriptor, it may be a network socket. Once a value of that type is no longer going to be used, it should “clean up” its resource by freeing the memory or closing the file or socket. This is the job of a destructor, and therefore the job of `Drop::drop`.

For more information and examples see the official documentation on [Drop](https://doc.rust-lang.org/std/ops/trait.Drop.html) and [std::mem::drop](https://doc.rust-lang.org/std/mem/fn.drop.html).

## Memory layout

<!-- TODO -->

### Variables Live on the Stack

Here's a program that defines a number `n` and calls a function `plus_one` on `n`.

```rust
fn main() {
    let n = 5; // L1
    let y = plus_one(n); // L3
    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // L2
}
```

Variables live in frames. A frame is a mapping from variables to values within a single scope, such as a function. For example:

- The frame for main at location L1 holds n = 5.
- The frame for plus_one at L2 holds x = 5.
- The frame for main at location L3 holds n = 5; y = 6.

Frames are organized into a stack of currently-called-functions. For example, at L2 the frame for main sits above the frame for the called function plus_one. After a function returns, Rust deallocates the function's frame. (Deallocation is also called freeing or dropping, and we use those terms interchangeably.) This sequence of frames is called a stack because the most recent frame added is always the next frame freed.

Note: this memory model does not fully describe how Rust actually works! As we saw earlier with the assembly code, the Rust compiler might put n or x into a register rather than a stack frame. But that distinction is an implementation detail. It shouldn't change your understanding of safety in Rust, so we can focus on the simpler case of frame-only variables.

When an expression reads a variable, the variable's value is copied from its slot in the stack frame. For example, if we run this program:

```rust
let a = 5; // L1
let mut b = a; // L2
b += 1; // L3
```
The value of a is copied into b, and a is left unchanged, even after changing b.

### `Box`s Live in the Heap

Copying data can take up a lot of memory. For example, here's a slightly different program. This program copies an array with 1 million elements:

```rust
let a = [0; 1_000_000]; // L1
let b = a; // L2
```

Observe that copying a into b causes the main frame to contain 2 million elements.

To transfer access to data without copying it, Rust uses pointers. A pointer is a value that describes a location in memory. The value that a pointer points-to is called its pointee. One common way to make a pointer is to allocate memory in the heap. The heap is a separate region of memory where data can live indefinitely. Heap data is not tied to a specific stack frame. Rust provides a construct called `Box` for putting data on the heap. For example, we can wrap the million-element array in `Box::new` like this:

```rust
let a = Box::new([0; 1_000_000]); // L1
let b = a; // L2
```

Observe that now, there is only ever a single array at a time. At L1, the value of a is a pointer (represented by dot with an arrow) to the array inside the heap. The statement let b = a copies the pointer from a into b, but the pointed-to data is not copied. Note that a is now grayed out because it has been moved — we will see what that means in a moment.


