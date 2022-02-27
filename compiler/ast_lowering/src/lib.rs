use ast::{
    block::*,
    expr::*,
    lit::*,
    op::{BinOp, UnOp},
    stmt::*,
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

    pub fn lower_body(&mut self, body: &Block) -> thir::Block {
        let mut stmts = Vec::new();

        for stmt in &body.stmts {
            let thir = self.lower_stmt(stmt);
            stmts.push(thir);
        }

        thir::Block { stmts }
    }

    pub fn lower_stmt(&mut self, stmt: &Stmt) -> thir::Stmt {
        match stmt {
            Stmt::Local { ident, ty, init } => {
                self.lower_stmt_local(ident.clone(), ty.clone(), &init)
            }
            Stmt::Expr(expr) => thir::Stmt::Expr(self.lower_expr(expr)),
            Stmt::Semi(expr) => thir::Stmt::Semi(self.lower_expr(expr)),
        }
    }

    fn lower_stmt_local(&mut self, ident: String, ty: Option<String>, init: &Expr) -> thir::Stmt {
        let ty = {
            let ty_ident = ty.expect("error: type annotation is required");
            match ty_ident.as_str() {
                "i32" => ty::Ty {
                    kind: ty::TyKind::Int(ty::IntTy::I32),
                },
                _ => panic!("error: unrecognized type"),
            }
        };

        self.ty_ctxt.insert(ident.clone(), ty);

        let thir_init = self.lower_expr(init);

        thir::Stmt::Local {
            ident: ident,
            init: thir_init,
        }
    }

    pub fn lower_expr(&mut self, expr: &Expr) -> thir::Expr {
        match expr {
            Expr::Binary { op, lhs, rhs } => self.lower_expr_binary(*op, &lhs, &rhs),
            Expr::Unary { op, expr } => self.lower_expr_unary(*op, &expr),
            Expr::Lit { lit } => self.lower_expr_lit(&lit),
            Expr::Ident { ident } => self.lower_expr_ident(ident.clone()),
        }
    }

    fn lower_expr_binary(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr) -> thir::Expr {
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
                    op: thir_op(op),
                    lhs: Box::new(thir_lhs),
                    rhs: Box::new(thir_rhs),
                    ty: i32_ty,
                }
            }
        }
    }

    fn lower_expr_unary(&mut self, op: UnOp, expr: &Expr) -> thir::Expr {
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
            Lit::Int { digits } => {
                let lit = {
                    let value: u128 = digits.parse().expect("error: couldn't parse LitInt.digits");
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

    fn lower_expr_ident(&mut self, ident: String) -> thir::Expr {
        let ty = *self
            .ty_ctxt
            .get(&ident)
            .expect("error: definition of identity not found");

        thir::Expr::Ident { ident, ty }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::builder::{expr, stmt};

    #[test]
    fn lower_stmt_local() {
        let stmt_local = stmt::stmt_local("a", "i32", expr::expr_lit_int("1"));
        let expr_ident = expr::expr_ident("a");

        let thir = {
            let i32_ty = ty::Ty {
                kind: ty::TyKind::Int(ty::IntTy::I32),
            };
            thir::Expr::Ident {
                ident: "a".into(),
                ty: i32_ty,
            }
        };

        let mut ctx = LoweringContext::new();
        ctx.lower_stmt(&stmt_local);
        assert_eq!(thir, ctx.lower_expr(&expr_ident));
    }

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
