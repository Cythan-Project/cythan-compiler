#[derive(Clone, Debug)]
pub enum Phase1Token {
    Relative,         // ~
    NumberSign(bool), // bool: is_negative
    Dot,              // .
    QuestionMark,     // ?
}

#[derive(Clone, Debug)]
pub enum Phase2Token {
    Variable(String), // A-Za-z0-9_
    Number(isize),
    LabelAssign(String),
    Label(String, isize),
    Range(isize, Option<isize>),
    Relative(isize),
    VariableIndexed(String),
    Or(usize),
    OrRelative(isize),
    OrLabel(String, isize),
}

#[derive(Clone, Debug)]
pub enum CompilationToken {
    Phase1Token(Phase1Token),
    Phase2Token(Phase2Token),
}