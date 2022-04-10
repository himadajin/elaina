use span::symbol::Ident;

use crate::{expr::*, stmt::*};

pub fn stmt_local(ident: Ident, ty: Option<Ident>, expr: Expr) -> Stmt {
    Stmt::Local {
        ident,
        ty,
        init: expr,
    }
}

pub fn stmt_expr(expr: Expr) -> Stmt {
    Stmt::Expr(expr)
}

pub fn stmt_semi(expr: Expr) -> Stmt {
    Stmt::Semi(expr)
}
