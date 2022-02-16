use crate::{expr::*, stmt::Stmt};

pub fn stmt_expr(expr: Expr) -> Stmt {
    Stmt::Expr(expr)
}
