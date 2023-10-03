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