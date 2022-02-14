use index::*;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct Body {
    stmts: Vec<Statement>,
    local_decls: IndexVec<LocalDecl>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Assign(Box<(Place, RValue)>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum RValue {
    Use(Operand),
    BinaryOp(BinOp, Box<(Operand, Operand)>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Copy(Place),
    Constant(Box<Constant>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct Place {
    pub local: Idx<LocalDecl>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct LocalDecl {
    name: Option<String>,
}

impl LocalDecl {
    pub fn new_anonymous() -> Self {
        LocalDecl { name: None }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Constant {
    Scalar(ScalarInt),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct ScalarInt {
    pub data: u128,
    pub size: u8,
}
