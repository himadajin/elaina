use crate::{expr::*, stmt::*};

pub fn stmt_local<T: Into<String>, U: Into<String>>(ident: T, ty: U, expr: Expr) -> Stmt {
    let ty = {
        let s = ty.into();
        match s.as_str() {
            "" => None,
            _ => Some(s),
        }
    };
    Stmt::Local {
        ident: ident.into(),
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
