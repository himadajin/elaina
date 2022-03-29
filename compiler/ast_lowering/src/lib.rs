use ast::{
    block::*,
    expr::*,
    lit::*,
    op::{BinOp, UnOp},
    stmt::*,
};
use span::symbol::{Kw, Symbol};
use thir;
use ty::*;

use std::collections::HashMap;

#[allow(dead_code)]
pub struct LoweringContext {
    ty_ctxt: HashMap<Symbol, Ty>,
}

impl LoweringContext {
    pub fn new() -> Self {
        LoweringContext {
            ty_ctxt: HashMap::new(),
        }
    }

    pub fn lower_body(&mut self, body: &Block) -> thir::Block {
        let (stmts, expr) = self.lower_stmts(&body.stmts);

        let ty = match expr {
            Some(ref e) => e.ty(),
            None => Ty {
                kind: TyKind::Tuple(Vec::new()),
            },
        };

        thir::Block { stmts, expr, ty }
    }

    pub fn lower_stmts(&mut self, mut ast_stmts: &[Stmt]) -> (Vec<thir::Stmt>, Option<thir::Expr>) {
        let mut stmts = Vec::new();
        let mut expr = None;

        while let [s, tail @ ..] = ast_stmts {
            match s {
                Stmt::Local { ident, ty, init } => {
                    stmts.push(self.lower_stmt_local(ident.clone(), ty.clone(), &init))
                }
                Stmt::Expr(e) => {
                    let e = self.lower_expr(e);
                    if tail.is_empty() {
                        expr = Some(e);
                    } else {
                        stmts.push(thir::Stmt::Expr(e));
                    }
                }
                Stmt::Semi(expr) => stmts.push(thir::Stmt::Semi(self.lower_expr(expr))),
                Stmt::Println(expr) => stmts.push(thir::Stmt::Println(self.lower_expr(expr))),
            }
            ast_stmts = &ast_stmts[1..];
        }

        (stmts, expr)
    }

