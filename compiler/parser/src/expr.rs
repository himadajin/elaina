use crate::{error::ParseError, Parser};

use ast::{
    expr::*,
    lit::*,
    op::*,
    token::{self, BinOpToken, DelimToken, TokenKind},
};
use span::symbol::*;

use anyhow::Result;

impl Parser<'_> {
    pub fn parse_lit_opt(&mut self) -> Option<Lit> {
        if let Some(lit) = self.parse_bool_opt() {
            return Some(lit);
        }

        // Parse integer literal
        if let TokenKind::Literal(lit) = &self.token.kind {
            match lit.kind {
                token::LitKind::Integer => {
                    let digits = self.symbol_map.get(lit.symbol);
                    let value = digits.parse().unwrap();
                    let span = self.token.span;

                    self.bump();
                    return Some(Lit {
                        kind: LitKind::Int(value),
                        span: span,
                    });
                }
            }
        }

        None
    }

    fn parse_bool_opt(&mut self) -> Option<Lit> {
        if let TokenKind::Ident(symbol) = self.token.kind {
            // Try to parse true literal
            if symbol == Kw::True.into() {
                let span = self.token.span;
                self.bump();
                return Some(Lit {
                    kind: LitKind::Bool(true),
                    span: span,
                });
            }

            // Try to parse false literal
            if symbol == Kw::False.into() {
                let span = self.token.span;
                self.bump();
                return Some(Lit {
                    kind: LitKind::Bool(false),
                    span: span,
                });
            }
        }

        None
    }

    fn parse_path(&mut self) -> Result<Path> {
        if let TokenKind::Ident(name) = self.token.kind {
            let span = self.token.span;
            self.bump();
            return Ok(Path {
                ident: Ident { name, span },
            });
        }

        Err(ParseError::NotFoundIdent {
            found: self.token.kind.clone(),
        }
        .into())
    }

    pub fn parse_expr(&mut self) -> Result<Expr> {
        if let Some(expr) = self.parse_expr_with_block()? {
            return Ok(expr);
        }

        self.parse_expr_without_block()
    }

    fn parse_expr_opt(&mut self) -> Result<Option<Expr>> {
        if self.token.can_begin_expr() {
            return Ok(Some(self.parse_expr()?));
        }

        Ok(None)
    }

    pub fn parse_expr_without_block(&mut self) -> Result<Expr> {
        if self.consume_keyword(Kw::Break) {
            let expr = self.parse_expr_opt()?.map(|e| Box::new(e));
            return Ok(Expr::Break { expr });
        }

        if self.consume_keyword(Kw::Continue) {
            let expr = self.parse_expr_opt()?.map(|e| Box::new(e));
            return Ok(Expr::Continue { expr });
        }

        self.parse_operator_expr()
    }

    pub fn parse_expr_with_block(&mut self) -> Result<Option<Expr>> {
        // Try to parse block expression
        if matches!(
            self.token.kind,
            TokenKind::OpenDelim(token::DelimToken::Brace)
        ) {
            return Ok(Some(self.parse_block_expr()?));
        }

        // Try to parse if expression
        if self.consume_keyword(Kw::If) {
            return Ok(Some(self.parse_if_expr()?));
        }

        // Try to parse loop expression
        if self.consume_keyword(Kw::Loop) {
            return Ok(Some(self.parse_loop_expr()?));
        }

        Ok(None)
    }

    fn parse_block_expr(&mut self) -> Result<Expr> {
        let block = self.parse_block()?;
        Ok(Expr::Block {
            block: Box::new(block),
        })
    }

    fn parse_if_expr(&mut self) -> Result<Expr> {
        let cond = self.parse_expr()?;
        let then = self.parse_block()?;

        // Try to parse if-else
        if self.consume_keyword(Kw::Else) {
            // If current token is `{`, block should be parsed.
            if matches!(
                self.token.kind,
                TokenKind::OpenDelim(token::DelimToken::Brace)
            ) {
                let block = self.parse_block()?;
                let expr_block = Expr::Block {
                    block: Box::new(block),
                };

                return Ok(Expr::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    else_opt: Some(Box::new(expr_block)),
                });
            }

            // Otherwise, if expression should be parsed.
            self.expect(&TokenKind::Ident(Kw::If.into()))?;
            let if_expr = self.parse_if_expr()?;
            return Ok(Expr::If {
                cond: Box::new(cond),
                then: Box::new(then),
                else_opt: Some(Box::new(if_expr)),
            });
        }

        Ok(Expr::If {
            cond: Box::new(cond),
            then: Box::new(then),
            else_opt: None,
        })
    }

    fn parse_loop_expr(&mut self) -> Result<Expr> {
        let block = self.parse_block()?;

        Ok(Expr::Loop {
            block: Box::new(block),
        })
    }

    fn parse_operator_expr(&mut self) -> Result<Expr> {
        let lhs = self.parse_expr_equality()?;

        if self.consume(&TokenKind::Eq) {
            let rhs = self.parse_expr()?;
            return Ok(Expr::Assign {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
        }

        Ok(lhs)
    }

    fn parse_expr_equality(&mut self) -> Result<Expr> {
        let lhs = self.parse_expr_relational()?;

        if self.consume(&TokenKind::EqEq) {
            let rhs = self.parse_expr_relational()?;
            return Ok(Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Eq,
                rhs: Box::new(rhs),
            });
        }

        if self.consume(&TokenKind::Ne) {
            let rhs = self.parse_expr_relational()?;
            return Ok(Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Ne,
                rhs: Box::new(rhs),
            });
        }

        Ok(lhs)
    }

    fn parse_expr_relational(&mut self) -> Result<Expr> {
        let lhs = self.parse_expr_add()?;

        if self.consume(&TokenKind::Lt) {
            let rhs = self.parse_expr_add()?;
            return Ok(Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Lt,
                rhs: Box::new(rhs),
            });
        }

        if self.consume(&TokenKind::Le) {
            let rhs = self.parse_expr_add()?;
            return Ok(Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Le,
                rhs: Box::new(rhs),
            });
        }

        if self.consume(&TokenKind::Ge) {
            let rhs = self.parse_expr_add()?;
            return Ok(Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Ge,
                rhs: Box::new(rhs),
            });
        }

        if self.consume(&TokenKind::Gt) {
            let rhs = self.parse_expr_add()?;
            return Ok(Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Gt,
                rhs: Box::new(rhs),
            });
        }

        Ok(lhs)
    }

    fn parse_expr_add(&mut self) -> Result<Expr> {
        let lhs = self.parse_expr_mul()?;

        if self.consume(&TokenKind::BinOp(BinOpToken::Plus)) {
            let rhs = self.parse_expr_add()?;
            return Ok(Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Add,
                rhs: Box::new(rhs),
            });
        }

        if self.consume(&TokenKind::BinOp(BinOpToken::Minus)) {
            let rhs = self.parse_expr_add()?;
            return Ok(Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Sub,
                rhs: Box::new(rhs),
            });
        }

        Ok(lhs)
    }

    fn parse_expr_mul(&mut self) -> Result<Expr> {
        let lhs = self.parse_expr_unary()?;

        if self.consume(&TokenKind::BinOp(BinOpToken::Star)) {
            let rhs = self.parse_expr_mul()?;
            return Ok(Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Mul,
                rhs: Box::new(rhs),
            });
        }

        if self.consume(&TokenKind::BinOp(BinOpToken::Slash)) {
            let rhs = self.parse_expr_mul()?;
            return Ok(Expr::Binary {
                lhs: Box::new(lhs),
                op: BinOp::Div,
                rhs: Box::new(rhs),
            });
        }

        Ok(lhs)
    }

    fn parse_expr_unary(&mut self) -> Result<Expr> {
        if self.consume(&TokenKind::BinOp(BinOpToken::Minus)) {
            let expr = self.parse_expr_primary()?;
            return Ok(Expr::Unary {
                op: UnOp::Neg,
                expr: Box::new(expr),
            });
        }

        self.parse_expr_primary()
    }

    fn parse_expr_primary(&mut self) -> Result<Expr> {
        // Try to parse parensized expression
        if self.consume(&TokenKind::OpenDelim(DelimToken::Paren)) {
            let expr = self.parse_expr()?;
            self.expect(&TokenKind::CloseDelim(DelimToken::Paren))?;

            return Ok(expr);
        }

        // Try to parse literal
        if let Some(lit) = self.parse_lit_opt() {
            return Ok(Expr::Lit { lit: lit });
        }

        // Parse path;
        let path = self.parse_path()?;
        Ok(Expr::Path(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::parse_all_token;
    use ast::builder::{block::*, expr::*, lit::*, stmt::*};
    use span::symbol::Symbol;

    macro_rules! test_lit {
        ($input: expr, $expected: expr) => {
            let tokens = parse_all_token($input);
            let result = Parser::new(&tokens).parse_lit_opt().unwrap();

            assert_eq!(result, $expected);
        };
    }

    macro_rules! test_expr {
        ($input: expr, $expected: expr) => {
            let tokens = parse_all_token($input);
            let result = Parser::new(&tokens).parse_expr().unwrap();

            assert_eq!(result, $expected);
        };
    }

    #[test]
    fn test_parse_lit() {
        test_lit!("10", lit_int(10));
        test_lit!("true", lit_bool(true));
        test_lit!("false", lit_bool(false));
    }

    #[test]
    fn test_parse_expr() {
        test_expr!(
            "1 * 2 + 3",
            expr_binary(
                expr_binary(expr_lit_int(1), BinOp::Mul, expr_lit_int(2)),
                BinOp::Add,
                expr_lit_int(3),
            )
        );

        test_expr!(
            "1 + 2 * 3",
            expr_binary(
                expr_lit_int(1),
                BinOp::Add,
                expr_binary(expr_lit_int(2), BinOp::Mul, expr_lit_int(3)),
            )
        );

        test_expr!(
            "1 * (2 + 3)",
            expr_binary(
                expr_lit_int(1),
                BinOp::Mul,
                expr_binary(expr_lit_int(2), BinOp::Add, expr_lit_int(3)),
            )
        );
    }

    #[test]
    fn test_parse_expr_if() {
        test_expr!(
            "if 1 + 2 == 3 { 0 }",
            expr_if(
                expr_binary(
                    expr_binary(expr_lit_int(1), BinOp::Add, expr_lit_int(2)),
                    BinOp::Eq,
                    expr_lit_int(3)
                ),
                block([stmt_expr(expr_lit_int(0))]),
                None
            )
        );

        test_expr!(
            "if true { 0 }",
            expr_if(
                expr_lit_bool(true),
                block([stmt_expr(expr_lit_int(0))]),
                None
            )
        );

        test_expr!(
            "if true { 0 } else { 1 }",
            expr_if(
                expr_lit_bool(true),
                block([stmt_expr(expr_lit_int(0))]),
                Some(expr_block([stmt_expr(expr_lit_int(1))]))
            )
        );

        test_expr!(
            "if true { 0 } else if true { 1 }",
            expr_if(
                expr_lit_bool(true),
                block([stmt_expr(expr_lit_int(0))]),
                Some(expr_if(
                    expr_lit_bool(true),
                    block([stmt_expr(expr_lit_int(1))]),
                    None
                ))
            )
        );

        test_expr!(
            "if true { 0 } else if true { 1 } else { 2 }",
            expr_if(
                expr_lit_bool(true),
                block([stmt_expr(expr_lit_int(0))]),
                Some(expr_if(
                    expr_lit_bool(true),
                    block([stmt_expr(expr_lit_int(1))]),
                    Some(expr_block([stmt_expr(expr_lit_int(2))]))
                ))
            )
        );
    }

    #[test]
    fn test_parse_expr_loop() {
        test_expr!("loop { 0 }", expr_loop(block([stmt_expr(expr_lit_int(0))])));
    }

    #[test]
    fn test_parse_expr_break() {
        test_expr!("break", expr_break(None));
        test_expr!("break 0;", expr_break(Some(expr_lit_int(0))));
    }

    #[test]
    fn test_parse_expr_continue() {
        test_expr!("continue", expr_continue(None));
        test_expr!("continue 0;", expr_continue(Some(expr_lit_int(0))));
    }

    #[test]
    fn test_parse_block() {
        test_expr!(
            "{0; 1}",
            expr_block([stmt_semi(expr_lit_int(0)), stmt_expr(expr_lit_int(1))])
        );
    }

    #[test]
    fn test_parse_expr_assign() {
        test_expr!(
            "a = 0",
            expr_assign(expr_path(Symbol::ident_nth(0)), expr_lit_int(0))
        );

        test_expr!(
            "a = 1 + 1",
            expr_assign(
                expr_path(Symbol::ident_nth(0)),
                expr_binary(expr_lit_int(1), BinOp::Add, expr_lit_int(1))
            )
        );
    }

    #[test]
    fn test_parse_relational() {
        test_expr!(
            "1 == 2",
            expr_binary(expr_lit_int(1), BinOp::Eq, expr_lit_int(2))
        );

        test_expr!(
            "1 < 2",
            expr_binary(expr_lit_int(1), BinOp::Lt, expr_lit_int(2))
        );

        test_expr!(
            "1 <= 2",
            expr_binary(expr_lit_int(1), BinOp::Le, expr_lit_int(2))
        );

        test_expr!(
            "1 != 2",
            expr_binary(expr_lit_int(1), BinOp::Ne, expr_lit_int(2))
        );

        test_expr!(
            "1 >= 2",
            expr_binary(expr_lit_int(1), BinOp::Ge, expr_lit_int(2))
        );

        test_expr!(
            "1 > 2",
            expr_binary(expr_lit_int(1), BinOp::Gt, expr_lit_int(2))
        );

        test_expr!(
            "1 + 2 == 3 + 4",
            expr_binary(
                expr_binary(expr_lit_int(1), BinOp::Add, expr_lit_int(2)),
                BinOp::Eq,
                expr_binary(expr_lit_int(3), BinOp::Add, expr_lit_int(4))
            )
        );

        test_expr!(
            "1 < 2 == 3 < 4",
            expr_binary(
                expr_binary(expr_lit_int(1), BinOp::Lt, expr_lit_int(2)),
                BinOp::Eq,
                expr_binary(expr_lit_int(3), BinOp::Lt, expr_lit_int(4))
            )
        );

        test_expr!(
            "1 + 2 < 3 + 4 == 5 + 6 < 7 + 8",
            expr_binary(
                expr_binary(
                    expr_binary(expr_lit_int(1), BinOp::Add, expr_lit_int(2)),
                    BinOp::Lt,
                    expr_binary(expr_lit_int(3), BinOp::Add, expr_lit_int(4))
                ),
                BinOp::Eq,
                expr_binary(
                    expr_binary(expr_lit_int(5), BinOp::Add, expr_lit_int(6)),
                    BinOp::Lt,
                    expr_binary(expr_lit_int(7), BinOp::Add, expr_lit_int(8))
                )
            )
        );
    }

    #[test]
    fn test_parse_add() {
        test_expr!(
            "1 + 2",
            expr_binary(expr_lit_int(1), BinOp::Add, expr_lit_int(2))
        );

        test_expr!(
            "1 - 2",
            expr_binary(expr_lit_int(1), BinOp::Sub, expr_lit_int(2))
        );

        test_expr!(
            "1 + 2 - 3",
            expr_binary(
                expr_lit_int(1),
                BinOp::Add,
                expr_binary(expr_lit_int(2), BinOp::Sub, expr_lit_int(3))
            )
        );

        test_expr!(
            "-1 - 2",
            expr_binary(
                expr_unary(UnOp::Neg, expr_lit_int(1)),
                BinOp::Sub,
                expr_lit_int(2),
            )
        );
    }

    #[test]
    fn test_parse_mul() {
        test_expr!(
            "1 * 2",
            expr_binary(expr_lit_int(1), BinOp::Mul, expr_lit_int(2))
        );

        test_expr!(
            "1 / 2",
            expr_binary(expr_lit_int(1), BinOp::Div, expr_lit_int(2))
        );

        test_expr!(
            "1 * 2 / 3",
            expr_binary(
                expr_lit_int(1),
                BinOp::Mul,
                expr_binary(expr_lit_int(2), BinOp::Div, expr_lit_int(3))
            )
        );

        test_expr!(
            "-1 * 2",
            expr_binary(
                expr_unary(UnOp::Neg, expr_lit_int(1)),
                BinOp::Mul,
                expr_lit_int(2)
            )
        );
    }

    #[test]
    fn test_parse_unary() {
        test_expr!("-1", expr_unary(UnOp::Neg, expr_lit_int(1)));
        test_expr!("1", expr_lit_int(1));
    }

    #[test]
    fn test_parse_primary() {
        test_expr!("1", expr_lit_int(1));
        test_expr!("(1)", expr_lit_int(1));
        test_expr!(
            "(1 * 2)",
            expr_binary(expr_lit_int(1), BinOp::Mul, expr_lit_int(2))
        );
    }

    // #[test]
    // fn ident() {
    //     test_expr!("a", expr_ident(Symbol::ident_nth(0)));
    //     test_expr!(
    //         "a + 1",
    //         expr_binary(
    //             expr_ident(Symbol::ident_nth(0)),
    //             BinOp::Add,
    //             expr_lit_int(1)
    //         )
    //     );
    // }

    #[test]
    fn path() {
        test_expr!("a", expr_path(Symbol::ident_nth(0)));
        test_expr!(
            "a + 1",
            expr_binary(expr_path(Symbol::ident_nth(0)), BinOp::Add, expr_lit_int(1))
        );
    }
}
