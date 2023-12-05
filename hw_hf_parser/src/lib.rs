//private by default
fn print_from_root() {
    println!("print from root")
}

use std::error::Error;
use clap::Parser;
mod download;
mod collections_fun;

use collections_fun::hashmap_fun as ham;

pub mod text_extraction;
//https://docs.rs/clap/latest/clap/index.html
#[derive(Parser, Debug)]
#[command(author="luke", version, about, long_about=None)]
///Boilerplate CLI
pub struct Args {
    #[arg(short, long)]
    name: String
}

pub fn get_args() -> Result<Args, Box<dyn Error>>
// pick up here - basic CLI to read name
    {
        let args = Args::parse();
        Ok(args)
    }

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    println!("Hello, {}!", args.name);
    let url = String::from("https://huggingface.co/datasets/bigcode/ta-prompt/blob/main/TA_prompt_v1.txt"); 
    let _reponse = download::get_response(url);
    text_extraction::get_text_extraction();
    ham::exp_with_hashmap();
    Ok(())
}

