use ast::{expr::*, lit::*};
use thir;
use ty::*;

use std::collections::HashMap;

#[allow(dead_code)]
pub struct LoweringContext {
    ty_ctxt: HashMap<String, Ty>,
}

impl LoweringContext {
    pub fn new() -> Self {
        LoweringContext {
            ty_ctxt: HashMap::new(),
        }
    }

    pub fn lower_expr(&mut self, expr: &Expr) -> thir::Expr {
        match expr {
            Expr::Binary(_) => todo!(),
            Expr::Unary(_) => todo!(),
            Expr::Lit(lit) => self.lower_expr_lit(&lit.lit),
            Expr::Ident(_) => todo!(),
        }
    }

    fn lower_expr_lit(&mut self, lit: &Lit) -> thir::Expr {
        match lit {
            Lit::Int(lit_int) => {
                let lit = {
                    let value: u128 = lit_int
                        .digits
                        .parse()
                        .expect("error: couldn't parse LitInt.digits");
                    let lit_int = thir::LitInt { value: value };

                    thir::Lit::Int(lit_int)
                };

                let ty = ty::Ty {
                    kind: ty::TyKind::Int(ty::IntTy::I32),
                };

                thir::Expr::Lit { lit, ty }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::builder::*;

    #[test]
    fn lower_expr_lit_int() {
        let ast = expr::expr_lit_int("1");
        let thir = {
            let lit = thir::Lit::Int(thir::LitInt { value: 1 });
            let ty = ty::Ty {
                kind: ty::TyKind::Int(ty::IntTy::I32),
            };

            thir::Expr::Lit { lit, ty }
        };

        assert_eq!(thir, LoweringContext::new().lower_expr(&ast));
    }
}
