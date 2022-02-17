pub mod constant;
pub mod stmt;

use crate::stmt::*;

use std::fmt;

use derive_more::{From, Into};
use typed_index_collections::TiVec;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct Body {
    pub blocks: TiVec<BlockId, Block>,
    pub local_decls: TiVec<LocalId, LocalDecl>,
}

#[derive(Debug, From, Into, PartialEq, Clone, Copy)]
pub struct BlockId(usize);

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub stmts: Vec<Statement>,
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
