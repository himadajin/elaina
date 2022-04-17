use ast::op::{BinOp, UnOp};
use hir::def_id::DefId;
use span::symbol::Symbol;
use ty;

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
