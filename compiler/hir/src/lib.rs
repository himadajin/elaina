pub mod pp;

use ::ty::res::{DefId, Res};

use ast::{
    op::{BinOp, UnOp},
    ty,
};
use span::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub res: Res,
    pub name: Symbol,
    pub kind: ItemKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ItemKind {
    Fn(Box<Fn>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Fn {
    pub inputs: Vec<Param>,
    pub output: Option<ast::ty::Ty>,
    pub body: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Param {
    pub res: Res,
    pub name: Symbol,
    pub ty: ast::ty::Ty,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    pub res: Res,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub expr: Option<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Pat {
    pub kind: PatKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatKind {
    Binding { res: Res, name: Symbol },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    /// Local represents a let statement: `let <ident> = <expr>;`
    Local {
        pat: Pat,
        ty: Option<ast::ty::Ty>,
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

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// A function call: `foo(a, b)`
    Call { fun: Box<Expr>, args: Vec<Expr> },

    /// A binary operation: `a + b`, "a * b"
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },

    /// A unary operation: `-x`
    Unary { op: UnOp, expr: Box<Expr> },

    /// An if expression: `if <cond> { <then> } else { <else_opt> }`
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

    /// A block expression: `{ <stmts> }`, `{ <stmts>; <expr>}`
    Block { block: Box<Block> },

    /// Assign expression: `a = 1`
    Assign { lhs: Box<Expr>, rhs: Box<Expr> },

    /// A literal in place of an expression: `1`
    Lit { lit: Lit },

    /// A path such as variables, functions, etx: `foo`, `bar`
    Path { path: Path },
}

pub const PREC_JUMP: i8 = -30;

// The range 2..=14 is reserved for AssocOp binary operator precedences.
pub const PREC_ASSIGN: i8 = 2;

pub const PREC_PREFIX: i8 = 50;
pub const PREC_POSTFIX: i8 = 60;
pub const PREC_PAREN: i8 = 99;
pub const PREC_FORCE_PAREN: i8 = 100;

impl Expr {
    pub fn precedence(&self) -> i8 {
        use Expr::*;
        match self {
            Break { .. } | Continue { .. } => PREC_JUMP,
            Binary { op, .. } => op.precedence() as i8,
            Assign { .. } => PREC_ASSIGN,
            Unary { .. } => PREC_PREFIX,
            Call { .. } => PREC_POSTFIX,
            Lit { .. } | Path { .. } | If { .. } | Loop { .. } | Block { .. } => PREC_PAREN,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Lit {
    /// A boolean literal: `true`, `false`
    Bool { value: bool },

    /// An integer literal: `0`, `1`, `64`
    Int(LitInt),
}

#[derive(Debug, PartialEq, Clone)]
pub struct LitInt {
    pub value: u128,
}
