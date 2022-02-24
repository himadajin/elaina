use ast::{expr::*, lit::*, op::UnOp};
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
            Expr::Unary(unary) => self.lower_expr_unary(&unary.op, &unary.expr),
            Expr::Lit(lit) => self.lower_expr_lit(&lit.lit),
            Expr::Ident(_) => todo!(),
        }
    }

    fn lower_expr_unary(&mut self, op: &UnOp, expr: &Expr) -> thir::Expr {
        match op {
            UnOp::Neg => {
                let thir_expr = self.lower_expr(expr);
                let ty = ty::Ty {
                    kind: ty::TyKind::Int(ty::IntTy::I32),
                };

                thir::Expr::Unary {
                    op: thir::UnOp::Neg,
                    expr: Box::new(thir_expr),
                    ty: ty,
                }
            }
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
    use ast::builder::expr;

    #[test]
    fn lower_expr_unary() {
        let ast = expr::expr_unary(UnOp::Neg, expr::expr_lit_int("1"));
        let thir = {
            let i32_ty = ty::Ty {
                kind: ty::TyKind::Int(ty::IntTy::I32),
            };

            let expr_lit = {
                let lit = thir::Lit::Int(thir::LitInt { value: 1 });

                thir::Expr::Lit { lit, ty: i32_ty }
            };

            thir::Expr::Unary {
                op: thir::UnOp::Neg,
                expr: Box::new(expr_lit),
                ty: i32_ty,
            }
        };

        assert_eq!(thir, LoweringContext::new().lower_expr(&ast));
    }

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
