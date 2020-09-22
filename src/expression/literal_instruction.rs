use crate::compiler::position::Position;
use crate::expression::range::Range;
use crate::parser::tokens::Phase2Token;
use crate::expression::defaultvalue::DefaultValue;
use crate::compiler::errors::Errors;

#[derive(Debug, Clone)]
pub enum LiteralInstruction {
    Variable(String, Option<Range>),
    Label(String, isize),
    Value(usize),
    Relative(isize),
}


impl LiteralInstruction {
    pub(crate) fn from_stage2(
        tokens: &[Phase2Token],
        expression: &str,
        position: &Position,
    ) -> Result<Self, Errors> {
        let mut labels: Vec<&Phase2Token> = tokens
            .iter()
            .filter(|x| !matches!(x, Phase2Token::LabelAssign(_)))
            .collect();
        if labels.is_empty() {
            panic!(
                "FATAL ERROR HOW AN EXPRESSION CAN HAVE NULL PARAMS {:?}",
                tokens
            );
        }
        Ok(match labels.remove(0) {
            Phase2Token::Label(string, added) => Self::Label(string.to_owned(), *added),
            Phase2Token::Number(number) => Self::Value(*number as usize),
            Phase2Token::Relative(number) => Self::Relative(*number),
            Phase2Token::Variable(variable) => {
                if labels.is_empty() {
                    Self::Variable(variable.to_owned(), None)
                } else {
                    match labels.remove(0) {
                        Phase2Token::Or(number) => Self::Variable(
                            variable.to_owned(),
                            Some(Range {
                                start: 0,
                                end: Some(1),
                                or: Some(DefaultValue::Value(*number)),
                            }),
                        ),
                        Phase2Token::OrLabel(label, added) => Self::Variable(
                            variable.to_owned(),
                            Some(Range {
                                start: 0,
                                end: Some(1),
                                or: Some(DefaultValue::Label(label.to_owned(), *added)),
                            }),
                        ),
                        Phase2Token::OrRelative(added) => Self::Variable(
                            variable.to_owned(),
                            Some(Range {
                                start: 0,
                                end: Some(1),
                                or: Some(DefaultValue::Relative(*added)),
                            }),
                        ),
                        _ => {
                            println!("Zone8");
                            return Err(Errors::ExpressionCompilingError {
                                position: position.clone(),
                                expression: expression.to_owned(),
                            });
                        }
                    }
                }
            }
            Phase2Token::VariableIndexed(variable) => {
                if labels.is_empty() {
                    println!("Zone9");
                    return Err(Errors::ExpressionCompilingError {
                        position: position.clone(),
                        expression: expression.to_owned(),
                    });
                } else {
                    match labels.remove(0) {
                        Phase2Token::Range(start, end) => {
                            if labels.is_empty() {
                                Self::Variable(
                                    variable.to_owned(),
                                    Some(Range {
                                        start: *start as usize,
                                        end: end.map(|x| x as usize),
                                        or: None,
                                    }),
                                )
                            } else {
                                match labels.remove(0) {
                                    Phase2Token::Or(number) => Self::Variable(
                                        variable.to_owned(),
                                        Some(Range {
                                            start: *start as usize,
                                            end: end.map(|x| x as usize),
                                            or: Some(DefaultValue::Value(*number)),
                                        }),
                                    ),
                                    Phase2Token::OrLabel(label, added) => Self::Variable(
                                        variable.to_owned(),
                                        Some(Range {
                                            start: *start as usize,
                                            end: end.map(|x| x as usize),
                                            or: Some(DefaultValue::Label(label.to_owned(), *added)),
                                        }),
                                    ),
                                    Phase2Token::OrRelative(added) => Self::Variable(
                                        variable.to_owned(),
                                        Some(Range {
                                            start: *start as usize,
                                            end: end.map(|x| x as usize),
                                            or: Some(DefaultValue::Relative(*added)),
                                        }),
                                    ),
                                    _ => {
                                        println!("Zone10");
                                        return Err(Errors::ExpressionCompilingError {
                                            expression: expression.to_owned(),
                                            position: position.clone(),
                                        });
                                    }
                                }
                            }
                        }
                        Phase2Token::Number(start) => {
                            if labels.is_empty() {
                                Self::Variable(
                                    variable.to_owned(),
                                    Some(Range {
                                        start: *start as usize,
                                        end: Some((start + 1) as usize),
                                        or: None,
                                    }),
                                )
                            } else {
                                match labels.remove(0) {
                                    Phase2Token::Or(number) => Self::Variable(
                                        variable.to_owned(),
                                        Some(Range {
                                            start: *start as usize,
                                            end: Some((start + 1) as usize),
                                            or: Some(DefaultValue::Value(*number)),
                                        }),
                                    ),
                                    Phase2Token::OrLabel(label, added) => Self::Variable(
                                        variable.to_owned(),
                                        Some(Range {
                                            start: *start as usize,
                                            end: Some((start + 1) as usize),
                                            or: Some(DefaultValue::Label(label.to_owned(), *added)),
                                        }),
                                    ),
                                    Phase2Token::OrRelative(added) => Self::Variable(
                                        variable.to_owned(),
                                        Some(Range {
                                            start: *start as usize,
                                            end: Some((start + 1) as usize),
                                            or: Some(DefaultValue::Relative(*added)),
                                        }),
                                    ),
                                    _ => {
                                        println!("Zone11");
                                        return Err(Errors::ExpressionCompilingError {
                                            position: position.clone(),
                                            expression: expression.to_owned(),
                                        });
                                    }
                                }
                            }
                        }
                        _ => {
                            println!("Zone12");
                            return Err(Errors::ExpressionCompilingError {
                                position: position.clone(),
                                expression: expression.to_owned(),
                            });
                        }
                    }
                }
            }
            _ => {
                println!("Zone13");
                return Err(Errors::ExpressionCompilingError {
                    position: position.clone(),
                    expression: expression.to_owned(),
                });
            }
        })
    }
}