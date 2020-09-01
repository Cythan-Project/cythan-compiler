#[derive(Debug, Clone)]
pub enum Stage3Token {
    Executable(String),
    FunctionExecution(String, Vec<Stage3Token>),
    FunctionCreation(String, Vec<Stage3Token>),
    VariableDefinition(String, Vec<Stage3Token>),
}

use super::stage2::Stage2Token;

use super::errors::Errors;

#[inline]
pub fn compile(expr: &Vec<Stage2Token>) -> Result<Vec<Stage3Token>, Errors> {
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
                    return Err(Errors::LiteralAfterAssignement {
                        literal: e.clone().into_owned(),
                        assignement: litteral,
                    });
                }
                if was_litteral {
                    output.push(Stage3Token::Executable(litteral));
                }
                was_litteral = true;
                litteral = e.clone().into_owned();
            }
            Stage2Token::Block(e) => {
                if was_assignement {
                    return Err(Errors::BlockAfterAssignement {
                        assignement: litteral.to_owned(),
                    });
                }
                if was_litteral {
                    output.push(Stage3Token::FunctionCreation(litteral, compile(e)?));
                    litteral = String::new();
                    was_litteral = false;
                } else {
                    return Err(Errors::BlockMustBePrecededByLiteral);
                }
            }
            Stage2Token::Parenthesis(e) => {
                if was_litteral {
                    output.push(Stage3Token::FunctionExecution(litteral, compile(e)?));
                    litteral = String::new();
                    was_litteral = false;
                } else if was_assignement {
                    output.push(Stage3Token::VariableDefinition(litteral, compile(e)?));
                    litteral = String::new();
                    was_assignement = false;
                } else {
                    return Err(Errors::ParenthesisNotInAssignementOrFunctionCall);
                }
            }
            Stage2Token::Assignement(e) => {
                if was_assignement {
                    return Err(Errors::AssignementFollowedByAnotherAssignement {
                        assignement1: litteral,
                        assignement2: e.clone().into_owned(),
                    });
                }
                if was_litteral {
                    output.push(Stage3Token::Executable(litteral));
                    was_litteral = false;
                }
                was_assignement = true;
                litteral = e.clone().into_owned();
            } /*_ => {
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
              }*/
        }
    }
    if was_litteral {
        output.push(Stage3Token::Executable(litteral));
    }
    Ok(output)
}

impl Stage3Token {
    pub fn to_string(&self) -> Vec<String> {
        match self {
            Self::Executable(e) => vec![e.to_owned()],
            Self::FunctionCreation(e, c) => {
                let mut v = Vec::new();
                v.push(String::new());
                v.push(format!("{} {{", e));
                v.extend(
                    c.iter()
                        .map(|x| x.to_string())
                        .flatten()
                        .map(|x| format!("    {}", x)),
                );
                v.push(format!("}}"));
                v.push(String::new());
                v
            }
            Self::VariableDefinition(e, c) => {
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
            Self::FunctionExecution(e, c) => {
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
