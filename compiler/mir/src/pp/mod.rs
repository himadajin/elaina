pub mod stmt;

use crate::*;

use printer::*;
use span::*;

pub fn print_bodies(map: &SymbolMap, items: &[Body]) -> String {
    let mut p = MIRPrinter::new(map);
    p.print_bodies(items);

    p.finish()
}

struct MIRPrinter<'a> {
    pub map: &'a SymbolMap<'a>,
    pub output: String,
    pub indent: usize,
}

impl Printer for MIRPrinter<'_> {
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

impl<'a> MIRPrinter<'a> {
    fn new(map: &'a SymbolMap<'a>) -> Self {
        MIRPrinter {
            map,
            output: String::new(),
            indent: 0,
        }
    }

    fn print_bodies(&mut self, items: &[Body]) {
        self.separated(
            items.iter(),
            |this| {
                this.newline();
                this.newline();
            },
            |this, item| {
                this.print_body(item);
            },
        );
    }

    fn print_body(&mut self, item: &Body) {
        self.print_space("fn");
        self.print_ident(item.name, item.def);

        self.with_delim(Delim::Brace, true, |this| {
            // print named locals
            let named_locals = item.local_decls.iter_enumerated().filter_map(|(id, decl)| {
                if let Some(name) = &decl.name {
                    Some((id, name))
                } else {
                    None
                }
            });
            this.lines(named_locals, |this, (id, name)| {
                this.print_space("debug");
                this.print(name);
                this.space_print_space("=>");
                this.print_local_id(id);
                this.semi();
            });
            this.newline();
            // print locals
            this.lines(item.local_decls.iter_enumerated(), |this, (id, _)| {
                this.print_space("let");
                this.print_local_id(id);
                this.semi();
            });
            this.newline();
            // print blocks
            this.separated(
                item.blocks.iter_enumerated(),
                |this| {
                    this.newline();
                    this.newline();
                },
                |this, (id, block)| {
                    this.print_block_id(id);
                    this.colon();
                    this.space();
                    this.with_delim(Delim::Brace, true, |this| {
                        // print statements
                        this.lines(block.stmts.iter(), |this, stmt| {
                            this.print_stmt(stmt);
                        });
                        if !block.stmts.is_empty() {
                            this.newline();
                        }

                        // print terminator
                        match &block.terminator {
                            Some(terminator) => this.print_terminator(terminator),
                            None => panic!("Error: terminator is None"),
                        }
                    })
                },
            )
        });
    }

    fn print_terminator(&mut self, terminator: &Terminator) {
        let this = self;
        match terminator {
            Terminator::Goto { target } => {
                this.print("goto");
                this.space_print_space("->");
                this.print_block_id(*target);
                this.semi();
            }
            Terminator::SwitchInt {
                discr,
                switch_ty: _,
                targets: SwitchTargets { values, targets },
            } => {
                assert!(values.len() == targets.len());
                this.print("switchInt");
                this.with_delim(Delim::Paren, false, |this| {
                    this.print_operand(&discr);
                });
                this.space_print_space("->");
                this.list(
                    values.iter().zip(targets),
                    Delim::Bracket,
                    |this, (value, target)| {
                        this.print(value);
                        this.colon();
                        this.space();
                        this.print_block_id(*target);
                    },
                );
                this.semi();
            }
            Terminator::Call {
                fun,
                args,
                destination,
            } => {
                let (dest_place, dest_block) = destination
                    .as_ref()
                    .unwrap_or_else(|| panic!("destination of Terminator::Call is None"));
                // print: "dest = ""
                this.print_place(dest_place.clone());
                this.space();
                this.eq();
                this.space();
                // print: "f(args)"
                this.print_operand(fun);
                this.list(args.iter(), Delim::Paren, |this, arg| {
                    this.print_operand(arg);
                });
                // print: " -> dest"
                this.space_print_space("->");
                this.print_block_id(*dest_block);
                this.semi();
            }
            Terminator::Return => {
                this.print("return");
                this.semi();
            }
        }
    }

    fn print_ident(&mut self, name: Symbol, def: DefId) {
        self.print(self.map.get(name));
        self.with_delim(Delim::Paren, false, |this| {
            this.print("%");
            this.print(def);
        });
    }

    fn print_place(&mut self, place: Place) {
        self.print_local_id(place.local);
    }

    fn print_local_id(&mut self, local: LocalId) {
        self.print("_");
        self.print(local.index());
    }

    fn print_block_id(&mut self, id: BlockId) {
        self.print("b");
        self.print(id.0);
    }
}
