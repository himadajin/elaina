use crate::expr::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    /// Expression statement: `1 + 1`
    Expr(Expr),

    /// Expression statement with semicolon: `1 + 1;`
    Semi(Expr),
}
