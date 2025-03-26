# References and borrowing

Rust references are created using the `&` operator and somewhat analogous to pointers in `C` and references in `C++`, but with some key differences.

* Like Rust variables, references are *immutable* by default, unless explicitly qualified as mutable with the `&mut` keyword.
* It is impossible to create a mutable reference to an immutable variable.
* References are associated with a `scope`, and the general rule is that reference cannot outlive the scope of the associated variable. This a key concept called `lifetime` and it will be discussed at length in a forthcoming chapter.
* There can only be a *single* active mutable reference to a mutable variable, and it must not overlap the scope of any other reference to the same variable.
* It is legal to have *any* number of immutable references to a variable at any point of time (subject to the above rule)

References can dereferenced using the `*` operator, but the compiler will implicitly perform the operation for immutable references. It is possible to create a reference to a reference (a type of `&&` in this case) ad infinitum, but there's seldom a need (if any) for such constructs in Rust. Unlike some other languages which require a matching number of `*` deferences to access the underlying variable, the compiler automatically inserts as many deferences as necessary.

In the example below, we create a **mutable** variable and define a nested lexical scope that has several **immutable** references to it. The scope or `lifetime` of these **immutable** references ends at the end of the nested scope, after which it's legal to create a *mutable* reference. Note that the compiler automatically deferences immutable references.

Notice how the `lifetime` of the variable `a` is always longer than all the references to the variable.

```rust
fn main() {
    // Lifetime of a begins here
    let mut a = 42;
    {
        let b = &a;
        let c = &a;  // Both b and c are immutable references to a
        let d = &b; // type of d (&&i32)
        println!("{} {} {}", b, c, d); // The compiler automatically dereferences *c

        // This won't compile because it's illegal to take a mutable reference when immutable references are still in scope
        // let e = &mut a;
        // d, c, b go out of scope in that order, ending their lifetimes
    }
    let e = &mut a; // Ok: b and c are not in scope
    *e = 43;    // Assignment requires explicit deferencing
    // Lifetime of e ends here
    // Lifetime of a ends here
}
```
References and lifetimes are closely associated with another Rust concept known as `ownership`, but we'll first take a look at `slices`.

### Slices

*Slices* let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership of the data it refers to. Slices are useful when you want to pass a part of a collection to a function, or when you want to work with a part of a collection.

Rust `slices` leverage `references` to create subsets of arrays. Unlike arrays, which have a static fixed length determined at compile time, slices can be of arbitrary size. Internally, slices are implemented as a "fat-pointer" that contains the length of the slice and a reference to a starting element in the original array. Like everything else in Rust, `slices` are immutable by default, unless created with a `&mut` qualifier, and they are subject to the same restrictions as references.

Rust offers several syntactical shortcuts with the to create array slices. It is possible to create additional slices from an existing slice. One important caveat is the statically specified slice bounds are **not checked by the compiler at compile time**. Exceeding the start or end bounds of the slice or the original array (or slice) will cause a runtime crash. The `len()` method can be used to determine the *runtime* length of a slice.

In the example below, we create an **immutable** integer array with four elements and create **immutable** slices from it.

```rust
fn main() {
    let a = [40, 41, 42, 43];
    let b = &a[1..a.len()]; // A slice starting with the second element in the original
    let c = &a[1..]; // Same as the above
    let d = &a[..]; // Same as &a[0..] or &a[0..a.len()]
    let e = &d[2..]; // Sub-slice
    println!("b:{b:?} c:{c:?} d:{d:?} e:{e:?}");
    // This will crash
    //let f = &a[0..a.len()+1];
}```

```bash
# Output
[41, 42, 43] [41, 42, 43] [40, 41, 42, 43]
```

The following example the illustrates the flexibility accorded by passing in slices as function parameters instead of arrays. Unlike array parameters that are constrained to a specific length, slices can be of arbitrary length.

```rust
fn takes_u8_slice(a: &[u8]) {
    for x in a {
        println!("{x}");
    }
}

fn main() {
    takes_u8_slice(&[40, 41, 42]);
}
```

For more examples and information on slices, see the [official Rust documentation](https://doc.rust-lang.org/std/primitive.slice.html).

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

###  Immutable borrowing and ownership

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

### Mutable borrowing and ownership

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

## Lifetimes and ownership

A lifetime is a construct the compiler (or more specifically, its borrow checker) uses to ensure all borrows are valid. Specifically, a variable's lifetime begins when it is created and ends when it is destroyed. While lifetimes and scopes are often referred to together, they are not the same. Often Rust can infer lifetimes for us, but sometimes we need to specify them. For more information on lifetimes, see the [official Rust documentation](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html).

> Another way to think about lifetimes is by looking at how the borrow checker works. Rather than treating lifetimes as a variable's scope, the borrow checker treats lifetimes as a ranges of memory.
>
> See [But What is 'a Lifetime](https://www.youtube.com/watch?v=gRAVZv7V91Q&t=65s&ab_channel=leddoo) a more in-depth explaination of how lifetimes relate to memory.

## Try it out!

Experiment with ownership, borrowing, and lifetimes in Rust by running and modifying the code.

_Compilation errors are expected, see if you can fix them by using the compiler's suggestions!_

```bash
$ cargo run -p ownership --bin scope
$ cargo run -p ownership --bin borrow
$ cargo run -p ownership --bin mut_borrow
```

---

### Resources

For a more in depth look at ownership, borrowing, and lifetimes, check out one of the following resources:

- [Memory](./memory.md)
- [Smart Pointers](./smart-pointers.md)
- [Unsafe](./unsafe.md)

Or try out the exercises at [Rust by Practice](https://practice.course.rs/why-exercise.html) on [ownership](https://practice.course.rs/ownership/ownership.html) and [borrowing](https://practice.course.rs/ownership/borrowing.html).

#### Takeway
Rust programs frequently use references, and the act of creating a reference is called as `borrowing`; as with references, borrows can be `immutable` (default) or `mutable`. Borrowing is associated with other important concepts like `lifetime` and `ownership`. The compiler implements a mechanism called `borrow checker` to ensure that the invariants around references are not violated.
