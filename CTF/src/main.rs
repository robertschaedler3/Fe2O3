mod tinker;
mod crypto;
use std::error::Error;

use clap::Parser;
use crypto::{decrypt1_rs, decrypt2_rs, encrypt1_rs, encrypt2_rs};

use crate::tinker::*;

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
        long,
        conflicts_with = "encrypt1",
        conflicts_with = "encrypt2",
        conflicts_with = "decrypt2",
    )]
    decrypt1: bool,

    /// Encrypt mode
    #[arg(
        long,
        conflicts_with = "decrypt1",
        conflicts_with = "encrypt2",
        conflicts_with = "decrypt2",
    )]
    encrypt1: bool,
    /// Decrypt mode
    #[arg(
        long,
        conflicts_with = "encrypt2",
        conflicts_with = "encrypt1",
        conflicts_with = "decrypt1",
    )]
    decrypt2: bool,

    /// Encrypt mode
    #[arg(
        long,
        conflicts_with = "decrypt2",
        conflicts_with = "encrypt1",
        conflicts_with = "decrypt1",
    )]
    encrypt2: bool,

    /// Encryption file
    #[arg(short, long, required = true, value_parser = parse_file)]
    file: String,

    /// Output file
    #[arg(short, long, default_value=None)]
    outfile: Option<String>,

    #[arg(long, required = false, default_value = None)]
    key1: Option<i32>,
    #[arg(long, required = false, default_value = None)]
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
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_encrypted_1() {
        let file = "./src/encrypted1.txt";
        let contents = fs::read_to_string(file).unwrap();
        let key1 = crack1(contents.clone()).unwrap();
        let result = decrypt1_rs(contents, key1);
        // Hash the result
        let result = sha256::digest(result.as_bytes());
        assert_eq!(result, "91bfb1666ea5046c6c105f666f3dd70720c0569125abaaa6e911b581f15cd5bd");
    }

    #[test]
    fn test_encrypted_2() {
        let file = "./src/encrypted2.txt";
        let contents = fs::read_to_string(file).unwrap();
        let key1 = crack2(contents.clone()).unwrap();
        let result = decrypt2_rs(contents, key1);
        
        // Hash the result
        let result = sha256::digest(result.as_bytes());
        assert_eq!(result, "3ce8c7e86f4d32a14fcbb8b5bb82c0c584ee409ff7536e8f968e6540c5077706");
    }

    #[test]
    fn test_encrypted_3() {
        let file = "./src/encrypted3.txt";
        let contents = fs::read_to_string(file).unwrap();
        let key1 = crack2(contents.clone()).unwrap();
        let result = decrypt2_rs(contents, key1);
        
        // Hash the result
        let result = sha256::digest(result.as_bytes());
        assert_eq!(result, "3ce8c7e86f4d32a14fcbb8b5bb82c0c584ee409ff7536e8f968e6540c5077706");
    }
}