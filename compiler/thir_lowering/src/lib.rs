mod builder;

use ast::op::{BinOp, UnOp};
use builder::MirBuilder;
use mir::{constant::*, stmt::*, terminator::*, *};
use span::*;
use thir;
use ty::res::DefId;

use core::panic;
use std::collections::HashMap;

#[allow(dead_code)]
struct ControlFlowResolver {
    scopes: Vec<(Place, Vec<BlockId>)>,
}

#[allow(dead_code)]
impl ControlFlowResolver {
    fn new() -> Self {
        Self { scopes: Vec::new() }
    }

    fn push_scope(&mut self, place: Place) {
        self.scopes.push((place, Vec::new()));
    }

    fn pop_scope(&mut self) -> Vec<BlockId> {
        self.scopes
            .pop()
            .unwrap_or_else(|| {
                panic!("error: Tried to pop scope even though there is no scope to resolve")
            })
            .1
    }

    fn push_late_resolved(&mut self, block: BlockId) {
        self.scopes
            .last_mut()
            .unwrap_or_else(|| {
                panic!("error: Tried to pop scope even though there is no scope to resolve")
            })
            .1
            .push(block);
    }

    fn get_place(&mut self) -> Place {
        self.scopes
            .last()
            .unwrap_or_else(|| {
                panic!("error: Tried to pop scope even though there is no scope to resolve")
            })
            .0
            .clone()
    }
}

#[allow(dead_code)]
pub struct LoweringCtx<'a> {
    builder: MirBuilder,
    break_resolver: ControlFlowResolver,
    continue_resolver: ControlFlowResolver,
    return_resolver: ControlFlowResolver,

    local_def: HashMap<DefId, Place>,
    symbol_map: &'a SymbolMap<'a>,
}

impl<'a> LoweringCtx<'a> {
    pub fn new(def: DefId, name: Symbol, symbol_map: &'a SymbolMap<'a>) -> Self {
        LoweringCtx {
            builder: MirBuilder::new(def, name),
            break_resolver: ControlFlowResolver::new(),
            continue_resolver: ControlFlowResolver::new(),

            return_resolver: ControlFlowResolver::new(),

            local_def: HashMap::new(),
            symbol_map,
        }
    }

    pub fn build(self) -> Body {
        self.builder.build()
    }

    pub fn lower_item_fun(
        &mut self,
        inputs: &Vec<thir::Param>,
        output: &ty::Ty,
        body: &thir::Block,
    ) {
        let return_place = self
            .builder
            .push_local_decl(LocalDecl::new(Some("ret".into()), output.clone()));
        for input in inputs {
            let name = self.symbol_map.get(input.name).to_string();
            let ty = input.ty.clone();
            let place = self.builder.push_local_decl(LocalDecl::new(Some(name), ty));
            self.local_def.insert(input.res.def, place);
        }
        self.builder.set_arg_count(inputs.len());

        self.return_resolver.push_scope(return_place);

        let entry_block = self.builder.push_block(None);
        let (tail, _) = self.lower_block(entry_block, &body.stmts, &body.expr);

        let return_block = self.builder.push_block(Some(Terminator::Return));
        self.builder.set_terminator(
            tail,
            Terminator::Goto {
                target: return_block,
            },
        );
        let return_from = self.return_resolver.pop_scope();
        for target in return_from {
            self.builder.set_terminator(
                target,
                Terminator::Goto {
                    target: return_block,
                },
            )
        }
    }

    fn lower_block(
        &mut self,
        entry: BlockId,
        stmts: &Vec<thir::Stmt>,
        expr: &Option<thir::Expr>,
    ) -> (BlockId, Operand) {
        let mut tail = entry;
        for stmt in stmts {
            tail = self.lower_stmt(tail, stmt);
        }

        match &expr {
            Some(e) => self.lower_expr(tail, e),
            None => (tail, Operand::Constant(Box::new(Constant::UNIT))),
        }
    }

