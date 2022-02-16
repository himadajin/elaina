use ast::{
    expr,
    lit::{self, LitInt},
    stmt,
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

    pub fn lower_stmt(&mut self, stmt: &stmt::Stmt) {
        match stmt {
            stmt::Stmt::Expr(expr) => {
                let operand = self.lower_expr(expr);

                let rvalue = RValue::Use(operand);
                let place = self.push_unnamed_local();
                let statement = Statement::Assign(Box::new((place, rvalue)));
                self.stmts.push(statement);
            }
        }
    }

    pub fn lower_expr(&mut self, expr: &expr::Expr) -> Operand {
        match expr {
            expr::Expr::Binary(expr) => self.lower_expr_binary(expr),
            expr::Expr::Unary(_) => todo!(),
            expr::Expr::Lit(lit) => self.lower_expr_lit(lit),
        }
    }

    fn lower_expr_binary(&mut self, expr: &expr::ExprBinary) -> Operand {
        let lhs = self.lower_expr(&expr.lhs);
        let rhs = self.lower_expr(&expr.rhs);

        let op = match expr.op {
            ast::op::BinOp::Add => BinOp::Add,
            ast::op::BinOp::Sub => BinOp::Sub,
            ast::op::BinOp::Mul => BinOp::Mul,
            ast::op::BinOp::Div => BinOp::Div,
        };

        let rvalue = RValue::BinaryOp(op, Box::new((lhs, rhs)));
        let place = self.push_unnamed_local();
        let statement = Statement::Assign(Box::new((place.clone(), rvalue)));
        self.stmts.push(statement);

        Operand::Copy(place)
    }

    fn lower_expr_lit(&mut self, expr: &expr::ExprLit) -> Operand {
        match &expr.lit {
            lit::Lit::Int(LitInt { digits }) => {
                let data: u128 = digits.parse().unwrap();
                let constant = Constant::Scalar(ScalarInt {
                    data: data,
                    size: 32,
                });
                Operand::Constant(Box::new(constant))
            }
        }
    }

    fn push_unnamed_local(&mut self) -> Place {
        let local_decl = LocalDecl::unnamed();
        let idx = self.locals.push(local_decl);

        Place::new(idx)
    }
}
