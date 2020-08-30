mod stage1;
mod stage2;
mod stage3;
pub mod executor;

pub use executor::Context;

pub fn generate_tokens(string: &str) -> Vec<stage3::Stage3Token> {
    stage3::compile(&stage2::compile(&string.lines().map(|x| {
        stage1::compile(x)
    }).flatten().collect::<Vec<stage1::Stage1Token>>().iter().collect()))
}