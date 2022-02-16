use crate::{expr::*, lit::*, op::*};

pub fn expr_binary(lhs: Expr, op: BinOp, rhs: Expr) -> Expr {
    Expr::Binary(ExprBinary {
        lhs: Box::new(lhs),
        op: op,
        rhs: Box::new(rhs),
    })
}

pub fn expr_unary(op: UnOp, expr: Expr) -> Expr {
    Expr::Unary(ExprUnary {
        op: op,
        expr: Box::new(expr),
    })
}

pub fn expr_lit_int<T: Into<String>>(digits: T) -> Expr {
    Expr::Lit(ExprLit {
        lit: Lit::Int(LitInt {
            digits: digits.into(),
        }),
    })
}
