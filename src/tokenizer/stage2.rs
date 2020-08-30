
use std::borrow::Cow;

#[derive(Debug,Clone)]
pub enum Stage2Token<'a> {
    Block(Vec<Stage2Token<'a>>),
    Parenthesis(Vec<Stage2Token<'a>>),
    KeywordUse,
    KeywordFn,
    Number(u32),
    Literal(Cow<'a, str>),
    Assignement(Cow<'a, str>)
    
}

use super::stage1::Stage1Token;

#[inline]
pub fn compile<'a>(token: &Vec<&Stage1Token<'a>>) -> Vec<Stage2Token<'a>> {
    let mut v = Vec::new();

    let mut in_p = 0;
    let mut in_b = 0;

    let mut p = Vec::new();

    for i in token {
        match i {
            Stage1Token::OpenParenthesis => {
                if in_b == 0 {
                    in_p+=1;
                    if in_p != 1 {
                        p.push(*i);
                    }
                } else {
                    p.push(*i);
                }
            },
            Stage1Token::CloseParenthesis => {
                if in_b == 0 {
                    in_p-=1;
                    if in_p == 0 {
                        v.push(Stage2Token::Parenthesis(compile(&p)));
                        p = Vec::new();
                    } else {
                        p.push(*i);
                    }
                } else {
                    p.push(*i);
                }
            },
            Stage1Token::Number(e) => {
                if in_p == 0 && in_b == 0 {
                    v.push(Stage2Token::Number(*e));
                } else {
                    p.push(*i);
                }
            },
            Stage1Token::Literal(e) => {
                if in_p == 0 && in_b == 0 {
                    v.push(Stage2Token::Literal(e.clone()));
                } else {
                    p.push(*i);
                }
            },
            Stage1Token::KeywordFn => {
                if in_p == 0 && in_b == 0 {
                    v.push(Stage2Token::KeywordFn);
                } else {
                    p.push(*i);
                }
            },
            Stage1Token::KeywordUse => {
                if in_p == 0 && in_b == 0 {
                    v.push(Stage2Token::KeywordUse);
                } else {
                    p.push(*i);
                }
            },
            Stage1Token::OpenBrackets => {
                if in_p == 0 {
                    in_b+=1;
                    if in_b != 1 {
                        p.push(*i);
                    }
                } else {
                    p.push(*i);
                }
            },
            Stage1Token::CloseBrackets => {
                if in_p == 0 {
                    in_b-=1;
                    if in_b == 0 {
                        v.push(Stage2Token::Block(compile(&p)));
                        p = Vec::new();
                    } else {
                        p.push(*i);
                    }
                } else {
                    p.push(*i);
                }
            },
            Stage1Token::Equals => {
                if in_p == 0 && in_b == 0 {
                    let tmp = v.pop();
                    if let Some(Stage2Token::Literal(e)) = tmp {
                        v.push(Stage2Token::Assignement(e));
                    } else {
                        println!("a `=` must be followed by a litteral");
                        println!("Example:");
                        println!(" var2 = (0 20 var1)");
                        println!("Please add a name for the variable or remove the =");
                    }
                } else {
                    p.push(*i);
                }
            }
        }
    }
    v
}