    fn lower_stmt(&mut self, entry_block: BlockId, stmt: &thir::Stmt) -> BlockId {
        match stmt {
            thir::Stmt::Local { pat, init } => {
                let place = match pat.kind.as_ref() {
                    thir::PatKind::Binding { res, name, ty } => {
                        self.push_local(res.def, Some(*name), ty.clone())
                    }
                };
                let (tail, operand) = self.lower_expr(entry_block, init);
                let rvalue = RValue::Use(operand);
                let stmt = Statement::Assign(Box::new((place, rvalue)));
                self.builder.push_stmt(tail, stmt);

                tail
            }
            thir::Stmt::Expr(e) => {
                let (tail, _) = self.lower_expr(entry_block, e);
                tail
            }
            thir::Stmt::Println(expr) => {
                let (tail, operand) = self.lower_expr(entry_block, expr);
                let stmt = Statement::Println(operand);
                self.builder.push_stmt(tail, stmt);
                tail
            }
        }
    }

    fn lower_expr(&mut self, entry_block: BlockId, expr: &thir::Expr) -> (BlockId, Operand) {
        match expr {
            thir::Expr::Call { fun, args, ty } => {
                self.lower_expr_call(entry_block, fun, args, ty.clone())
            }
            thir::Expr::Binary { op, lhs, rhs, ty } => {
                self.lower_expr_binary(entry_block, *op, lhs, rhs, ty.clone())
            }
            thir::Expr::Unary { op, expr, ty } => {
                self.lower_expr_unary(entry_block, *op, expr, ty.clone())
            }
            thir::Expr::If {
                cond,
                then,
                else_opt,
                ty,
            } => self.lower_expr_if(
                entry_block,
                cond.as_ref(),
                then.as_ref(),
                else_opt,
                ty.clone(),
            ),
            thir::Expr::Loop { block } => self.lower_expr_loop(entry_block, block.as_ref()),
            thir::Expr::Break { expr, ty } => self.lower_expr_break(entry_block, expr, ty.clone()),
            thir::Expr::Continue { expr, ty } => {
                self.lower_expr_continue(entry_block, expr, ty.clone())
            }
            thir::Expr::Return { expr, .. } => self.lower_expr_return(entry_block, expr),
            thir::Expr::Block { block } => {
                let id = self.builder.push_block(None);
                self.builder
                    .set_terminator(entry_block, Terminator::Goto { target: id });

                self.lower_block(id, &block.stmts, &block.expr)
            }
            thir::Expr::Assign { lhs, rhs, ty } => {
                self.lower_expr_assign(entry_block, lhs.as_ref(), rhs.as_ref(), ty.clone())
            }
            thir::Expr::Lit { lit, ty } => (entry_block, self.lower_expr_lit(lit, ty.clone())),
            thir::Expr::VarRef { res, ty } => {
                (entry_block, self.lower_expr_var_ref(res.def, ty.clone()))
            }
        }
    }

    fn lower_expr_call(
        &mut self,
        entry_block: BlockId,
        fun: &thir::Expr,
        args: &[thir::Expr],
        ty: ty::Ty,
    ) -> (BlockId, Operand) {
        let (tail, fun) = self.lower_expr(entry_block, fun);
        let (tail, args) = args
            .iter()
            .fold((tail, Vec::new()), |(tail, mut args), expr| {
                let (tail, arg) = self.lower_expr(tail, expr);
                args.push(arg);

                (tail, args)
            });
        let end = self.builder.push_block(None);
        let ret = self
            .builder
            .push_local_decl(LocalDecl::new(Some("call".into()), ty));
        let terminator = Terminator::Call {
            fun,
            args,
            destination: Some((ret.clone(), end)),
        };
        self.builder.set_terminator(tail, terminator);

        (end, Operand::Copy(ret))
    }

