use crate::*;

use super::THIRPrinter;

use printer::Delim;

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
                self.p.word("println");
                self.p.popen(Delim::Paren);
                self.print_expr(e);
                self.p.pclose(Delim::Paren);
                self.p.word(";");
            }
        }
    }

    fn print_local(&mut self, pat: &Pat, init: &Expr) {
        self.p.word("let ");
        self.print_pat(pat);
        self.p.word(" = ");
        self.print_expr(init);
        self.p.word(";");
    }
}
