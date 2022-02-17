pub mod constant;
pub mod stmt;

use crate::stmt::*;

use std::fmt;

use derive_more::{From, Into};
use typed_index_collections::TiVec;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct Body {
    pub stmts: Vec<Statement>,
    pub local_decls: TiVec<LocalId, LocalDecl>,
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
pub struct Place {
    pub local: LocalId,
}

impl Place {
    pub fn new(idx: LocalId) -> Self {
        Place { local: idx }
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.local.0)
    }
}

#[derive(Debug, From, Into, PartialEq, Clone, Copy)]
pub struct LocalId(usize);

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

