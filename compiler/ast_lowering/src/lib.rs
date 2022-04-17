use ast::{
    block::*,
    expr::*,
    lit::*,
    op::{BinOp, UnOp},
    stmt::*,
};
use hir::{self, def_id::DefId};
use span::{
    span::Span,
    symbol::{Ident, Kw, Symbol},
};
use ty::*;

use std::collections::HashMap;

#[allow(dead_code)]
pub struct LoweringContext {
    ty_ctxt: HashMap<Symbol, Ty>,
    name_res: HashMap<Span, DefId>,
}

impl LoweringContext {
    pub fn new(name_res: HashMap<Span, DefId>) -> Self {
        LoweringContext {
            ty_ctxt: HashMap::new(),
            name_res,
        }
    }

    pub fn lower_block(&mut self, body: &Block) -> hir::Block {
        let (stmts, expr) = self.lower_stmts(&body.stmts);
        hir::Block { stmts, expr }
    }

    pub fn lower_stmts(&mut self, mut ast_stmts: &[Stmt]) -> (Vec<hir::Stmt>, Option<hir::Expr>) {
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
                        stmts.push(hir::Stmt::Expr(e));
                    }
                }
                Stmt::Semi(expr) => stmts.push(hir::Stmt::Semi(self.lower_expr(expr))),
                Stmt::Println(expr) => stmts.push(hir::Stmt::Println(self.lower_expr(expr))),
            }
            ast_stmts = &ast_stmts[1..];
        }

        (stmts, expr)
    }

    fn lower_stmt_local(&mut self, ident: Ident, ty: Option<Ident>, init: &Expr) -> hir::Stmt {
        let pat = {
            let res = self.name_res[&ident.span];
            hir::Pat {
                kind: hir::PatKind::Binding {
                    res,
                    name: ident.name,
                },
            }
        };

        let ty = {
            let ty_ident = ty.expect("error: type annotation is required").name;
            if ty_ident == Kw::I32.into() {
                ty::Ty {
                    kind: ty::TyKind::Int(ty::IntTy::I32),
                }
            } else if ty_ident == Kw::Bool.into() {
                ty::Ty {
                    kind: ty::TyKind::Bool,
                }
            } else {
                panic!("Error: unrecognized type")
            }
        };

        self.ty_ctxt.insert(ident.name.clone(), ty.clone());

        let init = self.lower_expr(init);

        hir::Stmt::Local {
            pat,
            ty: Some(ty),
            init,
        }
    }

    pub fn lower_expr(&mut self, expr: &Expr) -> hir::Expr {
        match expr {
            Expr::Binary { op, lhs, rhs } => self.lower_expr_binary(*op, &lhs, &rhs),
            Expr::Unary { op, expr } => self.lower_expr_unary(*op, &expr),
            Expr::If {
                cond,
                then,
                else_opt,
            } => self.lower_expr_if(cond.as_ref(), then.as_ref(), else_opt),
            Expr::Loop { block } => self.lower_expr_loop(block.as_ref()),
            Expr::Break { expr } => self.lower_expr_break(expr),
            Expr::Continue { expr } => self.lower_expr_continue(expr),
            Expr::Block { block } => hir::Expr::Block {
                block: Box::new(self.lower_block(block.as_ref())),
            },
            Expr::Assign { lhs, rhs } => self.lower_expr_assign(lhs.as_ref(), rhs.as_ref()),
            Expr::Lit { lit } => self.lower_expr_lit(&lit),
            Expr::Ident { .. } => todo!(),
            Expr::Path(path) => self.lower_expr_path(path),
        }
    }

    fn lower_expr_binary(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr) -> hir::Expr {
        let hir_op = |op| -> hir::BinOp {
            match op {
                BinOp::Add => hir::BinOp::Add,
                BinOp::Mul => hir::BinOp::Mul,
                BinOp::Div => hir::BinOp::Div,
                BinOp::Sub => hir::BinOp::Sub,
                BinOp::Eq => hir::BinOp::Eq,
                BinOp::Lt => hir::BinOp::Lt,
                BinOp::Le => hir::BinOp::Le,
                BinOp::Ne => hir::BinOp::Ne,
                BinOp::Ge => hir::BinOp::Ge,
                BinOp::Gt => hir::BinOp::Gt,
            }
        };

        match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                let thir_lhs = self.lower_expr(lhs);
                let thir_rhs = self.lower_expr(rhs);

                hir::Expr::Binary {
                    op: hir_op(op),
                    lhs: Box::new(thir_lhs),
                    rhs: Box::new(thir_rhs),
                }
            }
            BinOp::Eq | BinOp::Lt | BinOp::Le | BinOp::Ne | BinOp::Ge | BinOp::Gt => {
                let thir_lhs = self.lower_expr(lhs);
                let thir_rhs = self.lower_expr(rhs);

                hir::Expr::Binary {
                    op: hir_op(op),
                    lhs: Box::new(thir_lhs),
                    rhs: Box::new(thir_rhs),
                }
            }
        }
    }

    fn lower_expr_unary(&mut self, op: UnOp, expr: &Expr) -> hir::Expr {
        match op {
            UnOp::Neg => {
                let thir_expr = self.lower_expr(expr);

                hir::Expr::Unary {
                    op: hir::UnOp::Neg,
                    expr: Box::new(thir_expr),
                }
            }
        }
    }

    fn lower_expr_if(
        &mut self,
        cond: &Expr,
        then: &Block,
        else_opt: &Option<Box<Expr>>,
    ) -> hir::Expr {
        let cond_thir = Box::new(self.lower_expr(cond));
        let then_thir = Box::new(self.lower_block(then));
        let else_thir = match else_opt {
            Some(e) => Some(Box::new(self.lower_expr(e.as_ref()))),
            None => None,
        };

        hir::Expr::If {
            cond: cond_thir,
            then: then_thir,
            else_opt: else_thir,
        }
    }

    fn lower_expr_loop(&mut self, block: &Block) -> hir::Expr {
        let block = Box::new(self.lower_block(block));

        hir::Expr::Loop { block }
    }

    fn lower_expr_break(&mut self, expr: &Option<Box<Expr>>) -> hir::Expr {
        let expr = expr.as_ref().map(|e| Box::new(self.lower_expr(e.as_ref())));
        hir::Expr::Break { expr }
    }

    fn lower_expr_continue(&mut self, expr: &Option<Box<Expr>>) -> hir::Expr {
        let expr = expr.as_ref().map(|e| Box::new(self.lower_expr(e.as_ref())));

        hir::Expr::Continue { expr }
    }

    fn lower_expr_assign(&mut self, lhs: &Expr, rhs: &Expr) -> hir::Expr {
        let lhs = match lhs {
            Expr::Path(path) => self.lower_expr_path(path),
            _ => panic!("error: invalid left-hand side of assignment."),
        };
        let rhs = self.lower_expr(rhs);

        hir::Expr::Assign {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    fn lower_expr_lit(&mut self, lit: &Lit) -> hir::Expr {
        match lit.kind {
            LitKind::Int(value) => {
                let lit = {
                    let lit_int = hir::LitInt { value: value };

                    hir::Lit::Int(lit_int)
                };

                let ty = ty::Ty {
                    kind: ty::TyKind::Int(ty::IntTy::I32),
                };

                hir::Expr::Lit { lit, ty }
            }
            LitKind::Bool(value) => {
                let lit = hir::Lit::Bool { value: value };
                let ty = ty::Ty {
                    kind: ty::TyKind::Bool,
                };

                hir::Expr::Lit { lit, ty }
            }
        }
    }

    fn lower_expr_path(&mut self, path: &Path) -> hir::Expr {
        let ident = &path.ident;
        let def = self.name_res[&ident.span];
        let path = hir::Path { res: def };

        hir::Expr::Path { path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::*;
    use resolve::ASTNameResolver;
    use span::symbol::Symbol;

    const I32_TY: ty::Ty = ty::Ty {
        kind: ty::TyKind::Int(ty::IntTy::I32),
    };

    #[test]
    fn lower_stmt_local() {
        let src = r"
{
    let x: i32 = 0;
    x
}";
        let (ast, _) = parse_block_from_source_str(src);
        let hir = hir::Block {
            stmts: vec![hir::Stmt::Local {
                pat: hir::Pat {
                    kind: hir::PatKind::Binding {
                        res: DefId::from_usize(0),
                        name: Symbol::ident_nth(0),
                    },
                },
                ty: Some(I32_TY.clone()),
                init: hir::Expr::Lit {
                    lit: hir::Lit::Int(hir::LitInt { value: 0 }),
                    ty: I32_TY.clone(),
                },
            }],
            expr: Some(hir::Expr::Path {
                path: hir::Path {
                    res: DefId::from_usize(0),
                },
            }),
        };

        let res = {
            let mut resolver = ASTNameResolver::new();
            resolver.resolve_block(&ast);
            resolver.finish()
        };
        let mut ctx = LoweringContext::new(res);
        assert_eq!(hir, ctx.lower_block(&ast));
    }

    #[test]
    fn lower_expr_binary() {
        let hir_lit_int = |value| {
            let lit = hir::Lit::Int(hir::LitInt { value });
            hir::Expr::Lit { lit, ty: I32_TY }
        };

        let hir_bin = |op, lhs, rhs| {
            let lhs = hir_lit_int(lhs);
            let rhs = hir_lit_int(rhs);
            hir::Expr::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }
        };

        {
            let src = r"1 + 2";
            let ast = parse_expr_from_source_str(src).0;
            let hir = hir_bin(hir::BinOp::Add, 1, 2);

            let res = {
                let mut resolver = ASTNameResolver::new();
                resolver.resolve_expr(&ast);
                resolver.finish()
            };
            let mut ctx = LoweringContext::new(res);
            assert_eq!(hir, ctx.lower_expr(&ast));
        }

        {
            let src = r"1 - 2";
            let ast = parse_expr_from_source_str(src).0;
            let hir = hir_bin(hir::BinOp::Sub, 1, 2);

            let res = {
                let mut resolver = ASTNameResolver::new();
                resolver.resolve_expr(&ast);
                resolver.finish()
            };
            let mut ctx = LoweringContext::new(res);
            assert_eq!(hir, ctx.lower_expr(&ast));
        }

        {
            let src = r"1 * 2";
            let ast = parse_expr_from_source_str(src).0;
            let hir = hir_bin(hir::BinOp::Mul, 1, 2);

            let res = {
                let mut resolver = ASTNameResolver::new();
                resolver.resolve_expr(&ast);
                resolver.finish()
            };
            let mut ctx = LoweringContext::new(res);
            assert_eq!(hir, ctx.lower_expr(&ast));
        }

        {
            let src = r"1 / 2";
            let ast = parse_expr_from_source_str(src).0;
            let hir = hir_bin(hir::BinOp::Div, 1, 2);

            let res = {
                let mut resolver = ASTNameResolver::new();
                resolver.resolve_expr(&ast);
                resolver.finish()
            };
            let mut ctx = LoweringContext::new(res);
            assert_eq!(hir, ctx.lower_expr(&ast));
        }
    }

    #[test]
    fn lower_expr_unary() {
        let src = r"-1";
        let ast = parse_expr_from_source_str(src).0;
        let hir = hir::Expr::Unary {
            op: hir::UnOp::Neg,
            expr: Box::new(hir::Expr::Lit {
                lit: hir::Lit::Int(hir::LitInt { value: 1 }),
                ty: I32_TY.clone(),
            }),
        };
        let res = {
            let mut resolver = ASTNameResolver::new();
            resolver.resolve_expr(&ast);
            resolver.finish()
        };
        let mut ctx = LoweringContext::new(res);
        assert_eq!(hir, ctx.lower_expr(&ast));
    }
}
