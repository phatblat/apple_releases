use std::collections::HashMap;

fn main() {
    get("https://httpbin.org/ip".to_string())
        .unwrap();
        // .json::<HashMap<String, String>>()
        // .unwrap()
        // .get("origin")
        // .unwrap()
        // .to_string();
}

fn get(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}
