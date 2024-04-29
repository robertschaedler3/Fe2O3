# Memory

<!-- TODO -->

## Memory Layout

<!-- TODO -->

## Memory Alignment

<!-- TODO -->

## Dynamically Sized Types

<!-- TODO -->

## Dropping values

In some cases, you may need to manually drop a value. When a value is no longer needed, Rust will automatically run a *"destructor"* on that value. The most common way that a value is no longer needed is when it goes out of scope.

This *descructor* consists of 2 components:

- A call to `Drop::drop` for that value, if a special `Drop` trait is implemented for its type
- The automatically generated implementation which recursively calls the destructors of all the fields of a value

Rust automatically calls the destructors of all contained fields, so you don't have to implement `Drop` in most cases. But there are some cases where it is useful, for example types which directly manage a resource. That resource may be memory, a file descriptor, or something else that must be cleaned up in a special way. Once a value of that type is no longer going to be used, it should "clean up" its resource by freeing the memory or closing the file or socket. This is the job of a destructor, and therefore the job of `Drop::drop`.

For more information and examples see the official documentation on [Drop](https://doc.rust-lang.org/std/ops/trait.Drop.html) and [std::mem::drop](https://doc.rust-lang.org/std/mem/fn.drop.html).
