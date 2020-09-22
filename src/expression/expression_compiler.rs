use tokesies::{FilteredTokenizer, Token, filters};
use crate::compiler::position::Position;
use crate::parser::tokens::{CompilationToken, Phase2Token, Phase1Token};
use crate::compiler::errors::Errors;

pub struct LiteralCompiler;

impl filters::Filter for LiteralCompiler {
    fn on_char(&self, c: &char) -> (bool, bool) {
        match *c {
            ':' | '.' | '?' | '+' | '-' | '~' => (true, true),
            _ => (false, false),
        }
    }
}

pub fn compile(literal: &str, position: &Position) -> Result<Vec<CompilationToken>, Errors> {
    let tokens = FilteredTokenizer::new(LiteralCompiler {}, literal).collect::<Vec<Token>>();
    let mut output: Vec<CompilationToken> = Vec::new();
    for token in tokens {
        let t = token.term.into_owned();
        if t == "." {
            match output.last() {
                Some(CompilationToken::Phase1Token(Phase1Token::Dot)) => {
                    output.pop();
                    if let Some(CompilationToken::Phase2Token(Phase2Token::Number(number))) =
                    output.last()
                    {
                        let number = *number;
                        output.pop();
                        output.push(CompilationToken::Phase2Token(Phase2Token::Range(
                            number as isize,
                            None,
                        )));
                        continue;
                    } else {
                        return Err(Errors::SelfExpressionMissingNumber {
                            expression: literal.to_owned(),
                            position: position.clone(),
                        });
                        // panic!("A range must have a number before it! example: 0..")
                    }
                }
                Some(CompilationToken::Phase2Token(Phase2Token::Variable(_))) => {
                    let n = if let CompilationToken::Phase2Token(Phase2Token::Variable(n)) =
                    output.pop().unwrap()
                    {
                        n
                    } else {
                        println!("Zone17");
                        return Err(Errors::ExpressionCompilingError {
                            expression: literal.to_owned(),
                            position: position.clone(),
                        });
                    };
                    output.push(CompilationToken::Phase2Token(Phase2Token::VariableIndexed(
                        n,
                    )));
                    continue;
                }
                _ => {
                    output.push(CompilationToken::Phase1Token(Phase1Token::Dot));
                }
            }
        } else if t == ":" {
            match output.last() {
                // TODO: Check why is added is not used and if it is normal
                Some(CompilationToken::Phase2Token(Phase2Token::Label(_, _))) => {
                    let (label, _added) =
                        if let CompilationToken::Phase2Token(Phase2Token::Label(label, added)) =
                        output.pop().unwrap()
                        {
                            (label, added)
                        } else {
                            println!("Zone1");
                            return Err(Errors::ExpressionCompilingError {
                                expression: literal.to_owned(),
                                position: position.clone(),
                            });
                        };
                    output.push(CompilationToken::Phase2Token(Phase2Token::LabelAssign(
                        label,
                    )));
                }
                _ => {
                    println!("Zone2");
                    return Err(Errors::ExpressionCompilingError {
                        expression: literal.to_owned(),
                        position: position.clone(),
                    });
                    //println!("Synthax error : can't be placed before elsewhere than <label>:");
                }
            }
        } else if t == "~" {
            output.push(CompilationToken::Phase1Token(Phase1Token::Relative));
        } else if t == "+" {
            output.push(CompilationToken::Phase1Token(Phase1Token::NumberSign(
                false,
            )));
        } else if t == "-" {
            output.push(CompilationToken::Phase1Token(Phase1Token::NumberSign(true)));
        } else if t == "?" {
            output.push(CompilationToken::Phase1Token(Phase1Token::QuestionMark));
        } else if let Ok(number) = t.parse::<usize>() {
            match output.last() {
                Some(CompilationToken::Phase1Token(Phase1Token::QuestionMark)) => {
                    output.pop();
                    output.push(CompilationToken::Phase2Token(Phase2Token::Or(number)))
                }
                Some(CompilationToken::Phase2Token(Phase2Token::Range(range_start, _))) => {
                    let range_start = *range_start;
                    output.pop();
                    output.push(CompilationToken::Phase2Token(Phase2Token::Range(
                        range_start,
                        Some(number as isize),
                    )));
                }
                Some(CompilationToken::Phase1Token(Phase1Token::NumberSign(sign))) => {
                    let sign = *sign;
                    output.pop();
                    match output.last() {
                        Some(CompilationToken::Phase2Token(Phase2Token::OrRelative(added))) => {
                            let added = *added;
                            output.pop();
                            output.push(CompilationToken::Phase2Token(Phase2Token::OrRelative(
                                if sign {
                                    added - (number as isize)
                                } else {
                                    added + number as isize
                                },
                            )));
                        }
                        Some(CompilationToken::Phase1Token(Phase1Token::Relative)) => {
                            output.pop();
                            if let Some(CompilationToken::Phase1Token(Phase1Token::QuestionMark)) =
                            output.last()
                            {
                                output.pop();
                                output.push(CompilationToken::Phase2Token(
                                    Phase2Token::OrRelative(if sign {
                                        -(number as isize)
                                    } else {
                                        number as isize
                                    }),
                                ));
                            } else {
                                output.push(CompilationToken::Phase2Token(Phase2Token::Relative(
                                    if sign {
                                        -(number as isize)
                                    } else {
                                        number as isize
                                    },
                                )));
                            }
                        }
                        Some(CompilationToken::Phase2Token(Phase2Token::Label(_, _))) => {
                            let (label, added) = if let CompilationToken::Phase2Token(
                                Phase2Token::Label(label, added),
                            ) = output.pop().unwrap()
                            {
                                (label, added)
                            } else {
                                println!("Zone3");
                                return Err(Errors::ExpressionCompilingError {
                                    expression: literal.to_owned(),
                                    position: position.clone(),
                                });
                            };
                            output.push(CompilationToken::Phase2Token(Phase2Token::Label(
                                label,
                                if sign {
                                    added - (number as isize)
                                } else {
                                    added + number as isize
                                },
                            )));
                        }
                        Some(CompilationToken::Phase2Token(Phase2Token::OrLabel(_, _))) => {
                            let (label, added) = if let CompilationToken::Phase2Token(
                                Phase2Token::OrLabel(label, added),
                            ) = output.pop().unwrap()
                            {
                                (label, added)
                            } else {
                                println!("Zone4");
                                return Err(Errors::ExpressionCompilingError {
                                    expression: literal.to_owned(),
                                    position: position.clone(),
                                });
                            };
                            output.push(CompilationToken::Phase2Token(Phase2Token::OrLabel(
                                label,
                                if sign {
                                    added - (number as isize)
                                } else {
                                    added + number as isize
                                },
                            )));
                        }
                        Some(CompilationToken::Phase2Token(Phase2Token::Number(n))) => {
                            let n = *n;
                            output.pop();
                            output.push(CompilationToken::Phase2Token(Phase2Token::Number(
                                number as isize + n,
                            )));
                        }
                        Some(CompilationToken::Phase2Token(Phase2Token::Range(start, end))) => {
                            let start = *start;
                            let end = *end;
                            output.pop();
                            output.push(CompilationToken::Phase2Token(Phase2Token::Range(
                                start,
                                Some(end.unwrap_or(0) + number as isize),
                            )));
                        }
                        _ => {
                            println!("Zone5");
                            return Err(Errors::ExpressionCompilingError {
                                expression: literal.to_owned(),
                                position: position.clone(),
                            });
                        }
                    }
                }
                Some(CompilationToken::Phase2Token(Phase2Token::VariableIndexed(_))) => output
                    .push(CompilationToken::Phase2Token(Phase2Token::Number(
                        number as isize,
                    ))),
                Some(CompilationToken::Phase2Token(Phase2Token::LabelAssign(_))) => output.push(
                    CompilationToken::Phase2Token(Phase2Token::Number(number as isize)),
                ),
                None => output.push(CompilationToken::Phase2Token(Phase2Token::Number(
                    number as isize,
                ))),
                _ => {
                    println!("Zone6");
                    return Err(Errors::ExpressionCompilingError {
                        expression: literal.to_owned(),
                        position: position.clone(),
                    });
                }
            }
        } else if t.starts_with('\'') {
            let t = t[1..t.len()].to_owned();
            if let Some(CompilationToken::Phase1Token(Phase1Token::QuestionMark)) = output.last() {
                output.pop();
                output.push(CompilationToken::Phase2Token(Phase2Token::OrLabel(t, 0)));
            } else {
                output.push(CompilationToken::Phase2Token(Phase2Token::Label(t, 0)));
            }
        } else {
            output.push(CompilationToken::Phase2Token(Phase2Token::Variable(t)));
        }
    }
    Ok(output)
}