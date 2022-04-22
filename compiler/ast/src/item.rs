use crate::{block::Block, ty::Ty};

use span::symbol::Ident;

pub struct Item {
    pub ident: Ident,
    pub kind: ItemKind,
}

pub enum ItemKind {
    Fn(Box<Fn>),
}

pub struct Fn {
    pub inputs: Vec<Param>,
    pub output: Option<Ty>,
    pub body: Block,
}

pub struct Param {
    pub ty: Ty,
    pub ident: Ident,
}
