# Ownership

Ownership is Rust's most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector, so it's important to understand how ownership works.

## Rules

Rust's ownership model is a unique feature that enables memory safety without a garbage collector. *Ownership* is defined by a set of rules that are enforced at compile time:

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

If any of these rules are violated, the program won't compile. None of the features of ownership will slow down your program while it's running. This also makes it easier to write code that has no execution paths that crash at runtime, more on that [later](<!-- TODO: reference Result/Option unwrap() in types -->).

> Many programming languages don't require you to think about the stack and the heap very often. But in a systems programming language like Rust, whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions.
>
> Later we'll discuss these concepts in more detail to understand how they will influence design descisions. For more details, check out the [official Rust documentation](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap).

As a first example of ownership, we'll look at the *scope* of variables. A scope is the range within a program for which an item is valid. Take the following variable:

```rust
let s = "hello";
```

The variable `s` refers to a string literal, where the value of the string is hardcoded into the text of our program. The variable is valid from the point at which it's declared until the end of the current scope. The following shows when `s` would be valid:

```rust
// s is not valid here, it's not yet declared

{
    let s = "hello";   // s is valid from this point forward

    // do stuff with s ...
}

// this scope is over, s is no longer valid
```

In other words, there are two important points:

- When `s` comes into scope, it is valid.
- It remains valid until it goes out of scope.

At this point, the relationship between scopes and when variables are valid is similar to that in other programming languages. But what about when you need to share data or pass a variable to a function?

This is where references and borrowing come in.

## References and Borrowing

A *reference* is like a pointer in that it's an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

<!-- TODO: how deep should we go with this to show memory layout -->

### Shared References

A shared reference `&T` is a porinter that may be shared. Any number of other references may point to the same data and each shared reference is `Copy`, meaning that the reference itself can be copied without affecting the data it points to.

Values behind shared references are not mutable, therefore you cannot modify or reassign the value a shared reference points to, not can you cast a shared reference to a mutable one. The Rust compiler assumes that the value a shared reference points to will not change while that reference lives. For example, if the compiler see that the value behind a shared reference is read multiple times in a function, it is well within its rights to read it only onces and reuse that value.

### Mutable References

The alternative to a shared reference is a mutable reference `&mut T`. A mutable reference is a pointer that allows you to change the value it points to. You can have only one mutable reference to a particular piece of data in a particular scope. In other words, the Rust compiler assumes that mutable references are _exclusive_.

<!-- TODO: interesting optimizations that occur -->

### Interrior Mutability

Some types provide _interior mutability_, meaning that you can modify the value behind a shared reference. These types usually rely on additional mechanisms (like atomic CPU instructions) or invariants to provide safe mutability without relying on the semantics of exclusive references. These types broadly fall into two categories: those that let you get a mutable reference through a shared reference (like `Mutex` or `RefCell`) and those that let you replace a value given only a shared reference (like `Cell`).

<!-- TODO: expand on these 2 types -->

###  Immutable borrowing

Imagine you are running a library with infinite copies of each book. When you loan out a book you are guaranteed a few things:

- The person who is borrowing our book is strictly forbidden from changing the text.
- After they return the book they cannot read it anymore (after the loan function has finished)

```rust
let i_robot = Book {};

loan_to_foo(&i_robot); // borrow
loan_to_bar(&i_robot); // borrow

// we are still owner of i_robot, it is just borrowed

remove_book(i_robot);  // give up ownership

// we are not allowed to use `i_robot` anymore, it has been moved
```

Suppose we try to loan out our book after it has been removed (after we have given up ownership of `i_robot`) by adding the following line to our program:

```rust
loan_to_baz(&i_robot);
```

Since we have given up ownership of `i_robot`, it is no longer valid and building will result in the following compiler error:

```bash
$ cargo build

error[E0382]: borrow of moved value: `i_robot`
  --> src/main.rs:45:17
   |
36 |     let i_robot = Book {};
   |         ------- move occurs because `i_robot` has type `Book`, which does not implement the `Copy` trait
...
43 |     remove_book(i_robot);
   |                   ------- value moved here
44 |
45 |     loan_to_baz(&i_robot);
   |                 ^^^^^^^^ value borrowed here after move
   |
```

### Mutable borrowing

Now imagine you are an author in the process of writing a book. You give your editor a mutable borrow to your draft. At this point, only they can change it. They must give it back when they are done so that it can be borrowed again.

We can't let it be mutably borrowed again, we have to wait for the first function, our editor, to return it.

```rust
let mut i_robot = Draft {};

edit_foo(&mut i_robot); // mutable borrow
edit_bar(&mut i_robot); // mutable borrow

// we are still owner of i_robot, it is just borrowed

sell(i_robot);  // pass ownership to the printing company
```

Suppose we try to give our book to another editor after it has been sold (after we have given up ownership):

```rust
edit_baz(&mut i_robot);
```

Similar to our previous example, we will get the following compiler error:

```bash
$ cargo build

error[E0382]: borrow of moved value: `i_robot`
  --> src/main.rs:45:14
   |
36 |     let mut i_robot = Book {};
   |         ----------- move occurs because `i_robot` has type `Book`, which does not implement the `Copy` trait
...
43 |     sell(i_robot);
   |          ------- value moved here
44 |
45 |     edit_baz(&mut i_robot);
   |              ^^^^^^^^^^^^ value borrowed here after move
   |
```

> Later we'll examine how to apply the concepts of mutable and immutable borrowing to concurrent/asynchronous programs. <!-- TODO: link -->

### Slices

*Slices* let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

<!-- TODO: example/expand on this -->

## Lifetimes

Often Rust can infer lifetimes for us, but sometimes we need to specify them. Lifetimes are a way to ensure that references are valid for a certain scope. For example, the following function will not compile:

```rust
// TODO
```

> Another way to think about lifetimes is by looking at how the borrow checker works. Rather than treating lifetimes as a variables scope, the borrow checker treats lifetimes as a ranges of memory.
>
> See [But What is 'a Lifetime](https://www.youtube.com/watch?v=gRAVZv7V91Q&t=65s&ab_channel=leddoo) a more in-depth explaination of how lifetimes relate to memory.

---

### Resources

For a more in depth look at ownership, borrowing, and lifetimes, check out one of the following resources:

- [Memory](./memory.md)
- [Smart Pointers](./smart-pointers.md)
- [Unsafe](./unsafe.md)

Or try out the exercises at [Rust by Practice](https://practice.course.rs/why-exercise.html) on [ownership](https://practice.course.rs/ownership/ownership.html) and [borrowing](https://practice.course.rs/ownership/borrowing.html).
