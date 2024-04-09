use std::error::Error;

fn encdec(c: char, keychar: char, dec: bool) -> char {
    if dec {
        let mut cphr = c as i32 - 'a' as i32;
        let keychar_i32 = keychar as i32 - 'a' as i32;
        cphr = if cphr - keychar_i32 < 0 {
            26 + (cphr - keychar_i32)
        } else {
            cphr - keychar_i32
        };
        cphr += 'a' as i32;
        println!("Decipher char [{}], shift [{}], output [{}]", c as u8 as char, keychar_i32, cphr as u8 as char);
        return cphr as u8 as char;
    } else {
        let c_i32 = c as i32;
        let keychar_i32 = keychar as i32 - 'a' as i32;
        let cphr = (c_i32 - 'a' as i32 + keychar_i32) % 26 + 'a' as i32;
        println!("Cipher char {}, shift {}. output {}", c as u8 as char, keychar_i32, cphr as u8 as char);
        return cphr as u8 as char;
    }
}

fn encryptdecrypt(str: String, key: String, dec: bool) -> Result<String, Box<dyn Error>> {
    let mut index = 0;
    let mut collector = Vec::new();
    let keychars = key.as_bytes();
    for i in str.as_str().chars() {
        if i.is_alphabetic() {
            let c = i.to_ascii_lowercase();
            let keyindex = index % 5;
            collector.push(
                encdec(
                    c,
                    keychars[keyindex] as char,
                    dec,
                ),
            );
            index += 1;
        } else if i.is_ascii() {
            collector.push(i);
        }
    }
    return Ok(collector.into_iter().collect());
}

pub fn decrypt(encrypted_string: String, key: String) -> Result<String, Box<dyn Error>> {
    return encryptdecrypt(encrypted_string, key.to_ascii_lowercase(), true);
}

pub fn encrypt(plain_string: String, key: String) -> Result<String, Box<dyn Error>> {
    return encryptdecrypt(plain_string, key.to_ascii_lowercase(), false);
}
