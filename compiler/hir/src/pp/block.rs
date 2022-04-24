use crate::*;

use super::HIRPrinter;

use printer::{Delim, Printer};

impl HIRPrinter<'_> {
    pub fn print_block(&mut self, block: &Block) {
        self.with_delim(Delim::Brace, true, |this| {
            this.lines(block.stmts.iter(), |this, stmt| {
                this.print_stmt(stmt);
            });

            if let Some(e) = &block.expr {
                this.newline();
                this.print_expr(e);
            }
        });
    }
}
