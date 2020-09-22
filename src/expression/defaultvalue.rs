use crate::compiler::position::Position;
use crate::compiler::value::Value;

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum DefaultValue {
    Value(usize),
    Label(String, isize),
    Relative(isize),
}

impl DefaultValue {
    pub fn to_value(&self, position: Position) -> Value {
        match self {
            Self::Value(value) => Value::Absolute(HashSet::new(), *value, position),
            Self::Label(label, added) => {
                Value::Label(HashSet::new(), label.to_owned(), *added, position)
            }
            Self::Relative(value) => Value::Relative(HashSet::new(), *value, position),
        }
    }
}