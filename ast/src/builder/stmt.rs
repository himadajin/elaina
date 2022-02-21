use crate::{expr::*, stmt::Stmt};

pub fn stmt_expr(expr: Expr) -> Stmt {
    Stmt::Expr(expr)
}

pub fn stmt_semi(expr: Expr) -> Stmt {
    Stmt::Semi(expr)
}
