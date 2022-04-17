use crate::Parser;
use ast::{block::*, token::*};

use anyhow::Result;

impl Parser<'_> {
    pub fn parse_block(&mut self) -> Result<Block> {
        self.expect(&TokenKind::OpenDelim(DelimToken::Brace))?;

        let mut stmts = Vec::new();
        while !self.consume(&TokenKind::CloseDelim(DelimToken::Brace)) {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
        }

        Ok(Block { stmts: stmts })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::parse_all_token;
    // use ast::builder::{block::*, expr::*, stmt::*};
    use ast::op::*;
    use ast::{expr::Expr, stmt::Stmt};

    macro_rules! test_block {
        ($input: expr, $expected: expr) => {
            let tokens = parse_all_token($input);
            let result = Parser::new(&tokens).parse_block().unwrap();

            assert_eq!(result, $expected);
        };
    }

    #[test]
    fn block_empty() {
        test_block!("{}", [].into());
    }

    #[test]
    fn block_one_stmt() {
        test_block!("{1}", [Stmt::expr(Expr::lit_from_value_dummy(1))].into());
        test_block!("{1;}", [Stmt::semi(Expr::lit_from_value_dummy(1))].into());
        test_block!(
            "{1+2}",
            [Stmt::expr(Expr::binary(
                BinOp::Add,
                Expr::lit_from_value_dummy(1),
                Expr::lit_from_value_dummy(2)
            ))]
            .into()
        );
        test_block!(
            "{1+2;}",
            [Stmt::semi(Expr::binary(
                BinOp::Add,
                Expr::lit_from_value_dummy(1),
                Expr::lit_from_value_dummy(2)
            ))]
            .into()
        );
    }

    #[test]
    fn block_stmts() {
        test_block!(
            "{1;2}",
            [
                Stmt::semi(Expr::lit_from_value_dummy(1)),
                Stmt::expr(Expr::lit_from_value_dummy(2))
            ]
            .into()
        );
        test_block!(
            "{1+2;3}",
            [
                Stmt::semi(Expr::binary(
                    BinOp::Add,
                    Expr::lit_from_value_dummy(1),
                    Expr::lit_from_value_dummy(2)
                )),
                Stmt::expr(Expr::lit_from_value_dummy(3))
            ]
            .into()
        );
    }
}
