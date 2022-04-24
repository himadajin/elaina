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
    pub output: String,
    pub indent: usize,
}

impl Printer for HIRPrinter<'_> {
    const INDENT_SIZE: usize = 4;

    type Output = String;

    fn finish(self) -> Self::Output {
        self.output
    }

    fn get_output_mut(&mut self) -> &mut Self::Output {
        &mut self.output
    }

    fn get_indent_mut(&mut self) -> &mut usize {
        &mut self.indent
    }
}

impl<'a> HIRPrinter<'a> {
    fn new(map: &'a SymbolMap<'a>) -> Self {
        HIRPrinter {
            map,
            output: String::new(),
            indent: 0,
        }
    }

    fn print_pat(&mut self, pat: &Pat) {
        match pat.kind {
            PatKind::Binding { res, name } => {
                self.print_ident(res, name);
            }
        }
    }

    fn print_ident(&mut self, res: Res, name: Symbol) {
        let name = self.map.get(name);
        self.print(name);
        self.with_delim(Delim::Paren, false, |this| {
            this.print_def(&res.def);
        });
    }

    fn print_def(&mut self, def: &DefId) {
        self.print("%");
        self.print(def.to_string());
    }

    fn print_ty(&mut self, ty: &ty::Ty) {
        match &ty.kind {
            ty::TyKind::Path(path) => {
                let name = self.map.get(path.ident.name);
                self.print(name);
            }
        }
    }
}
