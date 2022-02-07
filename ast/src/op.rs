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