    fn lower_expr_binary(
        &mut self,
        entry_block: BlockId,
        op: BinOp,
        lhs: &thir::Expr,
        rhs: &thir::Expr,
        ty: ty::Ty,
    ) -> (BlockId, Operand) {
        fn lower_bin_op(op: BinOp) -> mir::stmt::BinOp {
            match op {
                BinOp::Mul => mir::stmt::BinOp::Mul,
                BinOp::Div => mir::stmt::BinOp::Div,
                BinOp::Add => mir::stmt::BinOp::Add,
                BinOp::Sub => mir::stmt::BinOp::Sub,
                BinOp::Eq => mir::stmt::BinOp::Eq,
                BinOp::Lt => mir::stmt::BinOp::Lt,
                BinOp::Le => mir::stmt::BinOp::Le,
                BinOp::Ne => mir::stmt::BinOp::Ne,
                BinOp::Ge => mir::stmt::BinOp::Ge,
                BinOp::Gt => mir::stmt::BinOp::Gt,
            }
        }

        let (tail, lhs) = self.lower_expr(entry_block, lhs);
        let (tail, rhs) = self.lower_expr(tail, rhs);

        let op = lower_bin_op(op);

        let rvalue = RValue::BinaryOp(op, Box::new((lhs, rhs)));
        let place = self.push_temp(ty);
        let stmt = Statement::Assign(Box::new((place.clone(), rvalue)));

        self.builder.push_stmt(tail, stmt);

        (tail, Operand::Copy(place))
    }

    fn lower_expr_unary(
        &mut self,
        entry_block: BlockId,
        op: UnOp,
        expr: &thir::Expr,
        ty: ty::Ty,
    ) -> (BlockId, Operand) {
        fn lower_un_op(op: UnOp) -> mir::stmt::UnOp {
            match op {
                UnOp::Neg => mir::stmt::UnOp::Neg,
            }
        }

        let (tail, expr) = self.lower_expr(entry_block, expr);
        let op = lower_un_op(op);
        let rvalue = RValue::UnaryOp(op, Box::new(expr));
        let place = self.push_temp(ty);
        let stmt = Statement::Assign(Box::new((place.clone(), rvalue)));

        self.builder.push_stmt(tail, stmt);

        (tail, Operand::Copy(place))
    }

    fn lower_expr_if(
        &mut self,
        entry_block: BlockId,
        cond: &thir::Expr,
        then: &thir::Block,
        else_opt: &Option<Box<thir::Expr>>,
        ty: ty::Ty,
    ) -> (BlockId, Operand) {
        // Create cond block that represents condition expression.
        // Current Block jumps to cond block.
        let cond_entry = self.builder.push_block(None);
        self.builder
            .set_terminator(entry_block, Terminator::Goto { target: cond_entry });
        let (cond_tail, cond_operand) = self.lower_expr(cond_entry, cond);

        // If `ty` is not ZST(Zero Size Type), create local and treat it as the value of the expression.
        let expr_val = if !ty.is_zst() {
            Some(self.push_temp(ty.clone()))
        } else {
            None
        };

        // Create then block.
        let then_entry = self.builder.push_block(None);
        let (then_tail, then_operand) = self.lower_block(then_entry, &then.stmts, &then.expr);
        if let Some(p) = &expr_val {
            let rvalue = RValue::Use(then_operand);
            let stmt = Statement::Assign(Box::new((p.clone(), rvalue)));
            self.builder.push_stmt(then_tail, stmt);
        }

        // Create opt block if it exists.
        let (else_entry, else_tail) = match else_opt {
            Some(e) => {
                let else_entry = self.builder.push_block(None);
                let (else_tail, else_operand) = self.lower_expr(else_entry, e);

                if let Some(p) = &expr_val {
                    let rvalue = RValue::Use(else_operand);
                    let stmt = Statement::Assign(Box::new((p.clone(), rvalue)));
                    self.builder.push_stmt(else_tail, stmt);
                }

                (Some(else_entry), Some(else_tail))
            }
            None => (None, None),
        };

        // Create end block.
        let end_entry = self.builder.push_block(None);

        // Create terminator: cond block -> then_block or end_block.
        let cond_terminator = {
            let targets = {
                // targets: [else, then]
                let targets = match else_entry {
                    Some(else_entry) => vec![else_entry, then_entry],
                    None => vec![end_entry, then_entry],
                };

                SwitchTargets {
                    values: vec![0, 1],
                    targets,
                }
            };

            Terminator::SwitchInt {
                discr: cond_operand,
                switch_ty: ty::Ty {
                    kind: ty::TyKind::Bool,
                },
                targets,
            }
        };
        self.builder.set_terminator(cond_tail, cond_terminator);

        // Create terminator: then_block -> end_block.
        self.builder
            .set_terminator(then_tail, Terminator::Goto { target: end_entry });

        // Create terminator: else_block -> end_block.
        if let Some(else_tail) = else_tail {
            self.builder
                .set_terminator(else_tail, Terminator::Goto { target: end_entry });
        }

        // If expr_val exists, the operand is assigned evaluated value,
        // otherwise it is unit.
        let operand = match expr_val {
            Some(p) => Operand::Copy(p),
            None => Operand::Constant(Box::new(Constant::UNIT)),
        };

        (end_entry, operand)
    }

