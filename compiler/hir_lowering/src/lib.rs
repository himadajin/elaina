use ast::op::{BinOp, UnOp};
use hir::{
    self,
    res::{DefId, Res},
};
use span::*;
use thir::*;
use ty::*;

use std::collections::HashMap;

pub struct TyCtx {
    map: HashMap<DefId, Ty>,
    types: CommonTypes,
}

impl TyCtx {
    pub fn new() -> TyCtx {
        TyCtx {
            map: HashMap::new(),
            types: CommonTypes::new(),
        }
    }

    fn insert_ty(&mut self, def: DefId, ty: Ty) {
        self.map.insert(def, ty);
    }

    fn get_ty(&self, def: DefId) -> Ty {
        self.map.get(&def).unwrap().clone()
    }
}

impl TyCtx {
    pub fn lower_ty(&self, ty: &ast::Ty) -> Ty {
        match &ty.kind {
            ast::TyKind::Path(path) => self
                .types
                .from_name(path.ident.name)
                .expect("The type with the given name does not exist"),
        }
    }

    pub fn lower_fun_ty(
        &mut self,
        fn_def: DefId,
        hir_inputs: &Vec<hir::Param>,
        hir_output: &Option<ast::ty::Ty>,
    ) -> Ty {
        let mut inputs = Vec::new();
        for param in hir_inputs {
            let res = param.res;
            let ty = self.lower_ty(&param.ty);
            self.insert_ty(res.def, ty.clone());
            inputs.push(ty);
        }

        let output = hir_output.as_ref().map(|ty| self.lower_ty(ty));
        let ty = Ty {
            kind: TyKind::Fn(FnTy {
                inputs,
                output: Box::new(output),
            }),
        };

        self.insert_ty(fn_def, ty.clone());

        ty
    }
}

impl TyCtx {
    pub fn lower_lit(&self, lit: &hir::Lit) -> Expr {
        let ty = self.types.from_lit(lit);

        let lit = match lit {
            hir::Lit::Bool { value } => Lit::Bool { value: *value },
            hir::Lit::Int(lit) => Lit::Int(LitInt { value: lit.value }),
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
            hir::Expr::Call { .. } => todo!(),
            hir::Expr::Binary { op, lhs, rhs } => {
                let lhs = Box::new(self.lower_expr(lhs));
                let rhs = Box::new(self.lower_expr(rhs));

                let ty = match op {
                    BinOp::Mul | BinOp::Div | BinOp::Add | BinOp::Sub => self.types.i32.clone(),
                    BinOp::Eq | BinOp::Lt | BinOp::Le | BinOp::Ne | BinOp::Ge | BinOp::Gt => {
                        self.types.bool.clone()
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
                let ty = match op {
                    UnOp::Neg => self.types.i32.clone(),
                };
                Expr::Unary { op: *op, expr, ty }
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
                let ty = self.types.unit.clone();

                Expr::Assign { lhs, rhs, ty }
            }
            hir::Expr::Lit { lit } => self.lower_lit(lit),
            hir::Expr::Path { path } => {
                let def = path.res.def;
                let ty = self.get_ty(def).clone();
                Expr::VarRef { res: path.res, ty }
            }
        }
    }

    pub fn lower_stmt(&mut self, stmt: &hir::Stmt) -> Stmt {
        match stmt {
            hir::Stmt::Local { pat, ty, init } => {
                let init = self.lower_expr(init);

                let res = match pat.kind {
                    hir::PatKind::Binding { res, .. } => res,
                };
                let ty = ty
                    .as_ref()
                    .map(|ty| self.lower_ty(ty))
                    .expect("Type annotation is requred.");
                self.insert_ty(res.def, ty.clone());

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
        let ty = expr
            .as_ref()
            .map_or_else(|| self.types.unit.clone(), |e| e.ty());

        Block { stmts, expr, ty }
    }

    pub fn lower_items(&mut self, items: &[hir::Item]) -> Vec<Item> {
        items.iter().map(|item| self.lower_item(item)).collect()
    }

    pub fn lower_item(&mut self, item: &hir::Item) -> Item {
        let kind = match &item.kind {
            hir::ItemKind::Fn(fun) => self.lower_fun(item.res, &fun.inputs, &fun.output, &fun.body),
        };

        Item {
            res: item.res,
            name: item.name,
            kind,
        }
    }

    fn lower_fun(
        &mut self,
        res: Res,
        inputs: &Vec<hir::Param>,
        output: &Option<ast::ty::Ty>,
        body: &hir::Block,
    ) -> ItemKind {
        let ty = self.lower_fun_ty(res.def, inputs, output);

        let inputs = inputs
            .iter()
            .map(|param| Param {
                res: param.res,
                name: param.name,
            })
            .collect();
        let body = self.lower_block(body);

        ItemKind::Fn(Box::new(Fn { ty, inputs, body }))
    }
}

pub struct CommonTypes {
    pub unit: Ty,
    pub bool: Ty,
    pub i32: Ty,
    pub never: Ty,
}

impl CommonTypes {
    fn new() -> CommonTypes {
        use ty::TyKind::*;
        CommonTypes {
            unit: Ty {
                kind: Tuple(Vec::new()),
            },
            bool: Ty { kind: Bool },
            i32: Ty {
                kind: Int(IntTy::I32),
            },
            never: Ty { kind: Never },
        }
    }

    fn from_name(&self, name: Symbol) -> Option<Ty> {
        if name == Kw::Bool.into() {
            return Some(self.bool.clone());
        } else if name == Kw::I32.into() {
            return Some(self.i32.clone());
        }
        None
    }

    fn from_lit(&self, lit: &hir::Lit) -> Ty {
        match lit {
            hir::Lit::Bool { .. } => self.bool.clone(),
            hir::Lit::Int(_) => self.i32.clone(),
        }
    }
}
