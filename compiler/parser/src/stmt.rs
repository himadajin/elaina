use crate::Parser;

use ast::stmt::*;
use ast::token::*;
use span::symbol::Kw;

impl Parser<'_> {
    pub fn parse_stmt(&mut self) -> Stmt {
        // Try parse let statement.
        if self.consume_keyword(Kw::Let.as_symbol()) {
            return self.parse_let_stmt();
        }

        // Try parse println statement.
        // This is temporary and will be removed in the future.
        if self.consume_keyword(Kw::Println.as_symbol()) {
            return self.parse_println_stmt();
        }

        self.parse_expr_stmt()
    }

    /// Parse let statement
    /// Expect `let` token is already parsed
    fn parse_let_stmt(&mut self) -> Stmt {
        let ident_symbol = self.expect_ident();
        let ident = self.symbol_map.get(ident_symbol).to_string();

        let ty = if self.consume(&TokenKind::Colon) {
            let ty_ident = self.expect_ident();
            let ty_string = self.symbol_map.get(ty_ident).to_string();
            Some(ty_string)
        } else {
            None
        };

        self.expect(&TokenKind::Eq);
        let init = self.parse_expr();
        self.expect(&TokenKind::Semi);

        let local = Stmt::Local {
            ident: ident,
            ty: ty,
            init: init,
        };

        local
    }

    /// This function is temporary and will be removed in the future.
    fn parse_println_stmt(&mut self) -> Stmt {
        self.expect(&TokenKind::OpenDelim(DelimToken::Paren));
        let expr = self.parse_expr();
        self.expect(&TokenKind::CloseDelim(DelimToken::Paren));
        self.expect(&TokenKind::Semi);

        let stmt = Stmt::Println(expr);

        stmt
    }

    fn parse_expr_stmt(&mut self) -> Stmt {
        if let Some(expr) = self.parse_expr_with_block() {
            return Stmt::Expr(expr);
        }

        let expr = self.parse_expr();
        if self.consume(&TokenKind::Semi) {
            return Stmt::Semi(expr);
        }

        Stmt::Expr(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::parse_all_token;
    use ast::builder::{expr::*, stmt::*};

    macro_rules! test_stmt {
        ($input: expr, $expected: expr) => {
            let tokens = parse_all_token($input);
            let result = Parser::new(tokens).parse_stmt();

            assert_eq!(result, $expected);
        };
    }

    #[test]
    fn parse_local() {
        test_stmt!("let a = 1;", stmt_local("a", "", expr_lit_int(1)));
        test_stmt!(
            "let a = 1 + 2;",
            stmt_local(
                "a",
                "",
                expr_binary(expr_lit_int(1), ast::op::BinOp::Add, expr_lit_int(2))
            )
        );

        test_stmt!("let a:i32 = 1;", stmt_local("a", "i32", expr_lit_int(1)));
        test_stmt!(
            "let a:i32 = 1 + 2;",
            stmt_local(
                "a",
                "i32",
                expr_binary(expr_lit_int(1), ast::op::BinOp::Add, expr_lit_int(2))
            )
        );

        test_stmt!(
            "let a:bool = true;",
            stmt_local("a", "bool", expr_lit_bool(true))
        );
    }

    #[test]
    fn parse_expr() {
        test_stmt!(
            "1 + 2",
            stmt_expr(expr_binary(
                expr_lit_int(1),
                ast::op::BinOp::Add,
                expr_lit_int(2)
            ))
        );
    }

    #[test]
    fn parse_semi() {
        test_stmt!(
            "1 + 2;",
            stmt_semi(expr_binary(
                expr_lit_int(1),
                ast::op::BinOp::Add,
                expr_lit_int(2)
            ))
        );
    }
}
