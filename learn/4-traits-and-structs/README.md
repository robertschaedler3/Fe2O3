# Structs

Rust has three powerful types of structures that can be created using the
`struct` keyword.
- Tuple structs, which are basically named tuples
- Classic C-style structs
- Unit structs, which are field-less and useful for generics

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}
```

Methods are added to structs within the `impl` block. For example,
consider the following example:
```rust
struct Sheep;

impl Sheep {
    fn graze() {
        println!("Sheep is grazing");
    }

    fn nap() {
        println!("Sheep is napping");
    }
}
```
## Enum
The `enum` keyword in rust allows the creation of a much more powerful type
than traditional C-style enums. As seen in section 3, enums can be used to
define types that can hold variants. For example, imagine some code that is
meant to handle various events for a Web page. We could setup an enum in
the following way:
```rust
enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}
```

> This usage of enums is very common in Rust's error handling (See [Section # <>]()).
> All Result types are technically enums and branching logic can be handled
> with `match {}` statements as shown above.

## Type aliases
Rust provides the ability to create new type aliases similar to `typedef`
in C. For example:
```rust
type MyInt = u32;
```
Rust also allows to create type aliases for enums, which allow access to
enum members on the new type name. A common alias for all types are
present in the impl blocks, for example:
```rust
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
```

# Traits
While it may be helpful to think of Rust's traits as similar to interfaces,
there are subtle differences to make note of. Initially in this guide we
will use the same equivalence, but point out the subtleties in the [advanced
trait guide](#advanced).

A `trait` is simply a collection of methods defined for an unknown type:
`Self`. They can access other methods declared in the same trait. A trait
can be implemented for any data type, unlike C where that only applies to
structs and classes.

The Rust compiler also allows to derive a few well-known traits for types
using a macro `#[derive()]`, but programmers can add more complex
definitions for the trait methods manually if needed. Many arithmetic
operations are defined in this way. For example, in order to "overload
equality operators", Rust provides the `PartialEq` trait. For simple types this can
be derived, but more complex types can also be implemented manually. See
the below example:
```rust
// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

fn main() {
    let meter = Centimeters(100.0);
    let foot = Inches(12);
    if foot.to_centimeters() < meter {
        println!("Smaller");
    } else {
        println!("Bigger");
    }
}
```
Other operators that are supported by Rust have the traits `ops::Add`,
`ops::Sub`, `ops::Mul`, `ops::Div`, `ops::BitAnd`, etc. For a full list,
visit the [crate source](https://doc.rust-lang.org/core/ops/).
To implement a trait manually for a type, the `impl-for` block is used.
```rust
struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}
```

## Memory caveats
Unlike pure-virtual classes or interface types from Java/C#, the Rust
compiler needs to know example how much memory needs to be provided to
receive an object. If the incoming object is a trait, there is no
definition for the amount of memory that is needed to hold that.

There is a simple workaround: use the `Box` type. a `box` is just a
reference to some memory in the heap. Since the size of a reference is
staticalyl known, the compiler can guarantee that it points to a heap
allocated type and return a trait from a function instead of a concrete
type.

Example:
```rust
// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}
```

## Type aliases
Rust traits allow the definition of unknown type aliases as a
generification mechanism. Consider for example, the `Iterator` trait. It
defines a number of functions such as `next()`, `count()`, etc. The return
type for these is listed as Self::Item, where the Item type is defined by
the trait.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

Any type that is implementing the `Iterator` trait will first define the
type that it is to use. Note that this type could even be a generic
argument.

```rust
impl Iterator for MyCollection {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
```

## Useful traits
# Clone
The `Clone` trait is a derivable trait that helps us make copies of
resources using the `.clone()` method. This means that any assignments will
be implicitly cloned instead of moved. For example:
```rust
// A unit struct without resources
#[derive(Debug, Clone, Copy)]
struct Unit;

// A tuple struct with resources that implements the `Clone` trait
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // Instantiate `Unit`
    let unit = Unit;
    // Copy `Unit`, there are no resources to move
    let copied_unit = unit;

    // Both `Unit`s can be used independently
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    // Instantiate `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // Move `pair` into `moved_pair`, moves resources
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Error! `pair` has lost its resources
    //println!("original: {:?}", pair);
    // TODO ^ Try uncommenting this line

    // Clone `moved_pair` into `cloned_pair` (resources are included)
    let cloned_pair = moved_pair.clone();
    // Drop the moved original pair using std::mem::drop
    drop(moved_pair);

    // Error! `moved_pair` has been dropped
    //println!("moved and dropped: {:?}", moved_pair);
    // TODO ^ Try uncommenting this line

    // The result from .clone() can still be used!
    println!("clone: {:?}", cloned_pair);
}
```

## Supertraits
While Rust doesn't have "classic" inheritance, you can define a trait as
being a superset of another trait. Consider the example:
```rust
trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer 
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}
```

### Resources

- [Rust traits: Defining shared behavior](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-defining-shared-behavior)
- [Rust traits: Advanced traits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-supertraits-to-require-one-traits-functionality-within-another-trait)