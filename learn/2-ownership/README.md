# Rust Ownership basics [1]
## Understanding Ownership
Ownership is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector, so it’s important to understand how ownership works. In this chapter, we’ll talk about ownership as well as several related features: borrowing, slices, and how Rust lays data out in memory.

- Safety is the Absence of Undefined Behavior
Let's start with an example. This program is safe to execute:
```rust
fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    let x = true;
    read(x);
}
```
We can make this program unsafe to execute by moving the call to read before the definition of x:

This code does not compile!
```rust
fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    read(x); // oh no! x isn't defined!
    let x = true;
}
```
*A foundational goal of Rust is to ensure that your programs never have undefined behavior.* That is the meaning of "safety." Undefined behavior is especially dangerous for low-level programs with direct access to memory. About 70% of reported security vulnerabilities in low-level systems are caused by memory corruption, which is one form of undefined behavior.

A secondary goal of Rust is to prevent undefined behavior at compile-time instead of run-time. This goal has two motivations:

- Catching bugs at compile-time means avoiding those bugs in production, improving the reliability of your software.
- Catching bugs at compile-time means fewer runtime checks for those bugs, improving the performance of your software.

## Ownership as a Discipline for Memory Safety
Since safety is the absence of undefined behavior, and since ownership is about safety, then we need to understand ownership in terms of the undefined behaviors it prevents. The Rust Reference maintains a large list of "Behavior considered undefined". For now, we will focus on one category: operations on memory.

Memory is the space where data is stored during the execution of a program. There are many ways to think about memory:

If you are unfamiliar with systems programming, you might think of memory at a high level like "memory is the RAM in my computer" or "memory is the thing that runs out if I load too much data".
If you are familiar with systems programming, you might think of memory at a low level like "memory is an array of bytes" or "memory is the pointers I get back from malloc".
Both of these memory models are valid, but they are not useful ways to think about how Rust works. The high-level model is too abstract to explain how Rust works. You will need to understand the concept of a pointer, for instance. The low-level model is too concrete to explain how Rust works. Rust does not allow you to interpret memory as an array of bytes, for instance.

Rust provides a particular way to think about memory. Ownership is a discipline for safely using memory within that way of thinking. The rest of this chapter will explain the Rust model of memory.

### Variables Live in the Stack
Here's a program like the one you saw in Section 3.3 that defines a number n and calls a function plus_one on n. Beneath the program is a new kind of diagram. This diagram visualizes the contents of memory during the program's execution at the three marked points.

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

Boxes Live in the Heap
However, copying data can take up a lot of memory. For example, here's a slightly different program. This program copies an array with 1 million elements:

```rust
let a = [0; 1_000_000]; // L1
let b = a; // L2
```
Observe that copying a into b causes the main frame to contain 2 million elements.

To transfer access to data without copying it, Rust uses pointers. A pointer is a value that describes a location in memory. The value that a pointer points-to is called its pointee. One common way to make a pointer is to allocate memory in the heap. The heap is a separate region of memory where data can live indefinitely. Heap data is not tied to a specific stack frame. Rust provides a construct called Box for putting data on the heap. For example, we can wrap the million-element array in Box::new like this:
```rust
let a = Box::new([0; 1_000_000]); // L1
let b = a; // L2
```
Observe that now, there is only ever a single array at a time. At L1, the value of a is a pointer (represented by dot with an arrow) to the array inside the heap. The statement let b = a copies the pointer from a into b, but the pointed-to data is not copied. Note that a is now grayed out because it has been moved — we will see what that means in a moment.

### Rust Does Not Permit Manual Memory Management
Memory management is the process of allocating memory and deallocating memory. In other words, it's the process of finding unused memory and later returning that memory when it is no longer used. Stack frames are automatically managed by Rust. When a function is called, Rust allocates a stack frame for the called function. When the call ends, Rust deallocates the stack frame.

As we saw above, heap data is allocated when calling Box::new(..). But when is heap data deallocated? Imagine that Rust had a free() function that frees a heap allocation. Imagine that Rust let a programmer call free whenever they wanted. This kind of "manual" memory management easily leads to bugs. For example, we could read a pointer to freed memory:
```rust
fn free<T>(_t: T) {}
fn main() {
let b = Box::new([0; 100]);
free(b);
assert!(b[0] == 0);
}
```
Here, we allocate an array on the heap. Then we call free(b), which deallocates the heap memory of b. Therefore the value of b is a pointer to invalid memory, which we represent as the "⦻" icon. No undefined behavior has happened yet! The program is still safe at L2. It's not necessarily a problem to have an invalid pointer.

The undefined behavior happens when we try to use the pointer by reading b[0]. That would attempt to access invalid memory, which could cause the program to crash. Or worse, it could not crash and return arbitrary data. Therefore this program is unsafe.

Rust does not allow programs to manually deallocate memory. That policy avoids the kinds of undefined behaviors shown above.

A Box's Owner Manages Deallocation
Instead, Rust automatically frees a box's heap memory. Here is an almost correct description of Rust's policy for freeing boxes:

Box deallocation principle (almost correct): If a variable is bound to a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.

### Variables Cannot Be Used After Being Moved
The string program helps illustrate a key safety principle for ownership. Imagine that first were used in main after calling add_suffix. We can simulate such a program and see the undefined behavior that results:

```rust
fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}, originally {first}");// L1 // first is now used here
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
```
This code does not compile!

`first` points to deallocated memory after calling add_suffix. Reading first in println! would therefore be a violation of memory safety (undefined behavior). Remember: it's not a problem that first points to deallocated memory. It's a problem that we tried to use first after it became invalid.

Thankfully, Rust will refuse to compile this program, giving the following error:
```
error[E0382]: borrow of moved value: `first`
 --> test.rs:4:35
  |
2 |     let first = String::from("Ferris");
  |         ----- move occurs because `first` has type `String`, which does not implement the `Copy` trait
3 |     let full = add_suffix(first);
  |                           ----- value moved here
4 |     println!("{full}, originally {first}"); // first is now used here
  |                                   ^^^^^ value borrowed here after move
```
Let's walk through the steps of this error. Rust says that first is moved when we called add_suffix(first) on line 3. The error clarifies that first is moved because it has type String, which does not implement Copy. We will discuss Copy soon — in brief, you would not get this error if you used an i32 instead of String. Finally, the error says that we use first after being moved (it's "borrowed", which we discuss next section).

So if you move a variable, Rust will stop you from using that variable later. More generally, the compiler will enforce this principle:

Moved heap data principle: if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.

Now you should start to see the relationship between ownership, moves, and safety. Moving ownership of heap data avoids undefined behavior from reading deallocated memory.
```
fn main() {
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
```

## Summary
Ownership is primarily a discipline of heap management:

- All heap data must be owned by exactly one variable.
- Rust deallocates heap data once its owner goes out of scope.
- Ownership can be transferred by moves, which happen on assignments and function calls.
- Heap data can only be accessed through its current owner, not a previous owner.
We have emphasized not just how Rust's safeguards work, but why they avoid undefined behavior. When you get an error message from the Rust compiler, it's easy to get frustrated if you don't understand why Rust is complaining. These conceptual foundations should help you with interpreting Rust's error messages. They should also help you design more Rustic APIs.

---
## References
[1]: [The Rust Programming Language](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html)
