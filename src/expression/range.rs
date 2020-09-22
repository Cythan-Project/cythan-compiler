use crate::expression::defaultvalue::DefaultValue;
use crate::compiler::position::Position;
use crate::compiler::value::Value;

#[derive(Debug, Clone)]
pub struct Range {
    pub start: usize,
    pub end: Option<usize>,
    pub or: Option<DefaultValue>,
}

impl Range {

    pub fn generate(&self, variable: &[Value], position: &Position) -> Vec<Value> {
        if let Some(or) = &self.or {
            if let Some(end) = self.end {
                assert_eq!(
                    self.start < end,
                    true,
                    "In a range x..y x must be lower than y"
                );
                let mut list: Vec<Value> = variable
                    .iter()
                    .skip(self.start)
                    .take(end - self.start)
                    .cloned()
                    .collect();
                for _ in list.len()..(end - self.start) {
                    list.push(or.clone().to_value(position.clone()));
                }
                list
            } else {
                variable.iter().skip(self.start).cloned().collect()
            }
        } else if let Some(end) = self.end {
            assert_eq!(
                self.start < end,
                true,
                "In a range x..y x must be lower than y"
            );
            variable
                .iter()
                .skip(self.start)
                .take(end - self.start)
                .cloned()
                .collect()
        } else {
            variable.iter().skip(self.start).cloned().collect()
        }
    }
}