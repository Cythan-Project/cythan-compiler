pub mod tokenizer;

pub use tokenizer::*;

pub use tokenizer::errors::Errors;

#[test]
fn test() {
    let tokens =
        tokenizer::generate_tokens(&std::fs::read_to_string("cythan-in.ct").unwrap()).unwrap();
    std::fs::write(
        "cythan-clean.ct",
        tokens
            .iter()
            .map(|x| x.to_string())
            .flatten()
            .fold(String::new(), |a, b| a + "\r\n" + &b)
            .replace("}\r\n\r\n", "}\r\n"),
    )
    .unwrap();

    //println!("{:?}", &tokens);

    println!("{:?}", &tokenizer::Context::new().compute(&tokens));

    //assert_eq!(tokens.get(0).unwrap().term(), "hello");
}
