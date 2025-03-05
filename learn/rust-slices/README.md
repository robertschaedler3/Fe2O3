## Slices

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
