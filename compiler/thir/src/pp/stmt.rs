use crate::*;

use super::THIRPrinter;

use printer::{Delim, Printer};

impl THIRPrinter<'_> {
    pub fn print_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Local { pat, init } => {
                self.print_local(pat, init);
            }
            Stmt::Expr(e) => {
                self.print_expr(e);
            }
            Stmt::Println(e) => {
                self.print("println");
                self.with_delim(Delim::Paren, false, |this| {
                    this.print_expr(e);
                });
                self.semi();
            }
        }
    }

    fn print_local(&mut self, pat: &Pat, init: &Expr) {
        self.print_space("let");
        self.print_pat(pat);
        self.space();
        self.eq();
        self.space();
        self.print_expr(init);
        self.semi();
    }
}
