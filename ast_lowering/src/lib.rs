use ast::{
    expr,
    lit::{self, LitInt},
};
use index::*;
use ir::*;

#[allow(dead_code)]
struct LoweringContext {
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

    pub fn lower_expr(&mut self, expr: &expr::Expr) -> Idx<LocalDecl> {
        match expr {
            expr::Expr::Binary(_) => todo!(),
            expr::Expr::Unary(_) => todo!(),
            expr::Expr::Lit(lit) => self.lower_expr_lit(lit),
        }
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
                let local_decl = LocalDecl::new_anonymous();
                let idx = self.locals.push(local_decl);
                let place = Place {
                    local: idx.clone(),
                };
                let rvalue = RValue::Use(operand);
                let statement = Statement::Assign(Box::new((place, rvalue)));

                self.stmts.push(statement);

                idx
            }
        }
    }
}
