// https://docs.rs/serde_json/latest/serde_json/#

use std::collections::HashMap;
//use std::str::FromStr; //pretty confusing -- mapping Trait to 
//use std::io::Read; 

struct Todo {
    map: HashMap<String, bool>
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        // https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
        let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("db.json")?; //propogate error or return file value
        
        match serde_json::from_reader(f){
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e)
        }
        /*
        let mut content = String::new();
        f.read_to_string(&mut content)?;

        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k,v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect(); //collect::<Vec<&str>>()[doc] as described in the documentation is one of the most powerful methods in the standard library: 
                        // it transforms an iterator into a relevant collection.
        println!("{:?}", map);
        Ok(Todo { map })
         */
    }
}
impl Todo {
    fn insert(&mut self, key: String){
        // insert a new item into our map. 
        // we pass true as value
        self.map.insert(key, true); // feels like Python, navigate through self
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error >> // Result<(), std::io::Error> {
        /* 
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record) // string concatenate
        }
        println!("content-{:?}", content);
        std::fs::write("db.txt", content) // https://doc.rust-lang.org/std/fs/fn.write.html
        */
        // everything returns a result enum
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key){
            Some(v) => Some(*v = false), 
            None => None, 
        }
    }
}

fn main() {
    // https://doc.rust-lang.org/std/env/fn.args.html <-- returns an Args struct
    // https://doc.rust-lang.org/std/env/struct.Args.html <-- struct/iterator
    // says it returns a struct - I see all private fields
    //https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth <-- returns an option
    // https://doc.rust-lang.org/std/option/enum.Option.html <-- 
    // https://phaiax.github.io/rust-cheatsheet/ <-- Panics if the value is a None with a custom panic message provided by msg.
    let action = std::env::args().nth(1).expect("Please specify an action"); 
    let item = std::env::args().nth(2).expect("Please specify an item");
    println!("{:?}, {:?}", action, item);
    //let mut todo = Todo {
    //    map: HashMap::new(),
    //};
    let mut todo = Todo::new().expect("Initialization of db failed");
    if action == "add" {
        todo.insert(item); 
        match todo.save(){
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occured: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"), 
                Err(why) => println!("An error occurred: {}", why), 
            }
        }
    }
}
