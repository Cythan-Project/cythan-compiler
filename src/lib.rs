pub mod tokenizer;

pub use tokenizer::*;

#[test]
fn test() {
    let file = r#"'start 
    0 0 0 0 0 
    
    nullOp = (0 0)
    eraseable = (999)
    'start:main()
    
    
    
    
    fn ifBin { # <input jumpA jumpB>
        '0 1
        self.0?0 ~+3
        self.1?0 1
        999 0
        self.2?0 0
    }
    
    fn add4b_for {
        # save inputs
        'add4b_inputA 'd0
        'add4b_inputA+1 'd1
        'add4b_inputA+2 'd2
        'add4b_inputA+3 'd3
    
        'add4b_start 'add4b_inputA
        'add4b_start 'add4b_inputA+1
        'add4b_start 'add4b_inputA+2
        'add4b_start 'add4b_inputA+3
        '0 1
    
    }
    
    fn add4b {
        
        'add4b_start:
    
    
        'add4b_test:
    
        
        ifBin(1,')
        
        
    
        'add4b_retenue:0
        'add4b_actA:eraseable
        'add4b_A1:~+4
        'add4b_A2:~+4
        'add4b_A3:~+4
        'add4b_A4:~+4
        'add4b_inputA:self.0..4?0
        'add4b_actB:eraseable
        'add4b_B1:~+4
        'add4b_B2:~+4
        'add4b_B3:~+4
        'add4b_B4:~+4
        'add4b_inputB:self.4..8?0
        'add4b_actO:eraseable
        'add4b_O1:~+4
        'add4b_O2:~+4
        'add4b_O3:~+4
        'add4b_O4:~+4
        'add4b_output:0 0 0 0
        'add4b_out:nullOp # get out
    }
    
    fn main {
    
    }
    
    
    
    
    '0:0
    '1:1
    '2:2
    '3:3
    'd0:0
    'd1:0
    'd2:0
    'd3:0
    'd4:0
    'd5:0
    'd6:0
    'd7:0
    'd8:0
    'd9:0"#;

    let tokens = tokenizer::generate_tokens(file);

    //println!("{:?}", &tokens);

    println!("{:?}", &tokenizer::Context::new().compute(&tokens));

    //assert_eq!(tokens.get(0).unwrap().term(), "hello");
}
