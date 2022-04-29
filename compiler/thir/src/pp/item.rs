use crate::*;

use super::THIRPrinter;

use printer::{Delim, Printer};

impl THIRPrinter<'_> {
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
            ItemKind::Fn(fun) => self.print_fn(
                fun.header.def,
                fun.header.name,
                &fun.header.inputs,
                &fun.header.output,
                &fun.body,
            ),
        }
    }

    fn print_fn(
        &mut self,
        def: DefId,
        name: Symbol,
        inputs: &Vec<Param>,
        output: &ty::Ty,
        body: &Block,
    ) {
        self.print_space("fn");
        self.print_ident(def, name);

        // print fn args: (arg1:ty1, arg2:ty2, ..)
        self.list(inputs.iter(), Delim::Paren, |this, param| {
            this.print_ident(param.res.def, param.name);
            this.colon();
            this.print_ty(&param.ty);
        });

        // print return type: -> ty
        self.space_print_space("->");
        self.print_ty(output);

        self.space();
        self.print_block(body);
    }
}
