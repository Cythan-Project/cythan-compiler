pub struct LiteralCompiler;
use tokesies::*;

impl filters::Filter for LiteralCompiler {
    fn on_char(&self, c: &char) -> (bool, bool) {
        match *c {
            ':' | '.' | '?' | '+' | '-' | '~' => (true, true),
            _ => (false, false),
        }
    }
}

#[derive(Clone, Debug)]
enum Phase1Token {
    Relative,         // ~
    NumberSign(bool), // bool: is_negative
    Dot,              // .
    QuestionMark,     // ?
}

#[derive(Clone, Debug)]
enum Phase2Token {
    Variable(String), // A-Za-z0-9_
    Number(isize),
    LabelAssign(String),
    Label(String, isize),
    Range(isize, Option<isize>),
    Relative(isize),
    VariableIndexed(String),
    Or(usize),
    OrLabel(String, isize),
}

#[derive(Clone, Debug)]
enum CompilationToken {
    Phase1Token(Phase1Token),
    Phase2Token(Phase2Token),
}

fn compile(literal: &str) -> Vec<CompilationToken> {
    let tokens = FilteredTokenizer::new(LiteralCompiler {}, literal).collect::<Vec<Token>>();
    let mut output: Vec<CompilationToken> = Vec::new();
    for token in tokens {
        let t = token.term.into_owned();
        if t == "." {
            match output.last() {
                Some(CompilationToken::Phase1Token(Phase1Token::Dot)) => {
                    output.pop();
                    if let Some(CompilationToken::Phase2Token(Phase2Token::Number(number))) =
                        output.last()
                    {
                        let number = *number;
                        output.pop();
                        output.push(CompilationToken::Phase2Token(Phase2Token::Range(
                            number as isize,
                            None,
                        )));
                        continue;
                    } else {
                        panic!("A range must have a number before it! example: 0..")
                    }
                }
                Some(CompilationToken::Phase2Token(Phase2Token::Variable(_))) => {
                    let n = if let CompilationToken::Phase2Token(Phase2Token::Variable(n)) =
                        output.pop().unwrap()
                    {
                        n
                    } else {
                        unreachable!();
                    };
                    output.push(CompilationToken::Phase2Token(Phase2Token::VariableIndexed(
                        n,
                    )));
                    continue;
                }
                _ => {
                    output.push(CompilationToken::Phase1Token(Phase1Token::Dot));
                }
            }
        } else if t == ":" {
            match output.last() {
                Some(CompilationToken::Phase2Token(Phase2Token::Label(_, _))) => {
                    let (label, added) =
                        if let CompilationToken::Phase2Token(Phase2Token::Label(label, added)) =
                            output.pop().unwrap()
                        {
                            (label, added)
                        } else {
                            unreachable!();
                        };
                    output.push(CompilationToken::Phase2Token(Phase2Token::LabelAssign(
                        label,
                    )));
                }
                _ => {
                    println!("Synthax error : can't be placed before elsewhere than <label>:");
                }
            }
        } else if t == "~" {
            output.push(CompilationToken::Phase1Token(Phase1Token::Relative));
        } else if t == "+" {
            output.push(CompilationToken::Phase1Token(Phase1Token::NumberSign(
                false,
            )));
        } else if t == "-" {
            output.push(CompilationToken::Phase1Token(Phase1Token::NumberSign(true)));
        } else if t == "?" {
            output.push(CompilationToken::Phase1Token(Phase1Token::QuestionMark));
        } else if let Ok(number) = t.parse::<usize>() {
            match output.last() {
                Some(CompilationToken::Phase1Token(Phase1Token::QuestionMark)) => {
                    output.pop();
                    output.push(CompilationToken::Phase2Token(Phase2Token::Or(number)))
                }
                Some(CompilationToken::Phase2Token(Phase2Token::Range(range_start, _))) => {
                    let range_start = *range_start;
                    output.pop();
                    output.push(CompilationToken::Phase2Token(Phase2Token::Range(
                        range_start,
                        Some(number as isize),
                    )));
                }
                Some(CompilationToken::Phase1Token(Phase1Token::NumberSign(sign))) => {
                    let sign = *sign;
                    output.pop();
                    match output.last() {
                        Some(CompilationToken::Phase1Token(Phase1Token::Relative)) => {
                            output.pop();
                            output.push(CompilationToken::Phase2Token(Phase2Token::Relative(
                                if sign {
                                    -(number as isize)
                                } else {
                                    number as isize
                                },
                            )));
                        }
                        Some(CompilationToken::Phase2Token(Phase2Token::Label(_, _))) => {
                            let (label, added) = if let CompilationToken::Phase2Token(
                                Phase2Token::Label(label, added),
                            ) = output.pop().unwrap()
                            {
                                (label, added)
                            } else {
                                unreachable!();
                            };
                            output.push(CompilationToken::Phase2Token(Phase2Token::Label(
                                label,
                                if sign {
                                    added - (number as isize)
                                } else {
                                    added + number as isize
                                },
                            )));
                        }
                        Some(CompilationToken::Phase2Token(Phase2Token::OrLabel(_, _))) => {
                            let (label, added) = if let CompilationToken::Phase2Token(
                                Phase2Token::OrLabel(label, added),
                            ) = output.pop().unwrap()
                            {
                                (label, added)
                            } else {
                                unreachable!();
                            };
                            output.push(CompilationToken::Phase2Token(Phase2Token::OrLabel(
                                label,
                                if sign {
                                    added - (number as isize)
                                } else {
                                    added + number as isize
                                },
                            )));
                        }
                        Some(CompilationToken::Phase2Token(Phase2Token::Number(n))) => {
                            let n = *n;
                            output.pop();
                            output.push(CompilationToken::Phase2Token(Phase2Token::Number(
                                number as isize + n,
                            )));
                        }
                        Some(CompilationToken::Phase2Token(Phase2Token::Range(start, end))) => {
                            let start = *start;
                            let end = *end;
                            output.pop();
                            output.push(CompilationToken::Phase2Token(Phase2Token::Range(
                                start,
                                Some(end.unwrap_or(0) + number as isize),
                            )));
                        }
                        e => {
                            panic!("Invalid number pattern! {:?}", e);
                        }
                    }
                }
                Some(CompilationToken::Phase2Token(Phase2Token::VariableIndexed(_))) => output
                    .push(CompilationToken::Phase2Token(Phase2Token::Number(
                        number as isize,
                    ))),
                None => output.push(CompilationToken::Phase2Token(Phase2Token::Number(
                    number as isize,
                ))),
                e => {
                    panic!("Unreachable pattern reached : {:?}", e);
                }
            }
        } else if t.starts_with("'") {
            let t = t[1..t.len()].to_owned();
            if let Some(CompilationToken::Phase1Token(Phase1Token::QuestionMark)) = output.last() {
                output.pop();
                output.push(CompilationToken::Phase2Token(Phase2Token::OrLabel(t, 0)));
            } else {
                output.push(CompilationToken::Phase2Token(Phase2Token::Label(t, 0)));
            }
        } else {
            output.push(CompilationToken::Phase2Token(Phase2Token::Variable(t)));
        }
    }
    output
}

