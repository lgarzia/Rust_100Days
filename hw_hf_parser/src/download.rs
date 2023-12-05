// https://crates.io/crates/reqwest

//use std::collections::HashMap;

pub fn get_response(url: String)-> Result<(), Box<dyn std::error::Error>> {
    println!("in function call {}", url);
//    let resp = reqwest::blocking::get(url)?.text()?;
//        .json::<HashMap<String, String>>()?;
//    println!("in function before response");
//    println!("{}", resp);
    Ok(())
}
