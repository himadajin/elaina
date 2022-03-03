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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnOp {
    /// The `-` operator (negation)
    Neg,
}
