use super::stage1::{InnerNumber, Number};
use super::stage3::Stage3Token;
use std::collections::HashMap;

use super::errors::Errors;

type CompilerResult = Result<Vec<Number>, Errors>;

pub struct Context {
    functions: HashMap<String, Vec<Stage3Token>>,
    variables: HashMap<String, Vec<Number>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn execute(
        &mut self,
        token: &Stage3Token,
        function_data: &Vec<Number>,
        current_int: &mut u64,
    ) -> CompilerResult {
        match token {
            Stage3Token::FunctionCreation(name, code) => {
                self.functions.insert(name.to_owned(), code.clone());
                Ok(Vec::new())
            }
            Stage3Token::FunctionExecution(name, args) => {
                let e = if let Some(e) = self.functions.get(name) {
                    e.clone()
                } else {
                    return Err(Errors::FunctionNotFound {
                        function_name: name.to_owned(),
                    });
                };
                Ok(rename_labels(
                    execute_function(&e, args, self, function_data, current_int)?,
                    current_int,
                ))
            }
            Stage3Token::VariableDefinition(name, value) => {
                let result = value
                    .into_iter()
                    .map(|x| self.execute(x, function_data, current_int))
                    .collect::<Result<Vec<Vec<Number>>, Errors>>()?;
                self.variables
                    .insert(name.to_owned(), result.into_iter().flatten().collect());
                Ok(vec![])
            }
            Stage3Token::Executable(name) => get_value(name, &self.variables, &function_data),
        }
    }

    pub fn compute(&mut self, tokens: &Vec<Stage3Token>) -> Result<Vec<u32>, Errors> {
        let p = &Vec::new();
        let mut integer = 0u64;
        let mut labels = HashMap::new();
        Ok(tokens
            .iter()
            .map(|x| self.execute(x, &p, &mut integer))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .enumerate()
            .map(|(i, x)| x.get_value(i, &mut labels))
            .collect::<Result<Vec<u32>, Errors>>()?)
    }
}
// Variable
// Pattern contenant une variable
// Pattern
fn get_value(
    literal: &str,
    variables: &HashMap<String, Vec<Number>>,
    function_args: &Vec<Number>,
) -> CompilerResult {
    if literal.contains(":")
        && literal.starts_with("'")
        && !literal.contains("+")
        && !literal.contains("-")
    {
        if let Ok(e) = literal.parse::<Number>() {
            Ok(vec![e])
        } else {
            let mut iter = literal.split(":");
            let label = iter.next().unwrap();
            let label = label[1..label.len()].to_owned();
            let variable = iter.next().unwrap();
            if let Some(e) = get_var(function_args, variable, variables)? {
                if e.is_empty() {
                    Err(Errors::EmptyVariable {
                        varname: variable.to_owned(),
                    })
                } else {
                    let mut i = e.clone();
                    i[0] = i[0].labelize(label);
                    Ok(i)
                }
            } else {
                Err(Errors::UndefinedVariable {
                    varname: variable.to_owned(),
                })
            }
        }
    } else if !literal.contains(":")
        && !literal.starts_with("'")
        && !literal.contains("+")
        && !literal.contains("-")
    {
        if let Ok(e) = literal.parse::<Number>() {
            Ok(vec![e])
        } else if let Some(e) = get_var(function_args, literal, variables)? {
            Ok(e.clone())
        } else {
            Err(Errors::VariableNotFound {
                variable_name: literal.to_owned(),
            })
        }
    } else if let Ok(e) = literal.parse::<Number>() {
        Ok(vec![e])
    } else {
        Err(Errors::UnableToReadLitteral {
            litteral: literal.to_owned(),
        })
    }
}

fn rename_labels(code: Vec<Number>, current_int: &mut u64) -> Vec<Number> {
    let mut labels: HashMap<String, String> = HashMap::new();

    code.into_iter()
        .map(|mut x| {
            match &mut x {
                Number::Add(a, b) => try_rename_inner_number(a, current_int, &mut labels),
                Number::Plain(a) => try_rename_inner_number(a, current_int, &mut labels),
                Number::PointerDefine(a, b) => {
                    try_rename_string(a, current_int, &mut labels);
                    try_rename_inner_number(b, current_int, &mut labels)
                }
                Number::PointerDefineAndAdd(a, b, _) => {
                    try_rename_string(a, current_int, &mut labels);
                    try_rename_inner_number(b, current_int, &mut labels)
                }
            }
            x
        })
        .collect()
}

fn try_rename_string(
    reference: &mut String,
    current_int: &mut u64,
    labels: &mut HashMap<String, String>,
) {
    if reference.starts_with("#") {
        return;
    }
    if let Some(e) = labels.get(reference) {
        *reference = e.clone();
    } else {
        *current_int += 1;
        let new = format!("label{}_{}", current_int, &reference);
        labels.insert(reference.to_owned(), new.to_owned());
        *reference = new;
    }
}

