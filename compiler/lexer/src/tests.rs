use crate::token::*;
use crate::tokenize;

macro_rules! test_lexer {
    ($input: expr, $expected: expr) => {
        let tokens: Vec<Token> = tokenize($input).collect();

        assert_eq!($expected, tokens)
    };
}

#[test]
fn one_char_token() {
    test_lexer!(";", vec![Token::new(TokenKind::Semi, 1)]);

    test_lexer!("(", vec![Token::new(TokenKind::OpenParen, 1)]);
    test_lexer!(")", vec![Token::new(TokenKind::CloseParen, 1)]);
    test_lexer!("{", vec![Token::new(TokenKind::OpenBrace, 1)]);
    test_lexer!("}", vec![Token::new(TokenKind::CloseBrace, 1)]);

    test_lexer!("=", vec![Token::new(TokenKind::Eq, 1)]);
    test_lexer!("!", vec![Token::new(TokenKind::Bang, 1)]);
    test_lexer!("<", vec![Token::new(TokenKind::Lt, 1)]);
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
