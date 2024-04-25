# Macros

One of Rust's most powerful features is its rich macro system. Rust's macros, like macros in other languages, are a way of writing code that writes other code. This is often referred to as *metaprogramming*.

Rust has two types of macros: *declarative* macros using the `macro_rules!` syntax and three kinds of *procedural* macros:

- Custom `#[derive]` macros that specify code added with the derive attribute used on structs and enums
- Attribute-like macros that define custom attributes usable on any item
- Function-like macros that look like function calls but operate on the tokens specified as their argument

Metaprogramming is useful for reducing the amount of code you have to write and maintain, similar to how functions reduce code duplication. So why not just use functions? Macros have some additional capabilities that functions do not.

A function signature must declare the number and type of parameters the function has. Macros, on the other hand, can take a variable number of parameters: we can call `println!("hello")` with one argument or `println!("hello {}", name)` with two arguments. Additionally, macros are expanded before the compiler interprets the meaning of the code, so a macro can modify code before it is compiled. For example, implementing a trait on a given type.

The downside to implementing a macro instead of a function is that macro definitions are more complex than function definitions because you're writing Rust code that writes Rust code. Due to this indirection, macro definitions are generally more difficult to read, understand, and maintain than function definitions. *It is important to consider whether the additional power provided by macros is worth the added complexity.*

<!-- https://doc.rust-lang.org/book/ch19-06-macros.html -->

## Why Macros?

Metaprogramming sounds great, but when is it useful?

Rust macros span across a wide range of use cases, but most use cases are trying to optimize the time it takes to do something. Whether it's spending less time writing complex/repetative code, or optimizing the performance of a program, macros are meant to abstract away the complexity of a task.

The following are some common use cases for macros:

- Implement a complex `trait` for a type
- Reduce duplication and boilerplate code
- A user friendly way to implement an `unsafe` function
- Reduce the amount of code needed to write tests

As mentioned earlier, macros come at a cost. They can be difficult to write, read, understand, and debug. It's important to weigh the pros and cons of using a macro before implementing one.

## Declarative Macros

The most widely used form of macros in Rust is the declarative macro. These are also sometimes referred to as "macros by example," "`macro_rules!` macros," or just plain "macros." At their core, declarative macros allow you to write something similar to a Rust `match` expression.

> The two most common declarative macros are the `println!` and `vec!` macros.

During compilation, a delcarative macro will compare a value to the patterns that are associated with particular code. In the case of declarative macros, the input value is the Rust source code passed to the macro; the patterns are compared with the structure of that input source code; and the code associated with each pattern, when matched, replaces the code passed to the macro.

To define a declarative macro, you must use the `macro_rules!` construct. Let's examine a simplified version of the `vec!` macro:

```rust
// src/lib.rs

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

> Note: The actual definition of the `vec!` macro in the standard library includes code to preallocate the correct amount of memory up front. That is an optimization that is not included here.

At compile time, this macro will expand to the following code:

```rust
// Original code
let v: Vec<i32> = vec![1, 2, 3];
```

```rust
// Expanded code
let v: Vec<i32> = {
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
};
```

But how does it work?

<!-- TODO: explain matching -->

> For more information on the `macro_rules!` syntax, see the [official documentation](https://doc.rust-lang.org/reference/macros-by-example.html) that contains a detailed explanation of the syntax and all the features available in declarative macros.

<!-- TODO: macro_export -->

## Procedural Macros

Procedural macros allow creating syntax extensions as execution of a function. Procedural macros come in one of three flavors:

- Function-like macros - `custom!(...)`
- Derive macros - `#[derive(CustomDerive)]`
- Attribute macros - `#[CustomAttribute]`

Procedural macros allow you to run code at compile time that both consume and produce Rust syntax. Procedural macros run during compilation, and thus have the same resources that the compiler has. For example, standard input, error, and output are the same that the compiler has access to.

As functions, they must either return syntax, panic, or loop endlessly. Returned syntax either replaces or adds the syntax depending on the kind of procedural macro. Panics are caught by the compiler and are turned into a compiler error. Endless loops are not caught by the compiler which hangs the compiler.

