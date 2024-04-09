mod crypto;
mod tinker;
use std::error::Error;

use clap::Parser;

use crate::{
    crypto::{decrypt, encrypt},
    tinker::crack,
};

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
        conflicts_with = "encrypt",
    )]
    decrypt: bool,

    /// Encrypt mode
    #[arg(
        short,
        long,
        conflicts_with = "decrypt",
    )]
    encrypt: bool,

    /// Encryption file
    #[arg(short, long, required = true, value_parser = parse_file)]
    file: String,

    /// Output file
    #[arg(short, long, default_value=None)]
    outfile: Option<String>,

    #[arg(short, long, required = false, default_value = None, value_parser = parse_key)]
    key: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();

    // Open Encrypted File
    if cli.decrypt {
        let encrypted_file = cli.file;
        println!("Decrypting...");
        let out = decrypt(encrypted_file, cli.key.unwrap())?;
        let outfile = cli.outfile.unwrap_or("out.txt".to_string());
        std::fs::write(outfile, out)?;
    } else if cli.encrypt {
        println!("Encrypting...");
        let out = encrypt(cli.file, cli.key.unwrap())?;
        let outfile = cli.outfile.unwrap_or("out.txt".to_string());
        std::fs::write(outfile, out)?;
    } else {
        println!("Calling cracking fn...");
        let outkey = crack(cli.file)?;
        println!("Cracked key: {}", outkey)
    }
    Ok(())
}