    fn lower_expr_loop(&mut self, entry_block: BlockId, block: &thir::Block) -> (BlockId, Operand) {
        let loop_head = self.builder.push_block(None);
        self.builder
            .set_terminator(entry_block, Terminator::Goto { target: loop_head });

        let break_place = self
            .builder
            .push_local_decl(LocalDecl::new(Some("break".into()), block.ty.clone()));
        self.break_resolver.push_scope(break_place.clone());
        let continue_place = self
            .builder
            .push_local_decl(LocalDecl::new(Some("continue".into()), block.ty.clone()));
        self.continue_resolver.push_scope(continue_place);

        let (loop_tail, _) = self.lower_block(loop_head, &block.stmts, &block.expr);
        let end_head = self.builder.push_block(None);

        // Set terminator to tail of loop body.
        self.builder
            .set_terminator(loop_tail, Terminator::Goto { target: loop_head });

        // Set terminator <continue> -> <loop_head>
        let continue_from = self.continue_resolver.pop_scope();
        for target in continue_from {
            self.builder
                .set_terminator(target, Terminator::Goto { target: loop_head });
        }

        // Set terminator <break> -> <end_head>
        let break_from = self.break_resolver.pop_scope();
        for target in break_from {
            self.builder
                .set_terminator(target, Terminator::Goto { target: end_head });
        }

        (end_head, Operand::Copy(break_place))
    }

    fn lower_expr_break(
        &mut self,
        entry_block: BlockId,
        expr: &Option<Box<thir::Expr>>,
        _ty: ty::Ty,
    ) -> (BlockId, Operand) {
        // Expression in break expression is still ignored for now.
        let (block, operand) = match expr {
            Some(expr) => self.lower_expr(entry_block, expr.as_ref()),
            None => (entry_block, Operand::Constant(Box::new(Constant::UNIT))),
        };

        // assign value of expression.
        let place = self.break_resolver.get_place();
        let rvalue = RValue::Use(operand);
        let stmt = Statement::Assign(Box::new((place.clone(), rvalue)));
        self.builder.push_stmt(block, stmt);

        self.break_resolver.push_late_resolved(block);

        (block, Operand::Constant(Box::new(Constant::UNIT)))
    }

