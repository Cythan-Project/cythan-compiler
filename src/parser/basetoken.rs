use crate::compiler::position::Position;
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
pub enum BaseToken<'a> {
    Literal(Position, Cow<'a, str>),
    //KeywordFn,
    OpenParenthesis(Position),
    CloseParenthesis(Position),
    OpenBrackets(Position),
    CloseBrackets(Position),
    Equals(Position),
}

#[inline]
pub fn compile(line: &str, line_number: usize) -> Vec<BaseToken> {
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
        out.push(if t == "(" {
            BaseToken::OpenParenthesis(Position::new(line_number, from, to))
        } else if t == ")" {
            BaseToken::CloseParenthesis(Position::new(line_number, from, to))
        } else if t == "{" {
            BaseToken::OpenBrackets(Position::new(line_number, from, to))
        } else if t == "}" {
            BaseToken::CloseBrackets(Position::new(line_number, from, to))
        } else if t == "=" {
            BaseToken::Equals(Position::new(line_number, from, to))
        } else {
            BaseToken::Literal(Position::new(line_number, from, to), t)
        });
    }
    out
}
