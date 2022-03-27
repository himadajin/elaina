use crate::BlockId;

#[derive(Debug, PartialEq, Clone)]
pub enum Terminator {
    Goto { target: BlockId },

    Return,
}
