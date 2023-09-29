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

