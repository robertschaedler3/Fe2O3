mod tinker;
mod crypto;
use std::error::Error;

use clap::Parser;
use crypto::{decrypt1_rs, decrypt2_rs, encrypt1_rs, encrypt2_rs};

use crate::tinker::*;

fn parse_key(s: &str) -> Result<Option<String>, String> {
    if s.len() != 5 {
        Err("key must be 32 characters".to_string())
    } else {
        Ok(Some(s.to_string()))
    }
}

fn parse_file(file_name: &str) -> Result<String, String> {
    let file = std::fs::read_to_string(file_name);
    match file {
        Ok(file) => Ok(file),
        Err(_) => Err("File not found".to_string()),
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author = "Microsoft", version = "1", about, long_about = None)]
struct Args {
    /// Decrypt mode
    #[arg(
        short,
        long,
        conflicts_with = "encrypt1",
    )]
    decrypt1: bool,

    /// Encrypt mode
    #[arg(
        short,
        long,
        conflicts_with = "decrypt1",
    )]
    encrypt1: bool,
    /// Decrypt mode
    #[arg(
        short,
        long,
        conflicts_with = "encrypt2",
    )]
    decrypt2: bool,

    /// Encrypt mode
    #[arg(
        short,
        long,
        conflicts_with = "decrypt2",
    )]
    encrypt2: bool,

    /// Encryption file
    #[arg(short, long, required = true, value_parser = parse_file)]
    file: String,

    /// Output file
    #[arg(short, long, default_value=None)]
    outfile: Option<String>,

    #[arg(short, long, required = false, default_value = None, value_parser = parse_key)]
    key1: Option<i32>,
    #[arg(short, long, required = false, default_value = None, value_parser = parse_key)]
    key2: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();

    // Open Encrypted File
    if cli.decrypt1 {
        let encrypted_file = cli.file;
        println!("Decrypting...");
        let out = decrypt1_rs(encrypted_file, cli.key1.unwrap());
        let outfile = cli.outfile.unwrap_or("out.txt".to_string());
        std::fs::write(outfile, out)?;
    } else if cli.encrypt1 {
        println!("Encrypting...");
        let out = encrypt1_rs(cli.file, cli.key1.unwrap());
        let outfile = cli.outfile.unwrap_or("out.txt".to_string());
        std::fs::write(outfile, out)?;
    } else if cli.decrypt2 {
        let encrypted_file = cli.file;
        println!("Decrypting...");
        let out = decrypt2_rs(encrypted_file, cli.key2.unwrap());
        let outfile = cli.outfile.unwrap_or("out.txt".to_string());
        std::fs::write(outfile, out)?;
    } else if cli.encrypt2 {
        println!("Encrypting...");
        let out = encrypt2_rs(cli.file, cli.key2.unwrap());
        let outfile = cli.outfile.unwrap_or("out.txt".to_string());
        std::fs::write(outfile, out)?;
    } else {
        println!("Calling cracking fn...");
        let outkey = crack(cli.file)?;
        println!("Cracked key: {}", outkey)
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Run crack functions and get the key
    const key1: i32 = 10;
    const key2: &str = "rust";
    const key3: &str = "rustiscool";

    #[test]
    fn test_encrypted__1() {
        let file = "input1/encrypted1.txt";
        let result = decrypt1_rs(file.to_string(), key1.into());
        
        // Hash the result
        let result = sha256::digest(result.as_bytes());
        assert_eq!(result, "16b1a5e0e6db690416b4cc00e878ede9a2a61ef3ed3a848a4dd933fe199539b4");
    }

    #[test]
    fn test_encrypted__2() {
        let file = "input1/encrypted2.txt";
        let result = decrypt2_rs(file.to_string(), key2.into());
        
        // Hash the result
        let result = sha256::digest(result.as_bytes());
        assert_eq!(result, "16b1a5e0e6db690416b4cc00e878ede9a2a61ef3ed3a848a4dd933fe199539b4");
    }

    #[test]
    fn test_encrypted__3() {
        let file = "input1/encrypted3.txt";
        let result = decrypt2_rs(file.to_string(), key3.into());
        
        // Hash the result
        let result = sha256::digest(result.as_bytes());
        assert_eq!(result, "16b1a5e0e6db690416b4cc00e878ede9a2a61ef3ed3a848a4dd933fe199539b4");
    }
}