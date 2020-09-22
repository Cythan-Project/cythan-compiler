use crate::compiler::errors::Errors;
use crate::executor::executor::Executor;
use crate::parser::{basetoken, instruction, stage2token};

mod compiler;
mod expression;
mod parser;

mod executor;

pub use crate::compiler::errors::*;

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
    Executor::default().compute(&generate_tokens(&input)?)
}

pub fn generate_tokens(string: &str) -> Result<Vec<instruction::Instruction>, Errors> {
    Ok(instruction::compile(&stage2token::compile(
        &string
            .lines()
            .enumerate()
            .map(|(i, x)| basetoken::compile(x, i + 1))
            .flatten()
            .collect::<Vec<basetoken::BaseToken>>()
            .iter()
            .collect::<Vec<_>>(),
    )?)?)
}

pub fn generate_tokens_stage2(string: &str) -> Result<Vec<stage2token::Stage2Token>, Errors> {
    stage2token::compile(
        &string
            .lines()
            .enumerate()
            .map(|(i, x)| basetoken::compile(x, i + 1))
            .flatten()
            .collect::<Vec<basetoken::BaseToken>>()
            .iter()
            .collect::<Vec<_>>(),
    )
}
