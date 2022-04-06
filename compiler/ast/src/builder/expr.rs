use crate::{block::*, expr::*, lit::*, op::*, stmt::*};
use span::{
    span::DUMMY_SP,
    symbol::{Ident, Symbol},
};

pub fn expr_binary(lhs: Expr, op: BinOp, rhs: Expr) -> Expr {
    Expr::Binary {
        lhs: Box::new(lhs),
        op: op,
        rhs: Box::new(rhs),
    }
}

pub fn expr_unary(op: UnOp, expr: Expr) -> Expr {
    Expr::Unary {
        op: op,
        expr: Box::new(expr),
    }
}

pub fn expr_if(cond: Expr, then: Block, else_opt: Option<Expr>) -> Expr {
    Expr::If {
        cond: Box::new(cond),
        then: Box::new(then),
        else_opt: else_opt.map(|expr| Box::new(expr)),
    }
}

pub fn expr_loop(block: Block) -> Expr {
    Expr::Loop {
        block: Box::new(block),
    }
}

pub fn expr_break(expr: Option<Expr>) -> Expr {
    Expr::Break {
        expr: expr.map(|e| Box::new(e)),
    }
}

pub fn expr_continue(expr: Option<Expr>) -> Expr {
    Expr::Continue {
        expr: expr.map(|e| Box::new(e)),
    }
}

pub fn expr_block<T: Into<Vec<Stmt>>>(stmts: T) -> Expr {
    Expr::Block {
        block: Box::new(Block {
            stmts: stmts.into(),
        }),
    }
}

pub fn expr_assign(lhs: Expr, rhs: Expr) -> Expr {
    Expr::Assign {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

pub fn expr_lit_int(value: u128) -> Expr {
    Expr::Lit {
        lit: Lit {
            kind: LitKind::Int(value),
            span: DUMMY_SP,
        },
    }
}

pub fn expr_lit_bool(value: bool) -> Expr {
    Expr::Lit {
        lit: Lit {
            kind: LitKind::Bool(value),
            span: DUMMY_SP,
        },
    }
}

pub fn expr_ident(symbol: Symbol) -> Expr {
    Expr::Ident { ident: symbol }
}

pub fn expr_path(symbol: Symbol) -> Expr {
    Expr::Path(Path {
        ident: Ident::with_dummy_span(symbol),
    })
}
