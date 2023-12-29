use std::{path::Path, usize};
use rusqlite::{params, Connection, Result};

pub trait DbInsert {
    fn insert_record(&self, fpath: &str) -> Result<usize, rusqlite::Error>;
}

#[derive(Debug, PartialEq)]
struct Person {
    id: i32,
    name: String
}
impl Person {
    // Associated function (runs at "struct level")
    // https://doc.rust-lang.org/std/path/struct.Path.html#
    fn intitiate(fpath:&str) -> Result<usize, rusqlite::Error> {
        let ref conn = Connection::open(fpath).unwrap();  

        let create_results = conn.execute(
            r"CREATE TABLE IF NOT EXISTS person(
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
            [], // empty list of parameters.
        );

        match create_results {
            Ok(size) => Ok(size),
            Err(error) => {
                panic!("Problem creating table: {:?}", error);
            }
        }
    }
}

// Generalize across different structures
impl  DbInsert for Person {
    fn insert_record(&self, fpath: &str)-> Result<usize, rusqlite::Error>
    {
        let conn = Connection::open(fpath).unwrap();  
        conn.execute(
            "INSERT INTO person (name) VALUES (?)",
            [&self.name])
    }
}
// Create generic process
// develp a trait - create table if not exists
// insert 
#[cfg(test)]
mod tests {

use super::*;
use std::fmt::Display;

fn insert_record<T, U>(t: &T, u: U) -> Result<usize, rusqlite::Error> 
where 
    T: Display + Copy,
    U: DbInsert, 
{
    u.insert_record(t)
}

#[test]
fn closure_check() {

    fn call_with_closure(closure: &dyn Fn(u32) -> String) -> String{
        closure(42)
    }
    let res_ = call_with_closure(&|x| format!("Hello, {}!", x));
    assert_eq!(res_, "Hello, 42!")
}

#[test]
fn func_param_check()
{
    fn call_with_function_pointer(function: fn(u32) -> String) -> String{
        function(42)
    }
    
    fn my_function(x: u32) -> String {
        format!("Hello from my_function, {}!", x)
    }
    
    assert_eq!(call_with_function_pointer(my_function), "42")
    
}

#[test]
fn insert_test()
{
    let fpath = Path::new("db").join("test.db");
    let fpath_str = fpath.to_str().unwrap();
    let is_created = Person::intitiate(fpath_str);
    assert_eq!(is_created.unwrap(), 0);
    let me = Person {
        id: 1,
        name: "Steven".to_string()
    };
    let i = me.insert_record(fpath_str).expect("insert error");
    assert_eq!(i, 1);
/*
    let mut stmt = conn.prepare("SELECT id, name FROM person").unwrap();
    let first_row = stmt.query_row([], |row| {
        Ok(Person {
            id: row.get(0).unwrap(), 
            name: row.get(1).unwrap()
                })
            });
    let you = first_row.unwrap(); 
    assert_eq!(me, you);
*/     
}


}