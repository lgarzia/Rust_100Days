use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>, 
    lines: usize, 
    bytes: Option<usize>, 
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust head")
        // What goes here?
        .get_matches();

        Ok(Config {
            files: ...
            lines: ...
            bytes: ...
        })