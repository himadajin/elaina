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
        let (res, name) = (item.res, item.name);
        match &item.kind {
            ItemKind::Fn(fun) => self.print_fn(
                res.def,
                name,
                &fun.ty.kind.to_fn_ty().unwrap(),
                &fun.inputs,
                &fun.body,
            ),
        }
    }

    fn print_fn(
        &mut self,
        def: DefId,
        name: Symbol,
        ty: &ty::FnTy,
        inputs: &Vec<Param>,
        body: &Block,
    ) {
        self.print_space("fn");
        self.print_ident(def, name);

        // print fn args: (arg1:ty1, arg2:ty2, ..)
        self.list(
            ty.inputs.iter().zip(inputs),
            Delim::Paren,
            |this, (ty, param)| {
                this.print_ident(param.res.def, param.name);
                this.colon();
                this.print_ty(ty);
            },
        );

        // print return type: -> ty
        if let Some(output) = ty.output.as_ref() {
            self.print(" -> ");
            self.print_ty(output);
        }
        self.space();
        self.print_block(body);
    }
}
