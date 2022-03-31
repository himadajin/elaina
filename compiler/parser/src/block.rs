use crate::Parser;
use ast::{block::*, token::*};

impl Parser<'_> {
    pub fn parse_block(&mut self) -> Block {
        self.expect(&TokenKind::OpenDelim(DelimToken::Brace));

        let mut stmts = Vec::new();
        while !self.consume(&TokenKind::CloseDelim(DelimToken::Brace)) {
            let stmt = self.parse_stmt();
            stmts.push(stmt);
        }

        Block { stmts: stmts }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::parse_all_token;
    use ast::builder::{block::*, expr::*, stmt::*};
    use ast::op::*;

    macro_rules! test_block {
        ($input: expr, $expected: expr) => {
            let tokens = parse_all_token($input);
            let result = Parser::new(&tokens).parse_block();

            assert_eq!(result, $expected);
        };
    }

    #[test]
    fn block_empty() {
        test_block!("{}", block([]));
    }

    #[test]
    fn block_one_stmt() {
        test_block!("{1}", block([stmt_expr(expr_lit_int(1))]));
        test_block!("{1;}", block([stmt_semi(expr_lit_int(1))]));
        test_block!(
            "{1+2}",
            block([stmt_expr(expr_binary(
                expr_lit_int(1),
                BinOp::Add,
                expr_lit_int(2)
            ))])
        );
        test_block!(
            "{1+2;}",
            block([stmt_semi(expr_binary(
                expr_lit_int(1),
                BinOp::Add,
                expr_lit_int(2)
            ))])
        );
    }

    #[test]
    fn block_stmts() {
        test_block!(
            "{1;2}",
            block([stmt_semi(expr_lit_int(1)), stmt_expr(expr_lit_int(2))])
        );
        test_block!(
            "{1+2;3}",
            block([
                stmt_semi(expr_binary(expr_lit_int(1), BinOp::Add, expr_lit_int(2))),
                stmt_expr(expr_lit_int(3))
            ])
        );
    }
}
