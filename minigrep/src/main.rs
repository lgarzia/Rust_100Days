use std::env;
// use std::process;
// use minigrep::Config;
use std::fs; 
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
    
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
use std::io::{self, BufRead, BufReader, Lines};

fn read_lines_v<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where P: AnyRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let query = &args[1];
    let file_path = &args[2]; 

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read file");

    dbg!(&contents);
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
    fn read_lines_vec(filename: &str) -> Vec<String> {
        fs::read_to_string(filename) 
            .unwrap()  // panic on possible file-reading errors
            .lines()  // split the string into an iterator of string slices
            .map(String::from)  // make each slice into a string
            .collect()  // gather them together into a vector
    }
    let line_vec = read_lines_vec(&file_path);
    dbg!(&line_vec);
    // let config = Config::build(&args).unwrap_or_else(
    //     |err| {
    //         println!("problem parsing: {err}");
    //         process::exit(1)
    //     }
    // );
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // if let Err(e) = minigrep::run(config){
    //     println!("Application error: {e}");
    //     process::exit(1); 
    // }
}