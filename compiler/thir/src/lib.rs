pub mod pp;

use ast::op::{BinOp, UnOp};
use hir::res::DefId;
use span::*;
use ty;

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub res: DefId,
    pub name: Symbol,
    pub kind: ItemKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ItemKind {
    Fn(Box<Fn>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Fn {
    pub ty: ty::Ty,
    pub inputs: Vec<Param>,
    pub body: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Param {
    pub res: DefId,
    pub name: Symbol,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pat {
    pub ty: ty::Ty,
    pub kind: Box<PatKind>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PatKind {
    Binding {
        res: DefId,
        name: Symbol,
        ty: ty::Ty,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub expr: Option<Expr>,
    pub ty: ty::Ty,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    /// Local represents a let statement: `let <ident> = <expr>;`
    Local { pat: Pat, init: Expr },

    /// Expression statement: `1 + 1`
    Expr(Expr),

    /// Function call of `println`
    /// This statement is temporary, used until the function call is implemented
    Println(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// A binary operation: `a + b`, "a * b"
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        ty: ty::Ty,
    },

    /// A unary operation: `-x`
    Unary {
        op: UnOp,
        expr: Box<Expr>,
        ty: ty::Ty,
    },

    /// An if expression: `if <cond> { <then> } else { <else_opt> }`
    If {
        cond: Box<Expr>,
        then: Box<Block>,
        else_opt: Option<Box<Expr>>,
        ty: ty::Ty,
    },

    /// Loop expression: `loop { block }`
    Loop { block: Box<Block> },

    /// Break expression: `break;`, `break expr;`
    Break { expr: Option<Box<Expr>>, ty: ty::Ty },

    /// Continue expression: `continue;`, `continue expr;`
    Continue { expr: Option<Box<Expr>>, ty: ty::Ty },

    /// A block expression: `{ <stmts> }`, `{ <stmts>; <expr>}`
    Block { block: Box<Block> },

    /// Assign expression: `a = 1`
    Assign {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        ty: ty::Ty,
    },

    /// A literal in place of an expression: `1`
    Lit { lit: Lit, ty: ty::Ty },

    /// Local variable.
    VarRef { def: DefId, ty: ty::Ty },
}

impl Expr {
    pub fn ty(&self) -> ty::Ty {
        match self {
            Expr::Binary { ty, .. } => ty.clone(),
            Expr::Unary { ty, .. } => ty.clone(),
            Expr::If { ty, .. } => ty.clone(),
            Expr::Loop { block } => block.ty.clone(),
            Expr::Break { ty, .. } => ty.clone(),
            Expr::Continue { ty, .. } => ty.clone(),
            Expr::Block { block } => block.ty.clone(),
            Expr::Assign { ty, .. } => ty.clone(),
            Expr::Lit { ty, .. } => ty.clone(),
            Expr::VarRef { ty, .. } => ty.clone(),
        }
    }

    pub fn precedence(&self) -> i8 {
        use Expr::*;
        match self {
            Break { .. } | Continue { .. } => PREC_JUMP,
            Binary { op, .. } => op.precedence() as i8,
            Assign { .. } => PREC_ASSIGN,
            Unary { .. } => PREC_PREFIX,
            Lit { .. } | VarRef { .. } | If { .. } | Loop { .. } | Block { .. } => PREC_PAREN,
        }
    }
}

pub const PREC_JUMP: i8 = -30;

// The range 2..=14 is reserved for AssocOp binary operator precedences.
pub const PREC_ASSIGN: i8 = 2;

pub const PREC_PREFIX: i8 = 50;
pub const PREC_POSTFIX: i8 = 60;
pub const PREC_PAREN: i8 = 99;
pub const PREC_FORCE_PAREN: i8 = 100;

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
