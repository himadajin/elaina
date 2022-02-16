use crate::Parser;

use ast::stmt::*;

impl Parser {
    pub fn parse_stmt(&mut self) -> Stmt {
        let expr = self.parse_expr();

        Stmt::Expr(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::builder::{expr::*, stmt::*};
    use ast::token::Token;
    use lexer::Lexer;

    fn lex_all(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);

        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }

        return tokens;
    }

    macro_rules! test_stmt {
        ($input: expr, $expected: expr) => {
            let tokens = lex_all($input);
            let result = Parser::new(tokens).parse_stmt();

            assert_eq!(result, $expected);
        };
    }

    #[test]
    fn parse_expr() {
        test_stmt!(
            "1 + 2;",
            stmt_expr(expr_binary(
                expr_lit_int("1"),
                ast::op::BinOp::Add,
                expr_lit_int("2")
            ))
        );
    }
}
