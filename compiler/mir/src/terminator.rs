use crate::{BlockId, Operand};
use ty;

#[derive(Debug, PartialEq, Clone)]
pub enum Terminator {
    Goto {
        target: BlockId,
    },

    SwitchInt {
        discr: Operand,
        switch_ty: ty::Ty,
        targets: SwitchTargets,
    },

    Return,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchTargets {
    pub values: Vec<u128>,
    pub targets: Vec<BlockId>,
}
