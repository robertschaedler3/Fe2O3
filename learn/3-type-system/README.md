# Type System

One of Rust's most powerful features is its rich type system. It doesnt just enforce type safety, but allows you to build complex data models with deep integration with your probmel domain.

Rust uses an algebraic data type system, which is a way of defining types by combining other types. Most languages have *product* types (a container for a number of attributes):

```rust
struct Point {
    x: i32,
    y: i32,
}
```

In other languages, enums are typically an enumeration of integers, each with an assigned name. In Rust are proper *sum* types (some times referred to as *tagged unions*). They can be used to define a type that can be one of several variants:

```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}
```

> There are several resources available that dive deeper into type theory and why Rust's type system is so powerful from a mathematical perspective. While we won't go into the details here, it is worth noting that Rust's type system is based on solid theoretical foundations.
>
> Please refer to the [Resources](#resources) section for more information and further reading.

To understand the power of Rust's type system, lets look at some examples in other languages and how Rust's type system helps you to avoid common mistakes.

Let's examine the folowing line of code. What potential problems are there? Where could this code go wrong? If something goes wrong at runtime how would you know?

```python
int(item["view_count"]["total"])
```

There are many assumptions being made here:

1. `item` is a dictionary
1. `item` has a key `view_count`
1. `item["view_count"]` is a dictionary
1. `item["view_count"]` has a key `total`
1. `item["view_count"]["total"]` can be parsed as an integer with `int()`

If any of these assumptions are wrong, the code will crash at runtime.

> This problem is so common that it has a name: the "billion-dollar mistake". Tony Hoare, the inventor of null references, calls it his "billion-dollar mistake" because it has caused billions of dollars in damage to software systems over the years.

While we can write checks to catch these errors at runtime, they are often overlooked or forgotten, leading to bugs.

Lets examine how write this same code in Rust:

```rust
let item: HashMap<String, HashMap<String, String>> = ...;

i32::from_str_radix(
    item["view_count"].unwrap()
        .get("total").unwrap(),
    10
).unwrap()

// Note the use of `unwrap()` to handle the case where the key is missing
```

This code is exactly as unsafe as the previous Python example, however the three places where it can crash are clearly marked with `unwrap()`. To make this code safe, we can simply handle the error cases explicitly:

```rust
if let Some(view_count) = item.get("view_count") {
    match view_count.get("total") {
        Some(total) => {
            match total.parse::<i32>() {
                Ok(n) => n,
                Err(_) => {
                    // The value could not be parsed as an integer
                }
            }
        },
        None => {
            // There is no `total` key
        }
    }

} else {
    // There is no `view_count` key
}
```

This code is much more verbose and safer. It is clear where the code can fail and how to handle those cases. However, we can still do better. Notice what has replaced the `unwrap()` calls. We have replaced them with `if let` and `match` statements. Don't worry about the details Rust's pattern matching syntax for now. The important thing to notice is the types that are being matched against.

Rust does not have the concept of `null` or `nil`. Instead, Rust has an `Option` type that can be either `Some(...)` or `None`. This forces you to handle the case where a value is missing. This is a powerful feature that can prevent entire classes of bugs if used correctly.

> ```rust
> enum Option<T> {
>     None,
>     Some(T),
> }
> ```
>
> https://doc.rust-lang.org/std/option/enum.Option.html

Similarly, Rust has a `Result` type that can be either `Ok(...)` or `Err(...)`. This is used to handle errors in a similar way to `Option`.

> ```rust
> enum Result<T, E> {
>     Ok(T),
>     Err(E),
> }
> ```
>
> https://doc.rust-lang.org/std/result/enum.Result.html

But how can this help us with our original problem? Lets rewrite the code using Rust's `?` operator:

```rust
let view_count = item.get("view_count").ok_or("view_count is missing")?;
let total = view_count.get("total").ok_or("total is missing")?;
let n = total.parse::<i32>()?;
```

Later we will examine how to properly handle errors in more detail. For now, notice how the `?` operator is used at the end of an expression returning a `Result`. It is equivalent to a `match` expression, where the `Err(err)` branch expands to an early return `Err(From::from(err))`, and the `Ok(ok)` branch expands to an `ok` expression.

This code covers all the possible error cases and is much more concise than the previous examples.

---

### Resources

- [A Simpler Way to See Results](https://www.youtube.com/watch?v=s5S2Ed5T-dc&t=3s&ab_channel=LoganSmith)
- [Error Handling](https://doc.rust-lang.org/rust-by-example/error.html)
