use crate::*;

use super::THIRPrinter;

use ast::op::{BinOp, Fixity, UnOp};
use printer::Delim;

impl THIRPrinter<'_> {
    pub fn print_expr(&mut self, expr: &Expr) {
        let ty = expr.ty();
        self.print_with_ty(&ty, |this| match expr {
            Expr::Binary { op, lhs, rhs, .. } => {
                this.print_expr_binary(op, lhs, rhs);
            }
            Expr::Unary { op, expr, .. } => {
                this.print_expr_unary(op, expr);
            }
            Expr::If {
                cond,
                then,
                else_opt,
                ..
            } => {
                this.print_expr_if(cond, then, else_opt.as_deref());
            }
            Expr::Loop { block } => {
                this.p.word("loop ");
                this.print_block(block);
            }
            Expr::Break { expr, .. } => {
                this.p.word("break");
                if let Some(expr) = expr {
                    this.p.space();
                    this.print_expr(expr);
                }
            }
            Expr::Continue { expr, .. } => {
                this.p.word("continue");
                if let Some(expr) = expr {
                    this.p.space();
                    this.print_expr(expr);
                }
            }
            Expr::Block { block } => {
                this.print_block(block);
            }
            Expr::Assign { lhs, rhs, .. } => {
                let prec = crate::PREC_ASSIGN;
                this.print_expr_maybe_paren(lhs, prec + 1);
                this.p.space();
                this.p.word("= ");
                this.print_expr_maybe_paren(rhs, prec);
            }
            Expr::Lit { lit, .. } => this.print_lit(lit),
            Expr::VarRef { res, .. } => {
                this.print_def(res.def);
            }
        });
    }

    fn print_lit(&mut self, lit: &Lit) {
        match lit {
            Lit::Bool { value } => self.p.word(value.to_string()),
            Lit::Int(lit) => self.p.word(lit.value.to_string()),
        }
    }

    fn print_expr_maybe_paren(&mut self, expr: &Expr, prec: i8) {
        self.print_expr_cond_paren(expr, expr.precedence() < prec);
    }

    fn print_expr_cond_paren(&mut self, expr: &Expr, needs_par: bool) {
        if needs_par {
            self.p.popen(Delim::Paren);
        }
        self.print_expr(expr);
        if needs_par {
            self.p.pclose(Delim::Paren);
        }
    }

    fn print_expr_binary(&mut self, op: &BinOp, lhs: &Expr, rhs: &Expr) {
        let prec = op.precedence() as i8;
        let fixity = op.fixity();

        let (left_prec, right_prec) = match fixity {
            Fixity::Left => (prec, prec + 1),
            Fixity::Right => (prec + 1, prec),
            Fixity::None => (prec + 1, prec + 1),
        };

        self.print_expr_maybe_paren(lhs, left_prec);
        self.p.space();
        self.p.word(op.to_string());
        self.p.space();
        self.print_expr_maybe_paren(rhs, right_prec);
    }

    fn print_expr_unary(&mut self, op: &UnOp, expr: &Expr) {
        self.p.word(op.to_string());
        self.print_expr_maybe_paren(expr, crate::PREC_PREFIX);
    }

    fn print_expr_if(&mut self, cond: &Expr, then: &Block, else_opt: Option<&Expr>) {
        self.p.word("if ");
        self.print_expr_cond_paren(cond, false);
        self.p.space();
        self.print_block(then);
        self.print_else(else_opt);
    }

    fn print_else(&mut self, else_opt: Option<&Expr>) {
        if let Some(else_) = else_opt {
            match else_ {
                Expr::If {
                    cond,
                    then,
                    else_opt,
                    ty: _,
                } => {
                    self.p.word(" else if ");
                    self.print_expr_cond_paren(cond, false);
                    self.p.space();
                    self.print_block(then);
                    self.print_else(else_opt.as_deref());
                }
                Expr::Block { block } => {
                    self.p.word(" else ");
                    self.print_block(block);
                }
                _ => {
                    panic!("print_if saw if with weird alternative");
                }
            }
        }
    }
}
