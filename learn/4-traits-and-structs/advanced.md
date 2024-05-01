# Advanced topis: Traits vs. Interfaces
 A few notes about Rust's traits:
 - * Traits are Rust's sole notion of interface*: A trait can be
     implemented by multiple types and new traits can provide
     implementations for existing types.
 - * Traits can be statically dispatched*: Like C++ templates, you can have
     the compiler generate a separate copy of an abstractions for each wau
     it is instantiated.
 - * Traits can be dynamically dispatched*: Sometimes you really do need an
     indirection, and so it doesn't make sense to "erase" an abstraction at
     runtime. The same notion of interface -- the trait -- can also be used
     when you want to dispatch at runtime.
 - * Traits solve a variety of additional problems beyond simple
     abstraction*: This is the key difference between traits and
     interfaces. Traits can be used as "markers" for types, like the `Send`
     marker. They can be used to define "extension methods" -- that is, to
     add methods to an externally-defined type. They largely obviate the
     need for traditional method overloading. And they provide a simple
     scheme for operator overloading.

Consider the following example:
```rust
trait Hash {
    fn hash(&self) -> u64;
}
```

We can implement this trait for any type, based on the rules of traits in
Rust. Note that unlike Java and C#, we can implement this for *any existing
type* as well.
```rust
impl Hash for bool {
    fn hash(&self) -> u64 {
        if *self { 0 } else { 1 }
    }
}

impl Hash for i64 {
    fn hash(&self) -> u64 {
        *self as u64
    }
}
```
## Static dispatch
While consuming a trait, things can get interesting. Consider the following
function:
```rust
fn print_hash<T: Hash>(t: &T) {
    println!("The hash is {}", t.hash());
}
```
The `print_hash` function is generic over an unknown type T, but requires
that T implements the `Hash` trait. This means we can use it with `bool`
and `i64` values.

*Generics are compiled away, resulting in static dispatch.* That is, as
with C++ templates, the compiler will generate two copies of the
print_hash method to handle the above code, one for each concrete argument
type. That in turn means that the internal call to `t.hash()` -- the point
where the abstraction is actually used -- has zero cost: it will be
compiled to a direct, static call to the relevant implementation:

```
// The compiled code:
__print_hash_bool(&true);  // invoke specialized bool version directly
__print_hash_i64(&12_i64);   // invoke specialized i64 version directly
```

## Dynamic dispatch
Sometimes abstractions are necessary. Let's see it with an example:
```rust
trait Backend {
    fn compute(number: i32) -> i32;
}

struct Service <T: Backend> {
    backend: Vec<T>
}
...

let mut backends = Vec::new();
backends.push(TypeA);
backends.push(TypeB);  // <---- Type error here
```
But there is a way to solve this in Rust: Dynamic Dispatch. Trait objects
can be thought of like objects of an Interface Type in Java, defining
common functionality for the Types implementing them. When using a trait
object, we don’t care what exact type is used, we just make sure that given
functionality is present.

The above example could be rewritten to use trait objects like this:

```rust
struct Service{
    backends: Vec<Box<dyn Backend>>
}
...
let mut backends = Vec::new();
backends.push( Box::new(PositiveBackend{}) as Box<dyn Backend>);
backends.push( Box::new(NegativeBackend{}) as Box<dyn Backend>);
```

## Closures
A closure is a function that can directly use variables from the scope in
which it is defined. In Rust, this is an anonymous function defined with
pipes `|arguments...| body`. Rust utilizes traits to implement function
closure types in various possible ownership models.

Each closure implicitly implements some combination of three traits:
`Fn`, `FnOnce`, and `FnMut`.
There’s three traits, and so seven non-empty sets of traits that could6 possibly be implemented… but there’s actually only three interesting configurations:

- Fn, FnMut and FnOnce,
- FnMut and FnOnce,
- only FnOnce.
Why? Well, the three closure traits are actually three nested sets: every
closure that implements Fn can also implement FnMut (if &self works, &mut
self also works; proof: &*self), and similarly every closure implementing
FnMut can also implement FnOnce. This hierarchy is enforced at the type
level, e.g. FnMut has declaration:
```rust
pub trait FnMut<Args>: FnOnce<Args> {
    ...
}
```
In words: anything that implements FnMut must also implement FnOnce.

There’s no subtlety required when inferring what traits to implement as the
compiler can and will just implement every trait for which the
implementation is legal. This is in-keeping with the “offer maximum
flexibility” rule that was used for the inference of the capture types,
since more traits means more options. The subset nature of the Fn* traits
means that following this rule will always result in one of the three sets
listed above being implemented.

As an example, this code demonstrates a closure for which an implementation of Fn is illegal but both FnMut and FnOnce are fine.

```rust
let mut v = vec![];

// nice form
let closure = || v.push(1);

// explicit form
struct Environment<'v> {
    v: &'v mut Vec<i32>
}

// let's try implementing `Fn`
impl<'v> Fn() for Environment<'v> {
    fn call(&self) {
        self.v.push(1) // error: cannot borrow data mutably
    }
}
let closure = Environment { v: &mut v };
```
It is illegal to mutate via a & &mut ..., and &self is creating that outer
shared reference. If it was &mut self or self, it would be fine: the
former is more flexible, so the compiler implements FnMut for closure (and
also FnOnce).

Similarly, if closure was to be `|| drop(v);` --— that is, move out of `v`
--- it would be illegal to implement either Fn or FnMut, since the &self
(respectively &mut self) means that the method would be trying to steal
ownership out of borrowed data: criminal.

## Markers
Rust has a handful of "markers" that classify types: Send, Sync, Copy,
Sized. These markers are just traits with empty bodies, which can then be
used in both generics and trait objects. Markers can be defined in
libraries, and they automatically provide #[derive]-style implementations:
if all of a types components are Send, for example, so is the type.

Not everything obeys inherited mutability. Some types allow you to have
multiple aliases of a location in memory while mutating it. Unless these
types use synchronization to manage this access, they are absolutely not
thread-safe. Rust captures this through the Send and Sync traits.

- A type is Send if it is safe to send it to another thread.
- A type is Sync if it is safe to share between threads (T is Sync if and
  only if &T is Send).

Send and Sync are fundamental to Rust's concurrency story. As such, a
substantial amount of special tooling exists to make them work right.
First and foremost, they're unsafe traits. This means that they are unsafe
to implement, and other unsafe code can assume that they are correctly
implemented. Since they're marker traits (they have no associated items
like methods), correctly implemented simply means that they have the
intrinsic properties an implementor should have. Incorrectly implementing
Send or Sync can cause Undefined Behavior.

Send and Sync are also automatically derived traits. This means that,
unlike every other trait, if a type is composed entirely of Send or Sync
types, then it is Send or Sync. Almost all primitives are Send and Sync,
and as a consequence pretty much all types you'll ever interact with are
Send and Sync.

Major exceptions include:

- raw pointers are neither Send nor Sync (because they have no safety guards).
- UnsafeCell isn't Sync (and therefore Cell and RefCell aren't).
- Rc isn't Send or Sync (because the refcount is shared and
  unsynchronized).

### References
- [Rust blog: Traits](https://blog.rust-lang.org/2015/05/11/traits.html)
- [Rust closures](https://huonw.github.io/blog/2015/05/finding-closure-in-rust/)
- [Rust dynamic dispatch](https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b)
- [Rust markers: Send + Sync](https://doc.rust-lang.org/nomicon/send-and-sync.html)