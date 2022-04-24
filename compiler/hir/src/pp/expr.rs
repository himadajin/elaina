use super::HIRPrinter;

use crate::*;
use ast::op::{BinOp, Fixity, UnOp};
use printer::{Delim, Printer};

impl HIRPrinter<'_> {
    pub fn print_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Binary { op, lhs, rhs } => {
                self.print_expr_binary(op, lhs, rhs);
            }
            Expr::Unary { op, expr } => {
                self.print_expr_unary(op, expr);
            }
            Expr::If {
                cond,
                then,
                else_opt,
            } => {
                self.print_expr_if(cond, then, else_opt.as_deref());
            }
            Expr::Loop { block } => {
                self.print_space("loop");

                self.print_block(block);
            }
            Expr::Break { expr } => {
                self.print("break");
                if let Some(expr) = expr {
                    self.space();
                    self.print_expr(expr);
                }
            }
            Expr::Continue { expr } => {
                self.print("continue");
                if let Some(expr) = expr {
                    self.space();
                    self.print_expr(expr);
                }
            }
            Expr::Block { block } => {
                self.print_block(block);
            }
            Expr::Assign { lhs, rhs } => {
                let prec = crate::PREC_ASSIGN;
                self.print_expr_maybe_paren(lhs, prec + 1);
                self.space();
                self.eq();
                self.print_expr_maybe_paren(rhs, prec);
            }
            Expr::Lit { lit, .. } => self.print_lit(lit),
            Expr::Path { path } => {
                self.print_def(&path.res.def);
            }
        }
    }

    fn print_lit(&mut self, lit: &Lit) {
        match lit {
            Lit::Bool { value } => self.print(value.to_string()),
            Lit::Int(l) => self.print(l.value.to_string()),
        }
    }

    fn print_expr_maybe_paren(&mut self, expr: &Expr, prec: i8) {
        self.print_expr_cond_paren(expr, expr.precedence() < prec);
    }

    fn print_expr_cond_paren(&mut self, expr: &Expr, needs_par: bool) {
        if needs_par {
            self.with_delim(Delim::Paren, false, |this| {
                this.print_expr(expr);
            });
        } else {
            self.print_expr(expr);
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
        self.space_print_space(op.to_string());
        self.print_expr_maybe_paren(rhs, right_prec);
    }

    fn print_expr_unary(&mut self, op: &UnOp, expr: &Expr) {
        self.print(op.to_string());
        self.print_expr_maybe_paren(expr, crate::PREC_PREFIX);
    }

    fn print_expr_if(&mut self, cond: &Expr, then: &Block, else_opt: Option<&Expr>) {
        self.print_space("if");
        self.print_expr_cond_paren(cond, false);
        self.space();
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
                } => {
                    self.space_print_space("else if");
                    self.print_expr_cond_paren(cond, false);
                    self.space();
                    self.print_block(then);
                    self.print_else(else_opt.as_deref());
                }
                Expr::Block { block } => {
                    self.space_print_space("else");
                    self.print_block(block);
                }
                _ => {
                    panic!("print_if saw if with weird alternative");
                }
            }
        }
    }
}
