use span::symbol::Symbol;

use crate::{expr::*, stmt::*};

pub fn stmt_local(ident: Symbol, ty: Option<Symbol>, expr: Expr) -> Stmt {
    Stmt::Local {
        ident: ident,
        ty: ty,
        init: expr,
    }
}

pub fn stmt_expr(expr: Expr) -> Stmt {
    Stmt::Expr(expr)
}

pub fn stmt_semi(expr: Expr) -> Stmt {
    Stmt::Semi(expr)
}
