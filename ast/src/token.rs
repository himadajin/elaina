#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// A number token: `0`, `1`, `99`
    Integer(String),

    /// Identifier token: `foo`, `x`
    Ident(String),

    /// Keyword token: `let`
    Keyword(KwKind),

    /// The `=` token
    Eq,

    /// The `+` token
    Plus,

    /// The `-` token
    Minus,

    /// The `*` token
    Star,

    /// The `/` token
    Slash,

    /// The `(` token
    OpenParen,

    /// The `)` token
    CloseParen,

    /// The `'` token
    Semi,

    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub enum KwKind {
    /// The `let` keyword
    Let,
}
