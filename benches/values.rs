use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cythan_compiler::tokenizer::errors::Errors;
use cythan_compiler::tokenizer::stage1::*;
use cythan_compiler::tokenizer::value::*;

fn compile(n: &str) -> Result<Expression, Errors> {
    Expression::from_string(
        n,
        &Position {
            line_from: 10,
            line_to: 10,
            caret_from: 10,
            caret_to: 10,
        },
    )
}

fn criterion_benchmark(c: &mut Criterion) {
    let tests = vec![
        "~+1",
        "'label:~+1",
        "'label+1",
        "'label",
        "var",
        "10",
        "var.1..10",
        "'label:'label1:var.0+10..10-9?'label2+10-20+40",
    ];
    for i in tests {
        c.bench_function(&format!("compile({})", i), |b| {
            b.iter(|| compile(black_box(i)))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
