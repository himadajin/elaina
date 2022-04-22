mod block;
mod expr;
mod stmt;

use crate::*;

use printer::*;
use span::symbol::*;

pub fn print_block(map: &SymbolMap, block: &Block) -> String {
    let mut p = HIRPrinter::new(map);
    p.print_block(block);
    p.finish()
}

struct HIRPrinter<'a> {
    pub map: &'a SymbolMap<'a>,
    pub p: Printer,
}

impl<'a> HIRPrinter<'a> {
    fn new(map: &'a SymbolMap<'a>) -> Self {
        HIRPrinter {
            map,
            p: Printer::new(),
        }
    }

    fn finish(self) -> String {
        self.p.finish()
    }

    fn print_pat(&mut self, pat: &Pat) {
        match pat.kind {
            PatKind::Binding { res, name } => {
                let name = self.map.get(name).to_string();
                self.p.word(name);
                self.p.popen(Delim::Paren);
                self.print_def(&res);
                self.p.pclose(Delim::Paren);
            }
        }
    }

    fn print_def(&mut self, def: &DefId) {
        self.p.word("%");
        self.p.word(def.to_string())
    }

    fn print_ty(&mut self, ty: &ty::Ty) {
        match &ty.kind {
            ty::TyKind::Path(path) => {
                let name = self.map.get(path.ident.name).to_string();
                self.p.word(name);
            }
        }
    }
}
