use crate::{block::Block, stmt::*, ty::Ty};

use span::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub ident: Ident,
    pub kind: ItemKind,
}

impl Item {
    pub fn fn_dummy<T: Into<Vec<Param>>, U: Into<Vec<Stmt>>>(
        name: Symbol,
        inputs: T,
        output: Option<Ty>,
        block: U,
    ) -> Item {
        Item {
            ident: Ident::with_dummy_span(name),
            kind: ItemKind::Fn(Box::new(Fn {
                inputs: inputs.into(),
                output,
                body: Block::from(block),
            })),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ItemKind {
    Fn(Box<Fn>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Fn {
    pub inputs: Vec<Param>,
    pub output: Option<Ty>,
    pub body: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Param {
    pub ty: Ty,
    pub ident: Ident,
}

impl Param {
    pub fn new(ty: Ty, ident: Ident) -> Param {
        Param { ty, ident }
    }

    pub fn new_dummy(ty: Ty, name: Symbol) -> Param {
        Param {
            ty,
            ident: Ident::with_dummy_span(name),
        }
    }
}
