use crate::{BlockId, Operand, Place};
use ty;

#[derive(Debug, PartialEq, Clone)]
pub enum Terminator<'tcx> {
    Goto {
        target: BlockId,
    },

    SwitchInt {
        discr: Operand<'tcx>,
        switch_ty: ty::Ty<'tcx>,
        targets: SwitchTargets,
    },

    Call {
        fun: Operand<'tcx>,
        args: Vec<Operand<'tcx>>,
        destination: Option<(Place, BlockId)>,
    },

    Return,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SwitchTargets {
    pub values: Vec<u128>,
    pub targets: Vec<BlockId>,
}
