# Strings

In Rust, there are many ways to work with strings, each keeping safety, performance, and flexibility in mind:

- *No null terminator* - String types in Rust store the string length as metadata instead of using a null terminator like in C. This leads to more efficient runtime operations and prevents vulerabilities like buffer overflows.
- *UTF-8* - Rust strings are always valid UTF-8, which means they can contain any Unicode codepoint. This makes it easier to work with text in different languages and scripts.
- *Immutable* - Rust strings are immutable by default, which means you can't change the contents of a string after it's created. This makes it easier to reason about the code and prevents bugs caused by unexpected changes.

With these features in mind, let's explore the different string types in Rust and how to use them effectively.

## `str` vs `String`

The two main string types in Rust are `&str` and `String`.

The `String` type in Rust is a heap allocated, growable, UTF-8 encoded string. This is called an *owned* type because the memory is owned by the string and will be deallocated when the string goes out of scope. This type consists of a pointer to the string data, its length, and a capacity.

The `&str` type is a *borrowed* type that points to a string slice. This type is used to reference a portion of a `String` or a string literal. It consists of a pointer to the string data and its length. Unlike the `String` type, the `&str` type does not contain capacity information.

```rust
let my_string: String = String::from("Hello, Rust!");
let my_str: &str = &my_string;
```

While the `String` type is always allocated on the heap, `&str` can reference data on the heap or in the data section of the compiled binary (string literals). It is also possible to reference data on the stack, but this is less common.

### Byte representation

The `String` type is essentially a wrapper around a `Vec<u8>`, which is a growable vector of unsigned 8-bit integers. This means that a `String` is essentially a sequence of bytes that represent UTF-8 encoded text.

Similarly, the `&str` type is a reference to a sequence of bytes that represent UTF-8 encoded text. It is essentially a pointer to a slice of bytes, or a string slice `&[u8]`.

In both cases, the bytes are guaranteed to be valid UTF-8 encoded text. This allows the string type to provide safe operations on the string data without worrying about invalid byte sequences.

### When to use `String` vs `&str`

The `String` type is useful when you want to create or modify string data dynamically at runtime (e.g. collecting user input, reading from a file, or building a string from multiple parts). The `String` type is also useful when you need to transfer ownership of the string data between functions or threads.

On the other hand, `&str` is useful when you want to reference existing string data without taking ownership. This type is commonly used as a function parameter to avoid copying large strings. It is also useful when working with string slices or substrings.

## Types of string slices

The string slice type is made up of two parts: the `&` operator and the `str` type. The `str` type represents a _dynamically sized sequence_ of UTF-8 encoded bytes. In other words, it is a reference to a sequence of bytes that make up a string (i.e. a string slice). However, it cannot be used directly because its size is not known at compile time. Instead, we need to use `str` behind some type of pointer, like a reference: `&str`.

`&str` is by far the most common string slice type but it is not the only one.

Although it is less common, it is also possible to have a mutable string slice: `&mut str`. This type is useful when you need to modify the contents of a string slice in place.

### Smart pointers

#### `Box<str>`

`Box<T>` is a smart pointer that allows you to store data on the heap rather than the stack. Boxes are useful when you have a type whose size can't be known at compile time or when you want to transfer ownership of a value without copying the underlying data. Both make `Box<T>` useful for working with string slices.

A `Box<str>` represents an owned, non-growable, heap-allocated string slice. This means that the string data is stored on the heap and the memory is deallocated when the `Box` goes out of scope. This saves a small amount of memory (8 bytes) by dropping the capacity information that is present in a `String`. This is also useful when returning an owned string from a function that will not be modified.

Imagine we want to return an immutable string from a function. Since we know the string will not be modified, we decide to return a `&str` to avoid the overhead of a `String`:

```rust
fn get_data() -> &str {
    let my_string = String::from("Hello, Rust!"); // Owned string
    let my_str = &my_string; // Borrowed string slice

    return my_str;
}
```

