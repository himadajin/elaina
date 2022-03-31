pub mod constant;
pub mod stmt;
pub mod terminator;

pub mod pretty;

use crate::stmt::*;
use crate::terminator::*;
use ty;

use std::fmt;

use derive_more::{From, Into};
use typed_index_collections::TiVec;

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

        let local_ret = {
            let name = Some("ret".into());
            let ty_i32 = ty::Ty {
                kind: ty::TyKind::Int(ty::IntTy::I32),
            };
            LocalDecl::new(name, ty_i32)
        };
        body.local_decls.push(local_ret);

        body
    }

    pub fn local_return(&self) -> Place {
        Place { local: LocalId(0) }
    }
}

#[derive(Debug, From, Into, PartialEq, Eq, Clone, Copy, Hash)]
pub struct BlockId(usize);

impl BlockId {
    pub fn dummy() -> Self {
        BlockId(usize::MAX)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub stmts: Vec<Statement>,
    pub terminator: Option<Terminator>,
}

impl Block {
    pub fn new(terminator: Option<Terminator>) -> Self {
        Block {
            stmts: Vec::new(),
            terminator: terminator,
        }
    }
}

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

#[derive(Debug, PartialEq, Clone)]
pub struct LocalDecl {
    pub name: Option<String>,
    pub ty: ty::Ty,
}

impl LocalDecl {
    pub fn new(name: Option<String>, ty: ty::Ty) -> Self {
        LocalDecl { name: name, ty: ty }
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
