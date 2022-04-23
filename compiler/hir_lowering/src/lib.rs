use ast::op::{BinOp, UnOp};
use hir::{self, def_id::DefId};
use span::*;
use thir::*;
use ty::*;

use core::panic;
use std::collections::HashMap;

pub struct LoweringCtx {
    ty_ctxt: HashMap<DefId, ty::Ty>,
}

impl LoweringCtx {
    pub fn new() -> LoweringCtx {
        LoweringCtx {
            ty_ctxt: HashMap::new(),
        }
    }

    pub fn lower_lit(&self, lit: &hir::Lit) -> Expr {
        let (lit, ty) = match lit {
            hir::Lit::Bool { value } => (
                Lit::Bool { value: *value },
                ty::Ty {
                    kind: ty::TyKind::Bool,
                },
            ),
            hir::Lit::Int(value) => (
                Lit::Int(LitInt { value: value.value }),
                ty::Ty {
                    kind: ty::TyKind::Int(ty::IntTy::I32),
                },
            ),
        };

        Expr::Lit { lit, ty }
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

                let ty = match op {
                    BinOp::Mul | BinOp::Div | BinOp::Add | BinOp::Sub => Ty {
                        kind: TyKind::Int(IntTy::I32),
                    },
                    BinOp::Eq | BinOp::Lt | BinOp::Le | BinOp::Ne | BinOp::Ge | BinOp::Gt => {
                        Ty { kind: TyKind::Bool }
                    }
                };

                Expr::Binary {
                    op: *op,
                    lhs,
                    rhs,
                    ty,
                }
            }
            hir::Expr::Unary { op, expr } => {
                let expr = Box::new(self.lower_expr(expr));
                let (op, ty) = match op {
                    UnOp::Neg => (
                        UnOp::Neg,
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
            hir::Expr::Lit { lit } => self.lower_lit(lit),
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
                let ty = ty
                    .as_ref()
                    .map(|ty| lower_ty(ty))
                    .expect("Type annotation is required.");
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

pub fn lower_ty(ty: &ast::ty::Ty) -> ty::Ty {
    match &ty.kind {
        ast::ty::TyKind::Path(path) => {
            let name = path.ident.name;
            if name == Kw::I32.as_symbol() {
                return ty::Ty {
                    kind: TyKind::Int(ty::IntTy::I32),
                };
            } else if name == Kw::Bool.as_symbol() {
                return ty::Ty { kind: TyKind::Bool };
            }

            panic!("Undefined type given.");
        }
    }
}
