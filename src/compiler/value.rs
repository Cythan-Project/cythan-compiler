#[derive(Clone, Debug)]
pub enum Value {
    Relative(HashSet<String>, isize, Position),
    Absolute(HashSet<String>, usize, Position),
    Label(HashSet<String>, String, isize, Position), // bool: IsLocal
}

use crate::compiler::errors::Errors;
use crate::compiler::position::Position;
use std::collections::{HashMap, HashSet};

impl Value {
    fn get_labels(&self) -> &HashSet<String> {
        match self {
            Self::Relative(a, _, _) | Self::Absolute(a, _, _) | Self::Label(a, _, _, _) => a,
        }
    }
    pub fn add_labels(&mut self, labels: HashSet<String>) {
        match self {
            Self::Relative(a, _, _) | Self::Absolute(a, _, _) | Self::Label(a, _, _, _) => {
                a.extend(labels);
            }
        }
    }
    fn does_define_label(&self, label: &str) -> bool {
        self.get_labels().contains(label)
    }

    pub fn update_labels(&self, current: usize, labels: &mut HashMap<String, usize>) {
        for e in self.get_labels() {
            labels.insert(e.to_owned(), current);
        }
    }

    pub fn compute_value(
        &self,
        current_index: usize,
        labels: &HashMap<String, usize>,
        other_values: &[Value],
    ) -> Result<usize, Errors> {
        Ok(match self {
            Self::Relative(_, a, _) => (a + current_index as isize) as usize,
            Self::Absolute(_, a, _) => *a,
            Self::Label(_, a, i, pos) => {
                if let Some(e) = labels.get(a) {
                    *e
                } else if let Some(e) = other_values
                    .iter()
                    .skip(current_index)
                    .position(|x| x.does_define_label(a))
                {
                    (e as isize + i) as usize + current_index
                } else {
                    return Err(Errors::LabelNotFound {
                        label_name: a.to_owned(),
                        label_names: labels.keys().cloned().collect(),
                        position: pos.clone(),
                    });
                }
            }
        })
    }
}
