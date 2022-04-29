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
    pub output: String,
    pub indent: usize,
}

impl Printer for THIRPrinter<'_> {
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

impl<'a> THIRPrinter<'a> {
    fn new(map: &'a SymbolMap<'a>) -> Self {
        THIRPrinter {
            map,
            output: String::new(),
            indent: 0,
        }
    }

    fn print_with_ty<F>(&mut self, ty: &ty::Ty, f: F)
    where
        F: FnOnce(&mut THIRPrinter),
    {
        self.with_delim(Delim::Paren, false, |this| {
            f(this);
            this.colon();
            this.print_ty(ty);
        });
    }

    fn print_pat(&mut self, pat: &Pat) {
        match pat.kind.as_ref() {
            PatKind::Binding { res, name, ty } => {
                self.print_ident(res.def, *name);
                self.colon();
                self.print_ty(ty);
            }
        }
    }

    fn print_ident(&mut self, res: DefId, name: Symbol) {
        let name = self.map.get(name);
        self.print(name);
        self.with_delim(Delim::Paren, false, |this| {
            this.print_def(res);
        });
    }

    fn print_def(&mut self, def: DefId) {
        self.print("%");
        self.print(def);
    }

    fn print_ty(&mut self, ty: &ty::Ty) {
        match &ty.kind {
            ty::TyKind::Bool => self.print("bool"),
            ty::TyKind::Int(ty) => match ty {
                ty::IntTy::I32 => self.print("i32"),
            },
            ty::TyKind::Tuple(tys) => {
                self.list(tys.iter(), Delim::Paren, |this, ty| {
                    this.print_ty(ty);
                });
            }
            ty::TyKind::Fn(ty) => {
                self.print("Fn");
                self.list(ty.inputs.iter(), Delim::Paren, |this, ty| {
                    this.print_ty(ty);
                });

                if let Some(output) = ty.output.as_ref() {
                    self.space_print_space("->");
                    self.print_ty(output);
                }
            }
            ty::TyKind::FnDef(def) => {
                self.print("FnDef");
                self.with_delim(Delim::Paren, false, |this| {
                    this.print_def(*def);
                });
            }
            ty::TyKind::FnPtr(sig) => {
                self.print("FnPtr");
                self.list(sig.inputs.iter(), Delim::Paren, |this, ty| {
                    this.print_ty(ty);
                });

                if let Some(output) = sig.output.as_ref() {
                    self.space_print_space("->");
                    self.print_ty(output);
                }
            }
            ty::TyKind::Never => self.print("!"),
        }
    }
}
