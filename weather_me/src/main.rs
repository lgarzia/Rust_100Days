mod weather_data;
mod sqlite_util;
mod reflect_util;

use serde_json::from_str;
use std::env;
use toml::Table;
use reqwest;
use weather_data as ws;

fn main() {
    println!("Hello, world!");
    //Note current working directory is folder not location of main.rs
    let current_dir = env::current_dir().expect("Failed to get current working directory");
    println!("Current working directory: {}", current_dir.display());
    //https://doc.rust-lang.org/std/path/struct.PathBuf.html
    let toml_path = current_dir.join("src").join("weather_conf.toml");
    let toml_content = std::fs::read_to_string(toml_path)
    .expect("Failed to read TOML file");
    println!("{:?}", toml_content);
    let confs = toml_content.parse::<Table>().unwrap();
    // https://docs.rs/toml/latest/toml/type.Table.html
    println!("{:?}", confs);
    //https://docs.rs/toml/latest/toml/enum.Value.html
    let api_key = confs["keys"]["OpenWeather"].as_str().unwrap();
    let lat = 38.781151;
    let lon = -90.486931;
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={api_key}", lat=lat, lon=lon, api_key = api_key);
    //https://doc.rust-lang.org/std/macro.format.html
    println!("{}",url);
    // https://docs.rs/reqwest/latest/reqwest/
    let body = reqwest::blocking::get(url).expect("");
    
    let text: String = body.text().expect(&"".to_string());
    let weather_data: ws::WeatherData = from_str(&text).expect("failed to parse");
    //println!("text ref - {:?}", text);
    //let data:ws::WeatherData = body.json().expect("failed parsing");
    println!("{:?}", weather_data)
}
