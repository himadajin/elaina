use crate::{block::*, stmt::*};

pub fn block<T: Into<Vec<Stmt>>>(stmts: T) -> Block {
    Block {
        stmts: stmts.into(),
    }
}
