use ast::token::*;
use lexer::{first_token, token};
use span::{span::Span, symbol::*};

pub fn parse_all_token(src: &str) -> Tokens {
    let mut lexer = Lexer::new(src);

    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        match token.kind {
            TokenKind::Eof => break,
            _ => tokens.push(token),
        }
    }

    Tokens {
        tokens: tokens,
        map: lexer.finish(),
    }
}

pub struct Lexer<'a> {
    pos: usize,
    src: &'a str,
    symbol_map: SymbolMap<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            pos: 0,
            src: src,
            symbol_map: SymbolMap::new(),
        }
    }

    pub fn next_token(&mut self) -> ast::token::Token {
        loop {
            let text = &self.src[self.pos..];

            if text.is_empty() {
                let span = Span::new(self.pos as u32, self.pos as u32);
                return ast::token::Token::new(ast::token::TokenKind::Eof, span);
            }

            let token = first_token(text);
            let start = self.pos;
            self.pos += token.len;

            match self.cook_lexer_token(token.kind, start) {
                Some(kind) => {
                    let span = Span::new(start as u32, self.pos as u32);
                    return ast::token::Token::new(kind, span);
                }
                None => (),
            }
        }
    }

    fn finish(self) -> SymbolMap<'a> {
        self.symbol_map
    }

    fn cook_lexer_token(&mut self, token: token::TokenKind, start: usize) -> Option<TokenKind> {
        Some(match token {
            token::TokenKind::Whitespace => return None,
            token::TokenKind::Ident => {
                let ident = self.str_from(start);
                let symbol = self.symbol_map.insert(ident);

                TokenKind::Ident(symbol)
            }
            token::TokenKind::Literal { kind } => {
                let kind = match kind {
                    token::LiteralKind::Int => LitKind::Integer,
                };
                let string = self.str_from(start);
                let symbol = self.symbol_map.insert(string);
                TokenKind::Literal(Lit { kind, symbol })
            }
            token::TokenKind::Semi => TokenKind::Semi,
            token::TokenKind::Colon => TokenKind::Colon,
            token::TokenKind::OpenParen => TokenKind::OpenDelim(DelimToken::Paren),
            token::TokenKind::CloseParen => TokenKind::CloseDelim(DelimToken::Paren),
            token::TokenKind::OpenBrace => TokenKind::OpenDelim(DelimToken::Brace),
            token::TokenKind::CloseBrace => TokenKind::CloseDelim(DelimToken::Brace),
            token::TokenKind::Eq => TokenKind::Eq,
            token::TokenKind::Bang => todo!(),

            token::TokenKind::Lt => TokenKind::Lt,
            token::TokenKind::Le => TokenKind::Le,
            token::TokenKind::EqEq => TokenKind::EqEq,
            token::TokenKind::Ne => TokenKind::Ne,
            token::TokenKind::Ge => TokenKind::Ge,
            token::TokenKind::Gt => TokenKind::Gt,

            token::TokenKind::Minus => TokenKind::BinOp(BinOpToken::Minus),
            token::TokenKind::Plus => TokenKind::BinOp(BinOpToken::Plus),
            token::TokenKind::Star => TokenKind::BinOp(BinOpToken::Star),
            token::TokenKind::Slash => TokenKind::BinOp(BinOpToken::Slash),
            token::TokenKind::Unknown => todo!(),
        })
    }

    fn str_from(&self, start: usize) -> &'a str {
        self.str_from_to(start, self.pos)
    }

    fn str_from_to(&self, start: usize, end: usize) -> &'a str {
        &self.src[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_lexer {
        ($input: expr, $expected: expr) => {
            let tokens = parse_all_token($input).tokens;

            assert_eq!(tokens.len(), $expected.len());

            for (result, expeced) in tokens.iter().zip($expected) {
                assert_eq!(*result, expeced);
            }
        };
    }

    #[test]
    fn paren() {
        test_lexer!(
            "(",
            vec![Token::new(
                TokenKind::OpenDelim(DelimToken::Paren),
                Span::new(0, 1)
            )]
        );
        test_lexer!(
            ")",
            vec![Token::new(
                TokenKind::CloseDelim(DelimToken::Paren),
                Span::new(0, 1)
            )]
        );
        test_lexer!(
            "{",
            vec![Token::new(
                TokenKind::OpenDelim(DelimToken::Brace),
                Span::new(0, 1)
            )]
        );
        test_lexer!(
            "}",
            vec![Token::new(
                TokenKind::CloseDelim(DelimToken::Brace),
                Span::new(0, 1)
            )]
        );
    }

    #[test]
    fn symbol() {
        test_lexer!("=", vec![Token::new(TokenKind::Eq, Span::new(0, 1))]);
        test_lexer!("<", vec![Token::new(TokenKind::Lt, Span::new(0, 1))]);
        test_lexer!("<=", vec![Token::new(TokenKind::Le, Span::new(0, 2))]);
        test_lexer!("==", vec![Token::new(TokenKind::EqEq, Span::new(0, 2))]);
        test_lexer!("!=", vec![Token::new(TokenKind::Ne, Span::new(0, 2))]);
        test_lexer!(">=", vec![Token::new(TokenKind::Ge, Span::new(0, 2))]);
        test_lexer!(">", vec![Token::new(TokenKind::Gt, Span::new(0, 1))]);
        test_lexer!(";", vec![Token::new(TokenKind::Semi, Span::new(0, 1))]);
    }

    #[test]
    fn bin_op() {
        test_lexer!(
            "+",
            vec![Token::new(
                TokenKind::BinOp(BinOpToken::Plus),
                Span::new(0, 1)
            )]
        );
        test_lexer!(
            "-",
            vec![Token::new(
                TokenKind::BinOp(BinOpToken::Minus),
                Span::new(0, 1)
            )]
        );
        test_lexer!(
            "*",
            vec![Token::new(
                TokenKind::BinOp(BinOpToken::Star),
                Span::new(0, 1)
            )]
        );
        test_lexer!(
            "/",
            vec![Token::new(
                TokenKind::BinOp(BinOpToken::Slash),
                Span::new(0, 1)
            )]
        );
    }

    #[test]
    fn keyword() {
        test_lexer!(
            "let",
            vec![Token::new(
                TokenKind::Ident(Kw::Let.as_symbol()),
                Span::new(0, 3)
            )]
        );
        test_lexer!(
            "if",
            vec![Token::new(
                TokenKind::Ident(Kw::If.as_symbol()),
                Span::new(0, 2)
            )]
        );
        test_lexer!(
            "else",
            vec![Token::new(
                TokenKind::Ident(Kw::Else.as_symbol()),
                Span::new(0, 4)
            )]
        );
        test_lexer!(
            "true",
            vec![Token::new(
                TokenKind::Ident(Kw::True.as_symbol()),
                Span::new(0, 4)
            )]
        );
        test_lexer!(
            "false",
            vec![Token::new(
                TokenKind::Ident(Kw::False.as_symbol()),
                Span::new(0, 5)
            )]
        );
        test_lexer!(
            "println",
            vec![Token::new(
                TokenKind::Ident(Kw::Println.as_symbol()),
                Span::new(0, 7)
            )]
        );
    }

    #[test]
    fn literal() {
        test_lexer!(
            "0",
            vec![Token::new(
                TokenKind::Literal(Lit {
                    kind: LitKind::Integer,
                    symbol: Symbol::new(KEYWORDS.len())
                }),
                Span::new(0, 1)
            )]
        );

        test_lexer!(
            "12",
            vec![Token::new(
                TokenKind::Literal(Lit {
                    kind: LitKind::Integer,
                    symbol: Symbol::new(KEYWORDS.len())
                }),
                Span::new(0, 2)
            )]
        );
    }

    #[test]
    fn expr() {
        test_lexer!(
            "1 + 2",
            vec![
                Token::new(
                    TokenKind::Literal(Lit {
                        kind: LitKind::Integer,
                        symbol: Symbol::new(KEYWORDS.len())
                    }),
                    Span::new(0, 1)
                ),
                Token::new(TokenKind::BinOp(BinOpToken::Plus), Span::new(2, 3)),
                Token::new(
                    TokenKind::Literal(Lit {
                        kind: LitKind::Integer,
                        symbol: Symbol::new(KEYWORDS.len() + 1)
                    }),
                    Span::new(4, 5)
                )
            ]
        );
    }
}
