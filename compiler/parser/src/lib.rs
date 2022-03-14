pub mod lexer;

pub mod block;
pub mod expr;
pub mod stmt;

use crate::lexer::parse_all_token;
use ast::{block::Block, token::*};
use span::symbol::*;

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

pub struct Parser<'a> {
    token: Token,
    symbol_map: SymbolMap<'a>,
    cursor: TokenCursor,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, symbol_map: SymbolMap<'a>) -> Self {
        assert!(tokens.len() >= 1, "tokens is empty");

        let mut cursor = TokenCursor::new(tokens);

        let token = cursor.next().unwrap();

        Self {
            token: token,
            symbol_map: symbol_map,
            cursor: cursor,
        }
    }

    /// Advance one token.
    fn bump(&mut self) {
        let next_token = self
            .cursor
            .next()
            .unwrap_or(Token::new(TokenKind::Eof, span::span::DUMMY_SP));
        self.token = next_token;
    }

    /// Expect the next token to be the given argument, and advance one token.
    /// If it is not, panic.
    fn expect(&mut self, expected: &TokenKind) {
        if &self.token.kind != expected {
            panic!(
                "expected {:?} but current token is {:?}",
                expected, self.token.kind
            );
        }

        self.bump();
    }

    fn expect_ident(&mut self) -> Symbol {
        let symbol = match &self.token.kind {
            TokenKind::Ident(s) => *s,
            k => panic!("expected identifier, but got {:?}", &k),
        };

        self.bump();

        symbol
    }

    /// If the next token is equal to the given argument, advance one token and return `true`.
    /// Otherwise, do nothing and return `false`
    fn consume(&mut self, expected: &TokenKind) -> bool {
        if &self.token.kind == expected {
            self.bump();

            return true;
        }

        false
    }

    /// If the next token is Identifier and that symbol is equal to given keyword,
    /// advance one token and return `true`.
    /// Otherwise, do nothing and return `false`
    fn consume_keyword(&mut self, kw: Symbol) -> bool {
        if let TokenKind::Ident(s) = self.token.kind {
            if s == kw {
                self.bump();
                return true;
            }
        }

        false
    }
}

pub fn parse_block_from_source_str(src: &str) -> Block {
    let (tokens, symbol_map) = parse_all_token(src);

    Parser::new(tokens, symbol_map).parse_block()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use ast::token::Token;
    // use cra::lexer::token;
    use span::span::*;

    #[test]
    fn test_cursor() {
        let tokens = vec![
            Token::new(TokenKind::BinOp(BinOpToken::Plus), Span::new(0, 1)),
            Token::new(TokenKind::BinOp(BinOpToken::Minus), Span::new(0, 1)),
            Token::new(TokenKind::BinOp(BinOpToken::Star), Span::new(0, 1)),
            Token::new(TokenKind::BinOp(BinOpToken::Slash), Span::new(0, 1)),
        ];

        let mut cursor = TokenCursor::new(tokens);
        assert_eq!(
            cursor.next(),
            Some(Token::new(
                TokenKind::BinOp(BinOpToken::Plus),
                Span::new(0, 1)
            ))
        );
        assert_eq!(
            cursor.next(),
            Some(Token::new(
                TokenKind::BinOp(BinOpToken::Minus),
                Span::new(0, 1)
            ))
        );
        assert_eq!(
            cursor.next(),
            Some(Token::new(
                TokenKind::BinOp(BinOpToken::Star),
                Span::new(0, 1)
            ))
        );
        assert_eq!(
            cursor.next(),
            Some(Token::new(
                TokenKind::BinOp(BinOpToken::Slash),
                Span::new(0, 1)
            ))
        );
    }
}
