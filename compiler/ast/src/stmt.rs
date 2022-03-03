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

    /// Function call of `println`
    /// This statement is temporary, used until the function call is implemented
    Println(Expr),
}
