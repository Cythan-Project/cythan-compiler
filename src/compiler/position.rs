use std::cmp::Ordering;
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Position {
    pub line_from: usize,
    pub line_to: usize,
    pub caret_from: usize,
    pub caret_to: usize,
}

impl Position {
    pub(crate) fn new(line: usize, from: usize, to: usize) -> Self {
        Self {
            line_from: line,
            line_to: line,
            caret_from: from,
            caret_to: to,
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.line_from == self.line_to {
            write!(
                f,
                "Error line {} [{},{}]",
                self.line_to, self.caret_from, self.caret_to
            )
        } else {
            write!(
                f,
                "Error from line {} [{}] to line {} [{}]",
                self.line_from, self.caret_from, self.line_to, self.caret_to
            )
        }
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, position: &Position) -> Position {
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
        Position {
            line_from,
            line_to,
            caret_from,
            caret_to,
        }
    }
}
