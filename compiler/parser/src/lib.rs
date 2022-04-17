pub mod lexer;

pub mod block;
pub mod error;
pub mod expr;
pub mod stmt;

use crate::{error::*, lexer::parse_all_token};
use ast::{block::Block, expr::Expr, stmt::Stmt, token::*};
use span::symbol::*;

use anyhow::Result;

struct TokenCursor<'a> {
    tokens: &'a Vec<Token>,
    cursor: usize,
}

impl<'a> TokenCursor<'a> {
    fn new(tokens: &'a Vec<Token>) -> Self {
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
    symbol_map: &'a SymbolMap<'a>,
    cursor: TokenCursor<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Tokens<'a>) -> Self {
        assert!(tokens.tokens.len() >= 1, "tokens is empty");

        let mut cursor = TokenCursor::new(&tokens.tokens);

        let token = cursor.next().unwrap();

        Self {
            token: token,
            symbol_map: &tokens.map,
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
    fn expect(&mut self, expected: &TokenKind) -> Result<()> {
        if &self.token.kind != expected {
            let err = ParseError::UnexpectedToken {
                expected: expected.clone(),
                found: self.token.kind.clone(),
            };

            return Err(err.into());
        }

        self.bump();

        Ok(())
    }

    fn expect_ident(&mut self) -> Result<Ident> {
        let name = match &self.token.kind {
            TokenKind::Ident(s) => Ok(*s),
            k => Err(ParseError::NotFoundIdent { found: k.clone() }),
        }?;
        let span = self.token.span;

        self.bump();

        Ok(Ident { name, span })
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
    fn consume_keyword<S: Into<Symbol>>(&mut self, kw: S) -> bool {
        if let TokenKind::Ident(s) = self.token.kind {
            if s == kw.into() {
                self.bump();
                return true;
            }
        }

        false
    }
}

pub fn parse_block_from_source_str(src: &str) -> Result<(Block, SymbolMap)> {
    let tokens = parse_all_token(src);
    let block = Parser::new(&tokens).parse_block()?;

    Ok((block, tokens.map))
}

pub fn parse_stmt_from_source_str(src: &str) -> Result<(Stmt, SymbolMap)> {
    let tokens = parse_all_token(src);
    let stmt = Parser::new(&tokens).parse_stmt()?;
    Ok((stmt, tokens.map))
}

pub fn parse_expr_from_source_str(src: &str) -> Result<(Expr, SymbolMap)> {
    let tokens = parse_all_token(src);
    let expr = Parser::new(&tokens).parse_expr()?;

    Ok((expr, tokens.map))
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

        let mut cursor = TokenCursor::new(&tokens);
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
