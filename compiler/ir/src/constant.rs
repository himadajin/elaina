use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Constant {
    Bool { value: bool },
    Scalar(ScalarInt),
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant::Bool { value } => write!(f, "{}", value),
            Constant::Scalar(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScalarInt {
    pub data: u128,
    pub size: u8,
}

impl fmt::Display for ScalarInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
