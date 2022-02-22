use crate::constant::*;
use crate::Place;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Assign(Box<(Place, RValue)>),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Assign(assign) => write!(f, "{} = {}", assign.0, assign.1),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RValue {
    Use(Operand),
    BinaryOp(BinOp, Box<(Operand, Operand)>),
}

impl fmt::Display for RValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RValue::Use(operand) => write!(f, "{}", operand),
            RValue::BinaryOp(bo, operand) => {
                write!(f, "{}({}, {})", bo, operand.0, operand.1)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Add => write!(f, "Add"),
            BinOp::Sub => write!(f, "Sub"),
            BinOp::Mul => write!(f, "Mul"),
            BinOp::Div => write!(f, "Div"),
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
            Operand::Constant(c) => write!(f, "{}", c.as_ref()),
        }
    }
}
