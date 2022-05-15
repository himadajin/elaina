use crate::{CommonTypes, Interner, Ty};

use std::fmt;
use std::ops::Deref;

pub struct CommonConsts<'tcx> {
    pub true_: Const<'tcx>,
    pub false_: Const<'tcx>,
    pub unit: Const<'tcx>,
}

impl<'tcx> CommonConsts<'tcx> {
    pub(crate) fn new(
        interner: &Interner<'tcx>,
        common_types: &CommonTypes<'tcx>,
    ) -> CommonConsts<'tcx> {
        let mk = |ty, literal| {
            let value = ConstValue { ty, literal };
            interner.intern_const(value)
        };

        CommonConsts {
            true_: mk(
                common_types.bool,
                ConstLit::Scalar(ScalarInt { data: 1, size: 1 }),
            ),
            false_: mk(
                common_types.bool,
                ConstLit::Scalar(ScalarInt { data: 0, size: 1 }),
            ),
            unit: mk(common_types.unit, ConstLit::ZST),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Const<'tcx>(pub(crate) &'tcx ConstValue<'tcx>);

impl<'tcx> Const<'tcx> {
    #[inline]
    pub fn value(&self) -> &'tcx ConstValue<'tcx> {
        self.0
    }
}

impl<'tcx> Deref for Const<'tcx> {
    type Target = ConstValue<'tcx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.value()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConstValue<'tcx> {
    pub ty: Ty<'tcx>,
    pub literal: ConstLit,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConstLit {
    Scalar(ScalarInt),
}

impl ConstLit {
    pub const ZST: ConstLit = ConstLit::Scalar(ScalarInt { data: 0, size: 0 });
}

impl fmt::Display for ConstLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstLit::Scalar(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ScalarInt {
    pub data: u128,
    pub size: u8,
}

impl fmt::Display for ScalarInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
