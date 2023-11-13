mod crypto;
mod tinker;
use std::error::Error;

use clap::Parser;

use crate::{
    crypto::{decrypt, encrypt},
    tinker::crack,
};

fn parse_key(s: &str) -> Result<String, String> {
    if s.len() != 5 {
        Err("key must be 32 characters".to_string())
    } else {
        Ok(s.to_string())
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
        conflicts_with = "encrypt",
        required_unless_present = "encrypt"
    )]
    decrypt: bool,

    /// Encrypt mode
    #[arg(
        short,
        long,
        conflicts_with = "decrypt",
        required_unless_present = "decrypt"
    )]
    encrypt: bool,

    /// Encryption file
    #[arg(short, long, required = true, value_parser = parse_file)]
    file: String,

    /// Output file
    #[arg(short, long, required = true)]
    outfile: String,

    #[arg(short, long, required = true, value_parser = parse_key)]
    key: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();

    // Open Encrypted File
    if cli.decrypt {
        let encrypted_file = cli.file;
        println!("Decrypting...");
        let out = decrypt(encrypted_file, cli.key)?;
        std::fs::write(cli.outfile, out)?;
    } else if cli.encrypt {
        println!("Encrypting...");
        let out = encrypt(cli.file, cli.key)?;
        std::fs::write(cli.outfile, out)?;
    } else {
        println!("Calling cracking fn...");
        let outkey = crack(cli.file)?;
        println!("Cracked key: {}", outkey)
    }
    Ok(())
}
