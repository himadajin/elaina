use crate::Parser;

use ast::stmt::*;
use ast::token::Token;

impl Parser {
    pub fn parse_stmt(&mut self) -> Stmt {
        let expr = self.parse_expr();

        if self.consume(&Token::Semi) {
            return Stmt::Semi(expr);
        }

        Stmt::Expr(expr)
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
