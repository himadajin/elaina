#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// Any whitespace characters sequence.
    Whitespace,

    /// `;`
    Semi,

    /// `(`
    OpenParen,

    /// `)`
    CloseParen,

    /// `{`
    OpenBrace,

    /// `}`
    CloseBrace,

    /// `=`
    Eq,

    /// `!`
    Bang,

    /// `<`
    Lt,

    /// `>`
    Gt,

    /// `-`
    Minus,

    /// `+`
    Plus,

    /// `*`
    Star,

    /// `/`
    Slash,

    /// Unknown token, not expected by the lexer.
    Unknown,
}
