use ir::{constant::*, stmt::*, *};
use thir;

use std::collections::HashMap;

#[allow(dead_code)]
pub struct LoweringContext {
    body: Body,

    block_at: BlockId,

    local_name_table: HashMap<String, Place>,
}

impl LoweringContext {
    pub fn new() -> Self {
        LoweringContext {
            body: Body::new(),
            block_at: BlockId::dummy(),
            local_name_table: HashMap::new(),
        }
    }

    pub fn build(self) -> Body {
        self.body
    }

    pub fn lower_main_block(&mut self, block: &thir::Block) {
        self.block_at = self.body.blocks.push_and_get_key(Block::new());

        for stmt in &block.stmts {
            self.lower_stmt(stmt);
        }
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
            thir::Stmt::Expr(expr) => {
                // the value that generated by lowering expr is assign to local of return value
                let place = self.body.local_return();

                let rvalue = {
                    let operand = self.lower_expr(expr);
                    RValue::Use(operand)
                };

                let stmt = Statement::Assign(Box::new((place, rvalue)));
                self.push_stmt(stmt);
            }
            thir::Stmt::Semi(_) => todo!(),
        }
    }

    fn lower_expr(&mut self, expr: &thir::Expr) -> Operand {
        match expr {
            thir::Expr::Binary { op, lhs, rhs, ty } => self.lower_expr_binary(*op, lhs, rhs, *ty),
            thir::Expr::Unary { .. } => todo!(),
            thir::Expr::Lit { lit, ty } => self.lower_expr_lit(lit, *ty),
            thir::Expr::Ident { ident, ty } => self.lower_expr_ident(ident, *ty),
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
        };

        let lhs = self.lower_expr(lhs);
        let rhs = self.lower_expr(rhs);

        let rvalue = RValue::BinaryOp(op, Box::new((lhs, rhs)));
        let place = self.push_local(None, ty);
        let stmt = Statement::Assign(Box::new((place.clone(), rvalue)));

        self.push_stmt(stmt);

        Operand::Copy(place)
    }

    fn lower_expr_lit(&mut self, lit: &thir::Lit, _ty: ty::Ty) -> Operand {
        match &lit {
            thir::Lit::Int(thir::LitInt { value }) => {
                let scalar = Constant::Scalar(ScalarInt {
                    data: *value,
                    size: 32,
                });
                Operand::Constant(Box::new(scalar))
            }
        }
    }

    fn lower_expr_ident(&mut self, ident: &String, _ty: ty::Ty) -> Operand {
        let local = self.local_name_table.get(ident).unwrap().clone();
        Operand::Copy(local)
    }

    fn push_local(&mut self, name: Option<String>, ty: ty::Ty) -> Place {
        let local_decl = LocalDecl::new(name.clone(), ty);
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
}
