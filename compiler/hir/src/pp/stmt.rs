use super::HIRPrinter;
use crate::*;

use printer::Delim;
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
                self.p.word(";");
            }
            Stmt::Println(e) => {
                self.p.word("println");
                self.p.popen(Delim::Paren);
                self.print_expr(e);
                self.p.pclose(Delim::Paren);
                self.p.word(";");
            }
        }
    }

    fn print_local(&mut self, pat: &Pat, ty: &Option<ty::Ty>, init: &Expr) {
        self.p.word("let ");
        self.print_pat(pat);
        if let Some(ty) = ty {
            self.p.word(": ");
            self.print_ty(ty)
        }
        self.p.space();
        self.p.word("= ");
        self.print_expr(init);
        self.p.word(";");
    }
}
