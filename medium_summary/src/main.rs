use clap::{Parser, Subcommand};
use shlex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands

}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get {name: String}, 
    Set {
        key: String, 
        value: String, 
        is_true: Option<bool>
    }
}

fn show_commands() {
    println!(r#"COMMANDS:
get <KEY> - Gets the value of a given key and displays it. If no key given, retrieves all values and displays them.
set <KEY> <VALUE> - Sets the value of a given key.
    Flags: --is-true
"#);
}

fn main() {
    loop {
        let mut buf = String::from("medium_summary "); //crate_name!() 

        std::io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");
        let line = buf.trim();
        let cli = shlex::split(line).ok_or("error: Invalid quoting").unwrap();
        
        println!("{:?}" , cli);
        //let cli = Args::parse();
        match Args::try_parse_from(cli.iter()).map_err(|e| e.to_string()) {
            Ok(cli) => {
                match cli.cmd {
                    Commands::Get{name} => println!("{}", name),
                    Commands::Set{key, value, is_true} => println!("{}={}={:?}", key, value, is_true)
                }        
            }
            Err(_) => {println!("This is an error");
            std::process::exit(0);
        }
            
        }
    
    }
}