use crate::*;

use super::THIRPrinter;

use printer::{Delim, Printer};
impl THIRPrinter<'_> {
    pub fn print_block(&mut self, block: &Block) {
        self.with_delim(Delim::Brace, true, |this| {
            this.lines(block.stmts.iter(), |this, stmt| {
                this.print_stmt(stmt);
            });

            if let Some(e) = &block.expr {
                if !block.stmts.is_empty() {
                    this.newline();
                }
                this.print_expr(e);
            }
        });
    }
}
