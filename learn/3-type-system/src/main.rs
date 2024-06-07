use std::collections::HashMap;

#[cfg(feature="run1")]
fn fails_to_run(item: &HashMap<String, HashMap<String, String>>) {
    let int_repr = i32::from_str_radix(
        item.get("view_count").unwrap()
            .get("total").unwrap(),
        10
    ).unwrap();
}

#[cfg(feature="run2")]
fn let_match(item: &HashMap<String, HashMap<String, String>>) -> i32{
    if let Some(view_count) = item.get("view_count") {
        match view_count.get("total") {
            Some(total) => {
                match total.parse::<i32>() {
                    Ok(n) => (),
                    Err(_) => {
                        // The value could not be parsed as an integer
                        println!("Value could not be parsed as an integer");
                        return 0;
                    }
                }
            },
            None => {
                // There is no `total` key
                println!("There is no key named total");
                return 0;
            }
        }
    
    } else {
        // There is no `view_count` key
        println!("There is no key named view_count");
        return 0;
    }
    return 0;
}

#[cfg(feature="run3")]
fn use_options(item: &HashMap<String, HashMap<String, String>>) -> Result<(), std::error> {
    let view_count = item.get("view_count").ok_or("view_count is missing")?;
let total = view_count.get("total").ok_or("total is missing")?;
let n = total.parse::<i32>()?;
}

fn main() {
    let item: HashMap<String, HashMap<String, String>> = Default::default();

    #[cfg(feature="run1")]
    fails_to_run(&item);

    #[cfg(feature="run2")]
    let_match(&item);

    #[cfg(feature="run3")]
    use_options(&item)?;
}
