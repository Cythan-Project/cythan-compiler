use std::borrow::Cow;

use super::stage1::Position;

#[derive(Debug, Clone)]
pub enum Stage2Token<'a> {
    Block(Position, Vec<Stage2Token<'a>>),
    Parenthesis(Position, Vec<Stage2Token<'a>>),
    //KeywordFn,
    Literal(Position, Cow<'a, str>),
    Assignement(Position, Cow<'a, str>),
}

use super::errors::Errors;

use super::stage1::Stage1Token;

#[inline]
pub fn compile<'a>(token: &[&Stage1Token<'a>]) -> Result<Vec<Stage2Token<'a>>, Errors> {
    let mut v = Vec::new();

    let mut in_p = 0;
    let mut in_b = 0;

    let mut p = Vec::new();

    let mut caret = None;

    for i in token {
        match i {
            Stage1Token::OpenParenthesis(position) => {
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
            Stage1Token::CloseParenthesis(position) => {
                if in_b == 0 {
                    in_p -= 1;
                    if in_p == 0 {
                        v.push(Stage2Token::Parenthesis(
                            caret.unwrap().merge(position),
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
            Stage1Token::Literal(position, e) => {
                if e.trim().is_empty() {
                    continue;
                }
                if in_p == 0 && in_b == 0 {
                    v.push(Stage2Token::Literal(position.clone(), e.clone()));
                } else {
                    p.push(*i);
                }
            }
            /*Stage1Token::KeywordFn => {
                if in_p == 0 && in_b == 0 {
                    v.push(Stage2Token::KeywordFn);
                } else {
                    p.push(*i);
                }
            }*/
            Stage1Token::OpenBrackets(position) => {
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
            Stage1Token::CloseBrackets(position) => {
                if in_p == 0 {
                    in_b -= 1;
                    if in_b == 0 {
                        v.push(Stage2Token::Block(
                            caret.unwrap().merge(position),
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
            Stage1Token::Equals(position) => {
                if in_p == 0 && in_b == 0 {
                    let tmp = v.pop();
                    if let Some(Stage2Token::Literal(position1, e)) = tmp {
                        v.push(Stage2Token::Assignement(position.merge(&position1), e));
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
