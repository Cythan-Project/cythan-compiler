use crate::compiler::errors::Errors;
use crate::compiler::position::Position;
use crate::parser::basetoken::BaseToken;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum Stage2Token<'a> {
    Block(Position, Vec<Stage2Token<'a>>),
    Parenthesis(Position, Vec<Stage2Token<'a>>),
    //KeywordFn,
    Literal(Position, Cow<'a, str>),
    Assignement(Position, Cow<'a, str>),
}

#[inline]
pub fn compile<'a>(token: &[&BaseToken<'a>]) -> Result<Vec<Stage2Token<'a>>, Errors> {
    let mut v = Vec::new();

    let mut in_p = 0;
    let mut in_b = 0;

    let mut p = Vec::new();

    let mut caret = None;

    for i in token {
        match i {
            BaseToken::OpenParenthesis(position) => {
                if in_b == 0 {
                    in_p += 1;
                    if in_p != 1 {
                        p.push(*i);
                    } else {
                        caret = Some(position);
                    }
                } else {
                    p.push(*i);
                }
            }
            BaseToken::CloseParenthesis(position) => {
                if in_b == 0 {
                    in_p -= 1;
                    if in_p == 0 {
                        v.push(Stage2Token::Parenthesis(
                            caret.unwrap() + position,
                            compile(&p)?,
                        ));
                        p = Vec::new();
                    } else {
                        p.push(*i);
                    }
                } else {
                    p.push(*i);
                }
            }
            BaseToken::Literal(position, e) => {
                if e.trim().is_empty() {
                    continue;
                }
                if in_p == 0 && in_b == 0 {
                    v.push(Stage2Token::Literal(position.clone(), e.clone()));
                } else {
                    p.push(*i);
                }
            }
            /*BaseToken::KeywordFn => {
                if in_p == 0 && in_b == 0 {
                    v.push(Stage2Token::KeywordFn);
                } else {
                    p.push(*i);
                }
            }*/
            BaseToken::OpenBrackets(position) => {
                if in_p == 0 {
                    in_b += 1;
                    if in_b != 1 {
                        p.push(*i);
                    } else {
                        caret = Some(position);
                    }
                } else {
                    p.push(*i);
                }
            }
            BaseToken::CloseBrackets(position) => {
                if in_p == 0 {
                    in_b -= 1;
                    if in_b == 0 {
                        v.push(Stage2Token::Block(caret.unwrap() + position, compile(&p)?));
                        p = Vec::new();
                    } else {
                        p.push(*i);
                    }
                } else {
                    p.push(*i);
                }
            }
            BaseToken::Equals(position) => {
                if in_p == 0 && in_b == 0 {
                    let tmp = v.pop();
                    if let Some(Stage2Token::Literal(position1, e)) = tmp {
                        v.push(Stage2Token::Assignement(position + &position1, e));
                    } else {
                        return Err(Errors::EqualNotPrecededByLitteral {
                            position: position.clone(),
                        });
                    }
                } else {
                    p.push(*i);
                }
            }
        }
    }
    Ok(v)
}
