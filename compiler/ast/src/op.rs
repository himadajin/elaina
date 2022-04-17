use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Fixity {
    /// The operator is left-associative
    Left,
    /// The operator is right-associative
    Right,
    /// The operator is not-associative
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    /// The `*` operator (multiplication)
    Mul,

    /// The `/` operator (division)
    Div,

    /// The `+` operator (addition)
    Add,

    /// The `-` operator (subtraction)
    Sub,

    /// The `==` operator (equality)
    Eq,

    /// The `<` operator (less then)
    Lt,

    /// The `<=` operator (less than or equal to)
    Le,

    /// The `!=` operator (not equal to)
    Ne,

    /// The `>=` operator (greater than or equal to)
    Ge,

    /// the `>` operator (greater than)
    Gt,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Eq => write!(f, "=="),
            BinOp::Lt => write!(f, "<"),
            BinOp::Le => write!(f, "<="),
            BinOp::Ne => write!(f, "!="),
            BinOp::Ge => write!(f, ">="),
            BinOp::Gt => write!(f, ">"),
        }
    }
}

impl BinOp {
    pub fn precedence(&self) -> usize {
        use BinOp::*;
        match *self {
            Mul | Div => 13,
            Add | Sub => 12,
            Eq | Lt | Le | Ne | Ge | Gt => 7,
        }
    }

    pub fn fixity(&self) -> Fixity {
        use BinOp::*;
        match *self {
            Mul | Div | Add | Sub | Eq | Lt | Le | Ne | Ge | Gt => Fixity::Left,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnOp {
    /// The `-` operator (negation)
    Neg,
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnOp::Neg => write!(f, "-"),
        }
    }
}
