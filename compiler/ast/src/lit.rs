use span::span::Span;

#[derive(Debug, PartialEq, Clone)]
pub struct Lit {
    pub kind: LitKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LitKind {
    /// An integer literal: `0`, `1`, `64`
    Int(u128),

    /// A boolean literal: `true`, `false`
    Bool(bool),
}
