pub mod def_id;
pub mod pp;

use crate::def_id::DefId;

use span::symbol::Symbol;
use ty;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    pub res: DefId,
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
    Binding(DefId, Symbol),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    /// Local represents a let statement: `let <ident> = <expr>;`
    Local {
        pat: Pat,
        ty: Option<ty::Ty>,
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
    Lit { lit: Lit, ty: ty::Ty },

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

#[derive(PartialEq, Debug)]
pub enum Fixity {
    /// The operator is left-associative
    Left,
    /// The operator is right-associative
    Right,
    /// The operator is not-associative
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    /// The `*` operator (multiplication)
    Mul,

    /// The `/` operator (division)
    Div,

    /// The `+` operator (addition)
    Add,

    /// The `-` operator (subtraction)
    Sub,

    /// The `==` operator (equality)
    Eq,

    /// The `<` operator (less then)
    Lt,

    /// The `<=` operator (less than or equal to)
    Le,

    /// The `!=` operator (not equal to)
    Ne,

    /// The `>=` operator (greater than or equal to)
    Ge,

    /// the `>` operator (greater than)
    Gt,
}

impl BinOp {
    pub fn precedence(&self) -> usize {
        use BinOp::*;
        match *self {
            Mul | Div => 13,
            Add | Sub => 12,
            Eq | Lt | Le | Ne | Ge | Gt => 7,
        }
    }

    pub fn fixity(&self) -> Fixity {
        use BinOp::*;
        match *self {
            Mul | Div | Add | Sub | Eq | Lt | Le | Ne | Ge | Gt => Fixity::Left,
        }
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Eq => write!(f, "=="),
            BinOp::Lt => write!(f, "<"),
            BinOp::Le => write!(f, "<="),
            BinOp::Ne => write!(f, "!="),
            BinOp::Ge => write!(f, ">="),
            BinOp::Gt => write!(f, ">"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnOp {
    /// The `-` operator (negation)
    Neg,
}
