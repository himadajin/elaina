use crate::Parser;

use ast::lit::{self, Lit, LitInt};

impl Parser {
    pub fn parse_lit(&mut self) -> lit::Lit {
        let digits = self.expect_num();

        Lit::Int(LitInt { digits: digits })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::lit::{Lit, LitInt};
    use lexer::token::Token;

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
}
