pub mod block;
pub mod expr;
pub mod item;
pub mod lit;
pub mod op;
pub mod stmt;
pub mod token;
pub mod ty;

use span::symbol::Ident;

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    pub ident: Ident,
}
