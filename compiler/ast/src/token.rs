use span::{span::Span, symbol::Symbol};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BinOpToken {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DelimToken {
    Paren,
    Brace,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TokenKind {
    BinOp(BinOpToken),

    OpenDelim(DelimToken),
    CloseDelim(DelimToken),

    Ident(Symbol),
}

#[derive(Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
