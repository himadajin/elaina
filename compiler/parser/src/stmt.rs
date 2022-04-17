use crate::Parser;

use ast::stmt::*;
use ast::token::*;
use span::symbol::Kw;

use anyhow::Result;

impl Parser<'_> {
    pub fn parse_stmt(&mut self) -> Result<Stmt> {
        // Try parse let statement.
        if self.consume_keyword(Kw::Let) {
            return Ok(self.parse_let_stmt()?);
        }

        // Try parse println statement.
        // This is temporary and will be removed in the future.
        if self.consume_keyword(Kw::Println) {
            return Ok(self.parse_println_stmt()?);
        }

        self.parse_expr_stmt()
    }

    /// Parse let statement
    /// Expect `let` token is already parsed
    fn parse_let_stmt(&mut self) -> Result<Stmt> {
        let ident = self.expect_ident()?;

        let ty = if self.consume(&TokenKind::Colon) {
            Some(self.parse_ty()?)
        } else {
            None
        };

        self.expect(&TokenKind::Eq)?;
        let init = self.parse_expr()?;
        self.expect(&TokenKind::Semi)?;

        let local = Stmt::Local {
            ident: ident,
            ty: ty,
            init: init,
        };

        Ok(local)
    }

    /// This function is temporary and will be removed in the future.
    fn parse_println_stmt(&mut self) -> Result<Stmt> {
        self.expect(&TokenKind::OpenDelim(DelimToken::Paren))?;
        let expr = self.parse_expr()?;
        self.expect(&TokenKind::CloseDelim(DelimToken::Paren))?;
        self.expect(&TokenKind::Semi)?;

        Ok(Stmt::Println(expr))
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt> {
        if let Some(expr) = self.parse_expr_with_block()? {
            return Ok(Stmt::Expr(expr));
        }

        let expr = self.parse_expr()?;
        if self.consume(&TokenKind::Semi) {
            return Ok(Stmt::Semi(expr));
        }

        Ok(Stmt::Expr(expr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::parse_all_token;
    use ast::{expr::*, op::*, ty::*};
    use span::symbol::{Ident, Kw, Symbol};

    macro_rules! test_stmt {
        ($input: expr, $expected: expr) => {
            let tokens = parse_all_token($input);
            let result = Parser::new(&tokens).parse_stmt().unwrap();

            assert_eq!(result, $expected);
        };
    }

    #[test]
    fn parse_local() {
        test_stmt!(
            "let a = 1;",
            Stmt::local(
                Ident::with_dummy_span(Symbol::ident_nth(0)),
                None,
                Expr::lit_from_value_dummy(1)
            )
        );
        test_stmt!(
            "let a = 1 + 2;",
            Stmt::local(
                Ident::with_dummy_span(Symbol::ident_nth(0)),
                None,
                Expr::binary(
                    BinOp::Add,
                    Expr::lit_from_value_dummy(1),
                    Expr::lit_from_value_dummy(2)
                )
            )
        );

        test_stmt!(
            "let a:i32 = 1;",
            Stmt::local(
                Ident::with_dummy_span(Symbol::ident_nth(0)),
                Some(Ty::path_with_dummy_span(Kw::I32)),
                Expr::lit_from_value_dummy(1)
            )
        );
        test_stmt!(
            "let a:i32 = 1 + 2;",
            Stmt::local(
                Ident::with_dummy_span(Symbol::ident_nth(0)),
                Some(Ty::path_with_dummy_span(Kw::I32)),
                Expr::binary(
                    BinOp::Add,
                    Expr::lit_from_value_dummy(1),
                    Expr::lit_from_value_dummy(2)
                )
            )
        );

        test_stmt!(
            "let a:bool = true;",
            Stmt::local(
                Ident::with_dummy_span(Symbol::ident_nth(0)),
                Some(Ty::path_with_dummy_span(Kw::Bool)),
                Expr::lit_from_value_dummy(true)
            )
        );
    }

    #[test]
    fn parse_expr() {
        test_stmt!(
            "1 + 2",
            Stmt::expr(Expr::binary(
                BinOp::Add,
                Expr::lit_from_value_dummy(1),
                Expr::lit_from_value_dummy(2)
            ))
        );
    }

    #[test]
    fn parse_semi() {
        test_stmt!(
            "1 + 2;",
            Stmt::semi(Expr::binary(
                BinOp::Add,
                Expr::lit_from_value_dummy(1),
                Expr::lit_from_value_dummy(2)
            ))
        );
    }
}
