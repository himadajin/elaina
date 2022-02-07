use lexer::token::Token;

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

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::token::Token;

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
