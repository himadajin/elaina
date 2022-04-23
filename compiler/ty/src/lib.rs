#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ty {
    pub kind: TyKind,
}

impl Ty {
    pub fn is_zst(&self) -> bool {
        match &self.kind {
            TyKind::Tuple(ts) => ts.is_empty(),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TyKind {
    Bool,

    Int(IntTy),

    Tuple(Vec<Ty>),

    Fn(FnTy),

    Never,
}

impl TyKind {
    pub fn to_fn_ty(&self) -> Option<&FnTy> {
        match &self {
            TyKind::Fn(ty) => Some(ty),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntTy {
    I32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnTy {
    pub inputs: Vec<Ty>,
    pub output: Box<Option<Ty>>,
}
