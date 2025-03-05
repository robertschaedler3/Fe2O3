# Arrays

Rust arrays contain a statically fixed number of elements of the same type. Like all other Rust types, arrays are immutable by default (unless `mut` is used). Like many other languages, individual array elements can be accessed using indexing operator (`[]`). Array indexing starts with 0, and such access is checked both at compile time (known static index) as well as runtime (example: indexing using a variable). The built-in `len()` method can be used to obtain the length of the array (which is fixed at compile time).

Rust best practice is to avoid array indexing using `[]`, and we can use `for` and other alternatives discussed in forthcoming sections.

```rust
fn main() {
    // Initializes an array of 3 elements of type u8 and sets all to 42
    let a : [u8; 3] = [42; 3];
    // Alternative syntax; this is the equivalent of the above
    // let a = [42u8, 42u8, 42u8];
    for x in a {
        println!("{x}");
    }
    // The compiler will reject this code
    println!("{}", a[4]);

    // The compiler will reject this code unless
    // a is declared with mut
    a[0] = 1;

    // Uncommenting the below will cause a panic
    //println!("{}", a[a.len() + 1]);
}
````

Rust arrays can be nested. In the example below, we create a nested array Rust has several built-in formatters for printing. In the example below, we define an array with three rows and each element the array is array with two integers (`[i32; 2]`). In the below, we use the built-in `debug` print formatter (`:?`) to dump the elements of the array. Rust also provides the `:#?` formatter for `pretty print`. These formatters can be customized per type (more on this in forthcoming sections).

```rust
fn main() {
    let a = [
        [40, 0], // Define a nested array
        [41, 0],
        [42, 1],
    ];
    for x in a {
        println!("{x:?}");
    }
}
```

```bash
# Output
[40, 0]
[41, 0]
[42, 1]
```

The key takeway is that Rust arrays are intended for scenarios in which the exact type and size of elements is known and fixed at compile time. Rust functions can accept arrays as parameters, but this is seldom used because it requires callers to pass an array of the exact same size. For example, the compiler will reject the code below because `takes_two_u8_element_array` takes exactly an array of type `[u8; 2]`, which is a different type from `[u8; 3]`. In forthcoming chapters, we'll discuss how arrays are usually passed using `slices`, and also see how scenarios that require a dynamic number of elements are handled using vectors (`vec`).

```rust
fn takes_two_u8_element_array(a: [u8; 2]) {}

fn main() {
    // Rejected by the compiler; one u8 too many
    takes_two_u8_element_array([0, 0, 0]);
}
```
