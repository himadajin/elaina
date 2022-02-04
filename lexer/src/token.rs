#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// A number token: `0`, `1`, `99`
    Num(String),

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
}