use super::stage1::{InnerNumber, Number};
use super::stage3::Stage3Token;
use std::collections::HashMap;

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
    ) -> Vec<Number> {
        match token {
            Stage3Token::FunctionCreation(name, code) => {
                self.functions.insert(name.to_owned(), code.clone());
                vec![]
            }
            Stage3Token::FunctionExecution(name, args) => {
                let e = if let Some(e) = self.functions.get(name) {
                    e.clone()
                } else {
                    println!("Can't find function {}", name);
                    return vec![];
                };
                rename_labels(
                    execute_function(&e, args, self, function_data, current_int),
                    current_int,
                )
            }
            Stage3Token::VariableDefinition(name, value) => {
                let result = value
                    .iter()
                    .map(|x| self.execute(x, function_data, current_int))
                    .flatten()
                    .collect();
                self.variables.insert(name.to_owned(), result);
                vec![]
            }
            Stage3Token::Executable(name) => get_value(name, &self.variables, &function_data),
        }
    }

    pub fn compute(&mut self, tokens: &Vec<Stage3Token>) -> Vec<u32> {
        let p = &Vec::new();
        let mut integer = 0u64;
        let mut labels = HashMap::new();
        tokens
            .iter()
            .map(|x| self.execute(x, &p, &mut integer))
            .flatten()
            .enumerate()
            .map(|(i, x)| x.get_value(i, &mut labels))
            .collect()
    }
}
// Variable
// Pattern contenant une variable
// Pattern
fn get_value(
    literal: &str,
    variables: &HashMap<String, Vec<Number>>,
    function_args: &Vec<Number>,
) -> Vec<Number> {
    if literal.contains(":")
        && literal.starts_with("'")
        && !literal.contains("+")
        && !literal.contains("-")
    {
        if let Ok(e) = literal.parse::<Number>() {
            return vec![e];
        } else {
            let mut iter = literal.split(":");
            let label = iter.next().unwrap();
            let label = label[1..label.len()].to_owned();
            let variable = iter.next().unwrap();
            if let Some(e) = get_var(function_args, variable, variables) {
                if e.is_empty() {
                    panic!("empty variable `{}`", variable);
                } else {
                    let mut i = e.clone();
                    i[0] = i[0].labelize(label);
                    i
                }
            } else {
                panic!("undefined variable `{}`", variable);
            }
        }
    } else if !literal.contains(":")
        && !literal.starts_with("'")
        && !literal.contains("+")
        && !literal.contains("-")
    {
        if let Ok(e) = literal.parse::<Number>() {
            vec![e]
        } else if let Some(e) = get_var(function_args, literal, variables) {
            e.clone()
        } else {
            panic!("{} does not exists", literal);
        }
    } else if let Ok(e) = literal.parse::<Number>() {
        vec![e]
    } else {
        vec![]
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
        let new = format!("label{}", current_int);
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
) -> Option<Vec<Number>> {
    if pattern == "self" {
        Some(function_args.clone())
    } else if pattern.starts_with("self.") {
        Some(pattern_to_value(
            function_args,
            &pattern.replace("self.", ""),
        ))
    } else {
        map.get(pattern).map(|x| x.clone())
    }
}

fn pattern_to_value(function_args: &Vec<Number>, pattern: &str) -> Vec<Number> {
    if pattern.contains('?') {
        let mut iter = pattern.split('?');
        let before = iter
            .next()
            .expect("A `self.x?` expression must have a number `x` before the `?`");
        if before.is_empty() {
            panic!("A `self.x?` expression must have a number `x` before the `?`");
        }
        let (start, end) = if before.contains("..") {
            let p = format!(" {} ", before);
            let mut pattern = p.split("..");
            let a1 = pattern.next().unwrap().trim();
            let a2 = pattern.next().unwrap().trim();
            if a1.is_empty() {
                panic!("A `self.x..y?` expression must have a number `x` and / or `y`")
            }
            let start = a1
                .parse::<u32>()
                .expect("In a `self.x..y?`, x must be a number");
            let end = if a2.is_empty() {
                function_args.len() as u32
            } else {
                a2.parse::<u32>()
                    .expect("In a `self.x..y?`, y must be a number")
            };
            (start, end)
        } else {
            let content = before
                .parse::<u32>()
                .expect("In a `self.x?`, x must be a number");
            (content, content + 1)
        };

        let replace_with = iter
            .next()
            .map(|x| x.parse::<u32>().unwrap_or(0))
            .unwrap_or(0);

        (start..end)
            .map(|x| {
                function_args
                    .get(x as usize)
                    .unwrap_or(&Number::Plain(InnerNumber::Number(replace_with as usize)))
                    .clone()
            })
            .collect()
    } else {
        let (start, end) = if pattern.contains("..") {
            let p = format!(" {} ", pattern);
            let mut pattern = p.split("..");
            let a1 = pattern.next().unwrap().trim();
            let a2 = pattern.next().unwrap().trim();
            if a1.is_empty() {
                panic!("A `self.x..y` expression must have a number `x` and / or `y`")
            }
            let start = a1
                .parse::<u32>()
                .expect("In a `self.x..y`, x must be a number");
            let end = if a2.is_empty() {
                function_args.len() as u32
            } else {
                a2.parse::<u32>()
                    .expect("In a `self.x..y`, y must be a number")
            };
            (start, end)
        } else {
            let content = pattern
                .parse::<u32>()
                .expect("In a `self.x`, x must be a number");
            (content, content + 1)
        };

        (start..end)
            .flat_map(|x| function_args.get(x as usize).map(|x| x.clone()))
            .collect()
    }
}

fn execute_function(
    function_code: &Vec<Stage3Token>,
    arguments: &Vec<Stage3Token>,
    context: &mut Context,
    function_data: &Vec<Number>,
    integer: &mut u64,
) -> Vec<Number> {
    let args: Vec<Number> = arguments
        .iter()
        .map(|y| context.execute(y, function_data, integer))
        .flatten()
        .collect();
    let o = function_code
        .iter()
        .map(|x| context.execute(x, &args, integer))
        .flatten()
        .collect();
    o
}
