use crate::stmt::Stmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Block {
        Block { stmts }
    }
}

impl<T: Into<Vec<Stmt>>> From<T> for Block {
    fn from(stmts: T) -> Block {
        Block {
            stmts: stmts.into(),
        }
    }
}
