use std::borrow::Cow;
use tokesies::*;

pub struct CythanV1;

impl filters::Filter for CythanV1 {
    fn on_char(&self, c: &char) -> (bool, bool) {
        match *c {
            '(' | ')' | '{' | '}' => (true, true),
            ' ' => (true, true),
            _ => (false, false),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stage1Token<'a> {
    Literal(Position, Cow<'a, str>),
    //KeywordFn,
    OpenParenthesis(Position),
    CloseParenthesis(Position),
    OpenBrackets(Position),
    CloseBrackets(Position),
    Equals(Position),
}

#[derive(Debug, Clone)]
pub struct Position {
    pub line_from: usize,
    pub line_to: usize,
    pub caret_from: usize,
    pub caret_to: usize,
}

impl Position {
    fn new(line: usize, from: usize, to: usize) -> Self {
        Self {
            line_from: line,
            line_to: line,
            caret_from: from,
            caret_to: to,
        }
    }

    pub fn to_str(&self) -> String {
        if self.line_from == self.line_to {
            format!(
                "Error line {} [{},{}]",
                self.line_to, self.caret_from, self.caret_to
            )
        } else {
            format!(
                "Error from line {} [{}] to line {} [{}]",
                self.line_from, self.caret_from, self.line_to, self.caret_to
            )
        }
    }

    pub fn merge(&self, position: &Position) -> Self {
        let (line_from, caret_from) = if self.line_from < position.line_from {
            (self.line_from, self.caret_from)
        } else if self.line_from == position.line_from {
            (self.line_from, self.caret_from.min(position.caret_from))
        } else {
            (position.line_from, self.caret_from)
        };
        let (line_to, caret_to) = if self.line_to > position.line_to {
            (self.line_to, self.caret_to)
        } else if self.line_to == position.line_to {
            (self.line_to, self.caret_to.max(position.caret_to))
        } else {
            (position.line_to, self.caret_to)
        };
        Self {
            line_from,
            line_to,
            caret_from,
            caret_to,
        }
    }
}

#[inline]
pub fn compile(line: &str, line_number: usize) -> Vec<Stage1Token> {
    let tokens = FilteredTokenizer::new(CythanV1 {}, line).collect::<Vec<Token>>();
    let mut out = Vec::new();
    let mut caret = 0;
    for i in tokens {
        let t = i.term;
        let from = caret;
        caret += t.len();
        let to = caret;
        if t.trim().is_empty() {
            continue;
        }
        if t == "#" {
            break;
        }
        out.push(
            /*if t == "fn" {
                Stage1Token::KeywordFn
            } else */
            if t == "(" {
                Stage1Token::OpenParenthesis(Position::new(line_number, from, to))
            } else if t == ")" {
                Stage1Token::CloseParenthesis(Position::new(line_number, from, to))
            } else if t == "{" {
                Stage1Token::OpenBrackets(Position::new(line_number, from, to))
            } else if t == "}" {
                Stage1Token::CloseBrackets(Position::new(line_number, from, to))
            } else if t == "=" {
                Stage1Token::Equals(Position::new(line_number, from, to))
            } else {
                Stage1Token::Literal(Position::new(line_number, from, to), t)
            },
        );
    }
    out
}

#[derive(Clone, Debug)]
pub enum Number {
    Plain(InnerNumber),
    Add(InnerNumber, isize),
    PointerDefine(String, InnerNumber),
    PointerDefineAndAdd(String, InnerNumber, isize),
}

fn text_to_added_number(s: &str, position: Position) -> Option<(InnerNumber, isize)> {
    if s.contains('+') {
        let mut iter = s.split('+');
        Some((
            InnerNumber::from_str(iter.next().unwrap(), position)?,
            iter.next().map(|x| x.parse::<isize>().ok()).flatten()?,
        ))
    } else if s.contains('-') {
        let mut iter = s.split('-');
        Some((
            InnerNumber::from_str(iter.next().unwrap(), position)?,
            -iter.next().map(|x| x.parse::<isize>().ok()).flatten()?,
        ))
    } else {
        None
    }
}

#[derive(Clone, Debug)]
pub enum InnerNumber {
    Current(Position),                  // Tilde
    PointerReference(Position, String), // Pointer String,
    Number(Position, usize),            // Number
}

use super::errors::Errors;

impl Number {
    pub fn get_as_label(&self) -> Option<&str> {
        match self {
            Self::PointerDefine(pointer, _) => Some(pointer),
            Self::PointerDefineAndAdd(pointer, _, _) => Some(pointer),
            _ => None,
        }
    }

    pub fn labelize(&self, label: String) -> Self {
        match self {
            Number::Plain(e) => Number::PointerDefine(label, e.clone()),
            Number::Add(e, number) => Number::PointerDefineAndAdd(label, e.clone(), *number),
            Number::PointerDefine(label, e) => Number::PointerDefine(label.to_owned(), e.clone()),
            Number::PointerDefineAndAdd(label, e, g) => {
                Number::PointerDefineAndAdd(label.to_owned(), e.clone(), *g)
            }
        }
    }

    pub fn get_value(
        &self,
        current: usize,
        labels: &mut HashMap<String, u32>,
        tokens: &Vec<Number>,
    ) -> Result<u32, Errors> {
        Ok(match self {
            Self::Add(e, i) => (e.get_value(current, labels, tokens)? as isize + i) as u32,
            Self::Plain(e) => e.get_value(current, labels, tokens)?,
            Self::PointerDefine(name, e) => {
                labels.insert(name.to_owned(), current as u32);
                e.get_value(current, labels, tokens)?
            }
            Self::PointerDefineAndAdd(name, e, i) => {
                labels.insert(name.to_owned(), current as u32);
                (e.get_value(current, labels, tokens)? as isize + i) as u32
            }
        })
    }

    pub fn from_str(value: &str, position: Position) -> Option<Self> {
        if value.starts_with('\'') && value.contains(':') {
            // Pointer Define or Pointer and add

            let mut iter = value.split(':');

            let name = iter.next().unwrap();
            let name = name[1..name.len()].to_owned();

            let text = iter.next().unwrap();

            if value.contains('+') || value.contains('-') {
                if let Some((e, e2)) = text_to_added_number(text, position) {
                    Some(Number::PointerDefineAndAdd(name, e, e2))
                } else {
                    None
                }
            } else {
                Some(Number::PointerDefine(
                    name,
                    InnerNumber::from_str(text, position)?,
                ))
            }
        } else if value.contains('+') || value.contains('-') {
            if let Some((e, e2)) = text_to_added_number(value, position) {
                Some(Number::Add(e, e2))
            } else {
                None
            }
        } else {
            Some(Number::Plain(InnerNumber::from_str(value, position)?))
        }
    }
}

use std::collections::HashMap;

impl InnerNumber {
    pub fn get_value(
        &self,
        current: usize,
        labels: &HashMap<String, u32>,
        tokens: &Vec<Number>,
    ) -> Result<u32, Errors> {
        match self {
            Self::Current(_) => Ok(current as u32),
            Self::Number(_, e) => Ok(*e as u32),
            Self::PointerReference(position, e) => {
                if let Some(e) = labels.get(e) {
                    Ok(*e)
                } else if let Some(e) = tokens.iter().skip(current).position(|x| {
                    if let Some(tmp_e) = x.get_as_label() {
                        tmp_e == e
                    } else {
                        false
                    }
                }) {
                    Ok(e as u32)
                } else {
                    Err(Errors::LabelNotFound {
                        position: position.clone(),
                        label_names: labels.keys().cloned().collect(),
                        label_name: e.to_owned(),
                    })
                }
            }
        }
    }

    fn from_str(value: &str, position: Position) -> Option<Self> {
        if value.starts_with('\'') {
            Some(InnerNumber::PointerReference(
                position,
                value[1..value.len()].to_owned(),
            ))
        } else if value == "~" {
            Some(InnerNumber::Current(position))
        } else {
            value
                .parse::<usize>()
                .map(|x| InnerNumber::Number(position, x))
                .ok()
        }
    }
}

/*
Expression:
Plain(Nombre)
Added(Nombre, i32)

Enum: Nombre, pointer, ~

<operation> -> Enum + Enum

Nombre
Nombre Relatif
Pointer
Pointer Relatif
Pointer: Nombre
Pointer: Nombre
Pointer: Pointer
Pointer: Pointer

0 0 0 0 0 0
-> Pointeurs

-> 'test1


#[derive(Debug, Clone)]
pub enum Number {
    Relative(i32),
    Absolute(u32)
}

impl std::str::FromStr for Number {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.starts_with("~") {
            if let Ok(e) = value[1..value.len()].parse::<i32>() {
                Ok(Number::Relative(e))
            } else {
                Err("Not a number")
            }
        } else if let Ok(e) = value.parse::<u32>() {
            Ok(Number::Absolute(e))
        } else {
            Err("Not a number")
        }
    }
}*/
