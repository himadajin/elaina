use mir::{constant::*, stmt::*, terminator::*, *};
use span::symbol::{Symbol, SymbolMap};
use thir::{self};

use std::collections::HashMap;

#[allow(dead_code)]
pub struct LoweringContext<'a> {
    body: Body,

    block_at: BlockId,

    local_name_table: HashMap<Symbol, Place>,
    symbol_map: &'a SymbolMap<'a>,
}

impl<'a> LoweringContext<'a> {
    pub fn new(symbol_map: &'a SymbolMap<'a>) -> Self {
        LoweringContext {
            body: Body::new(),
            block_at: BlockId::dummy(),
            local_name_table: HashMap::new(),
            symbol_map: symbol_map,
        }
    }

    pub fn build(self) -> Body {
        self.body
    }

    pub fn lower_main_block(&mut self, block: &thir::Block) {
        self.lower_block(block);

        let prev_block = self.block_at;
        let return_block = self.push_block(Some(Terminator::Return));
        self.set_terminator(
            prev_block,
            Terminator::Goto {
                target: return_block,
            },
        );
    }

    fn lower_block(&mut self, block: &thir::Block) -> (BlockId, Operand) {
        let id = self.push_block(None);

        for stmt in &block.stmts {
            self.lower_stmt(stmt);
        }

        let operand = match &block.expr {
            Some(e) => self.lower_expr(e),
            None => Operand::Constant(Box::new(Constant::UNIT)),
        };

        (id, operand)
    }

    fn lower_stmt(&mut self, stmt: &thir::Stmt) {
        match stmt {
            thir::Stmt::Local { ident, init } => {
                let place = self.push_local(Some(ident.clone()), init.ty());
                let rvalue = {
                    let operand = self.lower_expr(init);
                    RValue::Use(operand)
                };
                let stmt = Statement::Assign(Box::new((place, rvalue)));
                self.push_stmt(stmt);
            }
            thir::Stmt::Expr(e) => {
                self.lower_expr(e);
            }
            thir::Stmt::Semi(_) => {
                // Do nothing because this statement does not have any side effect at this stage.
                // So there is no need to compile it.
                ()
            }
            thir::Stmt::Println(expr) => {
                let operand = self.lower_expr(expr);
                let stmt = Statement::Println(operand);
                self.push_stmt(stmt);
            }
        }
    }

    fn lower_expr(&mut self, expr: &thir::Expr) -> Operand {
        match expr {
            thir::Expr::Binary { op, lhs, rhs, ty } => {
                self.lower_expr_binary(*op, lhs, rhs, ty.clone())
            }
            thir::Expr::Unary { op, expr, ty } => self.lower_expr_unary(*op, expr, ty.clone()),
            thir::Expr::If {
                cond,
                then,
                else_opt,
                ty,
            } => self.lower_expr_if(cond.as_ref(), then.as_ref(), else_opt, ty.clone()),
            thir::Expr::Block { block: _block } => todo!(),
            thir::Expr::Lit { lit, ty } => self.lower_expr_lit(lit, ty.clone()),
            thir::Expr::Ident { ident, ty } => self.lower_expr_ident(ident, ty.clone()),
        }
    }

    fn lower_expr_binary(
        &mut self,
        op: thir::BinOp,
        lhs: &thir::Expr,
        rhs: &thir::Expr,
        ty: ty::Ty,
    ) -> Operand {
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

        let lhs = self.lower_expr(lhs);
        let rhs = self.lower_expr(rhs);

        let rvalue = RValue::BinaryOp(op, Box::new((lhs, rhs)));
        let place = self.push_local(None, ty);
        let stmt = Statement::Assign(Box::new((place.clone(), rvalue)));

        self.push_stmt(stmt);

        Operand::Copy(place)
    }

    fn lower_expr_unary(&mut self, op: thir::UnOp, expr: &thir::Expr, ty: ty::Ty) -> Operand {
        let op = match op {
            thir::UnOp::Neg => UnOp::Neg,
        };

        let expr = self.lower_expr(expr);
        let rvalue = RValue::UnaryOp(op, Box::new(expr));
        let place = self.push_local(None, ty);
        let stmt = Statement::Assign(Box::new((place.clone(), rvalue)));

        self.push_stmt(stmt);

        Operand::Copy(place)
    }

    fn lower_expr_if(
        &mut self,
        cond: &thir::Expr,
        then: &thir::Block,
        else_opt: &Option<Box<thir::Expr>>,
        ty: ty::Ty,
    ) -> Operand {
        // Create cond block that represents condition expression.
        // Current Block jumps to cond block.
        let entry_block = self.block_at;
        let cond_block = self.push_block(None);
        self.set_terminator(entry_block, Terminator::Goto { target: cond_block });
        let cond_operand = self.lower_expr(cond);

        // If `ty` is not ZST(Zero Size Type), create local and treat it as the value of the expression.
        let expr_val = if !ty.is_zst() {
            Some(self.push_local(None, ty.clone()))
        } else {
            None
        };

        // Create then block.
        let (then_block, then_operand) = self.lower_block(then);
        if let Some(p) = &expr_val {
            let rvalue = RValue::Use(then_operand);
            let stmt = Statement::Assign(Box::new((p.clone(), rvalue)));
            self.push_stmt(stmt);
        }

        // Create opt block if it exists.
        let else_block = match else_opt {
            Some(e) => {
                let else_block = self.push_block(None);
                let else_operand = self.lower_expr(e);

                if let Some(p) = &expr_val {
                    let rvalue = RValue::Use(else_operand);
                    let stmt = Statement::Assign(Box::new((p.clone(), rvalue)));
                    self.push_stmt(stmt);
                }

                Some(else_block)
            }
            None => None,
        };

        // Create end block.
        let end_block = self.push_block(None);

        // Create terminator: cond block -> then_block or end_block.
        let cond_terminator = {
            let targets = {
                // targets: [else, then]
                let targets = match else_block {
                    Some(else_block) => vec![else_block, then_block],
                    None => vec![end_block, then_block],
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
        self.set_terminator(cond_block, cond_terminator);

        // Create terminator: then_block -> end_block.
        self.set_terminator(then_block, Terminator::Goto { target: end_block });

        // Create terminator: else_block -> end_block.
        if let Some(else_block) = else_block {
            self.set_terminator(else_block, Terminator::Goto { target: end_block });
        }

        // If expr_val exists, the operand is assigned evaluated value,
        // otherwise it is unit.
        match expr_val {
            Some(p) => Operand::Copy(p),
            None => Operand::Constant(Box::new(Constant::UNIT)),
        }
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
        let local_decl = LocalDecl::new(name_string, ty);
        let id = self.body.local_decls.push_and_get_key(local_decl);
        let place = Place::new(id);

        if let Some(name) = name {
            self.local_name_table.insert(name, place.clone());
        }

        place
    }

    fn push_stmt(&mut self, stmt: Statement) {
        self.body.blocks[self.block_at].stmts.push(stmt);
    }

    fn push_block(&mut self, terminator: Option<Terminator>) -> BlockId {
        self.block_at = self.body.blocks.push_and_get_key(Block::new(terminator));
        self.block_at
    }

    fn set_terminator(&mut self, target: BlockId, terminator: Terminator) {
        self.body.blocks.get_mut(target).unwrap().terminator = Some(terminator);
    }
}
