# Rust_100Days
 100 Hundred - Minor, Incremental, Growth

Plan --> Mostly read for first 30 days or so
Goal --> Create container API service in GCP

Resources 
[1] The Rust Programming Language, 2nd Edition
9/27/2023: 1 - Finished Chapter 1 and Started Chapter 2
* rustup
* cargo
* regsitry: https://crates.io
* binary crate versus library crate
* Result Object -> Value or Error (Err)
* State of Result is Variant
* **&**mut guess <-- reference & mutable
* let var = value; <-- mutable
* var = value; <-- immutable
* package::lirary <--- from package import library
* cargo command line (cargo run, cargo build, etc. )
* new development - use cargo like javascript
* Cargo in src
* Cargo.TOML
* parckages called crates
* cargo new --vcs <-- turns off setting up Git 
* cargo new_project
* cd new_project
* cargo is rust build system, package manager, ...
* function**!** <-- ! makes it a macro
* rustc main.rs > .\main.exe <--- compile and then runs
* string - f-string -> "string {var}"
* https://www.rust-lang.org/community
* rustc --version

9/28/2023: 2 - Finished chapter 2
* Cargo.toml
 [dependencies] rand = "0.8.5"
* registry from https://crates.io
* cargo.lock
* cargo update
* use rand::Rng <-- Rng trait defines methods; train must be in scope to use methods
* rand::thead_rng <-- part of trait
* std::cmp::Ordering <-- bring into scope
* Ordering::less <-- enum
* match <var>.comp(&var) <-- match syntax (arms - patterns to match againsts)
* loop <-- keyword
* let var u32 = match guess.trim().parse() {
    Ok(num) => num, Err(_) => continue
}

9/29/2023: 3 - Chapter 3 (mid-point)
* keywords const, mut <--- default is immutable
* constants can be declared in any scope -
* constant THREE_HOURS_IN_SECONDS: u32
* let f: bool = false <-- Rust has default data types, optional to specify
* char literals single quotes versus string literals double quotes
* array type (fixed), vector type dynamic
* Rust ensures only memory block is used
* function syntax
 `fn <func_name>() {
    ...
 } 
 * function signatures
` <func_name>(value:i32, unit_label)`
* Rust is an expression-based language
* Call function is an expression; macro; result of scope block
* function - must declare return type -> 
* Rust leverage shadow-casting concept 

9/30/2023: 4 - Chapter 3 Competed
* return keyword - optional
* <func_name>() -> i32
* ; statement - doesn't return value
* // comments
* if <cond> {} else {}
* <cond> must be bool, rust won't determine truthiness
* else if {}
* book recommends `match` complex branching
* if condition return results 
number = if condition {5} else {6}; 
* 3 loops --> loop, while, for
* loop --> forever or until `break`
* continue
* let result = loop
* optional specify loop label - and break to label
* `while loop`
* for element in a

10/1/2023: 5 - Chapter 4 mid-point
* Ownership is Rust's most unique feature
* Rust makes memory safety guarantees without needing garbage collectors
* Ownership is a set of rules that govern how a Rust program manages memory
* Memory managed through a system of ownership that compiler checks
* Stack vs Heap
    * Stack stores vaues in the order it gets them (stack of plates)
    * Heap less organized - request certain amount of space
    * Heap - memory allocators finds and empty spot return pointer
    * "allocating on the heap"
* ownership manages life-cycle on heap
* Each value in Rust has an owner
* Only one owner at a time
* Owner goes out of scope, the value is dropped
* Rust calls drop automatically at the closing curly bracket
* after s2 = s1 <--- s1 is invalid
* default is shallow copy
* `clone` allows for deep copies
* Rust does tuple unpacking for functions
* Use `references` using value without transfering ownership
* let len = <func>(**&**s1)
* Signature of & of function uses& to indicate the type of parameter
* Act of creating reference is `borrowing`
* References are immutable, not allowed to modify something we have reference to

10/2/2023: 6 - Finished Chapter 4
* concepts of ownership, borrowing, and slices ensure memory saftey in Rust programs at compile time
* references are immutable - can't modify value
* fn <func_name>(s: &str) -> &str
* &str <-- see lower case; same function on both &String, &str
* Slices --> return &s[0..i];
* string slice is a reference to part of a String
* use patterns to destructure tuple
* slices - reference contiguous sequence of elements
* Rust - compiler guarantees references will never be dangling references
* scope ends after last used; 
* multiple immutable references are allowed
* {let r1 = &mut s;} // leaves scope
* r2 = &mut s; // is allowed
* Rust prevents data races at compile time
* Only 1 mutable reference to a value allowed at a time
* &mut var

10/3/2023: 7 - Chapter 5
* Structs and enums - are the building blocks
* struct User {active: bool, username: String, email: String, sign_in_count: u64}
* another example {active: true, username: String::from("someusername123")}
* use . to get specific value
* Rust does not allow only certain fields as mutable
* ..user1 must come last to specify that any remaining fields should get their values from the corresponding fields in user1
* `tuple structs` - structs look like tuple
* `unit-like structs` have no fields, behave similiarly to ()
* struct Rectangle {width: u32, height: u32}
* fn area(rectangle: &Rectangle) -> u32 {}

10/4/2023: 8 - Finished Chapter 5
* default curly brackets tell println! to use formatting known 
as Display: output intended for direct end user consumption
* Rust doesn't try to guess print out of struct
* `#[derive(Debug)] struct...`
* `println!("rect1 is {:?}")`
* another way to print out a value using debug formats is dbg!
* width: dbg!(30*scale)
* Methods declared with fn keywords
* ```impl Rectangle {
    fn area(&self) -> u32 
    {self.width*self.height}
}```
* rect1.area()
* &self <--- don't want to take ownership, just read data
* &mut self
* example below ... rect1.can_hold(&rect2))
* impl Rectangle {... 
fn can_hold(&self, other:&Rectangle) -> book{...}}
* All functions defined within an impl block are called associated functions
* impl Rectangle {fn ...) -> Self{Self {...}}}
* Self keyword in the return type and in the body of the function are aliases for the type that appears after impl
* To call associated function - let sq = Rectangle::square(3)
* The :: syntax is used for both asssociated functions and namespaces created by module
* Structs allowed to have multiple impl blocks
* In impl blocks, you can define functions that are associated with your type, and methods
are a kind of associated function that let you specify behavior that instances of your structs have

