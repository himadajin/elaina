use ty;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Constant {
    pub ty: ty::Ty,
    pub literal: ConstValue,
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
}

impl fmt::Display for ScalarInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
