use crate::Path;

use span::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Ty {
    pub kind: TyKind,
}

impl Ty {
    pub fn path_with_dummy_span<S: Into<Symbol>>(name: S) -> Ty {
        Ty {
            kind: TyKind::Path(Path {
                ident: Ident::with_dummy_span(name),
            }),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TyKind {
    Path(Path),
}
