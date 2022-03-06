use ast::token::*;
use lexer::{first_token, token};
use span::{span::Span, symbol::*};

pub fn parse_all_token(src: &str) -> impl Iterator<Item = Token> + '_ {
    let mut lexer = Lexer::new(src);
    std::iter::from_fn(move || {
        let token = lexer.next_token();
        match token.kind {
            TokenKind::Eof => None,
            _ => Some(token),
        }
    })
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
            self.pos = token.len;

            match self.cook_lexer_token(token.kind, start) {
                Some(kind) => {
                    let span = Span::new(start as u32, self.pos as u32);
                    return ast::token::Token::new(kind, span);
                }
                None => (),
            }
        }
    }

    fn cook_lexer_token(&mut self, token: token::TokenKind, start: usize) -> Option<TokenKind> {
        Some(match token {
            token::TokenKind::Whitespace => return None,
            token::TokenKind::Ident => {
                let ident = self.str_from(start);
                let symbol = self.symbol_map.insert(ident);

                TokenKind::Ident(symbol)
            }
            token::TokenKind::Literal { .. } => todo!(),
            token::TokenKind::Semi => todo!(),
            token::TokenKind::OpenParen => TokenKind::OpenDelim(DelimToken::Paren),
            token::TokenKind::CloseParen => TokenKind::CloseDelim(DelimToken::Paren),
            token::TokenKind::OpenBrace => TokenKind::OpenDelim(DelimToken::Brace),
            token::TokenKind::CloseBrace => TokenKind::CloseDelim(DelimToken::Brace),
            token::TokenKind::Eq => todo!(),
            token::TokenKind::Bang => todo!(),
            token::TokenKind::Lt => todo!(),
            token::TokenKind::Gt => todo!(),
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
            let tokens = parse_all_token($input);

            for (result, expeced) in tokens.zip($expected) {
                assert_eq!(result.kind, expeced);
            }
        };
    }

    #[test]
    fn paren() {
        test_lexer!("(", vec![TokenKind::OpenDelim(DelimToken::Paren)]);
        test_lexer!(")", vec![TokenKind::CloseDelim(DelimToken::Paren)]);
        test_lexer!("{", vec![TokenKind::OpenDelim(DelimToken::Brace)]);
        test_lexer!("}", vec![TokenKind::CloseDelim(DelimToken::Brace)]);
    }

    #[test]
    fn bin_op() {
        test_lexer!("+", vec![TokenKind::BinOp(BinOpToken::Plus)]);
        test_lexer!("-", vec![TokenKind::BinOp(BinOpToken::Minus)]);
        test_lexer!("*", vec![TokenKind::BinOp(BinOpToken::Star)]);
        test_lexer!("/", vec![TokenKind::BinOp(BinOpToken::Slash)]);
    }

    #[test]
    fn keyword() {
        test_lexer!("let", vec![TokenKind::Ident(Kw::Let.as_symbol())]);
        test_lexer!("if", vec![TokenKind::Ident(Kw::If.as_symbol())]);
        test_lexer!("else", vec![TokenKind::Ident(Kw::Else.as_symbol())]);
        test_lexer!("true", vec![TokenKind::Ident(Kw::True.as_symbol())]);
        test_lexer!("false", vec![TokenKind::Ident(Kw::False.as_symbol())]);
        test_lexer!("println", vec![TokenKind::Ident(Kw::Println.as_symbol())]);
    }
}
