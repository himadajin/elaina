use super::HIRPrinter;
use crate::*;

use ast::ty::Ty;
use printer::{Delim, Printer};

impl HIRPrinter<'_> {
    pub fn print_items(&mut self, items: &[Item]) {
        self.separated(
            items.iter(),
            |this| {
                this.newline();
                this.newline();
            },
            |this, item| {
                this.print_item(item);
            },
        );
    }

    pub fn print_item(&mut self, item: &Item) {
        match &item.kind {
            ItemKind::Fn(fun) => {
                self.print_item_fn(item.res, item.name, &fun.inputs, &fun.output, &fun.body)
            }
        }
    }

    pub fn print_item_fn(
        &mut self,
        res: Res,
        name: Symbol,
        inputs: &Vec<Param>,
        output: &Option<Ty>,
        body: &Block,
    ) {
        self.print_space("fn");
        self.print_ident(res, name);
        self.space();

        self.with_delim(Delim::Paren, false, |this| {
            this.separated(
                inputs.iter(),
                |this| {
                    this.comma();
                    this.space();
                },
                |this, param| {
                    this.print_ident(param.res, param.name);
                    this.colon();
                    this.space();
                    this.print_ty(&param.ty);
                },
            )
        });

        if let Some(output) = &output {
            self.space_print_space("->");
            self.print_ty(output);
        }

        self.space();
        self.print_block(body);
    }
}
