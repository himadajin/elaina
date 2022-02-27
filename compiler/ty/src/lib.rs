#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ty {
    pub kind: TyKind,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TyKind {
    Int(IntTy),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntTy {
    I32,
}