```bash
> cargo build

error[E0106]: missing lifetime specifier
 --> learn/3-type-system/src/main.rs:1:18
  |
1 | fn get_data() -> &str {
  |                  ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
  |
1 | fn get_data() -> &'static str {
  |                   +++++++
help: instead, you are more likely to want to return an owned value
  |
1 | fn get_data() -> String {
  |                  ~~~~~~

error[E0515]: cannot return value referencing local variable `my_string`
 --> learn/3-type-system/src/main.rs:7:12
  |
5 |     let my_str = &my_string;
  |                  ---------- `my_string` is borrowed here
6 |
7 |     return my_str;
  |            ^^^^^^ returns a value referencing data owned by the current function
```

In this case, we are returning a borrowed value from a local variable, which is not allowed because the owned `String` data may be deallocated when the function returns (i.e. the data is dropped when `my_string` goes out of scope) which would leave `my_str` pointing to invalid memory.

> We can also see the compliler suggesting to add a `'static` lifetime specifier to the return type. While this may work in some cases it is only useful if we are returning `const` or `static` values. Later we will see how to use string literals with `'static` lifetime.

To fix this, we can return an owned value by using a `Box<str>`:

```rust
fn get_data() -> Box<str> {
    let my_string = String::from("Hello, Rust!"); // Owned string
    let my_str = my_string.into_boxed_str(); // Convert to Box<str>

    return my_str;
}
```

