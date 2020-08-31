#![feature(test)]

extern crate test;

use test::*;

pub mod tokenizer;

pub use tokenizer::*;

#[test]
fn test() {
    let file = r#"0 12 2 3

    # This is an example
    
    3 # Test
    # Test of a multi line comment!
    # de

    a = (2 10)

    b = (4 a)
    
    test_func {
        1 3 9 6 8 9 ~-1 self.0 self.0..12?1 self.1?6 b
    }
    
    test_func(23)
    
    4 25 26 23 b"#;

    let tokens = tokenizer::generate_tokens(file);

    println!("{:?}",tokens);
    
    println!("{:?}",&tokenizer::Context::new().compute(&tokens));

    //assert_eq!(tokens.get(0).unwrap().term(), "hello");
}

#[bench]
fn bench_tokenize(bench: &mut Bencher) {
    let file = r#"0 12 2 3

    # This is an example
    
    3 # Test
    # Test of a multi line comment!
    # de

    a = (2 10)

    b = (4 a)
    
    test_func {
        1 3 9 6 8 9 self.0 self.0..12?1 self.1?6 b
    }
    
    test_func(23)
    
    4 25 26 23 b"#;
    bench.iter(|| {
        let a = tokenizer::generate_tokens(file);
    })
}

#[bench]
fn bench_compute(bench: &mut Bencher) {
    let file = r#"0 12 2 3

    # This is an example
    
    3 # Test
    # Test of a multi line comment!
    # de

    a = (2 10)

    b = (4 a)
    
    test_func {
        1 3 9 6 8 9 self.0 self.0..12?1 self.1?6 b
    }
    
    test_func(23)
    
    4 25 26 23 b"#;
    let a = tokenizer::generate_tokens(file);
    bench.iter(|| {
        tokenizer::Context::new().compute(&a)
    })
}

#[bench]
fn bench_whole(bench: &mut Bencher) {
    let file = r#"0 12 2 3

    # This is an example
    
    3 # Test
    # Test of a multi line comment!
    # de

    a = (2 10)

    b = (4 a)
    
    test_func {
        1 3 9 6 8 9 self.0 self.0..12?1 self.1?6 b
    }
    
    test_func(23)
    
    4 25 26 23 b"#;
    bench.iter(|| {
        tokenizer::Context::new().compute(&tokenizer::generate_tokens(file))
    })
}