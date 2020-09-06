pub mod errors;
pub mod executor;
pub mod quick_fix;
mod stage1;
mod stage2;
mod stage3;

use errors::Errors;

pub use executor::Context;
pub use quick_fix::*;

pub fn generate_tokens(string: &str) -> Result<Vec<stage3::Stage3Token>, Errors> {
    Ok(stage3::compile(&stage2::compile(
        &string
            .lines()
            .enumerate()
            .map(|(i, x)| stage1::compile(x, i + 1))
            .flatten()
            .collect::<Vec<stage1::Stage1Token>>()
            .iter()
            .collect::<Vec<_>>(),
    )?)?)
}

pub fn generate_tokens_stage2(string: &str) -> Result<Vec<stage2::Stage2Token>, Errors> {
    stage2::compile(
        &string
            .lines()
            .enumerate()
            .map(|(i, x)| stage1::compile(x, i + 1))
            .flatten()
            .collect::<Vec<stage1::Stage1Token>>()
            .iter()
            .collect::<Vec<_>>(),
    )
}