10/5/2023: 9 - Chapter 6 - Enums
* Enums - define type by enumerating variants
* Option is an enum (expresses value either something or nothing)
* enum IpAddrKind { V4, V6 }
* let four = IpAddrKind::V4; 
* fn rount(ip_kind: IpAddrKind){}
* enum IpAddr { V4(String), V6(String), }
* IpAddr::V4() - takes string argument returns an instance of IpAddr
* Advantage over structs; each variant can have different types and amounts of associated data
* can use impl to define methods
* Rust compiler - checks whether you've handled all the cases
* Rust doesn't have a null feature
* Rust has an enum that can encode concept of a value being present or not
* enum Option<T> {None, Some(T), }
* Option is part of *prelude*; don't need to bring into scope explicitly
* enum Coin {}
* match coin {Coin::Penny => 1, ...}

10/6/2023: 10 - finished chapter 6 - Enum, started ch 7, Started O'reilly book
* crate is the smallest amount of code that Rust compiler considers at a time
* single file is a crate
* paths - a way of naming an item, such as struct, function, or module
* package - build, test, share crates
* crates - a tree of modules that produces a library or executable
* modules & use - control the organization, scope, and privacy of paths
* scope - nested context
* if let cond=cond {...} else {...}
* Rust has pattern catch-all - _ is a special pattern that matches any value
* pattern-matching... can take else and turn into variable --- other => move_player(other)
* Rust compilers ensures all path are covered
* combining match & enums very common
* In Rust, Some() is an enum variant that is used to represent the presence of a value in an Option<T> type.
* Option<T> is a built-in Rust type that is used to represent the possibility of a value being absent. It has two variants: Some(T) and None. Some(T) is used to wrap a value of type T when it is present, while None is used to indicate the absence of a value.

10/7/2023: 11 - half-way chapter 7
* Items in a parent module can't use the private items inside child modules
* But child modules can use the items in their ancestor modules
* preference specify absolute paths
* crate's public API, marked with pub
* pub fn eat_at_restaurant
* support relative paths & absolute paths
* mod front_of_house { mod hosting { fn ...}}
* code in module is private be default
* Modules - used to organize code
* use keyword brings path into scope
* Cargo follows a convention that src/ main.rs is the crate root of a binary crate
* Library crates don't have a main function, they don't compile to an executable

10/8/2023: 12 - starting from top, working through code snippets
10/9/2023: 13 - con't part 2 - working through code snippets
10/10/2023: 14 - finished chapt 3, orielly set up website

