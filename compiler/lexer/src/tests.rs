use crate::token::*;
use crate::tokenize;

macro_rules! test_lexer {
    ($input: expr, $expected: expr) => {
        let tokens: Vec<Token> = tokenize($input).collect();

        assert_eq!($expected, tokens)
    };
}

#[test]
fn paren() {
    test_lexer!("(", vec![Token::new(TokenKind::OpenParen, 1)]);
    test_lexer!(")", vec![Token::new(TokenKind::CloseParen, 1)]);
    test_lexer!("{", vec![Token::new(TokenKind::OpenBrace, 1)]);
    test_lexer!("}", vec![Token::new(TokenKind::CloseBrace, 1)]);
}

#[test]
fn symbol() {
    test_lexer!(";", vec![Token::new(TokenKind::Semi, 1)]);
    test_lexer!(":", vec![Token::new(TokenKind::Colon, 1)]);

    test_lexer!("->", vec![Token::new(TokenKind::Arrow, 2)]);
    test_lexer!("=", vec![Token::new(TokenKind::Eq, 1)]);
    test_lexer!("!", vec![Token::new(TokenKind::Bang, 1)]);

    test_lexer!("<", vec![Token::new(TokenKind::Lt, 1)]);
    test_lexer!("<=", vec![Token::new(TokenKind::Le, 2)]);
    test_lexer!("==", vec![Token::new(TokenKind::EqEq, 2)]);
    test_lexer!("!=", vec![Token::new(TokenKind::Ne, 2)]);
    test_lexer!(">=", vec![Token::new(TokenKind::Ge, 2)]);
    test_lexer!(">", vec![Token::new(TokenKind::Gt, 1)]);

    test_lexer!("-", vec![Token::new(TokenKind::Minus, 1)]);
    test_lexer!("+", vec![Token::new(TokenKind::Plus, 1)]);
    test_lexer!("*", vec![Token::new(TokenKind::Star, 1)]);
    test_lexer!("/", vec![Token::new(TokenKind::Slash, 1)]);
}

#[test]
fn whitespace() {
    test_lexer!(
        " ;   ;",
        vec![
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::Semi, 1),
            Token::new(TokenKind::Whitespace, 3),
            Token::new(TokenKind::Semi, 1)
        ]
    );
}

#[test]
fn ident() {
    test_lexer!("true", vec![Token::new(TokenKind::Ident, 4)]);
    test_lexer!("false", vec![Token::new(TokenKind::Ident, 5)]);

    test_lexer!("aa1", vec![Token::new(TokenKind::Ident, 3)]);
    test_lexer!("a1a", vec![Token::new(TokenKind::Ident, 3)]);

    test_lexer!(
        " foo ",
        vec![
            Token::new(TokenKind::Whitespace, 1),
            Token::new(TokenKind::Ident, 3),
            Token::new(TokenKind::Whitespace, 1)
        ]
    );
}

#[test]
fn number() {
    test_lexer!(
        "0",
        vec![Token::new(
            TokenKind::Literal {
                kind: LiteralKind::Int
            },
            1
        ),]
    );
    test_lexer!(
        "1",
        vec![Token::new(
            TokenKind::Literal {
                kind: LiteralKind::Int
            },
            1
        )]
    );
    test_lexer!(
        "10",
        vec![Token::new(
            TokenKind::Literal {
                kind: LiteralKind::Int
            },
            2
        )]
    );
    test_lexer!(
        "01",
        vec![Token::new(
            TokenKind::Literal {
                kind: LiteralKind::Int
            },
            2
        )]
    );
}
