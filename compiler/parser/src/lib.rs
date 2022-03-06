pub mod lexer;

pub mod block;
pub mod expr;
pub mod stmt;

use ast::{block::Block, token_old::*};
use core::panic;
use lexer_old::run_lexer;

struct TokenCursor {
    tokens: Vec<Token>,
    cursor: usize,
}

impl TokenCursor {
    fn new(tokens: Vec<Token>) -> Self {
        TokenCursor {
            tokens: tokens,
            cursor: 0,
        }
    }

    fn next(&mut self) -> Option<Token> {
        if self.cursor >= self.tokens.len() {
            return None;
        }

        let token = self.tokens[self.cursor].clone();
        self.cursor += 1;

        Some(token)
    }
}

pub struct Parser {
    token: Token,

    cursor: TokenCursor,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        assert!(tokens.len() >= 1, "tokens is empty");

        let mut cursor = TokenCursor::new(tokens);

        let token = cursor.next().unwrap();

        Parser {
            token: token,
            cursor: cursor,
        }
    }

    /// Advance one token.
    fn bump(&mut self) {
        let next_token = self.cursor.next().unwrap_or(Token::Eof);
        self.token = next_token;
    }

    /// Expect the next token to be the given argument, and advance one token.
    /// If it is not, panic.
    fn expect(&mut self, expected: &Token) {
        if &self.token != expected {
            panic!(
                "expected {:?} but current token is {:?}",
                expected, self.token
            );
        }

        self.bump();
    }

    /// Expect the next token to be integer, and advance one token.
    /// If it is not, panic.
    fn expect_int(&mut self) -> String {
        let digits = match &self.token {
            Token::Integer(s) => s.clone(),
            _ => panic!("unexpected token"),
        };

        self.bump();

        digits
    }

    fn expect_ident(&mut self) -> String {
        let ident = match &self.token {
            Token::Ident(s) => s.clone(),
            _ => panic!("unexpected token"),
        };

        self.bump();

        ident
    }

    /// If the next token is equal to the given argument, advance one token and return `true`.
    /// Otherwise, do nothing and return `false`
    fn consume(&mut self, expected: &Token) -> bool {
        if &self.token == expected {
            self.bump();

            return true;
        }

        false
    }
}

pub fn parse_block_from_source_str(src: &str) -> Block {
    let tokens = run_lexer(src);

    Parser::new(tokens).parse_block()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::token_old::Token;

    #[test]
    fn test_cursor() {
        let tokens = vec![
            Token::Integer("1".into()),
            Token::Plus,
            Token::Integer("2".into()),
        ];
        let mut cursor = TokenCursor::new(tokens);

        assert_eq!(cursor.next(), Some(Token::Integer("1".into())));
        assert_eq!(cursor.next(), Some(Token::Plus));
        assert_eq!(cursor.next(), Some(Token::Integer("2".into())));
        assert_eq!(cursor.next(), None);
    }
}
