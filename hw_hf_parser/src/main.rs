 use hw_hf_parser::text_extraction::use_llm;

 fn main() {
    if let Err(e) = hw_hf_parser::get_args().and_then(hw_hf_parser::run) { 
        eprintln!("{}", e); 
        std::process::exit(1); 
    }
    use_llm::print_from_use_llm() 
}
