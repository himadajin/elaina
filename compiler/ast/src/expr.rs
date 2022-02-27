use crate::{lit::*, op::*};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// A binary operation: `a + b`, "a * b"
    Binary(ExprBinary),

    /// A unary operation: `-x`
    Unary(ExprUnary),

    /// A literal in place of an expression: `1`
    Lit(ExprLit),

    /// A identifier such as variables, functions, etx: `foo`, `bar`
    Ident(Ident),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprBinary {
    pub lhs: Box<Expr>,
    pub op: BinOp,
    pub rhs: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprUnary {
    pub op: UnOp,
    pub expr: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprLit {
    pub lit: Lit,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident {
    pub ident: String,
}
