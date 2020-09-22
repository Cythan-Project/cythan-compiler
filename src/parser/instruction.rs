use crate::compiler::errors::Errors;
use crate::compiler::position::Position;
use crate::expression::expression::LiteralExpression;
use crate::parser::stage2token::Stage2Token;

#[derive(Debug, Clone)]
pub enum Instruction {
    Executable(Position, LiteralExpression),
    FunctionExecution(Position, String, Vec<Instruction>),
    FunctionCreation(Position, String, Vec<Instruction>),
    VariableDefinition(Position, String, Vec<Instruction>),
}

impl Instruction {
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

fn compile_caret(caret: &Option<&Position>, merge: &Position) -> Position {
    if let Some(e) = caret {
        *e + merge
    } else {
        merge.clone()
    }
}

#[inline]
pub fn compile(expr: &[Stage2Token]) -> Result<Vec<Instruction>, Errors> {
    let mut output: Vec<Instruction> = Vec::new();

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
                        position: literal_caret.expect("A") + position,
                        literal: e.clone().into_owned(),
                        assignement: litteral,
                    });
                }
                if was_litteral {
                    output.push(Instruction::Executable(
                        literal_caret.expect("B").clone(),
                        LiteralExpression::from_string(&litteral, literal_caret.expect("B"))?,
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
                    output.push(Instruction::FunctionCreation(
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
                    output.push(Instruction::FunctionExecution(
                        compile_caret(&literal_caret, position),
                        litteral,
                        compile(e)?,
                    ));
                    litteral = String::new();
                    was_litteral = false;
                } else if was_assignement {
                    output.push(Instruction::VariableDefinition(
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
                    output.push(Instruction::Executable(
                        literal_caret.expect("H").clone(),
                        LiteralExpression::from_string(&litteral, literal_caret.expect("B"))?,
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
        output.push(Instruction::Executable(
            literal_caret.expect("I").clone(),
            LiteralExpression::from_string(&litteral, literal_caret.expect("B"))?,
        ));
    }
    Ok(output)
}
