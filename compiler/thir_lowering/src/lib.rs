mod builder;

use builder::MirBuilder;
use mir::{constant::*, stmt::*, terminator::*, *};
use span::symbol::{Symbol, SymbolMap};
use thir::{self};

use std::collections::HashMap;

struct ControlFlowResolution {
    heads: Vec<BlockId>,
    tails: Vec<BlockId>,
}

#[allow(dead_code)]
struct ControlFlowResolver {
    scopes: Vec<ControlFlowResolution>,
}

#[allow(dead_code)]
impl ControlFlowResolver {
    fn new() -> Self {
        Self { scopes: Vec::new() }
    }

    fn push_scope(&mut self) {
        let res = ControlFlowResolution {
            heads: Vec::new(),
            tails: Vec::new(),
        };

        self.scopes.push(res);
    }

    fn pop_scope(&mut self) -> ControlFlowResolution {
        self.scopes
            .pop()
            .expect("error: Tried to pop scope even though there is no scope to resolve")
    }

    fn push_head(&mut self, block: BlockId) {
        self.scopes
            .last_mut()
            .expect("error: Tried to pop scope even though there is no scope to resolve")
            .heads
            .push(block);
    }

    fn push_tail(&mut self, block: BlockId) {
        self.scopes
            .last_mut()
            .expect("error: Tried to pop scope even though there is no scope to resolve")
            .tails
            .push(block);
    }
}

#[allow(dead_code)]
pub struct LoweringContext<'a> {
    builder: MirBuilder,

    loop_resolver: ControlFlowResolver,

    local_name_table: HashMap<Symbol, Place>,
    symbol_map: &'a SymbolMap<'a>,
}

impl<'a> LoweringContext<'a> {
    pub fn new(symbol_map: &'a SymbolMap<'a>) -> Self {
        LoweringContext {
            builder: MirBuilder::new(),

            loop_resolver: ControlFlowResolver::new(),

            local_name_table: HashMap::new(),
            symbol_map: symbol_map,
        }
    }

    pub fn build(self) -> Body {
        self.builder.build()
    }

    pub fn lower_main_block(&mut self, block: &thir::Block) {
        let entry = self.builder.push_block(None);

        let (tail, _) = self.lower_block(entry, &block.stmts, &block.expr);

        let return_block = self.builder.push_block(Some(Terminator::Return));
        self.builder.set_terminator(
            tail,
            Terminator::Goto {
                target: return_block,
            },
        );
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
            thir::Stmt::Local { ident, init } => {
                let place = self.push_local(Some(ident.clone()), init.ty());
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
            thir::Stmt::Semi(e) => {
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
            thir::Expr::Ident { ident, ty } => {
                (entry_block, self.lower_expr_ident(ident, ty.clone()))
            }
        }
    }

    fn lower_expr_binary(
        &mut self,
        entry_block: BlockId,
        op: thir::BinOp,
        lhs: &thir::Expr,
        rhs: &thir::Expr,
        ty: ty::Ty,
    ) -> (BlockId, Operand) {
        let op = match op {
            thir::BinOp::Add => BinOp::Add,
            thir::BinOp::Sub => BinOp::Sub,
            thir::BinOp::Mul => BinOp::Mul,
            thir::BinOp::Div => BinOp::Div,
            thir::BinOp::Eq => BinOp::Eq,
            thir::BinOp::Lt => BinOp::Lt,
            thir::BinOp::Le => BinOp::Le,
            thir::BinOp::Ne => BinOp::Ne,
            thir::BinOp::Ge => BinOp::Ge,
            thir::BinOp::Gt => BinOp::Gt,
        };

        let (tail, lhs) = self.lower_expr(entry_block, lhs);
        let (tail, rhs) = self.lower_expr(tail, rhs);

        let rvalue = RValue::BinaryOp(op, Box::new((lhs, rhs)));
        let place = self.push_local(None, ty);
        let stmt = Statement::Assign(Box::new((place.clone(), rvalue)));

        self.builder.push_stmt(tail, stmt);

        (tail, Operand::Copy(place))
    }

    fn lower_expr_unary(
        &mut self,
        entry_block: BlockId,
        op: thir::UnOp,
        expr: &thir::Expr,
        ty: ty::Ty,
    ) -> (BlockId, Operand) {
        let op = match op {
            thir::UnOp::Neg => UnOp::Neg,
        };

        let (tail, expr) = self.lower_expr(entry_block, expr);
        let rvalue = RValue::UnaryOp(op, Box::new(expr));
        let place = self.push_local(None, ty);
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
            Some(self.push_local(None, ty.clone()))
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
                    targets: targets,
                }
            };

            Terminator::SwitchInt {
                discr: cond_operand,
                switch_ty: ty::Ty {
                    kind: ty::TyKind::Bool,
                },
                targets: targets,
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
        self.loop_resolver.push_scope();

        let (loop_tail, _) = self.lower_block(loop_head, &block.stmts, &block.expr);
        let end_head = self.builder.push_block(None);

        // Set terminator to tail of loop body.
        self.builder
            .set_terminator(loop_tail, Terminator::Goto { target: loop_head });

        let res = self.loop_resolver.pop_scope();

        // Set terminator <resolved> -> <loop_head>
        for resolved in res.heads {
            self.builder
                .set_terminator(resolved, Terminator::Goto { target: loop_head });
        }

        // Set terminator <resolved> -> <end_head>
        for resolved in res.tails {
            self.builder
                .set_terminator(resolved, Terminator::Goto { target: end_head });
        }

        (end_head, Operand::Constant(Box::new(Constant::UNIT)))
    }

    fn lower_expr_break(
        &mut self,
        entry_block: BlockId,
        expr: &Option<Box<thir::Expr>>,
        _ty: ty::Ty,
    ) -> (BlockId, Operand) {
        // Expression in break expression is still ignored for now.
        let (block, _) = match expr {
            Some(expr) => self.lower_expr(entry_block, expr.as_ref()),
            None => (entry_block, Operand::Constant(Box::new(Constant::UNIT))),
        };

        self.loop_resolver.push_tail(block);

        (block, Operand::Constant(Box::new(Constant::UNIT)))
    }

    fn lower_expr_continue(
        &mut self,
        entry_block: BlockId,
        expr: &Option<Box<thir::Expr>>,
        _ty: ty::Ty,
    ) -> (BlockId, Operand) {
        // Expression in break expression is still ignored for now.
        let (block, _) = match expr {
            Some(expr) => self.lower_expr(entry_block, expr.as_ref()),
            None => (entry_block, Operand::Constant(Box::new(Constant::UNIT))),
        };

        self.loop_resolver.push_head(block);

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
            thir::Expr::Ident { ident, .. } => {
                let place = self
                    .local_name_table
                    .get(ident)
                    .expect("error: cannot found place of given ident")
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
                    ty: ty,
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

    fn lower_expr_ident(&mut self, ident: &Symbol, _ty: ty::Ty) -> Operand {
        let local = self.local_name_table.get(ident).unwrap().clone();
        Operand::Copy(local)
    }

    fn push_local(&mut self, name: Option<Symbol>, ty: ty::Ty) -> Place {
        let name_string = name.map(|s| self.symbol_map.get(s).to_string());
        let decl = LocalDecl::new(name_string, ty);
        let place = self.builder.push_local_decl(decl);

        if let Some(name) = name {
            self.local_name_table.insert(name, place.clone());
        }

        place
    }
}