fn try_rename_inner_number(
    inner_number: &mut InnerNumber,
    current_int: &mut u64,
    labels: &mut HashMap<String, String>,
) {
    match inner_number {
        InnerNumber::PointerReference(reference) => {
            try_rename_string(reference, current_int, labels)
        }
        _ => (),
    }
}
/*
self.0 // Will get the value of 0 only if the value exists
self.0? // Will try to get value 0 or if the value don't exists will replace it by 0
self.0?5 // Will try to get value 0 or if the value don't exists will replace it by 5
self.0..40 // Will get values between 0 and 40 if one don't exists will not replace inexisting values
self.0..40? // Will get values between 0 and 40 if one don't exists it will be replace by a 0
self.0..40?6 // Will get values between 0 and 40 if one don't exists it will be replace by a 6
self.3.. // Will get values between 3 and the end of the arguments

self will get everything
*/

fn get_var(
    function_args: &Vec<Number>,
    pattern: &str,
    map: &HashMap<String, Vec<Number>>,
) -> Result<Option<Vec<Number>>, Errors> {
    if pattern == "self" {
        Ok(Some(function_args.clone()))
    } else if pattern.starts_with("self.") {
        Ok(Some(pattern_to_value(
            function_args,
            &pattern.replace("self.", ""),
        )?))
    } else {
        Ok(map.get(pattern).map(|x| x.clone()))
    }
}

macro_rules! expect_r {
    ($ty:expr,$error:expr) => {
        if let Ok(e) = $ty {
            e
        } else {
            return Err($error);
        }
    };
}

fn pattern_to_value(function_args: &Vec<Number>, pattern: &str) -> CompilerResult {
    if pattern.contains('?') {
        let mut iter = pattern.split('?');
        let before = if let Some(e) = iter.next() {
            e
        } else {
            return Err(Errors::SelfExpressionMissingNumberBeforeQuestionMark {
                expression: pattern.to_owned(),
            });
        };
        if before.is_empty() {
            return Err(Errors::SelfExpressionMissingNumberBeforeQuestionMark {
                expression: pattern.to_owned(),
            });
        }
        let (start, end) = if before.contains("..") {
            let p = format!(" {} ", before);
            let mut pattern1 = p.split("..");
            let a1 = pattern1.next().unwrap().trim();
            let a2 = pattern1.next().unwrap().trim();
            if a1.is_empty() {
                return Err(Errors::SelfExpressionMissingNumber {
                    expression: pattern.to_owned(),
                });
            }
            let start = expect_r!(
                a1.parse::<u32>(),
                Errors::SelfExpressionXNotNumber {
                    expression: pattern.to_owned(),
                }
            );
            let end = if a2.is_empty() {
                function_args.len() as u32
            } else {
                expect_r!(
                    a2.parse::<u32>(),
                    Errors::SelfExpressionYNotNumber {
                        expression: pattern.to_owned(),
                    }
                )
            };
            (start, end)
        } else {
            let content = expect_r!(
                before.parse::<u32>(),
                Errors::SelfExpressionXNotNumber {
                    expression: pattern.to_owned(),
                }
            );
            (content, content + 1)
        };

        let replace_with = iter
            .next()
            .map(|x| x.parse::<u32>().unwrap_or(0))
            .unwrap_or(0);

        Ok((start..end)
            .map(|x| {
                function_args
                    .get(x as usize)
                    .unwrap_or(&Number::Plain(InnerNumber::Number(replace_with as usize)))
                    .clone()
            })
            .collect())
    } else {
        let (start, end) = if pattern.contains("..") {
            let p = format!(" {} ", pattern);
            let mut pattern1 = p.split("..");
            let a1 = pattern1.next().unwrap().trim();
            let a2 = pattern1.next().unwrap().trim();
            if a1.is_empty() {
                return Err(Errors::SelfExpressionMissingNumber {
                    expression: pattern.to_owned(),
                });
            }
            let start = expect_r!(
                a1.parse::<u32>(),
                Errors::SelfExpressionXNotNumber {
                    expression: pattern.to_owned(),
                }
            );
            let end = if a2.is_empty() {
                function_args.len() as u32
            } else {
                expect_r!(
                    a2.parse::<u32>(),
                    Errors::SelfExpressionYNotNumber {
                        expression: pattern.to_owned(),
                    }
                )
            };
            (start, end)
        } else {
            let content = expect_r!(
                pattern.parse::<u32>(),
                Errors::SelfExpressionXNotNumber {
                    expression: pattern.to_owned(),
                }
            );
            (content, content + 1)
        };

        Ok((start..end)
            .flat_map(|x| function_args.get(x as usize).map(|x| x.clone()))
            .collect())
    }
}

fn execute_function(
    function_code: &Vec<Stage3Token>,
    arguments: &Vec<Stage3Token>,
    context: &mut Context,
    function_data: &Vec<Number>,
    integer: &mut u64,
) -> CompilerResult {
    let args: Vec<Number> = arguments
        .iter()
        .map(|y| context.execute(y, function_data, integer))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<Number>>();
    Ok(function_code
        .iter()
        .map(|x| context.execute(x, &args, integer))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect())
}
