pub mod stmt;
pub mod terminator;

pub mod pp;

use crate::stmt::*;
use crate::terminator::*;
use span::Symbol;
use ty::{self, res::DefId};

use std::fmt;

use derive_more::{From, Into};
use typed_index_collections::TiVec;

#[derive(Debug, PartialEq, Clone)]
pub struct Body<'tcx> {
    pub def: DefId,
    pub name: Symbol,

    pub blocks: TiVec<BlockId, Block<'tcx>>,

    /// The first local is return value
    pub local_decls: TiVec<LocalId, LocalDecl<'tcx>>,
    pub arg_count: usize,
}

impl<'tcx> Body<'tcx> {
    pub fn new(def: DefId, name: Symbol) -> Self {
        Body {
            def,
            name,
            blocks: TiVec::new(),
            local_decls: TiVec::new(),
            arg_count: 0,
        }
    }

    pub fn id_return(&self) -> LocalId {
        LocalId(0)
    }

    pub fn id_args(&self) -> impl Iterator<Item = LocalId> {
        (1..(self.arg_count + 1)).map(|i| LocalId(i))
    }

    pub fn id_locals(&self) -> impl Iterator<Item = LocalId> {
        ((self.arg_count + 1)..).map(|i| LocalId(i))
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
pub struct Block<'tcx> {
    pub stmts: Vec<Statement<'tcx>>,
    pub terminator: Option<Terminator<'tcx>>,
}

impl<'tcx> Block<'tcx> {
    pub fn new(terminator: Option<Terminator<'tcx>>) -> Block<'tcx> {
        Block {
            stmts: Vec::new(),
            terminator,
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

    pub fn local(&self) -> LocalId {
        self.local
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
pub struct LocalDecl<'tcx> {
    pub name: Option<String>,
    pub ty: ty::Ty<'tcx>,
}

impl<'tcx> LocalDecl<'tcx> {
    pub fn new(name: Option<String>, ty: ty::Ty<'tcx>) -> LocalDecl<'tcx> {
        LocalDecl { name, ty }
    }
}

impl<'tcx> fmt::Display for LocalDecl<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => write!(f, "{}", name),
            None => Ok(()),
        }
    }
}
