# Tinker challenge

We have an exciting challenge to get you folks tinkering
with rust. Provided with this directory is an encrypted file.
Someone from the crypto team (let's say James) put this encryption algorithm
together and claimed this was a stronger encryption than AES256.
The source for the encryption/decryption algorithm has been
provided, take a look and try to prove James wrong.

## Setup
Build the project with `cargo build`. This will generate the executable file
/target/debug/tinker (tinker.exe on Windows). The executable describes how
to run itself, try `tinker --help`.

## Challenge
Find the key used to encrypt the file. The key will always be a 5 letter
alphabetical key. Implement your solution in the file
`tinker/src/tinker.rs` inside the function body `crack`. The function
receives the encrypted file and outputs the cracked key as per the
signature below:

`pub fn crack(encrypted_text: String) -> Result<String, Box<dyn Error>> {`

The first person to come up with a solution will win *the prize*