#[test]
pub fn test_value() {
    /*let tests = vec![
        "~+1",
        "~-1",
        "'label:~+1",
        "'label+1",
        "'label-2",
        "'label",
        "'label:'label1+1",
        "'label:'label1-1",
        "var",
        "10",
        "var.0",
        "var.1?0",
        "var.1..",
        "'label:'label2:var.0",
        "var.1..10",
        "var.1..?0",
        "var.0..10?10",
        "'label:var.0",
        "'label:var.0+20",
        "'label:var.10..1+20",
        "'label:var.10+11..1-20",
        "'label:var.1?0",
        "'label:var.0..10?10",
        "'label:'label1:var.0..10?'label2+10-20+40",
        "'label:'label1:var.0+10..10-9?'label2+10-20+40",
        "'label:'label1:var.0..10?'label2",
        "'label:var.1..",
        "'label:var.1..10",
        "'label:var.1..?0",
    ];
    for i in tests {
        println!("{}: {:?}", i, Expression::from_string(i));
    }*/
}

#[derive(Debug, Clone)]
pub struct Expression {
    labels: HashSet<String>,
    instruction: Instruction,
}

impl Expression {
    pub fn from_string(string: &str) -> Self {
        Self::from_stage2(
            &compile(string)
                .into_iter()
                .flat_map(|e| {
                    if let CompilationToken::Phase2Token(a) = e {
                        Some(a)
                    } else {
                        panic!("None: {:?}", e);
                    }
                })
                .collect(),
        )
    }

