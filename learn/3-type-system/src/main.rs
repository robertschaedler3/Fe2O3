use std::collections::HashMap;

#[cfg(feature = "panic")]
fn get_viewcount_total(item: &HashMap<String, HashMap<String, String>>) -> i32 {
    i32::from_str_radix(item.get("view_count").unwrap().get("total").unwrap(), 10).unwrap()
}

#[cfg(feature = "explicit")]
fn get_viewcount_total(item: &HashMap<String, HashMap<String, String>>) -> i32 {
    if let Some(view_count) = item.get("view_count") {
        match view_count.get("total") {
            Some(total) => match total.parse::<i32>() {
                Ok(n) => n,
                Err(_) => {
                    println!("Value could not be parsed as an integer");
                    0
                }
            },
            None => {
                println!("There is no key named `total`");
                0
            }
        }
    } else {
        println!("There is no key named `view_count`");
        0
    }
}

#[cfg(feature = "idiomatic")]
fn get_viewcount_total(
    item: &HashMap<String, HashMap<String, String>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let view_count = item.get("view_count").ok_or("view_count is missing")?;
    let total = view_count.get("total").ok_or("total is missing")?;
    let n = total.parse::<i32>()?;

    Ok(n)
}

fn main() {
    let json = serde_json::json!({
        "view_count": {
            "total": "100"
        }
    });

    let data: HashMap<String, HashMap<String, String>> = serde_json::from_value(json).unwrap();

    println!("{:?}", get_viewcount_total(&data));
}
