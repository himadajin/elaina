use crate::{
    expr::*,
    stmt::{Local, Stmt},
};

pub fn stmt_local<T: Into<String>>(ident: T, expr: Expr) -> Stmt {
    Stmt::Local(Local {
        ident: ident.into(),
        init: expr,
    })
}

pub fn stmt_expr(expr: Expr) -> Stmt {
    Stmt::Expr(expr)
}

pub fn stmt_semi(expr: Expr) -> Stmt {
    Stmt::Semi(expr)
}
