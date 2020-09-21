use super::value::Expression;

#[derive(Debug, Clone)]
pub enum Stage3Token {
    Executable(Position, Expression),
    FunctionExecution(Position, String, Vec<Stage3Token>),
    FunctionCreation(Position, String, Vec<Stage3Token>),
    VariableDefinition(Position, String, Vec<Stage3Token>),
}

use super::stage1::Position;

use super::stage2::Stage2Token;

use super::errors::Errors;

pub fn compile_caret(caret: &Option<&Position>, merge: &Position) -> Position {
    if let Some(e) = caret {
        e.merge(merge)
    } else {
        merge.clone()
    }
}

#[inline]
pub fn compile(expr: &[Stage2Token]) -> Result<Vec<Stage3Token>, Errors> {
    let mut output: Vec<Stage3Token> = Vec::new();

    let mut was_litteral = false;
    let mut was_assignement = false;
    let mut litteral = String::new();

    let mut literal_caret: Option<&Position> = None;

    for i in expr {
        match i {
            Stage2Token::Literal(position, e) => {
                if e.trim().is_empty() {
                    continue;
                }
                if was_assignement {
                    return Err(Errors::LiteralAfterAssignement {
                        position: literal_caret.expect("A").merge(position),
                        literal: e.clone().into_owned(),
                        assignement: litteral,
                    });
                }
                if was_litteral {
                    output.push(Stage3Token::Executable(
                        literal_caret.expect("B").clone(),
                        Expression::from_string(&litteral, literal_caret.expect("B"))?,
                    ));
                }
                was_litteral = true;
                litteral = e.clone().into_owned();
                literal_caret = Some(position);
            }
            Stage2Token::Block(position, e) => {
                if was_assignement {
                    return Err(Errors::BlockAfterAssignement {
                        position: compile_caret(&literal_caret, position),
                        assignement: litteral,
                    });
                }
                if was_litteral {
                    output.push(Stage3Token::FunctionCreation(
                        compile_caret(&literal_caret, position),
                        litteral,
                        compile(e)?,
                    ));
                    litteral = String::new();
                    was_litteral = false;
                } else {
                    return Err(Errors::BlockMustBePrecededByLiteral {
                        position: position.clone(),
                    });
                }
            }
            Stage2Token::Parenthesis(position, e) => {
                if was_litteral {
                    output.push(Stage3Token::FunctionExecution(
                        compile_caret(&literal_caret, position),
                        litteral,
                        compile(e)?,
                    ));
                    litteral = String::new();
                    was_litteral = false;
                } else if was_assignement {
                    output.push(Stage3Token::VariableDefinition(
                        compile_caret(&literal_caret, position),
                        litteral,
                        compile(e)?,
                    ));
                    litteral = String::new();
                    was_assignement = false;
                } else {
                    return Err(Errors::ParenthesisNotInAssignementOrFunctionCall {
                        position: position.clone(),
                    });
                }
            }
            Stage2Token::Assignement(position, e) => {
                if was_assignement {
                    return Err(Errors::AssignementFollowedByAnotherAssignement {
                        position: compile_caret(&literal_caret, position),
                        assignement1: litteral,
                        assignement2: e.clone().into_owned(),
                    });
                }
                if was_litteral {
                    output.push(Stage3Token::Executable(
                        literal_caret.expect("H").clone(),
                        Expression::from_string(&litteral, literal_caret.expect("B"))?,
                    ));
                    was_litteral = false;
                }
                literal_caret = Some(position);
                was_assignement = true;
                litteral = e.clone().into_owned();
            }
        }
    }
    if was_litteral {
        output.push(Stage3Token::Executable(
            literal_caret.expect("I").clone(),
            Expression::from_string(&litteral, literal_caret.expect("B"))?,
        ));
    }
    Ok(output)
}

impl Stage3Token {
    pub fn to_string(&self) -> Vec<String> {
        match self {
            Self::Executable(_, e) => vec![format!("{:?}", e)],
            Self::FunctionCreation(_, e, c) => {
                let mut v = Vec::new();
                v.push(String::new());
                v.push(format!("{} {{", e));
                v.extend(
                    c.iter()
                        .map(|x| x.to_string())
                        .flatten()
                        .map(|x| format!("    {}", x)),
                );
                v.push("}}".to_owned());
                v.push(String::new());
                v
            }
            Self::VariableDefinition(_, e, c) => {
                if c.len() > 3 {
                    let mut v = Vec::new();
                    v.push(format!("{} = (", e));
                    for x in c {
                        for y in x.to_string() {
                            v.push(format!("    {}", y.to_string()));
                        }
                    }
                    v.push(")".to_owned());
                    v
                } else {
                    vec![format!(
                        "{} = ({})",
                        e,
                        c.iter()
                            .map(|x| x.to_string())
                            .flatten()
                            .collect::<Vec<String>>()
                            .join(" ")
                    )]
                }
            }
            Self::FunctionExecution(_, e, c) => {
                if c.len() > 3 {
                    let mut v = Vec::new();
                    v.push(format!("{}(", e));
                    for x in c {
                        for y in x.to_string() {
                            v.push(format!("    {}", y.to_string()));
                        }
                    }
                    v.push(")".to_owned());
                    v
                } else {
                    vec![format!(
                        "{}({})",
                        e,
                        c.iter()
                            .map(|x| x.to_string())
                            .flatten()
                            .collect::<Vec<String>>()
                            .join(" ")
                    )]
                }
            }
        }
    }
}