    fn lower_expr_continue(
        &mut self,
        entry_block: BlockId,
        expr: &Option<Box<thir::Expr>>,
        _ty: ty::Ty,
    ) -> (BlockId, Operand) {
        // Expression in break expression is still ignored for now.
        let (block, operand) = match expr {
            Some(expr) => self.lower_expr(entry_block, expr.as_ref()),
            None => (entry_block, Operand::Constant(Box::new(Constant::UNIT))),
        };

        // assign value of expression.
        let place = self.continue_resolver.get_place();
        let rvalue = RValue::Use(operand);
        let stmt = Statement::Assign(Box::new((place.clone(), rvalue)));
        self.builder.push_stmt(block, stmt);

        self.continue_resolver.push_late_resolved(block);

        (block, Operand::Constant(Box::new(Constant::UNIT)))
    }

    fn lower_expr_return(
        &mut self,
        entry_block: BlockId,
        expr: &Option<Box<thir::Expr>>,
    ) -> (BlockId, Operand) {
        // Expression in return expression is still ignored for now.
        let (block, operand) = match expr {
            Some(expr) => self.lower_expr(entry_block, expr.as_ref()),
            None => (entry_block, Operand::Constant(Box::new(Constant::UNIT))),
        };

        self.return_resolver.push_late_resolved(block);

        // assign value of expression.
        let place = self.return_resolver.get_place();
        let rvalue = RValue::Use(operand);
        let stmt = Statement::Assign(Box::new((place.clone(), rvalue)));
        self.builder.push_stmt(block, stmt);

        (block, Operand::Constant(Box::new(Constant::UNIT)))
    }

    fn lower_expr_assign(
        &mut self,
        entry_block: BlockId,
        lhs: &thir::Expr,
        rhs: &thir::Expr,
        _ty: ty::Ty,
    ) -> (BlockId, Operand) {
        let (block, rhs) = self.lower_expr(entry_block, rhs);

        match lhs {
            thir::Expr::VarRef { res, ty: _ } => {
                let place = self
                    .local_def
                    .get(&res.def)
                    .expect("error: cannot found place of given def")
                    .clone();
                let rvalue = RValue::Use(rhs);
                let stmt = Statement::Assign(Box::new((place, rvalue)));
                self.builder.push_stmt(block, stmt);
            }
            _ => unreachable!(),
        }

        (block, Operand::Constant(Box::new(Constant::UNIT)))
    }

    fn lower_expr_lit(&mut self, lit: &thir::Lit, ty: ty::Ty) -> Operand {
        match &lit {
            thir::Lit::Int(thir::LitInt { value }) => {
                let scalar = ConstValue::Scalar(ScalarInt {
                    data: *value,
                    size: 32,
                });
                let constant = Constant {
                    ty,
                    literal: scalar,
                };

                Operand::Constant(Box::new(constant))
            }
            thir::Lit::Bool { value } => {
                let constant = match value {
                    true => Constant::TRUE,
                    false => Constant::FALSE,
                };

                Operand::Constant(Box::new(constant))
            }
        }
    }

    fn lower_expr_var_ref(&mut self, def: DefId, ty: ty::Ty) -> Operand {
        if let Some(local) = self.local_def.get(&def) {
            return Operand::Copy(local.clone());
        }

        if let ty::TyKind::FnDef(_) = ty.kind {
            return Operand::Constant(Box::new(Constant {
                ty,
                literal: ConstValue::Scalar(ScalarInt::ZST),
            }));
        }

        panic!("cannot lowering VarRef of {{def: {}, ty: {:?}}}", def, &ty);
    }

    fn push_local(&mut self, res: DefId, name: Option<Symbol>, ty: ty::Ty) -> Place {
        let name_string = name.map(|s| self.symbol_map.get(s).to_string());
        let decl = LocalDecl::new(name_string, ty);
        let place = self.builder.push_local_decl(decl);

        self.local_def.insert(res, place.clone());

        place
    }

    fn push_temp(&mut self, ty: ty::Ty) -> Place {
        let decl = LocalDecl::new(None, ty);
        self.builder.push_local_decl(decl)
    }
}