> For more information on `Box<T>`, see the official documentation on [Box](https://doc.rust-lang.org/std/boxed/struct.Box.html).

#### `Rc<str>` and `Arc<str>`

Like `Box<T>`, `Rc<T>` and `Arc<T>` are smart pointers that allow you to store data on the heap. However, they have the added feature of _reference counting_. This means that there can be multiple owners pointing to the same data. `Arc<T>` is a thread-safe version of `Rc<T>` that uses _atomic_ operations to manage the reference count.

`Rc<str>` and `Arc<str>` are useful when you want to share ownership of an immutable string slice across multiple parts of your program without actually cloning the string data. While a `Box<str>` allows you to have a single owner of the string data, `Rc<str>` and `Arc<str>` allow you to have multiple owners of the underlying string data.

```rust
let large_string: &'static str = "This is some large string data that we want to share across multiple parts of our program.";

let substring: Rc<str> = Rc::new(&large_string[0..10]);

let another_reference: Rc<str> = sub_string.clone();
let yet_another_reference: Rc<str> = sub_string.clone();
```

> It is important to note that `Rc<String>` and `Arc<String>` results in double indirection. This means that the `Rc` or `Arc` itself is a pointer to the string data, which is also a pointer. This can add a small amount of overhead when accessing the string data and generally not recommended.

> For more information on `Rc<T>` and `Arc<T>`, see the official documentation on [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html) and [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html).

### `Vec<T>` vs `Box<[T]>`, `Rc<[T]>`, and `Arc<[T]>`

Given that a `String` is essentially a `Vec<u8>` and `&str` is a `&[u8]`, we can generalize the benefits and usecases of different types of strings to other data types.

For immutable data, especially when stored in structs or arrays/collections and shared across multiple parts of a program, `Rc<[T]>` and `Arc<[T]>` can provide significant benefits in terms of memory efficiency and performance over `Vec<T>`:

- Extremely cheap, _O(1)_ clone. No matter how large the data is, cloning an `Rc` or `Arc` is just a matter of incrementing a reference count.
- Smaller stack size (16 bytes vs 24). `Rc` and `Arc` only store a pointer and a length, not a capacity. If storing a large number of elements, this can add up.
- _Implements `Deref<[T]>`, just like `Vec<T>`_. This means all of the read-only operations that are possible on a `Vec<T>` are also possible on an `Rc<[T]>` or `Arc<[T]>`.

`Box<[T]>` can be an even better alternative if the data does not ever need to be cloned (cloning will result in a deep copy of the data and allocates memory). `Box<[T]>` only stores a pointer and a length, and does not have the overhead of reference counting. However, it is not possible to share ownership of a `Box<[T]>` across multiple parts of a program.

### `Cow<'a, str>`

`Cow` stands for "copy on write" and is a smart pointer that allows you to work with borrowed or owned data. It is useful when you want to avoid unnecessary copying of data.

> `Cow<'a, T>` is an enum that can hold either a borrowed reference `&'a T` or an owned value `T`. For more information, see the official documentation on [Cow](https://doc.rust-lang.org/std/borrow/enum.Cow.html).

For example, if you have a function that takes a string and returns a modified version of that string, you can use `Cow` to avoid copying the string if it is already owned:

```rust
use std::borrow::Cow;

fn sanitize(input: Cow<str>) -> Cow<str> {
    if input.contains("bad_word") {
        let sanitized = input.replace("bad_word", "****");
        return Cow::Owned(sanitized);
    }
    Cow::Borrowed(input)
}
```

### String literal

A string literal, `&'static str`, is a string slice with a static lifetime. This means that the data is stored in the binary itself and is available for the entire duration of the program.

```rust
let hello: &'static str = "I am a string literal!";
```

Most of the time it is *not* necessary to specify the `'static` lifetime of a string slice because the Rust compiler can infer it based on the context. However, there are cases where you may need to be explicit (e.g. when storing string slices in structs or enums, or when returning a string slice from a function).

#### Raw string literals

If we wanted to include special characters like double quotes or backslashes in a string literal, we would need to use an escape sequence:

```rust
let special_chars = "This is a string with \"double quotes\" and \\backslashes\\.";
```

This can become tedious in some cases like when working with regular expressions or file paths. In these cases, we can use a _raw string literal_ by prefixing the string with `r#`:

```rust
let raw_string = r#"This is a raw string with "double quotes" and \backslashes\."#;
```

#### Byte strings

Byte strings, `b"..."`, are a sequence of bytes that are not necessarily valid UTF-8. They are useful when working with binary data, like network protocols or file formats.

```rust
let http_ok: &[u8; 17] = b"HTTP/1.1 200 OK\r\n";
```

#### Combining byte strings and string literals

Byte strings and string literals can also be combined to create byte string literals:

```rust
let png_signature: &[u8; 8] = br"\x89PNG\r\n\x1a\n";
```

## Interoperability

Rust provides several types that are used for interoperability with other languages and operating systems. These types are designed to handle strings in a way that is compatible with the target platform.

### `CString` and `CStr`

These types are used to handle strings in a way that is compatible with the C programming language. `CString` is a null-terminated string that is used to pass strings to C functions that expect null-terminated strings. `CStr` is a string slice that is used to read or inspect null-terminated strings.

These types provide a safe and efficient way to work with C strings in Rust. They handle the conversion between Rust strings and C strings, including the null terminator, and ensure that the memory is properly managed.

Later we will see how to use these types to interact with functions and libraries across different languages.

### `OsString` and `OsStr`

These types are used to handle strings in a way that is compatible with operating systems. Unlike `String` and `str`, which are UTF-8 encoded, `OsString` and `OsStr` are platform-specific. This means that they can represent strings that are not valid UTF-8.

On Unix-like systems, they can contain any sequence of bytes. Or on Windows, they can contain any sequence of 16-bit values. This is useful when working with system calls that do not require or expect UTF-8 encoded strings.

### `Path` and `PathBuf`

Similar to `OsString` and `OsStr`, `Path` and `PathBuf` are specialized strings that are used for handling file system paths.

A `Path` is an immutable view of a file path, similar to a string slice. It can be used for reading or inspecting paths. A `PathBuf` is a mutable and owned version of a file path, similar to the `String` type. It can be used for building or modifying paths.

These types are useful for interoperability with the file system and for working with file paths in a platform-independent way.
