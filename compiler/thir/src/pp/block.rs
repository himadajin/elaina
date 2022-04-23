use crate::*;

use super::THIRPrinter;

use printer::Delim;
impl THIRPrinter<'_> {
    pub fn print_block(&mut self, block: &Block) {
        self.p.begin(Delim::Brace);

        let mut stmts = block.stmts.as_slice();
        while let [stmt, tail @ ..] = stmts {
            self.print_stmt(stmt);

            if tail.is_empty() {
                if block.expr.is_some() {
                    self.p.new_line();
                }
                break;
            }

            self.p.new_line();
            stmts = &stmts[1..];
        }

        if let Some(e) = &block.expr {
            self.print_expr(e);
        }

        self.p.end(Delim::Brace);
    }
}
