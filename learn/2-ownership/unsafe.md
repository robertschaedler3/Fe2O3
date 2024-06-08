# Manual Memory Management

Previously, we saw that Rust manages memory automatically to provide safety. But sometimes, particularly in low-level code, you might want or need to interract with memory directly.

Rust provides a way to do this with the `unsafe` keyword. The `unsafe` keyword allows you to write code that the Rust compiler can't verify is safe. This is useful when you need to interact with code that doesn't follow Rust's safety rules, like when using C libraries or hardware interfaces.

The `unsafe` keyword allows you to do things that are normally not allowed by the Rust compiler, such as:

- [Dereferencing raw pointers](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer)
- [Calling unsafe functions or methods](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#calling-an-unsafe-function-or-method)
- [Accessing or modifying static mutable variables](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#accessing-or-modifying-a-mutable-static-variable)
- [Implementing unsafe traits](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#implementing-an-unsafe-trait)
- [Accessing fields of `union`s](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#accessing-fields-of-a-union)

It's important to understand that `unsafe` doesn't turn off the borrow checker or disable any other of Rust's safety checks. The `unsafe` keyword only gives you access to these five features that are then not checked by the compiler for memory safety.

By requiring these five unsafe operations to be inside unsafe blocks, memory errors can be localized and minimized. By keeping unsafe blocks small and isolated, it is easier to debug errors when they occur.

Despite its name, `unsafe` does not mean the code inside the block is necessarily dangerous or that it will definitely have memory safety problems. Instead, it means that the code inside the block is not checked by the Rust compiler for memory safety. It is up to the programmer to ensure that the code inside the `unsafe` block is safe.

Most importantly, unsafe code is **not** a way to bypass the various rules and checks that Rust enforces, like borrow checking. Instead it is a way to enforce those rules using reasoning that is beyond the compiler. When using the `unsafe` keyword, the burden of ensuring memory safety is on the programmer.

<!-- TODO: helpful compiler/linting for unsafe https://youtu.be/8j_FbjiowvE?si=Nfq5Z4nJSnoNMCy0 -->

## Interacting with unsafe code

The `unsafe` keyword serves a dual purpose: it marks a particular function as unsafe to call, and it enables you to invoke unstae functionality in a particular code block. For example, the following function is marked as `unsafe` even though it doesn't contain any unsafe code. The `unsafe` keyword serves as a warning to the caller that there are additional guarantees that need to be upheld when invoking this function.

```rust
unsafe fn decr(x: usize) -> usize {
    x - 1
}
```

Similarly, a function can contain unsafe code without being marked as `unsafe`:

```rust
impl<T> SomeType<T> {
    pub fn as_ref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}
```

These two examples differ in the usage of `unsafe` becasue they represent different contracts. `decr` requires the caller to be careful when invoking it, while `as_ref` assumes that the caller was careful when invoking other unsafe methods (like `decr`).

One of the most fundamental reasons to use `unsafe` is to deal with Rust's raw pointer types `*const T` and `*mut T`. These are analogous to `&T` and `&mut T` references, except that they do not have lifetimes and are not subject to the same validity rules as references. Since, fewer rules apply to `*` than `&`, you can cast a reference to a pointer even outside an `unsafe` block. Only if you want to go the other way, from a pointer to a reference, do you need an `unsafe` block. Generally, a pointer is turned back into a reference to do useful things with the pointed-to data (like reading or modifying its value). For that reason, a common operation to use on pointers is `unsafe { &*ptr }` to get a reference from a pointer (this gives you a reference to what `ptr` is a pointer *to*).

### FFI (Foreign Function Interface)

Another common place for `unsafe` is when interracting with *foreign* functions or static variables defined in a language other than Rust (like a C library).

When you need to interact with code written in another language, the `extern` keyword, facilitates the creation and use of a *Foreign Function Interface (FFI)*. An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.

```c
#include <stdio.h>

void hello_from_c() {
    printf("Hello from C!\n");
}
```

```rust
#[link(name="hello")]
extern "C" {
    fn hello_from_c();
}

fn main() {
    unsafe {
        hello_from_c();
    }
}
```

The `extern` block is a list of function signatures in a foreign library, in this case with the platform's C ABI. The `#[link(...)]` attribute is used to instruct the linker to link against the `hello` library so the symbols can be resolved.

There are many tools and libraries that can help with FFI in Rust for generating bindings between languages, interracting with foreign types, and much more:

- [bindgen](https://github.com/rust-lang/rust-bindgen) - Automatically generates Rust FFI bindings to C (and some C++) libraries
- [cbindgen](https://github.com/mozilla/cbindgen) - Generates C header files from Rust source files
- [libc](https://github.com/rust-lang/libc) - Raw FFI bindings to platforms' system libraries
- [corrosion](https://github.com/corrosion-rs/corrosion) - Integrate Rust into an existing CMake project

> Using the C ABI, it is (generally) possible to use Rust with any language that can link to C libraries.

#### Creating a safe interface

<!-- TODO -->

### Using `extern` functions to call external code

Ultimately, FFI is about accessing bytes that originiate somehwere outside of your application's Rust code.

Since `extern` exists outside of Rust's control, it is inherently unsafe. If a C function is called from Rust, the Rust compiler cannot guarantee that the C function will not violate Rust's memory safety guarantees.

<!-- TODO: A little C with your Rust -->

### Using Rust from other languages

Rust can also be used from other languages. The `#[no_mangle]` attribute is used to prevent the Rust compiler from changing the name of the function. This is necessary because the Rust compiler mangles the names of functions to include information about the function's signature.

```rust
#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}
```

```c
#include <stdio.h>

int add_one(int x);

int main() {
    printf("1 + 1 = %d\n", add_one(1));
    return 0;
}
```

> A more robust way to create the bindings, is to generate them automatically using a tool like `cbindgen` as a part of the build process. This way, the bindings are always up-to-date and you don't have to worry about maintaining both the Rust implementation and the C bindings.

> For a complete example for compiling a Rust library, generating bindings, and linking it with a C program, see [A little Rust with your C](https://github.com/robertschaedler3/c-rust).
