
use tokesies::*;
use std::borrow::Cow;

pub struct CythanV1;

impl filters::Filter for CythanV1 {
    fn on_char(&self, c: &char) -> (bool, bool) {
        match *c {
            ',' | '/'
            | '*' | '(' | ')'
            | '{' | '}' => (true, true),
            ' ' => (true,false),
            _ => (false, false),
        }
    }
}

#[derive(Debug,Clone)]
pub enum Stage1Token<'a> {
    Number(Number),
    Literal(Cow<'a, str>),
    KeywordFn,
    KeywordUse,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrackets,
    CloseBrackets,
    Equals
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
        out.push(if let Ok(e) = t.parse::<Number>() {
            Stage1Token::Number(e)
        } else if t == "fn" {
            Stage1Token::KeywordFn
        } else if t == "use" {
            Stage1Token::KeywordUse
        } else if t == "(" {
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
        });
    }
    out
}

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
}