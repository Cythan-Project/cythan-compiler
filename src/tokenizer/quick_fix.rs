use super::stage1::Position;

pub struct QuickFix {
    pub placement: QuickFixPosition,
    pub position: Position,
    pub text: String,
}

pub enum QuickFixPosition {
    BEFORE,
    AFTER,
    REPLACE,
    REPLACE_FIRST(usize),
}
