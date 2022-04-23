use crate::{error::ParseError, Parser};

use ast::{
    token::{DelimToken, TokenKind},
    *,
};
use span::*;

use anyhow::{Context, Result};

impl Parser<'_> {
    pub fn parse_items(&mut self) -> Result<Vec<Item>> {
        let mut items = Vec::new();

        while !(self.token.kind == TokenKind::Eof) {
            let item = self.parse_item()?;
            items.push(item);
        }

        Ok(items)
    }

    pub fn parse_item(&mut self) -> Result<Item> {
        if self.consume_keyword(Kw::Fn) {
            let ident = self
                .expect_ident()
                .with_context(|| format!("Parsing name of function"))?;
            let kind = self
                .parse_fn()
                .with_context(|| format!("Parsing function"))?;

            return Ok(Item { ident, kind });
        }

        Err(ParseError::UnexpectedToken {
            expected: vec![TokenKind::Ident(Kw::Fn.into())],
            found: self.token.kind.clone(),
        }
        .into())
    }

    fn parse_fn(&mut self) -> Result<ItemKind> {
        self.expect(&TokenKind::OpenDelim(DelimToken::Paren))?;

        let mut inputs = Vec::new();
        while !self.consume(&TokenKind::CloseDelim(DelimToken::Paren)) {
            let param = self.parse_param()?;
            inputs.push(param);

            if self.consume(&TokenKind::CloseDelim(DelimToken::Paren)) {
                break;
            }

            self.expect(&TokenKind::Comma)?;
        }

        let output = if self.consume(&TokenKind::Arrow) {
            Some(self.parse_ty()?)
        } else {
            None
        };

        let body = self.parse_block()?;
        Ok(ItemKind::Fn(Box::new(Fn {
            inputs,
            output,
            body,
        })))
    }

    fn parse_param(&mut self) -> Result<Param> {
        let ident = self
            .expect_ident()
            .with_context(|| format!("Parsing parameter."))?;
        self.expect(&TokenKind::Colon)?;
        let ty = self.parse_ty()?;

        Ok(Param { ident, ty })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::parse_all_token;

    macro_rules! test_item {
        ($input: expr, $expected: expr) => {
            let tokens = parse_all_token($input);
            let result = Parser::new(&tokens).parse_item().unwrap();

            assert_eq!(result, $expected);
        };
    }

    macro_rules! test_items {
        ($input: expr, $expected: expr) => {
            let expected: Vec<Item> = $expected.into();
            let tokens = parse_all_token($input);
            let result = Parser::new(&tokens).parse_items().unwrap();

            assert_eq!(result, expected);
        };
    }

    #[test]
    fn fn_decl() {
        test_item!(
            "fn f() {}",
            Item::fn_dummy(Symbol::ident_nth(0), [], None, [])
        );
        test_item!(
            "fn f(a:i32) {}",
            Item::fn_dummy(
                Symbol::ident_nth(0),
                [Param::new_dummy(
                    Ty::path_with_dummy_span(Kw::I32),
                    Symbol::ident_nth(1)
                )],
                None,
                []
            )
        );
        test_item!(
            "fn f(a:i32,) {}",
            Item::fn_dummy(
                Symbol::ident_nth(0),
                [Param::new_dummy(
                    Ty::path_with_dummy_span(Kw::I32),
                    Symbol::ident_nth(1)
                )],
                None,
                []
            )
        );
        test_item!(
            "fn f(a:i32, b: bool) {}",
            Item::fn_dummy(
                Symbol::ident_nth(0),
                [
                    Param::new_dummy(Ty::path_with_dummy_span(Kw::I32), Symbol::ident_nth(1)),
                    Param::new_dummy(Ty::path_with_dummy_span(Kw::Bool), Symbol::ident_nth(2))
                ],
                None,
                []
            )
        );
        test_item!(
            "fn f(a:i32, b: bool,) {}",
            Item::fn_dummy(
                Symbol::ident_nth(0),
                [
                    Param::new_dummy(Ty::path_with_dummy_span(Kw::I32), Symbol::ident_nth(1)),
                    Param::new_dummy(Ty::path_with_dummy_span(Kw::Bool), Symbol::ident_nth(2))
                ],
                None,
                []
            )
        );

        test_item!(
            "fn f() -> i32 {}",
            Item::fn_dummy(
                Symbol::ident_nth(0),
                [],
                Some(Ty::path_with_dummy_span(Kw::I32)),
                []
            )
        );
        test_item!(
            "fn f(a:i32) -> i32 {}",
            Item::fn_dummy(
                Symbol::ident_nth(0),
                [Param::new_dummy(
                    Ty::path_with_dummy_span(Kw::I32),
                    Symbol::ident_nth(1)
                )],
                Some(Ty::path_with_dummy_span(Kw::I32)),
                []
            )
        );
    }

    #[test]
    fn items() {
        test_items!(
            r"
fn f() {} 
fn g() {}",
            [
                Item::fn_dummy(Symbol::ident_nth(0), [], None, []),
                Item::fn_dummy(Symbol::ident_nth(1), [], None, [])
            ]
        );
    }
}
