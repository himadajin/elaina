use crate::Parser;

use ast::ty::*;

use anyhow::Result;

impl Parser<'_> {
    pub fn parse_ty(&mut self) -> Result<Ty> {
        self.parse_ty_path()
    }

    pub fn parse_ty_path(&mut self) -> Result<Ty> {
        let path = self.parse_path()?;

        Ok(Ty {
            kind: TyKind::Path(path),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::parse_all_token;
    use ast::Path;
    use span::symbol::{Ident, Kw};

    macro_rules! test_ty {
        ($input: expr, $expected: expr) => {
            let tokens = parse_all_token($input);
            let result = Parser::new(&tokens).parse_ty().unwrap();

            assert_eq!(result, $expected);
        };
    }

    #[test]
    fn parse_primary_types() {
        test_ty!(
            "i32",
            Ty {
                kind: TyKind::Path(Path {
                    ident: Ident::with_dummy_span(Kw::I32)
                })
            }
        );

        test_ty!(
            "bool",
            Ty {
                kind: TyKind::Path(Path {
                    ident: Ident::with_dummy_span(Kw::Bool)
                })
            }
        );
    }
}
