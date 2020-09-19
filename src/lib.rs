pub mod tokenizer;

pub use tokenizer::*;

pub use tokenizer::errors::Errors;

#[test]
fn run() {
    match compile(&std::fs::read_to_string("cythan-in/cythan.ct").unwrap()) {
        Ok(e) => {
            println!("{:?}", e);
        }
        Err(e) => {
            println!("{}", e.to_pretty_print());
            println!("{:?}", e.get_fixes());
        }
    }
    //
}

pub fn compile(input: &str) -> Result<Vec<usize>, Errors> {
    Context::default().compute(&tokenizer::generate_tokens(&input)?)
}
