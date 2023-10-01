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
* 