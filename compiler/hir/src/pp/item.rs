use super::HIRPrinter;
use crate::*;

use ast::ty::Ty;
use printer::Delim;

impl HIRPrinter<'_> {
    pub fn print_items(&mut self, mut items: &[Item]) {
        while let [item, tail @ ..] = items {
            self.print_item(item);

            if tail.is_empty() {
                break;
            }

            self.p.new_line();
            self.p.new_line();
            items = &items[1..];
        }
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
        self.p.word("fn");
        self.p.space();

        self.print_ident(res, name);

        self.p.space();
        self.print_fn_params(inputs.as_slice());

        if let Some(output) = &output {
            self.p.word(" -> ");
            self.print_ty(output);
        }

        self.p.space();
        self.print_block(body);
    }

    pub fn print_fn_params(&mut self, mut params: &[Param]) {
        self.p.popen(Delim::Paren);
        while let [param, tail @ ..] = params {
            self.print_ident(param.res, param.name);
            self.p.word(": ");
            self.print_ty(&param.ty);

            if tail.is_empty() {
                break;
            }

            self.p.word(", ");
            params = &params[1..];
        }
        self.p.pclose(Delim::Paren);
    }
}
