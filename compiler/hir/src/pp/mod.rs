mod block;
mod expr;
mod item;
mod stmt;

use crate::*;

use printer::*;
use span::*;

pub fn print_items(map: &SymbolMap, items: &[Item]) -> String {
    let mut p = HIRPrinter::new(map);

    p.print_items(items);

    p.finish()
}

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
                self.print_ident(res, name);
            }
        }
    }

    fn print_ident(&mut self, res: DefId, name: Symbol) {
        let name = self.map.get(name);
        self.p.word(name);
        self.p.popen(Delim::Paren);
        self.print_def(&res);
        self.p.pclose(Delim::Paren);
    }

    fn print_def(&mut self, def: &DefId) {
        self.p.word("%");
        self.p.word(def.to_string())
    }

    fn print_ty(&mut self, ty: &ty::Ty) {
        match &ty.kind {
            ty::TyKind::Path(path) => {
                let name = self.map.get(path.ident.name);
                self.p.word(name);
            }
        }
    }
}
