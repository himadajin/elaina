use crate::{block::*, lit::*, op::*, stmt::Stmt, *};
use span::{span::Span, symbol::Symbol};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// A binary operation: `a + b`, "a * b"
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },

    /// A unary operation: `-x`
    Unary { op: UnOp, expr: Box<Expr> },

    /// If expression: `if a == b { 0 } else { 1 }`
    If {
        cond: Box<Expr>,
        then: Box<Block>,
        else_opt: Option<Box<Expr>>,
    },

    /// Loop expression: `loop { block }`
    Loop { block: Box<Block> },

    /// Break expression: `break;`, `break expr;`
    Break { expr: Option<Box<Expr>> },

    /// Continue expression: `continue;`, `continue expr;`
    Continue { expr: Option<Box<Expr>> },

    /// Block expression: `{ 0 }`, `{let a = 1; a}`
    Block { block: Box<Block> },

    /// Assign expression: `a = 1`
    Assign { lhs: Box<Expr>, rhs: Box<Expr> },

    /// A literal in place of an expression: `1`
    Lit { lit: Lit },

    /// A path such as variables, functions, etx: `foo`, `bar`
    Path(Path),
}

impl Expr {
    pub fn binary(op: BinOp, lhs: Expr, rhs: Expr) -> Expr {
        Expr::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn unary(op: UnOp, expr: Expr) -> Expr {
        Expr::Unary {
            op,
            expr: Box::new(expr),
        }
    }

    pub fn if_<T: Into<Block>>(cond: Expr, then: T, else_opt: Option<Expr>) -> Expr {
        Expr::If {
            cond: Box::new(cond),
            then: Box::new(then.into()),
            else_opt: else_opt.map(|e| Box::new(e)),
        }
    }

    pub fn loop_(block: Block) -> Expr {
        Expr::Loop {
            block: Box::new(block),
        }
    }

    pub fn loop_from<T: Into<Vec<Stmt>>>(stmts: T) -> Expr {
        Expr::Loop {
            block: Box::new(Block::from(stmts)),
        }
    }

    pub fn break_(expr: Option<Expr>) -> Expr {
        Expr::Break {
            expr: expr.map(|e| Box::new(e)),
        }
    }

    pub fn continue_(expr: Option<Expr>) -> Expr {
        Expr::Continue {
            expr: expr.map(|e| Box::new(e)),
        }
    }

    pub fn block(block: Block) -> Expr {
        Expr::Block {
            block: Box::new(block),
        }
    }

    pub fn block_from<T: Into<Vec<Stmt>>>(stmts: T) -> Expr {
        Expr::Block {
            block: Box::new(Block::from(stmts)),
        }
    }

    pub fn assign(lhs: Expr, rhs: Expr) -> Expr {
        Expr::Assign {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn lit(lit: Lit) -> Expr {
        Expr::Lit { lit }
    }

    pub fn lit_from_value<T: Into<LitKind>>(value: T, span: Span) -> Expr {
        Expr::Lit {
            lit: Lit::new(value.into(), span),
        }
    }

    pub fn lit_from_value_dummy<T: Into<LitKind>>(value: T) -> Expr {
        Expr::Lit {
            lit: Lit::new_dummy(value.into()),
        }
    }

    pub fn path(path: Path) -> Expr {
        Expr::Path(path)
    }

    pub fn path_dummy(symbol: Symbol) -> Expr {
        Expr::Path(Path {
            ident: Ident::with_dummy_span(symbol),
        })
    }
}
