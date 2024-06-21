# CTF challenge

We have an exciting challenge to get you folks tinkering
with rust. We've provided three encrypted files: encrypted[1..3].txt. Your job
is to figure out the encryption scheme for each of those and implement the functions
crack1() and crack2() under `tinker.rs`. Quick hint: Two of the files are encrypted
with the same encryption method but different keys.

## Setup
Build the project with `cargo build`. This will generate the executable file
/target/debug/tinker (tinker.exe on Windows). The executable describes how
to run itself, try `tinker --help`.

## Challenge
Implement the two functions `crack1()` and `crack2()`. The first function should return
a `Result<i32, dyn Error>`, where the Ok value is the integer key.

The second function returns a `Result<String, dyn Error>` where the Ok value is the string
key. Once both the functions have been implemented, run `cargo test` to validate that your
algorithm is correct!

To make things simpler, you can encrypt and decrypt any file (ASCII text only) using the tinker
executable that `cargo build` generates. `--encrypt1` and `--decrypt1` use the integer keys,
`--encrypt2` and `--decrypt2` use the string key type.