use crate::{BlockId, Operand, Place};
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

    Call {
        fun: Operand,
        args: Vec<Operand>,
        destination: Option<(Place, BlockId)>,
    },

    Return,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchTargets {
    pub values: Vec<u128>,
    pub targets: Vec<BlockId>,
}
