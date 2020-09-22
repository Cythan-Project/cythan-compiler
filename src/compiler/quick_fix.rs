use crate::compiler::position::Position;

#[derive(Clone, Debug)]
pub struct QuickFix {
    pub placement: QuickFixPosition,
    pub position: Position,
    pub text: String,
}

#[derive(Clone, Debug)]
pub enum QuickFixPosition {
    Before,
    After,
    Replace,
    ReplaceFirst(usize),
}
