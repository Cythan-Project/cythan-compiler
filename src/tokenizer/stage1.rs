use std::borrow::Cow;
use tokesies::*;

pub struct CythanV1;

impl filters::Filter for CythanV1 {
    fn on_char(&self, c: &char) -> (bool, bool) {
        match *c {
            '(' | ')' | '{' | '}' => (true, true),
            ' ' => (true, false),
            _ => (false, false),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stage1Token<'a> {
    Literal(Cow<'a, str>),
    //KeywordFn,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrackets,
    CloseBrackets,
    Equals,
}

#[inline]
pub fn compile(line: &str) -> Vec<Stage1Token> {
    let tokens = FilteredTokenizer::new(CythanV1 {}, line).collect::<Vec<Token>>();
    let mut out = Vec::new();
    for i in tokens {
        let t = i.term;
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
                Stage1Token::OpenParenthesis
            } else if t == ")" {
                Stage1Token::CloseParenthesis
            } else if t == "{" {
                Stage1Token::OpenBrackets
            } else if t == "}" {
                Stage1Token::CloseBrackets
            } else if t == "=" {
                Stage1Token::Equals
            } else {
                Stage1Token::Literal(t)
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

fn text_to_added_number(s: &str) -> Option<(InnerNumber, isize)> {
    if s.contains('+') {
        let mut iter = s.split('+');
        Some((
            iter.next().unwrap().parse().ok()?,
            iter.next().map(|x| x.parse::<isize>().ok()).flatten()?,
        ))
    } else if s.contains('-') {
        let mut iter = s.split('-');
        Some((
            iter.next().unwrap().parse().ok()?,
            -iter.next().map(|x| x.parse::<isize>().ok()).flatten()?,
        ))
    } else {
        None
    }
}

impl std::str::FromStr for Number {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.starts_with('\'') && value.contains(':') {
            // Pointer Define or Pointer and add

            let mut iter = value.split(':');

            let name = iter.next().unwrap();
            let name = name[1..name.len()].to_owned();

            let text = iter.next().unwrap();

            if value.contains('+') || value.contains('-') {
                if let Some((e, e2)) = text_to_added_number(text) {
                    Ok(Number::PointerDefineAndAdd(name, e, e2))
                } else {
                    Err("Not a valid PointerDefineAndAdd")
                }
            } else {
                Ok(Number::PointerDefine(name, text.parse::<InnerNumber>()?))
            }
        } else if value.contains('+') || value.contains('-') {
            if let Some((e, e2)) = text_to_added_number(value) {
                Ok(Number::Add(e, e2))
            } else {
                Err("Not a valid Add")
            }
        } else {
            Ok(Number::Plain(value.parse::<InnerNumber>()?))
        }
    }
}

#[derive(Clone, Debug)]
pub enum InnerNumber {
    Current,                  // Tilde
    PointerReference(String), // Pointer String,
    Number(usize),            // Number
}

impl std::str::FromStr for InnerNumber {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.starts_with('\'') {
            Ok(InnerNumber::PointerReference(
                value[1..value.len()].to_owned(),
            ))
        } else if value == "~" {
            Ok(InnerNumber::Current)
        } else {
            value
                .parse::<usize>()
                .map(InnerNumber::Number)
                .map_err(|_| "Invalid number")
        }
    }
}

use super::errors::Errors;

impl Number {
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
    ) -> Result<u32, Errors> {
        Ok(match self {
            Self::Add(e, i) => (e.get_value(current, labels)? as isize + i) as u32,
            Self::Plain(e) => e.get_value(current, labels)?,
            Self::PointerDefine(name, e) => {
                labels.insert(name.to_owned(), current as u32);
                e.get_value(current, labels)?
            }
            Self::PointerDefineAndAdd(name, e, i) => {
                labels.insert(name.to_owned(), current as u32);
                (e.get_value(current, labels)? as isize + i) as u32
            }
        })
    }
}

use std::collections::HashMap;

impl InnerNumber {
    pub fn get_value(&self, current: usize, labels: &HashMap<String, u32>) -> Result<u32, Errors> {
        match self {
            Self::Current => Ok(current as u32),
            Self::Number(e) => Ok(*e as u32),
            Self::PointerReference(e) => {
                if let Some(e) = labels.get(e) {
                    Ok(*e)
                } else {
                    Err(Errors::LabelNotFound {
                        label_name: e.to_owned(),
                    })
                }
            }
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
