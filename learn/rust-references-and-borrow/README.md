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
#### Takeway
Rust programs frequently use references, and the act of creating a reference is called as `borrowing`; as with references, borrows can be `immutable` (default) or `mutable`. We'll discuss these concepts in more detail in a fortcoming chapter, but the compiler implements a mechanism called `borrow checker` to ensure that the invariants around references are not violated.
