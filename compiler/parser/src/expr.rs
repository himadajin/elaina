use crate::Parser;

use ast::{
    expr::*,
    lit::{self, Lit},
    op::*,
    token::{KwKind, Token},
};

impl Parser {
    pub fn parse_lit(&mut self) -> lit::Lit {
        // Try to parse true literal
        if matches!(self.token, Token::Keyword(KwKind::True)) {
            self.bump();
            return Lit::Bool { value: true };
        }

        // Try to parse false literal
        if matches!(self.token, Token::Keyword(KwKind::False)) {
            self.bump();
            return Lit::Bool { value: false };
        }

        // Parse integer literal
        let digits = self.expect_int();

        Lit::Int { digits: digits }
    }

    pub fn parse_expr(&mut self) -> Expr {
        // Try to parse block expression
        if matches!(self.token, Token::OpenBrace) {
            return self.parse_expr_block();
        }

        self.parse_expr_equality()
    }

    fn parse_expr_block(&mut self) -> Expr {
        let block = self.parse_block();
        Expr::Block {
            block: Box::new(block),
        }
    }

    fn parse_expr_equality(&mut self) -> Expr {
        let lhs = self.parse_expr_relational();

        if self.consume(&Token::EqEq) {
            let rhs = self.parse_expr_relational();
            let binary = Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Eq,
                rhs: Box::new(rhs),
            };

            return binary;
        }

        if self.consume(&Token::Ne) {
            let rhs = self.parse_expr_relational();
            let binary = Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Ne,
                rhs: Box::new(rhs),
            };

            return binary;
        }

        lhs
    }

    fn parse_expr_relational(&mut self) -> Expr {
        let lhs = self.parse_expr_add();

        if self.consume(&Token::Lt) {
            let rhs = self.parse_expr_add();
            let binary = Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Lt,
                rhs: Box::new(rhs),
            };

            return binary;
        }

        if self.consume(&Token::Le) {
            let rhs = self.parse_expr_add();
            let binary = Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Le,
                rhs: Box::new(rhs),
            };

            return binary;
        }

        if self.consume(&Token::Ge) {
            let rhs = self.parse_expr_add();
            let binary = Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Ge,
                rhs: Box::new(rhs),
            };

            return binary;
        }

        if self.consume(&Token::Gt) {
            let rhs = self.parse_expr_add();
            let binary = Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Gt,
                rhs: Box::new(rhs),
            };

            return binary;
        }

        lhs
    }

    fn parse_expr_add(&mut self) -> Expr {
        let lhs = self.parse_expr_mul();

        if self.consume(&Token::Plus) {
            let rhs = self.parse_expr_add();
            let res = Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Add,
                rhs: Box::new(rhs),
            };

            return res;
        }

        if self.consume(&Token::Minus) {
            let rhs = self.parse_expr_add();
            let res = Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Sub,
                rhs: Box::new(rhs),
            };

            return res;
        }

        lhs
    }

    fn parse_expr_mul(&mut self) -> Expr {
        let lhs = self.parse_expr_unary();

        if self.consume(&Token::Star) {
            let rhs = self.parse_expr_mul();
            let res = Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Mul,
                rhs: Box::new(rhs),
            };

            return res;
        }

        if self.consume(&Token::Slash) {
            let rhs = self.parse_expr_mul();
            let res = Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Div,
                rhs: Box::new(rhs),
            };

            return res;
        }

        lhs
    }

    fn parse_expr_unary(&mut self) -> Expr {
        if self.consume(&Token::Minus) {
            let expr = self.parse_expr_primary();
            let res = Expr::Unary {
                op: UnOp::Neg,
                expr: Box::new(expr),
            };

            return res;
        }

        self.parse_expr_primary()
    }

    fn parse_expr_primary(&mut self) -> Expr {
        // Try to parse parensized expression
        if self.consume(&Token::OpenParen) {
            let expr = self.parse_expr();
            self.expect(&Token::CloseParen);

            return expr;
        }

        // Try to parse identifier
        if matches!(self.token, Token::Ident(_)) {
            let ident = self.expect_ident();
            return Expr::Ident { ident: ident };
        }

        let lit = self.parse_lit();

        Expr::Lit { lit: lit }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::builder::{expr::*, lit::*, stmt::*};
    use lexer::run_lexer;

    macro_rules! test_lit {
        ($input: expr, $expected: expr) => {
            let tokens = run_lexer($input);
            let result = Parser::new(tokens).parse_lit();

            assert_eq!(result, $expected);
        };
    }

    macro_rules! test_expr {
        ($input: expr, $expected: expr) => {
            let tokens = run_lexer($input);
            let result = Parser::new(tokens).parse_expr();

            assert_eq!(result, $expected);
        };
    }

    #[test]
    fn test_parse_lit() {
        test_lit!("10", lit_int("10"));
        test_lit!("true", lit_bool(true));
        test_lit!("false", lit_bool(false));
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
    fn test_parse_block() {
        test_expr!(
            "{0; 1}",
            expr_block([stmt_semi(expr_lit_int("0")), stmt_expr(expr_lit_int("1"))])
        );
    }

    #[test]
    fn test_parse_relational() {
        test_expr!(
            "1 == 2",
            expr_binary(expr_lit_int("1"), BinOp::Eq, expr_lit_int("2"))
        );

        test_expr!(
            "1 < 2",
            expr_binary(expr_lit_int("1"), BinOp::Lt, expr_lit_int("2"))
        );

        test_expr!(
            "1 <= 2",
            expr_binary(expr_lit_int("1"), BinOp::Le, expr_lit_int("2"))
        );

        test_expr!(
            "1 != 2",
            expr_binary(expr_lit_int("1"), BinOp::Ne, expr_lit_int("2"))
        );

        test_expr!(
            "1 >= 2",
            expr_binary(expr_lit_int("1"), BinOp::Ge, expr_lit_int("2"))
        );

        test_expr!(
            "1 > 2",
            expr_binary(expr_lit_int("1"), BinOp::Gt, expr_lit_int("2"))
        );

        test_expr!(
            "1 + 2 == 3 + 4",
            expr_binary(
                expr_binary(expr_lit_int("1"), BinOp::Add, expr_lit_int("2")),
                BinOp::Eq,
                expr_binary(expr_lit_int("3"), BinOp::Add, expr_lit_int("4"))
            )
        );

        test_expr!(
            "1 < 2 == 3 < 4",
            expr_binary(
                expr_binary(expr_lit_int("1"), BinOp::Lt, expr_lit_int("2")),
                BinOp::Eq,
                expr_binary(expr_lit_int("3"), BinOp::Lt, expr_lit_int("4"))
            )
        );

        test_expr!(
            "1 + 2 < 3 + 4 == 5 + 6 < 7 + 8",
            expr_binary(
                expr_binary(
                    expr_binary(expr_lit_int("1"), BinOp::Add, expr_lit_int("2")),
                    BinOp::Lt,
                    expr_binary(expr_lit_int("3"), BinOp::Add, expr_lit_int("4"))
                ),
                BinOp::Eq,
                expr_binary(
                    expr_binary(expr_lit_int("5"), BinOp::Add, expr_lit_int("6")),
                    BinOp::Lt,
                    expr_binary(expr_lit_int("7"), BinOp::Add, expr_lit_int("8"))
                )
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

    #[test]
    fn ident() {
        test_expr!("a", expr_ident("a"));
        test_expr!(
            "a + 1",
            expr_binary(expr_ident("a"), BinOp::Add, expr_lit_int("1"))
        );
    }
}