    fn from_stage2(tokens: &Vec<Phase2Token>) -> Self {
        Self {
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
            instruction: Instruction::from_stage2(tokens),
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Variable(String, Option<Range>),
    Label(String, isize),
    Value(usize),
    Relative(isize),
}

impl Instruction {
    fn from_stage2(tokens: &[Phase2Token]) -> Self {
        let mut labels: Vec<&Phase2Token> = tokens
            .iter()
            .filter(|x| !matches!(x, Phase2Token::LabelAssign(_)))
            .collect();
        match labels.remove(0) {
            Phase2Token::Label(string, added) => Self::Label(string.to_owned(), *added),
            Phase2Token::Number(number) => Self::Value(*number as usize),
            Phase2Token::Relative(number) => Self::Relative(*number),
            Phase2Token::Variable(variable) => {
                if labels.is_empty() {
                    Self::Variable(variable.to_owned(), None)
                } else {
                    match labels.remove(0) {
                        Phase2Token::Or(number) => Self::Variable(
                            variable.to_owned(),
                            Some(Range {
                                start: 0,
                                end: Some(1),
                                or: Some(DefaultValue::Value(*number)),
                            }),
                        ),
                        Phase2Token::OrLabel(label, added) => Self::Variable(
                            variable.to_owned(),
                            Some(Range {
                                start: 0,
                                end: Some(1),
                                or: Some(DefaultValue::Label(label.to_owned(), *added)),
                            }),
                        ),
                        _ => unreachable!(),
                    }
                }
            }
            Phase2Token::VariableIndexed(variable) => {
                if labels.is_empty() {
                    unreachable!()
                } else {
                    match labels.remove(0) {
                        Phase2Token::Range(start, end) => {
                            if labels.is_empty() {
                                Self::Variable(
                                    variable.to_owned(),
                                    Some(Range {
                                        start: *start as usize,
                                        end: end.map(|x| x as usize),
                                        or: None,
                                    }),
                                )
                            } else {
                                match labels.remove(0) {
                                    Phase2Token::Or(number) => Self::Variable(
                                        variable.to_owned(),
                                        Some(Range {
                                            start: *start as usize,
                                            end: end.map(|x| x as usize),
                                            or: Some(DefaultValue::Value(*number)),
                                        }),
                                    ),
                                    Phase2Token::OrLabel(label, added) => Self::Variable(
                                        variable.to_owned(),
                                        Some(Range {
                                            start: *start as usize,
                                            end: end.map(|x| x as usize),
                                            or: Some(DefaultValue::Label(label.to_owned(), *added)),
                                        }),
                                    ),
                                    _ => unreachable!(),
                                }
                            }
                        }
                        Phase2Token::Number(start) => {
                            if labels.is_empty() {
                                Self::Variable(
                                    variable.to_owned(),
                                    Some(Range {
                                        start: *start as usize,
                                        end: Some((start + 1) as usize),
                                        or: None,
                                    }),
                                )
                            } else {
                                match labels.remove(0) {
                                    Phase2Token::Or(number) => Self::Variable(
                                        variable.to_owned(),
                                        Some(Range {
                                            start: *start as usize,
                                            end: Some((start + 1) as usize),
                                            or: Some(DefaultValue::Value(*number)),
                                        }),
                                    ),
                                    Phase2Token::OrLabel(label, added) => Self::Variable(
                                        variable.to_owned(),
                                        Some(Range {
                                            start: *start as usize,
                                            end: Some((start + 1) as usize),
                                            or: Some(DefaultValue::Label(label.to_owned(), *added)),
                                        }),
                                    ),
                                    _ => unreachable!(),
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => {
                unreachable!();
            }
        }
    }
}

impl Expression {
    pub fn execute(
        self,
        vars: &HashMap<String, Vec<Value>>,
        function_data: &Vec<Value>,
    ) -> Vec<Value> {
        match self.instruction {
            Instruction::Label(name, added) => vec![Value::Label(self.labels, name, added)],
            Instruction::Relative(added) => vec![Value::Relative(self.labels, added)],
            Instruction::Value(value) => vec![Value::Absolute(self.labels, value)],
            Instruction::Variable(name, range) => {
                let variable = if let Some(e) = vars.get(&name) {
                    e
                } else if name == "self" {
                    function_data
                } else {
                    panic!("variable {:?} not found", name);
                };
                match range {
                    Some(e) => {
                        let mut variable: Vec<Value> = e.generate(variable);
                        if !variable.is_empty() {
                            variable[0].add_labels(self.labels);
                        }
                        variable
                    }
                    None => {
                        let mut variable: Vec<Value> = variable.clone();
                        variable[0].add_labels(self.labels);
                        variable
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    end: Option<usize>,
    or: Option<DefaultValue>,
}

impl Range {
    fn generate(&self, variable: &Vec<Value>) -> Vec<Value> {
        if let Some(end) = self.end {
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

#[derive(Debug, Clone)]
enum DefaultValue {
    Value(usize),
    Label(String, isize),
}

#[derive(Clone, Debug)]
pub enum Value {
    Relative(HashSet<String>, isize),
    Absolute(HashSet<String>, usize),
    Label(HashSet<String>, String, isize),
}

use std::collections::{HashMap, HashSet};

impl Value {
    fn get_labels(&self) -> &HashSet<String> {
        match self {
            Self::Relative(a, _) | Self::Absolute(a, _) | Self::Label(a, _, _) => a,
        }
    }
    pub fn add_labels(&mut self, labels: HashSet<String>) {
        match self {
            Self::Relative(a, _) | Self::Absolute(a, _) | Self::Label(a, _, _) => {
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
        other_values: &Vec<Value>,
    ) -> usize {
        match self {
            Self::Relative(_, a) => (a + current_index as isize) as usize,
            Self::Absolute(_, a) => *a,
            Self::Label(_, a, i) => {
                if let Some(e) = labels.get(a) {
                    *e
                } else if let Some(e) = other_values
                    .iter()
                    .skip(current_index)
                    .position(|x| x.does_define_label(a))
                {
                    (e as isize + i) as usize
                } else {
                    panic!("Can't get label {:?}", a)
                }
            }
        }
    }
}

/*
expression:
    Vec<String>: labels_to_define

instruction:
    Variable {
        name: String,
        range: Option<Range>
    }
Range:
    start: usize
    end: Option<usize>
    or: Option<DefaultValue>
DefaultValue:
    Value(isize)
    Label(String, isize)

*/
