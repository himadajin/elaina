use crate::Path;

#[derive(Clone, Debug, PartialEq)]
pub struct Ty {
    pub kind: TyKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TyKind {
    Path(Path),
}
