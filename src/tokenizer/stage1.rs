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
        use std::cmp::Ordering;

        let (line_from, caret_from) = match self
            .line_from
            .partial_cmp(&position.line_from)
            .expect("I don't like NaNs")
        {
            Ordering::Less => (self.line_from, self.caret_from),
            Ordering::Greater => (position.line_from, self.caret_from),
            Ordering::Equal => (self.line_from, self.caret_from.min(position.caret_from)),
        };
        let (line_to, caret_to) = match self
            .line_to
            .partial_cmp(&position.line_to)
            .expect("I don't like NaNs")
        {
            Ordering::Less => (position.line_to, self.caret_to),
            Ordering::Greater => (self.line_to, self.caret_to),
            Ordering::Equal => (self.line_to, self.caret_to.max(position.caret_to)),
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
        out.push(if t == "(" {
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
        });
    }
    out
}
