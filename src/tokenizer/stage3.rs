#[derive(Debug, Clone)]
pub enum Stage3Token {
    Executable(String),
    FunctionExecution(String, Vec<Stage3Token>),
    FunctionCreation(String, Vec<Stage3Token>),
    VariableDefinition(String, Vec<Stage3Token>),
}

use super::stage2::Stage2Token;

#[inline]
pub fn compile(expr: &Vec<Stage2Token>) -> Vec<Stage3Token> {
    let mut output: Vec<Stage3Token> = Vec::new();

    let mut was_litteral = false;
    let mut was_assignement = false;
    let mut litteral = String::new();
    for i in expr {
        match i {
            Stage2Token::Literal(e) => {
                if e.trim().is_empty() {
                    continue;
                }
                if was_assignement {
                    println!("Can't place a literal after an assignement");
                    println!(" HERE > {}", &litteral);
                    println!("Have you forgot parenthesis ?");
                    println!(" var1 = (10)");
                    was_assignement = false;
                }
                if was_litteral {
                    output.push(Stage3Token::Executable(litteral));
                }
                was_litteral = true;
                litteral = e.clone().into_owned();
            }
            Stage2Token::Block(e) => {
                if was_assignement {
                    println!("Can't place a block after an assignement");
                    println!(" HERE > {}", &litteral);
                    println!("Have you forgot parenthesis ?");
                    println!(" var1 = (10)");
                    println!("Or added a = in your function definition");
                    println!(" fn testfunc {{");
                    println!("    10 20 10");
                    println!(" }}");
                    was_assignement = false;
                }
                if was_litteral {
                    output.push(Stage3Token::FunctionCreation(litteral, compile(e)));
                    litteral = String::new();
                    was_litteral = false;
                } else {
                    println!("A code block must be preceded by a litteral.");
                    println!("Example:");
                    println!(" test {{");
                    println!(" 0 5 6 self");
                    println!(" }}");
                    println!("Please add a litteral to create a function or remove the block");
                }
            }
            Stage2Token::Parenthesis(e) => {
                if was_litteral {
                    output.push(Stage3Token::FunctionExecution(litteral, compile(e)));
                    litteral = String::new();
                    was_litteral = false;
                } else if was_assignement {
                    output.push(Stage3Token::VariableDefinition(litteral, compile(e)));
                    litteral = String::new();
                    was_assignement = false;
                } else {
                    println!("A code function call must be preceded by a function name or add a = to make an assignement.");
                    println!("Example:");
                    println!(" test(0 1 26 var1)");
                    println!("Example:");
                    println!(" var1 = (0 1 26 var2)");
                    println!("Please add a litteral to make a function call, add a litteral and a `=` to make a variable assignement or remove both parenthesis");
                }
            }
            Stage2Token::Assignement(e) => {
                if was_assignement {
                    println!("Can't place a assignement after an assignement");
                    println!(" HERE > {}", &litteral);
                    println!("Try to unwrap your statement");
                    println!(" var1 = (10 13)");
                    println!(" var2 = (10 13)");
                }
                if was_litteral {
                    output.push(Stage3Token::Executable(litteral));
                    was_litteral = false;
                }
                was_assignement = true;
                litteral = e.clone().into_owned();
            }
            _ => {
                if was_litteral {
                    output.push(Stage3Token::Executable(litteral));
                    litteral = String::new();
                    was_litteral = false;
                } else if was_assignement {
                    was_assignement = false;
                    println!("Can't assign a variable to nothing");
                    println!("HERE >  {}", litteral);
                    println!("Example:");
                    println!(" a = (0 12)");
                    println!("Add parenthesis add a value between the parenthesis");
                }
            }
        }
    }
    if was_litteral {
        output.push(Stage3Token::Executable(litteral));
    }
    output
}
