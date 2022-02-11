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

    #[allow(dead_code)]
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
            let tokens = lex_str($input);
            let result = Parser::new(tokens).parse_lit();

            assert_eq!(result, $expected);
        };
    }

    #[test]
    fn test_parse_lit() {
        test_lit!("10", lit_int("10"));
    }

    #[test]
    fn test_parse_mul() {
        {
            let tokens = vec![Token::Num("1".into()), Token::Star, Token::Num("2".into())];
            let mut parser = Parser::new(tokens);

            assert_eq!(
                parser.parse_expr_mul(),
                expr_binary(expr_lit_int("1"), BinOp::Mul, expr_lit_int("2"))
            );
        }

        {
            let tokens = vec![Token::Num("1".into()), Token::Slash, Token::Num("2".into())];
            let mut parser = Parser::new(tokens);

            assert_eq!(
                parser.parse_expr_mul(),
                expr_binary(expr_lit_int("1"), BinOp::Div, expr_lit_int("2"))
            );
        }
    }

    #[test]
    fn test_parse_unary() {
        {
            let tokens = vec![Token::Minus, Token::Num("1".into())];
            let mut parser = Parser::new(tokens);

            assert_eq!(
                parser.parse_expr_unary(),
                expr_unary(UnOp::Neg, expr_lit_int("1")),
            );
        }

        {
            let tokens = vec![Token::Num("1".into())];
            let mut parser = Parser::new(tokens);

            assert_eq!(parser.parse_expr_unary(), expr_lit_int("1"),);
        }
    }
}
