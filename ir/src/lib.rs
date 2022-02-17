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

    /// The first local is return value
    pub local_decls: TiVec<LocalId, LocalDecl>,
}

impl Body {
    pub fn new() -> Self {
        let mut body = Body {
            blocks: TiVec::new(),
            local_decls: TiVec::new(),
        };

        body.local_decls.push(LocalDecl::named("ret".into()));

        body
    }

    pub fn local_return(&self) -> Place {
        Place { local: LocalId(0) }
    }
}

#[derive(Debug, From, Into, PartialEq, Clone, Copy)]
pub struct BlockId(usize);

impl BlockId {
    pub fn dummy() -> Self {
        BlockId(usize::MAX)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub stmts: Vec<Statement>,
}

impl Block {
    pub fn new() -> Self {
        Block { stmts: Vec::new() }
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

impl LocalId {
    pub fn index(&self) -> usize {
        self.0
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct LocalDecl {
    pub name: Option<String>,
}

impl LocalDecl {
    pub fn named(name: String) -> Self {
        LocalDecl { name: Some(name) }
    }

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
