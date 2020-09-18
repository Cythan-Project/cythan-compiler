use super::stage1::Position;
use super::stage3::Stage3Token;
use std::collections::HashMap;

use super::errors::Errors;

use super::value::Value;

type CompilerResult = Result<Vec<Value>, Errors>;

pub struct Context {
    functions: HashMap<String, Vec<Stage3Token>>,
    variables: HashMap<String, Vec<Value>>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }
}

impl Context {
    pub fn execute(
        &mut self,
        token: &Stage3Token,
        function_data: &Vec<Value>,
        current_int: &mut u64,
    ) -> CompilerResult {
        match token {
            Stage3Token::FunctionCreation(position, name, code) => {
                self.functions.insert(name.to_owned(), code.clone());
                Ok(Vec::new())
            }
            Stage3Token::FunctionExecution(position, name, args) => {
                let (fnname, label) = if name.starts_with("'") && name.contains(":") {
                    let mut iter = name.split(":");
                    let label = iter.next().unwrap();

                    (
                        iter.next().unwrap().to_owned(),
                        Some(label[1..label.len()].to_owned()),
                    )
                } else {
                    (name.to_owned(), None)
                };
                let e = if let Some(e) = self.functions.get(&fnname) {
                    e.clone()
                } else {
                    return Err(Errors::FunctionNotFound {
                        position: position.clone(),
                        function_names: self.functions.keys().cloned().collect(),
                        function_name: fnname,
                    });
                };
                let mut out = rename_labels(
                    execute_function(&e, args, self, function_data, current_int)?,
                    current_int,
                );
                if let Some(e) = label {
                    if !out.is_empty() {
                        let mut set = std::collections::HashSet::with_capacity(1);
                        set.insert(e);
                        out[0].add_labels(set);
                    }
                }
                Ok(out)
            }
            Stage3Token::VariableDefinition(position, name, value) => {
                let result = value
                    .iter()
                    .map(|x| self.execute(x, function_data, current_int))
                    .collect::<Result<Vec<Vec<Value>>, Errors>>()?;
                self.variables
                    .insert(name.to_owned(), result.into_iter().flatten().collect());
                Ok(vec![])
            }
            Stage3Token::Executable(position, name) => {
                Ok(name.clone().execute(&self.variables, function_data))
            }
        }
    }

    pub fn compute(&mut self, tokens: &[Stage3Token]) -> Result<Vec<usize>, Errors> {
        let p = &Vec::new();
        let mut integer = 0u64;
        let mut labels = HashMap::new();
        let list = tokens
            .iter()
            .map(|x| self.execute(x, &p, &mut integer))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<Value>>();
        Ok(list
            .iter()
            .enumerate()
            .map(|(i, x)| x.compute_value(i, &mut labels, &list))
            .collect::<Vec<usize>>())
    }
}
// Variable
// Pattern contenant une variable
// Pattern
/*fn get_value(
    literal: &str,
    variables: &HashMap<String, Vec<Value>>,
    function_args: &Vec<Value>,
    position: Position,
) -> CompilerResult {
    if literal.contains(':')
        && literal.starts_with('\'')
        && !literal.contains('+')
        && !literal.contains('-')
    {
        if let Some(e) = Value::from_str(literal, position.clone()) {
            Ok(vec![e])
        } else {
            let mut iter = literal.split(':');
            let label = iter.next().unwrap();
            let label = label[1..label.len()].to_owned();
            let variable = iter.next().unwrap();
            if let Some(mut e) = get_var(function_args, variable, variables, position.clone())? {
                if e.is_empty() {
                    Err(Errors::EmptyVariable {
                        position,
                        varname: variable.to_owned(),
                    })
                } else {
                    e[0] = e[0].labelize(label);
                    Ok(e)
                }
            } else {
                Err(Errors::UndefinedVariable {
                    position,
                    varname: variable.to_owned(),
                })
            }
        }
    } else if !literal.contains(':')
        && !literal.starts_with('\'')
        && !literal.contains('+')
        && !literal.contains('-')
    {
        if let Some(e) = Number::from_str(literal, position.clone()) {
            Ok(vec![e])
        } else if let Some(e) = get_var(function_args, literal, variables, position.clone())? {
            Ok(e)
        } else {
            Err(Errors::VariableNotFound {
                position,
                variable_names: variables.keys().cloned().collect(),
                variable_name: literal.to_owned(),
            })
        }
    } else if let Some(e) = Number::from_str(literal, position.clone()) {
        Ok(vec![e])
    } else {
        Err(Errors::UnableToReadLitteral {
            position,
            litteral: literal.to_owned(),
        })
    }
}*/

fn rename_labels(code: Vec<Value>, current_int: &mut u64) -> Vec<Value> {
    let mut labels: HashMap<String, String> = HashMap::new();

    code.into_iter()
        .map(|mut x| {
            match &mut x {
                Value::Label(lbls, label, _) => {
                    try_rename_string(label, current_int, &mut labels);
                    *lbls = remap_labels(&lbls, current_int, &mut labels);
                }
                Value::Absolute(lbls, _) => {
                    *lbls = remap_labels(&lbls, current_int, &mut labels);
                }
                Value::Relative(lbls, _) => {
                    *lbls = remap_labels(&lbls, current_int, &mut labels);
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
    if reference.starts_with('#') {
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

use std::collections::HashSet;

fn remap_labels(
    set: &HashSet<String>,
    current_int: &mut u64,
    labels: &mut HashMap<String, String>,
) -> HashSet<String> {
    set.iter()
        .flat_map(|reference| {
            if reference.contains('#') {
                return None;
            }
            Some(if let Some(e) = labels.get(reference) {
                e.to_owned()
            } else {
                *current_int += 1;
                let new = format!("label{}_{}", current_int, &reference);
                labels.insert(reference.to_owned(), new.to_owned());
                new
            })
        })
        .collect()
}

fn execute_function(
    function_code: &[Stage3Token],
    arguments: &[Stage3Token],
    context: &mut Context,
    function_data: &Vec<Value>,
    integer: &mut u64,
) -> CompilerResult {
    let args: Vec<Value> = arguments
        .iter()
        .map(|y| context.execute(y, function_data, integer))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<Value>>();
    Ok(function_code
        .iter()
        .map(|x| context.execute(x, &args, integer))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect())
}
