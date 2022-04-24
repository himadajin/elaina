use super::HIRPrinter;
use crate::*;

use printer::{Delim, Printer};
impl HIRPrinter<'_> {
    pub fn print_stmt(&mut self, stmt: &Stmt) {
        match &stmt {
            Stmt::Local { pat, ty, init } => {
                self.print_local(pat, ty, init);
            }
            Stmt::Expr(e) => {
                self.print_expr(e);
            }
            Stmt::Semi(e) => {
                self.print_expr(e);
                self.semi();
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

    fn print_local(&mut self, pat: &Pat, ty: &Option<ty::Ty>, init: &Expr) {
        self.print_space("let");
        self.print_pat(pat);

        if let Some(ty) = ty {
            self.colon();
            self.print_ty(ty);
        }

        self.space();
        self.eq();
        self.space();

        self.print_expr(init);
        self.semi();
    }
}
