pub mod block;
pub mod expr;
pub mod item;
pub mod lit;
pub mod op;
pub mod stmt;
pub mod token;
pub mod ty;

pub use block::*;
pub use expr::*;
pub use item::*;
pub use lit::*;
pub use op::*;
pub use stmt::*;
pub use ty::*;

use span::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    pub ident: Ident,
}
