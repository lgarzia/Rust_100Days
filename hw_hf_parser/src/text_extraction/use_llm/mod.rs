
pub fn print_from_use_llm() -> () {
    println!("hello from use_llm");
    crate::print_from_root();
    // following doesn't work
    //crate::text_extraction::regex_me::private_print_regex_me(); 
}

