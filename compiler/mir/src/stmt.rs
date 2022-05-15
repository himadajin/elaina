use crate::Place;
use std::fmt;

use ty::Const;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement<'tcx> {
    Assign(Box<(Place, RValue<'tcx>)>),

    /// Function call of `println`
    /// This statement is temporary, used until the function call is implemented
    Println(Operand<'tcx>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum RValue<'tcx> {
    Use(Operand<'tcx>),
    BinaryOp(BinOp, Box<(Operand<'tcx>, Operand<'tcx>)>),
    UnaryOp(UnOp, Box<Operand<'tcx>>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Lt,
    Le,
    Ne,
    Ge,
    Gt,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Add => write!(f, "Add"),
            BinOp::Sub => write!(f, "Sub"),
            BinOp::Mul => write!(f, "Mul"),
            BinOp::Div => write!(f, "Div"),
            BinOp::Eq => write!(f, "Eq"),
            BinOp::Lt => write!(f, "Lt"),
            BinOp::Le => write!(f, "Le"),
            BinOp::Ne => write!(f, "Ne"),
            BinOp::Ge => write!(f, "Ge"),
            BinOp::Gt => write!(f, "Gt"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnOp {
    Neg,
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnOp::Neg => write!(f, "Neg"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operand<'tcx> {
    Copy(Place),
    Constant(Box<Const<'tcx>>),
}
