use crate::{block::*, expr::*, lit::*, op::*, stmt::*};

pub fn expr_binary(lhs: Expr, op: BinOp, rhs: Expr) -> Expr {
    Expr::Binary {
        lhs: Box::new(lhs),
        op: op,
        rhs: Box::new(rhs),
    }
}

pub fn expr_unary(op: UnOp, expr: Expr) -> Expr {
    Expr::Unary {
        op: op,
        expr: Box::new(expr),
    }
}

pub fn expr_if(cond: Expr, then: Block, else_opt: Option<Expr>) -> Expr {
    Expr::If {
        cond: Box::new(cond),
        then: Box::new(then),
        else_opt: else_opt.map(|expr| Box::new(expr)),
    }
}

pub fn expr_block<T: Into<Vec<Stmt>>>(stmts: T) -> Expr {
    Expr::Block {
        block: Box::new(Block {
            stmts: stmts.into(),
        }),
    }
}

pub fn expr_lit_int<T: Into<String>>(digits: T) -> Expr {
    Expr::Lit {
        lit: Lit::Int {
            digits: digits.into(),
        },
    }
}

pub fn expr_lit_bool(value: bool) -> Expr {
    Expr::Lit {
        lit: Lit::Bool { value: value },
    }
}

pub fn expr_ident<T: Into<String>>(ident: T) -> Expr {
    Expr::Ident {
        ident: ident.into(),
    }
}
