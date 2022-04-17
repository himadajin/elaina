pub mod block;
pub mod expr;
pub mod lit;
pub mod op;
pub mod stmt;
pub mod token;

pub mod builder;

use span::symbol::Ident;

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    pub ident: Ident,
}
