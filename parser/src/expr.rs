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
    fn parse_expr_unary(&mut self) -> Expr {
        match &self.token {
            Token::Minus => (),
            t => panic!("Expected unary operator but got {:?}", t),
        }

        self.bump();

        let expr = self.parse_expr_primary();

        Expr::Unary(ExprUnary {
            op: UnOp::Neg,
            expr: Box::new(expr),
        })
    }

    fn parse_expr_primary(&mut self) -> Expr {
        let lit = self.parse_lit();

        Expr::Lit(ExprLit { lit: lit })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{
        lit::{Lit, LitInt},
        token::*,
    };

    #[test]
    fn test_parse_lit() {
        let tokens = vec![Token::Num("10".into())];
        let mut parser = Parser::new(tokens);

        assert_eq!(
            parser.parse_lit(),
            Lit::Int(LitInt {
                digits: "10".into()
            })
        );
    }

    #[test]
    fn test_parse_unary() {
        let tokens = vec![Token::Minus, Token::Num("1".into())];
        let mut parser = Parser::new(tokens);

        assert_eq!(
            parser.parse_expr_unary(),
            Expr::Unary(ExprUnary {
                op: UnOp::Neg,
                expr: Box::new(Expr::Lit(ExprLit {
                    lit: Lit::Int(LitInt { digits: "1".into() })
                }))
            })
        );
    }
}
