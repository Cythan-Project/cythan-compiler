
use super::stage3::Stage3Token;
use std::collections::HashMap;

pub struct Context {
    functions: HashMap<String,Vec<Stage3Token>>,
    variables: HashMap<String,Vec<u32>>
}

impl Context {

    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new()
        }
    }
    
    pub fn execute(&mut self, token: &Stage3Token, function_data: &Vec<u32>) -> Vec<u32> {
        match token {
            Stage3Token::Value(e) => {
                vec![*e]
            },
            Stage3Token::FunctionCreation(name,code) => {
                self.functions.insert(name.to_owned(),code.clone());
                vec![]
            },
            Stage3Token::FunctionExecution(name,args) => {
                let e = if let Some(e) = self.functions.get(name) {
                    e.clone()
                } else {
                    println!("Can't find function {}",name);
                    return vec![];
                };
                execute_function(&e,args,self,function_data)
            },
            Stage3Token::VariableDefinition(name,value) => {
                let result = value.iter().map(|x| self.execute(x, function_data)).flatten().collect();
                self.variables.insert(name.to_owned(),result);
                vec![]
            },
            Stage3Token::Variable(name) => {
                if name.contains("self.") {
                    let pattern = name.replace("self.","");
                    pattern_to_value(function_data,&pattern)
                } else if name == "self" {
                    function_data.clone()
                } else if let Some(e) = self.variables.get(name) {
                    e.clone()
                } else {
                    println!("Variable {} not found",name);
                    vec![]
                }
                
            }
        }
    }

    pub fn compute(&mut self, tokens: &Vec<Stage3Token>) -> Vec<u32> {
        let p = &Vec::new();
        tokens.iter().map(|x| self.execute(x, &p)).flatten().collect()
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

fn pattern_to_value(function_args: &Vec<u32>,pattern: &str) -> Vec<u32> {
    if pattern.contains("?") {
        let mut iter = pattern.split("?");
        let before = iter.next().expect("A `self.x?` expression must have a number `x` before the `?`");
        if before.is_empty() {
            panic!("A `self.x?` expression must have a number `x` before the `?`");
        }
        let (start,end) = if before.contains("..") {
            let p = format!(" {} ",before);
            let mut pattern = p.split("..");
            let a1 = pattern.next().unwrap().trim();
            let a2 = pattern.next().unwrap().trim();
            if a1.is_empty() {
                panic!("A `self.x..y?` expression must have a number `x` and / or `y`")
            }
            let start = a1.parse::<u32>().expect("In a `self.x..y?`, x must be a number");
            let end = if a2.is_empty() {
                function_args.len() as u32
            } else {
                a2.parse::<u32>().expect("In a `self.x..y?`, y must be a number")
            };
            (start,end)

        } else {
            let content = before.parse::<u32>().expect("In a `self.x?`, x must be a number");
            (content,content+1)
        };

        let replace_with = iter.next().map(|x| x.parse::<u32>().unwrap_or(0)).unwrap_or(0);

        (start..end).map(|x| {
            *function_args.get(x as usize).unwrap_or(&replace_with)
        }).collect()
    } else {
        let (start,end) = if pattern.contains("..") {
            let p = format!(" {} ",pattern);
            let mut pattern = p.split("..");
            let a1 = pattern.next().unwrap().trim();
            let a2 = pattern.next().unwrap().trim();
            if a1.is_empty() {
                panic!("A `self.x..y` expression must have a number `x` and / or `y`")
            }
            let start = a1.parse::<u32>().expect("In a `self.x..y`, x must be a number");
            let end = if a2.is_empty() {
                function_args.len() as u32
            } else {
                a2.parse::<u32>().expect("In a `self.x..y`, y must be a number")
            };
            (start,end)

        } else {
            let content = pattern.parse::<u32>().expect("In a `self.x`, x must be a number");
            (content,content+1)
        };

        (start..end).flat_map(|x| {
            function_args.get(x as usize).map(|x| *x)
        }).collect()
    }
}

fn execute_function(function_code: &Vec<Stage3Token>,arguments: &Vec<Stage3Token>, context: &mut Context, function_data: &Vec<u32>) -> Vec<u32> {
    let args: Vec<u32> = arguments.iter().map(|y| context.execute(y,function_data)).flatten().collect();
    let o = function_code.iter().map(|x| {
        context.execute(x,&args)
    }).flatten().collect();
    o
} 
