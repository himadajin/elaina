use crate::{expr::Expr, ty::Ty};
use span::symbol::Ident;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    /// Local represents a let statement: `let <ident> = <expr>;`
    Local {
        ident: Ident,
        ty: Option<Ty>,
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

impl Stmt {
    pub fn local(ident: Ident, ty: Option<Ty>, init: Expr) -> Stmt {
        Stmt::Local { ident, ty, init }
    }

    pub fn expr(expr: Expr) -> Stmt {
        Stmt::Expr(expr)
    }

    pub fn semi(expr: Expr) -> Stmt {
        Stmt::Semi(expr)
    }

    pub fn println(expr: Expr) -> Stmt {
        Stmt::Println(expr)
    }
}
