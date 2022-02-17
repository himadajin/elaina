use crate::constant::*;
use crate::Place;

use std::fmt;

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum RValue {
    Use(Operand),
    BinaryOp(BinOp, Box<(Operand, Operand)>),
}

#[allow(dead_code)]
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
