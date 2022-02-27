use crate::{lit::*, op::*};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// A binary operation: `a + b`, "a * b"
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },

    /// A unary operation: `-x`
    Unary { op: UnOp, expr: Box<Expr> },

    /// A literal in place of an expression: `1`
    Lit { lit: Lit },

    /// A identifier such as variables, functions, etx: `foo`, `bar`
    Ident { ident: String },
}
