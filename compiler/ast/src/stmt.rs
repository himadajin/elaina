use crate::expr::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    /// Local represents a let statement: `let <ident> = <expr>;`
    Local {
        ident: String,
        ty: Option<String>,
        init: Expr,
    },

    /// Expression statement: `1 + 1`
    Expr(Expr),

    /// Expression statement with semicolon: `1 + 1;`
    Semi(Expr),
}
