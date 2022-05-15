pub mod pp;

use ast::op::{BinOp, UnOp};
use span::*;
use ty::{
    self,
    res::{DefId, Res},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Item<'tcx> {
    pub res: Res,
    pub name: Symbol,
    pub kind: ItemKind<'tcx>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ItemKind<'tcx> {
    Fn(Box<Fn<'tcx>>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Fn<'tcx> {
    pub header: FnHeader<'tcx>,
    pub body: Block<'tcx>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FnHeader<'tcx> {
    pub def: DefId,
    pub name: Symbol,

    pub inputs: Vec<Param<'tcx>>,
    pub output: ty::Ty<'tcx>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Param<'tcx> {
    pub res: Res,
    pub name: Symbol,
    pub ty: ty::Ty<'tcx>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pat<'tcx> {
    pub ty: ty::Ty<'tcx>,
    pub kind: Box<PatKind<'tcx>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PatKind<'tcx> {
    Binding {
        res: Res,
        name: Symbol,
        ty: ty::Ty<'tcx>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block<'tcx> {
    pub stmts: Vec<Stmt<'tcx>>,
    pub expr: Option<Expr<'tcx>>,
    pub ty: ty::Ty<'tcx>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt<'tcx> {
    /// Local represents a let statement: `let <ident> = <expr>;`
    Local { pat: Pat<'tcx>, init: Expr<'tcx> },

    /// Expression statement: `1 + 1`
    Expr(Expr<'tcx>),

    /// Function call of `println`
    /// This statement is temporary, used until the function call is implemented
    Println(Expr<'tcx>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'tcx> {
    /// A function call: `foo(a, b)`
    Call {
        fun: Box<Expr<'tcx>>,
        args: Vec<Expr<'tcx>>,
        ty: ty::Ty<'tcx>,
    },

    /// A binary operation: `a + b`, "a * b"
    Binary {
        op: BinOp,
        lhs: Box<Expr<'tcx>>,
        rhs: Box<Expr<'tcx>>,
        ty: ty::Ty<'tcx>,
    },

    /// A unary operation: `-x`
    Unary {
        op: UnOp,
        expr: Box<Expr<'tcx>>,
        ty: ty::Ty<'tcx>,
    },

    /// An if expression: `if <cond> { <then> } else { <else_opt> }`
    If {
        cond: Box<Expr<'tcx>>,
        then: Box<Block<'tcx>>,
        else_opt: Option<Box<Expr<'tcx>>>,
        ty: ty::Ty<'tcx>,
    },

    /// Loop expression: `loop { block }`
    Loop { block: Box<Block<'tcx>> },

    /// Break expression: `break;`, `break expr;`
    Break {
        expr: Option<Box<Expr<'tcx>>>,
        ty: ty::Ty<'tcx>,
    },

    /// Continue expression: `continue;`, `continue expr;`
    Continue {
        expr: Option<Box<Expr<'tcx>>>,
        ty: ty::Ty<'tcx>,
    },

    /// Return expression: `return`, `return expr`
    Return {
        expr: Option<Box<Expr<'tcx>>>,
        ty: ty::Ty<'tcx>,
    },

    /// A block expression: `{ <stmts> }`, `{ <stmts>; <expr>}`
    Block { block: Box<Block<'tcx>> },

    /// Assign expression: `a = 1`
    Assign {
        lhs: Box<Expr<'tcx>>,
        rhs: Box<Expr<'tcx>>,
        ty: ty::Ty<'tcx>,
    },

    /// A literal in place of an expression: `1`
    Lit { lit: Lit, ty: ty::Ty<'tcx> },

    /// Local variable.
    VarRef { res: Res, ty: ty::Ty<'tcx> },
}

impl<'tcx> Expr<'tcx> {
    pub fn ty(&self) -> ty::Ty<'tcx> {
        match self {
            Expr::Call { ty, .. } => ty.clone(),
            Expr::Binary { ty, .. } => ty.clone(),
            Expr::Unary { ty, .. } => ty.clone(),
            Expr::If { ty, .. } => ty.clone(),
            Expr::Loop { block } => block.ty.clone(),
            Expr::Break { ty, .. } => ty.clone(),
            Expr::Continue { ty, .. } => ty.clone(),
            Expr::Return { ty, .. } => ty.clone(),
            Expr::Block { block } => block.ty.clone(),
            Expr::Assign { ty, .. } => ty.clone(),
            Expr::Lit { ty, .. } => ty.clone(),
            Expr::VarRef { ty, .. } => ty.clone(),
        }
    }

    pub fn precedence(&self) -> i8 {
        use Expr::*;
        match self {
            Break { .. } | Continue { .. } | Return { .. } => PREC_JUMP,
            Binary { op, .. } => op.precedence() as i8,
            Assign { .. } => PREC_ASSIGN,
            Unary { .. } => PREC_PREFIX,
            Call { .. } => PREC_POSTFIX,
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
