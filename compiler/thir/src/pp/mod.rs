use crate::*;

pub mod block;
pub mod expr;
pub mod item;
pub mod stmt;

use printer::*;
use span::*;

pub fn print_items(map: &SymbolMap, items: &[Item]) -> String {
    let mut p = THIRPrinter::new(map);
    p.print_items(items);

    p.finish()
}

struct THIRPrinter<'a> {
    pub map: &'a SymbolMap<'a>,
    pub p: Printer,
}

impl<'a> THIRPrinter<'a> {
    fn new(map: &'a SymbolMap<'a>) -> Self {
        THIRPrinter {
            map,
            p: Printer::new(),
        }
    }

    fn finish(self) -> String {
        self.p.finish()
    }

    fn print_with_ty<F>(&mut self, ty: &ty::Ty, f: F)
    where
        F: FnOnce(&mut THIRPrinter),
    {
        self.p.popen(Delim::Paren);
        f(self);
        self.p.word(":");
        self.print_ty(ty);
        self.p.pclose(Delim::Paren);
    }

    fn print_pat(&mut self, pat: &Pat) {
        match pat.kind.as_ref() {
            PatKind::Binding { res, name, ty } => {
                self.print_ident(*res, *name);
                self.p.word(":");
                self.print_ty(ty);
            }
        }
    }

    fn print_ident(&mut self, res: DefId, name: Symbol) {
        let name = self.map.get(name);
        self.p.word(name);
        self.p.popen(Delim::Paren);
        self.print_def(res);
        self.p.pclose(Delim::Paren);
    }

    fn print_def(&mut self, def: DefId) {
        self.p.word("%");
        self.p.word(def.to_string());
    }

    fn print_ty(&mut self, ty: &ty::Ty) {
        match &ty.kind {
            ty::TyKind::Bool => self.p.word("bool"),
            ty::TyKind::Int(ty) => match ty {
                ty::IntTy::I32 => self.p.word("i32"),
            },
            ty::TyKind::Tuple(tys) => {
                self.p.popen(Delim::Paren);

                let mut tys = tys.as_slice();
                while let [ty, tail @ ..] = tys {
                    self.print_ty(ty);

                    if !tail.is_empty() {
                        self.p.word(", ");
                    }

                    tys = &tys[1..];
                }

                self.p.pclose(Delim::Paren);
            }
            ty::TyKind::Fn(ty) => {
                self.p.word("fn");
                self.p.popen(Delim::Paren);

                let mut inputs = ty.inputs.as_slice();
                while let [ty, tail @ ..] = inputs {
                    self.print_ty(ty);

                    if !tail.is_empty() {
                        self.p.word(", ");
                    }

                    inputs = &inputs[1..];
                }

                self.p.pclose(Delim::Paren);

                if let Some(output) = ty.output.as_ref() {
                    self.p.word(" -> ");
                    self.print_ty(output);
                }
            }
            ty::TyKind::Never => self.p.word("!"),
        }
    }
}
