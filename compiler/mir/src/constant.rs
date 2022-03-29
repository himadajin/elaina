use ty;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Constant {
    pub ty: ty::Ty,
    pub literal: ConstValue,
}

impl Constant {
    pub const TRUE: Constant = Constant {
        ty: ty::Ty {
            kind: ty::TyKind::Bool,
        },
        literal: ConstValue::Scalar(ScalarInt::TRUE),
    };

    pub const FALSE: Constant = Constant {
        ty: ty::Ty {
            kind: ty::TyKind::Bool,
        },
        literal: ConstValue::Scalar(ScalarInt::FALSE),
    };

    pub const UNIT: Constant = Constant {
        ty: ty::Ty {
            kind: ty::TyKind::Tuple(Vec::new()),
        },
        literal: ConstValue::Scalar(ScalarInt::ZST),
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConstValue {
    Scalar(ScalarInt),
}

impl fmt::Display for ConstValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstValue::Scalar(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScalarInt {
    pub data: u128,
    pub size: u8,
}

impl ScalarInt {
    pub const TRUE: ScalarInt = ScalarInt { data: 1, size: 1 };
    pub const FALSE: ScalarInt = ScalarInt { data: 0, size: 1 };
    pub const ZST: ScalarInt = ScalarInt { data: 0, size: 0 };
}

impl fmt::Display for ScalarInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
