use hir::{self, def_id::DefId};
use thir::*;
use ty::*;

use std::collections::HashMap;

pub struct LoweringContext {
    ty_ctxt: HashMap<DefId, Ty>,
}

impl LoweringContext {
    pub fn new() -> LoweringContext {
        LoweringContext {
            ty_ctxt: HashMap::new(),
        }
    }

    pub fn lower_lit(&self, lit: &hir::Lit) -> Lit {
        match lit {
            hir::Lit::Bool { value } => Lit::Bool { value: *value },
            hir::Lit::Int(value) => {
                let value = LitInt { value: value.value };
                Lit::Int(value)
            }
        }
    }

    pub fn lower_pat(&self, pat: &hir::Pat, ty: ty::Ty) -> Pat {
        let kind = match &pat.kind {
            hir::PatKind::Binding { res, name } => PatKind::Binding {
                res: *res,
                name: *name,
                ty: ty.clone(),
            },
        };
        Pat {
            ty,
            kind: Box::new(kind),
        }
    }

    pub fn lower_expr(&mut self, expr: &hir::Expr) -> Expr {
        match expr {
            hir::Expr::Binary { op, lhs, rhs } => {
                let lhs = Box::new(self.lower_expr(lhs));
                let rhs = Box::new(self.lower_expr(rhs));
                let op = match op {
                    hir::BinOp::Mul => thir::BinOp::Mul,
                    hir::BinOp::Div => thir::BinOp::Div,
                    hir::BinOp::Add => thir::BinOp::Add,
                    hir::BinOp::Sub => thir::BinOp::Sub,
                    hir::BinOp::Eq => thir::BinOp::Eq,
                    hir::BinOp::Lt => thir::BinOp::Lt,
                    hir::BinOp::Le => thir::BinOp::Le,
                    hir::BinOp::Ne => thir::BinOp::Ne,
                    hir::BinOp::Ge => thir::BinOp::Ge,
                    hir::BinOp::Gt => thir::BinOp::Gt,
                };

                let ty = match op {
                    BinOp::Mul | BinOp::Div | BinOp::Add | BinOp::Sub => Ty {
                        kind: TyKind::Int(IntTy::I32),
                    },
                    BinOp::Eq | BinOp::Lt | BinOp::Le | BinOp::Ne | BinOp::Ge | BinOp::Gt => {
                        Ty { kind: TyKind::Bool }
                    }
                };

                Expr::Binary { op, lhs, rhs, ty }
            }
            hir::Expr::Unary { op, expr } => {
                let expr = Box::new(self.lower_expr(expr));
                let (op, ty) = match op {
                    hir::UnOp::Neg => (
                        thir::UnOp::Neg,
                        Ty {
                            kind: TyKind::Int(IntTy::I32),
                        },
                    ),
                };
                Expr::Unary { op, expr, ty }
            }
            hir::Expr::If {
                cond,
                then,
                else_opt,
            } => {
                let cond = Box::new(self.lower_expr(cond));
                let then = Box::new(self.lower_block(then));
                let else_opt = else_opt.as_ref().map(|e| Box::new(self.lower_expr(e)));
                let ty = then.ty.clone();

                Expr::If {
                    cond,
                    then,
                    else_opt,
                    ty,
                }
            }
            hir::Expr::Loop { block } => {
                let block = Box::new(self.lower_block(block));

                Expr::Loop { block }
            }
            hir::Expr::Break { expr } => {
                let expr = expr.as_ref().map(|e| Box::new(self.lower_expr(e)));
                let ty = Ty {
                    kind: TyKind::Never,
                };

                Expr::Break { expr, ty }
            }
            hir::Expr::Continue { expr } => {
                let expr = expr.as_ref().map(|e| Box::new(self.lower_expr(e)));
                let ty = Ty {
                    kind: TyKind::Never,
                };

                Expr::Continue { expr, ty }
            }
            hir::Expr::Block { block } => {
                let block = Box::new(self.lower_block(block));

                Expr::Block { block }
            }
            hir::Expr::Assign { lhs, rhs } => {
                let rhs = Box::new(self.lower_expr(rhs));
                let lhs = Box::new(self.lower_expr(lhs));
                let ty = Ty {
                    kind: TyKind::Tuple(Vec::new()),
                };

                Expr::Assign { lhs, rhs, ty }
            }
            hir::Expr::Lit { lit, ty } => Expr::Lit {
                lit: self.lower_lit(lit),
                ty: ty.clone(),
            },
            hir::Expr::Path { path } => {
                let def = path.res;
                let ty = self.ty_ctxt[&def].clone();

                Expr::VarRef { def, ty }
            }
        }
    }

    pub fn lower_stmt(&mut self, stmt: &hir::Stmt) -> Stmt {
        match stmt {
            hir::Stmt::Local { pat, ty, init } => {
                let init = self.lower_expr(init);

                let def = match pat.kind {
                    hir::PatKind::Binding { res, .. } => res,
                };
                let ty = ty.clone().expect("Type annotation is required");
                self.ty_ctxt.insert(def, ty.clone());

                let pat = self.lower_pat(pat, ty.clone());

                Stmt::Local { pat, init }
            }
            hir::Stmt::Expr(expr) | hir::Stmt::Semi(expr) => Stmt::Expr(self.lower_expr(expr)),
            hir::Stmt::Println(expr) => Stmt::Println(self.lower_expr(expr)),
        }
    }

    pub fn lower_block(&mut self, block: &hir::Block) -> Block {
        let stmts = block.stmts.iter().map(|s| self.lower_stmt(s)).collect();
        let expr = block.expr.as_ref().map(|e| self.lower_expr(e));
        let ty = expr.as_ref().map_or(
            Ty {
                kind: TyKind::Tuple(Vec::new()),
            },
            |e| e.ty(),
        );

        Block { stmts, expr, ty }
    }
}
