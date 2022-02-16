use index::*;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct Body {
    pub stmts: Vec<Statement>,
    pub local_decls: IndexVec<LocalDecl>,
}

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

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct Place {
    pub local: Idx<LocalDecl>,
}

impl Place {
    pub fn new(idx: Idx<LocalDecl>) -> Self {
        Place { local: idx }
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.local)
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct LocalDecl {
    name: Option<String>,
}

impl LocalDecl {
    pub fn unnamed() -> Self {
        LocalDecl { name: None }
    }
}

impl fmt::Display for LocalDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "{}", name),
            None => Ok(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Constant {
    Scalar(ScalarInt),
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant::Scalar(i) => write!(f, "{}", i),
        }
    }
}

#[allow(dead_code)]
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
