use std::error;
use error_handling::{any_error, get_cluster_info, get_cluster_info_with_context, NegativeNonzeroInteger, PositiveNonzeroInteger};

pub fn append_hello(value: String) -> Result<String, String> {
    if value.is_empty() {
        Err("invalid format. value is empty".into())
    } else {
        Ok(format!("Hello {}", value))
    }
}

fn main() -> Result<(), anyhow::Error> {
    
    //example 1 - change the value to empty string to see the error
    println!("{:?}", append_hello("world".into()).unwrap());

    //example 2 - change the value to negative number to see the error
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    
    //example 3 - change the value to positive number to see the error
    let pretend_user_input = "-42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", NegativeNonzeroInteger::new(x)?);
    
    // example 4 - uncomment each line to see error reported by thiserror
    // get_cluster_info()?;
    // get_cluster_info_with_context()?;

    Ok(())

    // example 5 - uncomment to see error reported by anyhow
    // any_error()
}

