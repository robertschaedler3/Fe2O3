# Error Handling

Rust doesn't provide a comprehensive exception handling primitive. Instead, it provides the simplest primitive: the `Error` trait.

```rust
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
    fn description(&self) -> &str { ... }
    fn cause(&self) -> Option<&dyn Error> { ... }
    fn provide<'a>(&'a self, request: &mut Request<'a>) { ... }
}
```

## Return types in `Result`

As you have seen before, `Ok` and `Err` are the two variants of `Result`. In this example, `append_hello` returns a `Result` of return type `String` and error type `String`. 

```rust
pub fn append_hello(value: String) -> Result<String, String> {
    if value.is_empty() {
        Err("invalid format. value is empty".into())
    } else {
        Ok(format!("Hello {}", value))
    }
}
```

> Dev tip: Defining `Result` error type as `String` is sometimes useful for trace debugging scenarios. Yet, it is considered an anti-pattern. "Structured" errors are preferred for easier handling by calling code.

## Standard practices in error handling

The standard practice in Rust is to structure errors based on the application domain (e.g. `CreationError` below) so that clients of the module can react to various error conditions.

```rust
#[derive(PartialEq, Debug)]
pub enum CreationError {
    Negative,
    Zero,
}

impl error::Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}
```

## The `?` operator 

The `?` operator makes error handling ergonomic in Rust. It shorthands much of what a developer would do with a `match` statement on the callsite, such as the `value.parse::<i32>()` in the example below, where the matched cases are `Ok(something)` and `Err(something)`. The `map_err` is commonly used to convert expected errors at callsites to another well-known error (in the example below, `ParseIntError` is convert to a `String` type).

```rust
pub fn convert_to_num(value: &str) -> Result<i32, String> {
    let qty = value
        .parse::<i32>()
        .map_err(|_: ParseIntError| "invalid digit found in string")?; //this is equivalent of doing match on the parse() result

    Ok(qty)
}
```

In methods where capturing precise errors returned from call sites isn't significant, such as the main method, it is possible to use a boxed trait to bubble up the error (in this case, it bubbles upto the console). 

```rust
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
```

> Dev tip: Under the hood, the `?` operator calls `From::from` on the error value to convert it to a boxed trait object, a `Box<dyn error::Error>`. This boxed trait object is polymorphic, and since all errors implement the `error::Error` trait, we can capture lots of different errors in one "Box" object.

When calling third-party crates, one way to handle errors is using a `match` statement on the error throwing (Result returning code), such as the `value.parse::<i32>()`, where the matched cases are `Ok(something)` and `Err(something)`.



It's common to return something like `Result<(), ErrorType>` from your main() function, even though it isn't required. The unit (`()`) type is useful to return in these cases.


```rust
fn main() -> Result<(), ParseIntError>{ ... }
```

## Error definition styles

### Rust ergonomics of `String`s as `Error` types

`String` is unique due to its relation with Rust standard library `fmt::Debug` and `fmt::Display` traits. The `Error` trait inherits from these traits as seen in the `Error` trait definition above.

```rust
// traits inside Rust standard library core fmt module/ std::fmt
pub trait Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;
}

pub trait Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;
}
```

`Box<dyn Error>` implements the `From::from()` method, which makes it ergonomic to convert `&str` or `String` into `Error`. This results in the ability to report strings as errors without defining "structured" errors as follows (line 3): 

```rust
pub fn error() -> Result<(), String> {
    Err("invalid format. value is empty".into())
}
```


### Structured errors

#### Manual method

The bare mimimum trait implementation for a custom error defined within a module requires a human-readable description of the error.  

```rust
use std::fmt;

#[derive(Debug)]
enum MyError {
    GizmoError,
    WidgetNotFoundError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::GizmoError => write!(f, "A gizmo error occurred!"),
            MyError::WidgetNotFoundError(name) => write!(f, "Could not find widget '{}'", name),
        }
    }
}
```

#### thiserror crate

The `thiserror` crate simplifies the process of defining custom error types. It automatically derives traits like `Debug`and `Display`.

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("A gizmo error occurred!")]
    GizmoError,
    #[error("Could not find widget '{name}'")]
    WidgetNotFoundError { name: String },
}
```

#### anyhow crate

The `anyhow` crate provides the `anyhow::Error`, a trait object based error type for easy idiomatic error handling in Rust applications. It improves productivity when the module author has multiple calllsites that return different types of errors, requiring context propagation and minimal boilerplate error handling.  

```rust
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ClusterMap { 
    x: u32,
    y: u32
}

fn get_cluster_info() -> Result<ClusterMap> {
    let config = std::fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}

fn main() -> Result<()> {
    let path = "./path/to/instrs.json";
    let _content = std::fs::read(path)
        .with_context(|| format!("Failed to read instrs from {}", path))?;
    
    Ok(())
}

```

Here is an example of `anyhow::Result` being used to chain different errors within a single function (fn any_error example below chains ) 

```rust
fn string_error() -> Result<()> {
    Err(anyhow::anyhow!("invalid format. value is empty"))
}

pub fn io_error() -> Result<()> {
    Err(anyhow::anyhow!(std::io::Error::new(std::io::ErrorKind::Other, "invalid IO packet")))
}

fn any_error() -> Result<()> {
    string_error()?;
    io_error()?;
    Ok(())
}

```

