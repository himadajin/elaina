use crate::Parser;

use ast::{
    expr::*,
    lit::{self, Lit, LitInt},
    op::*,
    token::Token,
};

impl Parser {
    pub fn parse_lit(&mut self) -> lit::Lit {
        let digits = self.expect_num();

        Lit::Int(LitInt { digits: digits })
    }

    pub fn parse_expr(&mut self) -> Expr {
        self.parse_expr_add()
    }

    fn parse_expr_add(&mut self) -> Expr {
        let lhs = self.parse_expr_mul();

        if self.consume(&Token::Plus) {
            let rhs = self.parse_expr();
            let res = Expr::Binary(ExprBinary {
                left: Box::new(lhs),
                op: BinOp::Add,
                right: Box::new(rhs),
            });

            return res;
        }

        if self.consume(&Token::Minus) {
            let rhs = self.parse_expr();
            let res = Expr::Binary(ExprBinary {
                left: Box::new(lhs),
                op: BinOp::Sub,
                right: Box::new(rhs),
            });

            return res;
        }

        lhs
    }

    fn parse_expr_mul(&mut self) -> Expr {
        let lhs = self.parse_expr_unary();

        if self.consume(&Token::Star) {
            let rhs = self.parse_expr_mul();
            let res = Expr::Binary(ExprBinary {
                left: Box::new(lhs),
                op: BinOp::Mul,
                right: Box::new(rhs),
            });

            return res;
        }

        if self.consume(&Token::Slash) {
            let rhs = self.parse_expr_mul();
            let res = Expr::Binary(ExprBinary {
                left: Box::new(lhs),
                op: BinOp::Div,
                right: Box::new(rhs),
            });

            return res;
        }

        lhs
    }

    fn parse_expr_unary(&mut self) -> Expr {
        if self.consume(&Token::Minus) {
            let expr = self.parse_expr_primary();
            let res = Expr::Unary(ExprUnary {
                op: UnOp::Neg,
                expr: Box::new(expr),
            });

            return res;
        }

        self.parse_expr_primary()
    }

    fn parse_expr_primary(&mut self) -> Expr {
        if self.consume(&Token::OpenParen) {
            let expr = self.parse_expr();
            self.expect(&Token::CloseParen);

            return expr;
        }

        let lit = self.parse_lit();

        Expr::Lit(ExprLit { lit: lit })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::builder::{expr::*, lit::*};
    use lexer::Lexer;

    fn lex_all(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);

        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }

        return tokens;
    }

    macro_rules! test_lit {
        ($input: expr, $expected: expr) => {
            let tokens = lex_all($input);
            let result = Parser::new(tokens).parse_lit();

            assert_eq!(result, $expected);
        };
    }

    macro_rules! test_expr {
        ($input: expr, $expected: expr) => {
            let tokens = lex_all($input);
            let result = Parser::new(tokens).parse_expr();

            assert_eq!(result, $expected);
        };
    }

    #[test]
    fn test_parse_lit() {
        test_lit!("10", lit_int("10"));
    }

    #[test]
    fn test_parse_expr() {
        test_expr!(
            "1 * 2 + 3",
            expr_binary(
                expr_binary(expr_lit_int("1"), BinOp::Mul, expr_lit_int("2")),
                BinOp::Add,
                expr_lit_int("3"),
            )
        );

        test_expr!(
            "1 + 2 * 3",
            expr_binary(
                expr_lit_int("1"),
                BinOp::Add,
                expr_binary(expr_lit_int("2"), BinOp::Mul, expr_lit_int("3")),
            )
        );

        test_expr!(
            "1 * (2 + 3)",
            expr_binary(
                expr_lit_int("1"),
                BinOp::Mul,
                expr_binary(expr_lit_int("2"), BinOp::Add, expr_lit_int("3")),
            )
        );
    }

    #[test]
    fn test_parse_add() {
        test_expr!(
            "1 + 2",
            expr_binary(expr_lit_int("1"), BinOp::Add, expr_lit_int("2"))
        );

        test_expr!(
            "1 - 2",
            expr_binary(expr_lit_int("1"), BinOp::Sub, expr_lit_int("2"))
        );

        test_expr!(
            "1 + 2 - 3",
            expr_binary(
                expr_lit_int("1"),
                BinOp::Add,
                expr_binary(expr_lit_int("2"), BinOp::Sub, expr_lit_int("3"))
            )
        );

        test_expr!(
            "-1 - 2",
            expr_binary(
                expr_unary(UnOp::Neg, expr_lit_int("1")),
                BinOp::Sub,
                expr_lit_int("2"),
            )
        );
    }

    #[test]
    fn test_parse_mul() {
        test_expr!(
            "1 * 2",
            expr_binary(expr_lit_int("1"), BinOp::Mul, expr_lit_int("2"))
        );

        test_expr!(
            "1 / 2",
            expr_binary(expr_lit_int("1"), BinOp::Div, expr_lit_int("2"))
        );

        test_expr!(
            "1 * 2 / 3",
            expr_binary(
                expr_lit_int("1"),
                BinOp::Mul,
                expr_binary(expr_lit_int("2"), BinOp::Div, expr_lit_int("3"))
            )
        );

        test_expr!(
            "-1 * 2",
            expr_binary(
                expr_unary(UnOp::Neg, expr_lit_int("1")),
                BinOp::Mul,
                expr_lit_int("2")
            )
        );
    }

    #[test]
    fn test_parse_unary() {
        test_expr!("-1", expr_unary(UnOp::Neg, expr_lit_int("1")));
        test_expr!("1", expr_lit_int("1"));
    }

    #[test]
    fn test_parse_primary() {
        test_expr!("1", expr_lit_int("1"));
        test_expr!("(1)", expr_lit_int("1"));
        test_expr!(
            "(1 * 2)",
            expr_binary(expr_lit_int("1"), BinOp::Mul, expr_lit_int("2"))
        );
    }
}
