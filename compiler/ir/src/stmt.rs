use crate::constant::*;
use crate::Place;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Assign(Box<(Place, RValue)>),

    /// Function call of `println`
    /// This statement is temporary, used until the function call is implemented
    Println(Operand),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Assign(assign) => write!(f, "{} = {}", assign.0, assign.1),
            Statement::Println(operand) => write!(f, "println({});", &operand),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RValue {
    Use(Operand),
    BinaryOp(BinOp, Box<(Operand, Operand)>),
    UnaryOp(UnOp, Box<Operand>),
}

impl fmt::Display for RValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RValue::Use(operand) => write!(f, "{}", operand),
            RValue::BinaryOp(bo, operand) => {
                write!(f, "{}({}, {})", bo, operand.0, operand.1)
            }
            RValue::UnaryOp(up, operand) => write!(f, "{}({})", up, operand),
        }
    }
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
pub enum Operand {
    Copy(Place),
    Constant(Box<Constant>),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Copy(p) => write!(f, "{}", p),
            Operand::Constant(c) => write!(f, "{}", c.literal),
        }
    }
}
