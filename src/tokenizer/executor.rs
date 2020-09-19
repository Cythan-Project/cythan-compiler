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
        function_data: &[Value],
        current_int: &mut u64,
    ) -> CompilerResult {
        match token {
            Stage3Token::FunctionCreation(_position, name, code) => {
                self.functions.insert(name.to_owned(), code.clone());
                Ok(Vec::new())
            }
            Stage3Token::FunctionExecution(position, name, args) => {
                let (fnname, label) = if name.starts_with('\'') && name.contains(':') {
                    let mut iter = name.split(':');
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
            Stage3Token::VariableDefinition(_position, name, value) => {
                let result = value
                    .iter()
                    .map(|x| self.execute(x, function_data, current_int))
                    .collect::<Result<Vec<Vec<Value>>, Errors>>()?;
                self.variables
                    .insert(name.to_owned(), result.into_iter().flatten().collect());
                Ok(vec![])
            }
            Stage3Token::Executable(_position, name) => {
                name.clone().execute(&self.variables, function_data)
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
        list.iter()
            .enumerate()
            .map(|(i, x)| {
                let o = x.compute_value(i, &labels, &list);
                x.update_labels(i, &mut labels);
                o
            })
            .collect::<Result<Vec<usize>, _>>()
    }
}

fn rename_labels(code: Vec<Value>, current_int: &mut u64) -> Vec<Value> {
    let mut labels: HashMap<String, String> = HashMap::new();

    let function_defined_label: HashSet<String> = code
        .iter()
        .map(|mut x| match &mut x {
            Value::Label(lbls, _, _, _)
            | Value::Absolute(lbls, _, _)
            | Value::Relative(lbls, _, _) => lbls,
        })
        .flatten()
        .cloned()
        .collect::<HashSet<String>>();

    code.into_iter()
        .map(|mut x| {
            match &mut x {
                Value::Label(lbls, label, _, _) => {
                    if function_defined_label.contains(label) {
                        try_rename_string(label, current_int, &mut labels);
                    }
                    *lbls = remap_labels(&lbls, current_int, &mut labels);
                }
                Value::Absolute(lbls, _, _) => {
                    *lbls = remap_labels(&lbls, current_int, &mut labels);
                }
                Value::Relative(lbls, _, _) => {
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
    function_data: &[Value],
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
