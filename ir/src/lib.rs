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
    local: Idx<LocalDecl>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct LocalDecl {
    name: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Constant {
    Scalar(ScalarInt),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub struct ScalarInt {
    data: u128,
    size: u8,
}
