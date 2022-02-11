pub mod expr;

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

    fn expect_num(&mut self) -> String {
        let digits = match &self.token {
            Token::Num(s) => s.clone(),
            _ => panic!("unexpected token"),
        };

        self.bump();

        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::token::Token;

    #[test]
    fn test_cursor() {
        let tokens = vec![Token::Num("1".into()), Token::Plus, Token::Num("2".into())];
        let mut cursor = TokenCursor::new(tokens);

        assert_eq!(cursor.next(), Some(Token::Num("1".into())));
        assert_eq!(cursor.next(), Some(Token::Plus));
        assert_eq!(cursor.next(), Some(Token::Num("2".into())));
        assert_eq!(cursor.next(), None);
    }
}
