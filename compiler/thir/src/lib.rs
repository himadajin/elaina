use ty;

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    /// Local represents a let statement: `let <ident> = <expr>;`
    Local { ident: String, init: Expr },

    /// Expression statement: `1 + 1`
    Expr(Expr),

    /// Expression statement with semicolon: `1 + 1;`
    Semi(Expr),
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

    /// A literal in place of an expression: `1`
    Lit { lit: Lit, ty: ty::Ty },

    /// A identifier such as variables, functions, etx: `foo`, `bar`
    Ident { ident: String, ty: ty::Ty },
}

impl Expr {
    pub fn ty(&self) -> ty::Ty {
        match self {
            Expr::Binary { ty, .. } => *ty,
            Expr::Unary { ty, .. } => *ty,
            Expr::Lit { ty, .. } => *ty,
            Expr::Ident { ty, .. } => *ty,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Lit {
    /// An integer literal: `0`, `1`, `64`
    Int(LitInt),
}

#[derive(Debug, PartialEq, Clone)]
pub struct LitInt {
    pub value: u128,
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
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnOp {
    /// The `-` operator (negation)
    Neg,
}
