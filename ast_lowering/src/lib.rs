use ast::{
    expr::*,
    lit::*,
    op::{BinOp, UnOp},
};
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
            Expr::Binary(binary) => self.lower_expr_binary(&binary.op, &binary.lhs, &binary.rhs),
            Expr::Unary(unary) => self.lower_expr_unary(&unary.op, &unary.expr),
            Expr::Lit(lit) => self.lower_expr_lit(&lit.lit),
            Expr::Ident(_) => todo!(),
        }
    }

    fn lower_expr_binary(&mut self, op: &BinOp, lhs: &Expr, rhs: &Expr) -> thir::Expr {
        let thir_op = |op| match op {
            BinOp::Add => thir::BinOp::Add,
            BinOp::Mul => thir::BinOp::Mul,
            BinOp::Div => thir::BinOp::Div,
            BinOp::Sub => thir::BinOp::Sub,
        };

        match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                let thir_lhs = self.lower_expr(lhs);
                let thir_rhs = self.lower_expr(rhs);
                let i32_ty = ty::Ty {
                    kind: ty::TyKind::Int(ty::IntTy::I32),
                };

                thir::Expr::Binary {
                    op: thir_op(*op),
                    lhs: Box::new(thir_lhs),
                    rhs: Box::new(thir_rhs),
                    ty: i32_ty,
                }
            }
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
    fn lower_expr_binary() {
        let thir_binary = |op, lhs, rhs| {
            let i32_ty = ty::Ty {
                kind: ty::TyKind::Int(ty::IntTy::I32),
            };

            let lhs = {
                let lit = thir::Lit::Int(thir::LitInt { value: lhs });

                thir::Expr::Lit { lit, ty: i32_ty }
            };

            let rhs = {
                let lit = thir::Lit::Int(thir::LitInt { value: rhs });

                thir::Expr::Lit { lit, ty: i32_ty }
            };

            thir::Expr::Binary {
                op: op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                ty: i32_ty,
            }
        };

        {
            let ast =
                expr::expr_binary(expr::expr_lit_int("1"), BinOp::Add, expr::expr_lit_int("2"));
            let thir = thir_binary(thir::BinOp::Add, 1, 2);

            assert_eq!(thir, LoweringContext::new().lower_expr(&ast));
        }

        {
            let ast =
                expr::expr_binary(expr::expr_lit_int("1"), BinOp::Sub, expr::expr_lit_int("2"));
            let thir = thir_binary(thir::BinOp::Sub, 1, 2);

            assert_eq!(thir, LoweringContext::new().lower_expr(&ast));
        }
        

        {
            let ast =
                expr::expr_binary(expr::expr_lit_int("1"), BinOp::Mul, expr::expr_lit_int("2"));
            let thir = thir_binary(thir::BinOp::Mul, 1, 2);

            assert_eq!(thir, LoweringContext::new().lower_expr(&ast));
        }

        {
            let ast =
                expr::expr_binary(expr::expr_lit_int("1"), BinOp::Div, expr::expr_lit_int("2"));
            let thir = thir_binary(thir::BinOp::Div, 1, 2);

            assert_eq!(thir, LoweringContext::new().lower_expr(&ast));
        }
    }

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
