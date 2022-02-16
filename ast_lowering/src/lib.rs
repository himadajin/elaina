use ast::{
    expr,
    lit::{self, LitInt},
};
use index::*;
use ir::*;

pub struct LoweringContext {
    locals: IndexVec<LocalDecl>,
    stmts: Vec<Statement>,
}

#[allow(dead_code)]
impl LoweringContext {
    pub fn new() -> Self {
        LoweringContext {
            locals: IndexVec::new(),
            stmts: Vec::new(),
        }
    }

    pub fn build(self) -> Body {
        Body {
            stmts: self.stmts,
            local_decls: self.locals,
        }
    }

    pub fn lower_expr(&mut self, expr: &expr::Expr) -> Idx<LocalDecl> {
        match expr {
            expr::Expr::Binary(expr) => self.lower_expr_binary(expr),
            expr::Expr::Unary(_) => todo!(),
            expr::Expr::Lit(lit) => self.lower_expr_lit(lit),
        }
    }

    fn lower_expr_binary(&mut self, expr: &expr::ExprBinary) -> Idx<LocalDecl> {
        let lhs = self.lower_expr(&expr.left);
        let rhs = self.lower_expr(&expr.right);

        let operand_lhs = Operand::Copy(Place::new(lhs));
        let operand_rhs = Operand::Copy(Place::new(rhs));

        let op = match expr.op {
            ast::op::BinOp::Add => BinOp::Add,
            ast::op::BinOp::Sub => BinOp::Sub,
            ast::op::BinOp::Mul => BinOp::Mul,
            ast::op::BinOp::Div => BinOp::Div,
        };

        let rvalue = RValue::BinaryOp(op, Box::new((operand_lhs, operand_rhs)));
        let idx = self.push_unnamed_local();
        let place = Place::new(idx.clone());
        let statement = Statement::Assign(Box::new((place, rvalue)));
        self.stmts.push(statement);

        idx
    }

    fn lower_expr_lit(&mut self, expr: &expr::ExprLit) -> Idx<LocalDecl> {
        match &expr.lit {
            lit::Lit::Int(LitInt { digits }) => {
                let data: u128 = digits.parse().unwrap();
                let constant = Constant::Scalar(ScalarInt {
                    data: data,
                    size: 32,
                });
                let operand = Operand::Constant(Box::new(constant));
                let idx = self.push_unnamed_local();
                let place = Place::new(idx.clone());
                let rvalue = RValue::Use(operand);
                let statement = Statement::Assign(Box::new((place, rvalue)));

                self.stmts.push(statement);

                idx
            }
        }
    }

    fn push_unnamed_local(&mut self) -> Idx<LocalDecl> {
        let local_decl = LocalDecl::unnamed();
        let idx = self.locals.push(local_decl);

        idx
    }
}
