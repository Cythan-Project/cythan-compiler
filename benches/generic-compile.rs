use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cythan_compiler::tokenizer::errors::Errors;
use cythan_compiler::*;

fn criterion_benchmark(c: &mut Criterion) {
    let tests = r#"'start
    0 0 0 0 0 0 0 0 0 # you can test number from 0 to 9
    # Default pointors
    '#0:0
    '#1:1
    '#2:2
    '#3:3
    '#4:4
    '#5:5
    '#6:6
    '#7:7
    '#8:8
    '#9:9
    _0 = ('#0)
    _1 = ('#1)
    _2 = ('#2)
    _3 = ('#3)
    _4 = ('#4)
    _5 = ('#5)
    _6 = ('#6)
    _7 = ('#7)
    _8 = ('#8)
    _9 = ('#9)
    
    # delimiter for compiled version (to see the result better)
    7070
    # return value from functions
    
    '#return_0:0
    '#return_1:0
    '#return_2:0
    '#return_3:0
    '#return_4:0
    '#return_5:0
    '#return_6:0
    '#return_7:0
    '#return_8:0
    '#return_9:0
    
    # delimiter for compiled version (to see the result better)
    7070
    
    
    no_op = (0 0)
    earasable = (999)
    
    stop = (~+2 0 ~-2) # Stop the program
    sure_stop = (~+2 0 ~-2 ~+2 0 ~-2 ) # Stop the program even if pt is missplaced
    
    # self.0: case to test
    # will match self.0 to self.1 => jump self.2, self.2 => jump self.3
    # Don't test for other thing that 0-9, because it will corrupt memory at all possibilitys position
    # use safe_switch_<number_of_values> for a safe switch
    switch {
        self.0 'test
        self.1..
        'test:earasable 0
    }
    
    # self.0: case to test
    # self.1: first element to test, jump to self.2 if true
    # self.3: 2nd element to test, jump to self.4 if true
    safe_switch_2val {
        self.0 'test
        # save and prepare cases
        self.1 '1_save
        'case1 self.1
        self.3 '2_save
        'case2 self.3
        'test:earasable 0 # test
        # restore and jump
        'case1:'1_save self.1 ~+2 0 self.2
        'case2:'2_save self.3 ~+2 0 self.4
        '1_save:0
        '2_save:0
    }
    
    # self.0: case to test
    # self.1: first element to test, jump to self.2 if true
    # self.3: 2nd element to test, jump to self.4 if true
    # etc.. until self.7 => self.8
    safe_switch_4val {
        self.0 'test
        # save and prepare cases
        self.1 '1_save
        'case1 self.1
        self.3 '2_save
        'case2 self.3
        self.5 '3_save
        'case3 self.5
        self.7 '4_save
        'case4 self.7
        'test:earasable 0 # test
        # restore and jump
        'case1:'1_save self.1 ~+2 0 self.2
        'case2:'2_save self.3 ~+2 0 self.4
        'case3:'3_save self.5 ~+2 0 self.6
        'case4:'4_save self.7 ~+2 0 self.8
        '1_save:0
        '2_save:0
        '3_save:0
        '4_save:0
    }
    
    # jump to self.0
    jump {
        ~+2 0 self.0
    }
    
    # jump to self.0 even if pt missplaced, or center pt if self.0 not precised
    sure_jump {
        ~+3 ~+2 0 self.0?~+1
    }
    
    
    add_4b {
        'start_loop:no_op
        
        #  ---- shift data ---
        'inputA0 'actA
        'inputA1 'inputA0
        'inputA2 'inputA1
        'inputA3 'inputA2
        'inputA4 'inputA3
        
        # test 'actA == 2 (end)
        switch ('actA 2 'end 1 '_ok)
        '_ok:no_op
        
        'inputB0 'actB
        'inputB1 'inputB0
        'inputB2 'inputB1
        'inputB3 'inputB2
        
        'output0 'actOut
        'output1 'output0
        'output2 'output1
        'output3 'output2
        
        # --- add ---
        switch ('retenue 1 'r1 0 'r0)
        
        'r1:switch('actA 1 'r1a1 0 'r1a0)
        'r0:switch('actA 1 'r0a1 0 'r0a0)
        
        'r1a1:switch ('actB 1 'result_11 0 'result_10)
        'r1a0:switch ('actB 1 'result_10 0 'result_01)
        'r0a1:switch ('actB 1 'result_10 0 'result_01)
        'r0a0:switch ('actB 1 'result_01 0 'result_00)
        
        'result_00:jump('start_loop)
        'result_01:'actOut ~+2 _1 earasable jump('start_loop)
        'result_10:_1 'retenue jump('start_loop)
        'result_11:_1 'retenue 'actOut ~+2 _1 earasable jump('start_loop)
        
        # --- data ---
        'actA:0
        'inputA0:self.0
        'inputA1:self.1
        'inputA2:self.2
        'inputA3:self.3
        'inputA4:2
        
        'actB:0
        'inputB0:self.4
        'inputB1:self.5
        'inputB2:self.6
        'inputB3:self.7
        
        'actOut:0
        'output0:'#return_0
        'output1:'#return_1
        'output2:'#return_2
        'output3:'#return_3
        
        'retenue:self.8?0
        'end:no_op
    }
    
    'start:no_op
    
    add_4b(0 1 0 1 1 0 1 1)
    
    sure_stop"#;
    c.bench_function("compile_adder4()", |b| b.iter(|| compile(black_box(tests))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
