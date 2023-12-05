mod regex_me; 
pub mod use_llm; 

pub fn get_text_extraction() -> ()
{
    println!("hello from text_extraction");
    regex_me::print_regex_me();
    use_llm::print_from_use_llm();
}

