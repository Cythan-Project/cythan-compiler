use crate::compiler::position::Position;
use crate::expression::expression_compiler::compile;
use crate::parser::tokens::{CompilationToken, Phase2Token};
use crate::compiler::errors::Errors;
use std::collections::HashMap;
use crate::parser::instruction::Instruction;
use std::collections::HashSet;
use crate::expression::literal_instruction::LiteralInstruction;
use crate::compiler::value::Value;

#[derive(Debug, Clone)]
pub struct LiteralExpression {
    labels: HashSet<String>,
    instruction: LiteralInstruction,
    position: Position,
    expression: String,
}

impl LiteralExpression {
    pub fn from_string(string: &str, position: &Position) -> Result<Self, Errors> {
        Self::from_stage2(
            &compile(string, position)?
                .into_iter()
                .flat_map(|e| {
                    if let CompilationToken::Phase2Token(a) = e {
                        Some(Ok(a))
                    } else {
                        println!("Zone7 {:?}", e);
                        Some(Err(Errors::ExpressionCompilingError {
                            position: position.clone(),
                            expression: string.to_owned(),
                        }))
                    }
                })
                .collect::<Result<Vec<_>, _>>()?,
            string,
            position,
        )
    }

    fn from_stage2(
        tokens: &[Phase2Token],
        string: &str,
        position: &Position,
    ) -> Result<Self, Errors> {
        Ok(Self {
            labels: tokens
                .iter()
                .flat_map(|x| {
                    if let Phase2Token::LabelAssign(name) = x {
                        Some(name.to_owned())
                    } else {
                        None
                    }
                })
                .collect(),
            instruction: LiteralInstruction::from_stage2(tokens, string, position)?,
            expression: string.to_owned(),
            position: position.clone(),
        })
    }

    pub fn execute(
        self,
        vars: &HashMap<String, Vec<Value>>,
        function_data: &[Value],
    ) -> Result<Vec<Value>, Errors> {
        Ok(match self.instruction {
            LiteralInstruction::Label(name, added) => {
                vec![Value::Label(self.labels, name, added, self.position)]
            }
            LiteralInstruction::Relative(added) => {
                vec![Value::Relative(self.labels, added, self.position)]
            }
            LiteralInstruction::Value(value) => vec![Value::Absolute(self.labels, value, self.position)],
            LiteralInstruction::Variable(name, range) => {
                let variable = if let Some(e) = vars.get(&name) {
                    e
                } else if name == "self" {
                    function_data
                } else {
                    return Err(Errors::VariableNotFound {
                        variable_name: name,
                        variable_names: vars.keys().cloned().collect(),
                        position: self.position,
                    });
                };
                match range {
                    Some(e) => {
                        let mut variable: Vec<Value> = e.generate(variable, &self.position);
                        if !variable.is_empty() {
                            variable[0].add_labels(self.labels);
                        }
                        variable
                    }
                    None => {
                        let mut variable: Vec<Value> = variable.to_vec();
                        variable[0].add_labels(self.labels);
                        variable
                    }
                }
            }
        })
    }
}