> When using Cargo, procedural macros must be defined in their own crate using the `proc-macro` crate type:
>
> ```toml
> [lib]
> proc-macro = true
> ```

### Procedural Macro Hygiene

Procedural macros are *unhygienic*. This means they behave as if the output token stream was simply written inline to the code it's next to. This means that it's affected by external items and also affects external imports.

Macro authors need to be careful to ensure their macros work in as many contexts as possible given this limitation. This often includes using absolute paths to items in libraries (for example, `::std::option::Option` instead of `Option`) or by ensuring that generated functions have names that are unlikely to clash with other functions.

### Function-like

Function-like procedural macros are procedural macros that are invoked using the macro invocation operator (`!`).

These macros are defined by a public function with the proc_macro attribute and a signature of `(TokenStream) -> TokenStream`. The input `TokenStream` is what is inside the delimiters of the macro invocation and the output `TokenStream` **replaces** the entire macro invocation.

> For a practical example of a function-like procedural macro in action, check out the [`lazy_static`](https://github.com/dtolnay/syn/tree/master/examples/lazy-static) example in the [syn](https://github.com/dtolnay/syn) repository.

### Derive

Derive macros define new inputs for the derive attribute. These macros can create new items given the token stream of a `struct`, `enum`, or `union`. They can also define derive macro [*helper attributes*](https://doc.rust-lang.org/reference/procedural-macros.html#derive-macro-helper-attributes).

Custom derive macros are defined by a public function with the `proc_macro_derive` attribute and a signature of `(TokenStream) -> TokenStream`.

The input `TokenStream` is the token stream of the item that has the derive attribute on it. The output `TokenStream` must be a set of items that are then **appended** to the module or block that the item from the input `TokenStream` is in.

> For a practical example of a derive procedural macro in action, check out the [`heapsize`](https://github.com/dtolnay/syn/tree/master/examples/heapsize) example in the [syn](https://github.com/dtolnay/syn) repository.

> For a more in-depth look at derive macros, check out the `Serialize` and `Deserialize` derive macros in the [serde](https://serde.rs/) crate.

### Attribute-like

Attribute macros define new outer attributes which can be attached to [items](https://doc.rust-lang.org/reference/items.html), including items in [`extern` blocks](https://doc.rust-lang.org/reference/items/external-blocks.html), inherent and trait [implementations](https://doc.rust-lang.org/reference/items/implementations.html), and [trait definitions](https://doc.rust-lang.org/reference/items/traits.html).

Attribute macros are defined by a public function with the `proc_macro_attribute` attribute that has a signature of `(TokenStream, TokenStream) -> TokenStream`.

- The first `TokenStream` is the delimited token tree following the attribute's name, not including the outer delimiters. If the attribute is written as a bare attribute name, the attribute `TokenStream` is empty.
- The second `TokenStream` is the rest of the item including other attributes on the item.
- The returned `TokenStream` replaces the item with an arbitrary number of items.

> For a practical example of an attribute-like procedural macro in action, check out the [`trace-var`](https://github.com/dtolnay/syn/tree/master/examples/trace-var) example in the [syn](https://github.com/dtolnay/syn) repository.

> For a more in-depth look at attribute macros, check out the `route` attribute macro definied in the [rocket](https://rocket.rs/) crate.

## Error Handling

<!-- TODO: how to handle errors in proc macros -->

## Debugging Macros

In addition to being more difficult to read and understand, macros can also be more difficult to debug. When you're writing a function, you can use `println!` or a debugger to see what's happening with your variables. Although errors can be handled nicely and the compiler can provide helpful error messages it can be difficult to understand what's going wrong with a macro while you're writing it.

Macros don't have this capability because they're expanded before the code is compiled. This often means that a faulty macro will result in a compilation error that can be difficult to understand, since the error will be in the expanded code, not the code that is written by the programmer.

<!-- https://doc.rust-lang.org/book/ch19-06-macros.html -->

---

### Resources

<!-- TODO -->

<!-- TODO: common/popular macros -->
<!-- https://github.com/dtolnay/syn/tree/master/examples -->