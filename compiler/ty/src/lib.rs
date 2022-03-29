#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ty {
    pub kind: TyKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TyKind {
    Bool,

    Int(IntTy),

    Tuple(Vec<Ty>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntTy {
    I32,
}
