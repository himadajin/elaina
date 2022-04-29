pub mod res;

use res::DefId;

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

    FnDef(DefId),

    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntTy {
    I32,
}
