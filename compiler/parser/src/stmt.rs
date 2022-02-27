use crate::Parser;

use ast::stmt::*;
use ast::token::{KwKind, Token};

impl Parser {
    pub fn parse_stmt(&mut self) -> Stmt {
        // Try parse local statement.
        if let Some(local) = self.parse_stmt_local() {
            return local;
        }

        let expr = self.parse_expr();

        if self.consume(&Token::Semi) {
            return Stmt::Semi(expr);
        }

        Stmt::Expr(expr)
    }

    fn parse_stmt_local(&mut self) -> Option<Stmt> {
        if !self.consume(&Token::Keyword(KwKind::Let)) {
            return None;
        }

        let ident = self.expect_ident();

        let ty = if self.consume(&Token::Colon) {
            Some(self.expect_ident())
        } else {
            None
        };

        self.expect(&Token::Eq);
        let init = self.parse_expr();
        self.expect(&Token::Semi);

        let local = Stmt::Local {
            ident: ident,
            ty: ty,
            init: init,
        };

        Some(local)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::builder::{expr::*, stmt::*};
    use lexer::run_lexer;

    macro_rules! test_stmt {
        ($input: expr, $expected: expr) => {
            let tokens = run_lexer($input);
            let result = Parser::new(tokens).parse_stmt();

            assert_eq!(result, $expected);
        };
    }

    #[test]
    fn parse_local() {
        test_stmt!("let a = 1;", stmt_local("a", "", expr_lit_int("1")));
        test_stmt!(
            "let a = 1 + 2;",
            stmt_local(
                "a",
                "",
                expr_binary(expr_lit_int("1"), ast::op::BinOp::Add, expr_lit_int("2"))
            )
        );

        test_stmt!("let a:i32 = 1;", stmt_local("a", "i32", expr_lit_int("1")));
        test_stmt!(
            "let a:i32 = 1 + 2;",
            stmt_local(
                "a",
                "i32",
                expr_binary(expr_lit_int("1"), ast::op::BinOp::Add, expr_lit_int("2"))
            )
        );
    }

    #[test]
    fn parse_expr() {
        test_stmt!(
            "1 + 2",
            stmt_expr(expr_binary(
                expr_lit_int("1"),
                ast::op::BinOp::Add,
                expr_lit_int("2")
            ))
        );
    }

    #[test]
    fn parse_semi() {
        test_stmt!(
            "1 + 2;",
            stmt_semi(expr_binary(
                expr_lit_int("1"),
                ast::op::BinOp::Add,
                expr_lit_int("2")
            ))
        );
    }
}
