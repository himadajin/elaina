pub mod expr;
pub mod stmt;

use ast::token::*;
use core::panic;

struct TokenCursor {
    tokens: Vec<Token>,
    cursor: usize,
}

impl TokenCursor {
    #[allow(dead_code)]
    fn new(tokens: Vec<Token>) -> Self {
        TokenCursor {
            tokens: tokens,
            cursor: 0,
        }
    }

    #[allow(dead_code)]
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

    fn bump(&mut self) {
        let next_token = self.cursor.next().unwrap_or(Token::Eof);
        self.token = next_token;
    }

    fn expect(&mut self, expected: &Token) {
        if &self.token != expected {
            panic!("expected {:?} but current token is {:?}", expected, self.token);
        }

        self.bump();
    }

    fn expect_int(&mut self) -> String {
        let digits = match &self.token {
            Token::Integer(s) => s.clone(),
            _ => panic!("unexpected token"),
        };

        self.bump();

        digits
    }

    fn consume(&mut self, expected: &Token) -> bool {
        if &self.token == expected {
            self.bump();

            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::token::Token;

    #[test]
    fn test_cursor() {
        let tokens = vec![Token::Integer("1".into()), Token::Plus, Token::Integer("2".into())];
        let mut cursor = TokenCursor::new(tokens);

        assert_eq!(cursor.next(), Some(Token::Integer("1".into())));
        assert_eq!(cursor.next(), Some(Token::Plus));
        assert_eq!(cursor.next(), Some(Token::Integer("2".into())));
        assert_eq!(cursor.next(), None);
    }
}