    fn lower_stmt_local(&mut self, ident: Symbol, ty: Option<Symbol>, init: &Expr) -> thir::Stmt {
        let ty = {
            let ty_ident = ty.expect("error: type annotation is required");
            if ty_ident == Kw::I32.as_symbol() {
                ty::Ty {
                    kind: ty::TyKind::Int(ty::IntTy::I32),
                }
            } else if ty_ident == Kw::Bool.as_symbol() {
                ty::Ty {
                    kind: ty::TyKind::Bool,
                }
            } else {
                panic!("Error: unrecognized type")
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
            Expr::If {
                cond,
                then,
                else_opt,
            } => self.lower_expr_if(cond.as_ref(), then.as_ref(), else_opt),
            Expr::Block { block: _block } => todo!(),
            Expr::Lit { lit } => self.lower_expr_lit(&lit),
            Expr::Ident { ident } => self.lower_expr_ident(ident.clone()),
        }
    }

    fn lower_expr_binary(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr) -> thir::Expr {
        let thir_op = |op| -> thir::BinOp {
            match op {
                BinOp::Add => thir::BinOp::Add,
                BinOp::Mul => thir::BinOp::Mul,
                BinOp::Div => thir::BinOp::Div,
                BinOp::Sub => thir::BinOp::Sub,
                BinOp::Eq => thir::BinOp::Eq,
                BinOp::Lt => thir::BinOp::Lt,
                BinOp::Le => thir::BinOp::Le,
                BinOp::Ne => thir::BinOp::Ne,
                BinOp::Ge => thir::BinOp::Ge,
                BinOp::Gt => thir::BinOp::Gt,
            }
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
            BinOp::Eq | BinOp::Lt | BinOp::Le | BinOp::Ne | BinOp::Ge | BinOp::Gt => {
                let thir_lhs = self.lower_expr(lhs);
                let thir_rhs = self.lower_expr(rhs);
                let bool_ty = ty::Ty {
                    kind: ty::TyKind::Bool,
                };

                thir::Expr::Binary {
                    op: thir_op(op),
                    lhs: Box::new(thir_lhs),
                    rhs: Box::new(thir_rhs),
                    ty: bool_ty,
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

    fn lower_expr_if(
        &mut self,
        cond: &Expr,
        then: &Block,
        else_opt: &Option<Box<Expr>>,
    ) -> thir::Expr {
        let cond_thir = Box::new(self.lower_expr(cond));
        let then_thir = Box::new(self.lower_body(then));
        let else_thir = match else_opt {
            Some(e) => Some(Box::new(self.lower_expr(e.as_ref()))),
            None => None,
        };

        let then_ty = then_thir.ty.clone();

        thir::Expr::If {
            cond: cond_thir,
            then: then_thir,
            else_opt: else_thir,
            ty: then_ty,
        }
    }

    fn lower_expr_lit(&mut self, lit: &Lit) -> thir::Expr {
        match lit.kind {
            LitKind::Int(value) => {
                let lit = {
                    let lit_int = thir::LitInt { value: value };

                    thir::Lit::Int(lit_int)
                };

                let ty = ty::Ty {
                    kind: ty::TyKind::Int(ty::IntTy::I32),
                };

                thir::Expr::Lit { lit, ty }
            }
            LitKind::Bool(value) => {
                let lit = thir::Lit::Bool { value: value };
                let ty = ty::Ty {
                    kind: ty::TyKind::Bool,
                };

                thir::Expr::Lit { lit, ty }
            }
        }
    }

    fn lower_expr_ident(&mut self, ident: Symbol) -> thir::Expr {
        let ty = self
            .ty_ctxt
            .get(&ident)
            .expect("error: definition of identity not found")
            .clone();

        thir::Expr::Ident { ident, ty }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::builder::{expr, stmt};
    use span::symbol::Symbol;

    #[test]
    fn lower_stmt_local() {
        let stmt_local = stmt::stmt_local(
            Symbol::ident_nth(0),
            Some(Kw::I32.as_symbol()),
            expr::expr_lit_int(1),
        );
        let expr_ident = expr::expr_ident(Symbol::ident_nth(0));

        let thir = {
            let i32_ty = ty::Ty {
                kind: ty::TyKind::Int(ty::IntTy::I32),
            };
            thir::Expr::Ident {
                ident: Symbol::ident_nth(0),
                ty: i32_ty,
            }
        };

        let mut ctx = LoweringContext::new();
        ctx.lower_stmts(&[stmt_local]);
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

                thir::Expr::Lit {
                    lit,
                    ty: i32_ty.clone(),
                }
            };

            let rhs = {
                let lit = thir::Lit::Int(thir::LitInt { value: rhs });

                thir::Expr::Lit {
                    lit,
                    ty: i32_ty.clone(),
                }
            };

            thir::Expr::Binary {
                op: op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                ty: i32_ty.clone(),
            }
        };

        {
            let ast = expr::expr_binary(expr::expr_lit_int(1), BinOp::Add, expr::expr_lit_int(2));
            let thir = thir_binary(thir::BinOp::Add, 1, 2);

            assert_eq!(thir, LoweringContext::new().lower_expr(&ast));
        }

        {
            let ast = expr::expr_binary(expr::expr_lit_int(1), BinOp::Sub, expr::expr_lit_int(2));
            let thir = thir_binary(thir::BinOp::Sub, 1, 2);

            assert_eq!(thir, LoweringContext::new().lower_expr(&ast));
        }

        {
            let ast = expr::expr_binary(expr::expr_lit_int(1), BinOp::Mul, expr::expr_lit_int(2));
            let thir = thir_binary(thir::BinOp::Mul, 1, 2);

            assert_eq!(thir, LoweringContext::new().lower_expr(&ast));
        }

        {
            let ast = expr::expr_binary(expr::expr_lit_int(1), BinOp::Div, expr::expr_lit_int(2));
            let thir = thir_binary(thir::BinOp::Div, 1, 2);

            assert_eq!(thir, LoweringContext::new().lower_expr(&ast));
        }
    }

    #[test]
    fn lower_expr_unary() {
        let ast = expr::expr_unary(UnOp::Neg, expr::expr_lit_int(1));
        let thir = {
            let i32_ty = ty::Ty {
                kind: ty::TyKind::Int(ty::IntTy::I32),
            };

            let expr_lit = {
                let lit = thir::Lit::Int(thir::LitInt { value: 1 });

                thir::Expr::Lit {
                    lit,
                    ty: i32_ty.clone(),
                }
            };

            thir::Expr::Unary {
                op: thir::UnOp::Neg,
                expr: Box::new(expr_lit),
                ty: i32_ty.clone(),
            }
        };

        assert_eq!(thir, LoweringContext::new().lower_expr(&ast));
    }

    #[test]
    fn lower_expr_lit_int() {
        let ast = expr::expr_lit_int(1);
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
