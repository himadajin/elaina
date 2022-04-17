use span::span::{Span, DUMMY_SP};

#[derive(Debug, PartialEq, Clone)]
pub struct Lit {
    pub kind: LitKind,
    pub span: Span,
}

impl Lit {
    pub fn new(kind: LitKind, span: Span) -> Lit {
        Lit { kind, span }
    }

    pub fn new_dummy(kind: LitKind) -> Lit {
        Lit {
            kind,
            span: DUMMY_SP,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LitKind {
    /// An integer literal: `0`, `1`, `64`
    Int(u128),

    /// A boolean literal: `true`, `false`
    Bool(bool),
}

impl From<u128> for LitKind {
    fn from(value: u128) -> LitKind {
        LitKind::Int(value)
    }
}

impl From<bool> for LitKind {
    fn from(value: bool) -> LitKind {
        LitKind::Bool(value)
    }
}
