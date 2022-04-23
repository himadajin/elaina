use crate::*;

use super::THIRPrinter;

use printer::Delim;

impl THIRPrinter<'_> {
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
        let (res, name) = (item.res, item.name);
        match &item.kind {
            ItemKind::Fn(fun) => self.print_fun(res, name, &fun.ty, &fun.inputs, &fun.body),
        }
    }

    fn print_fun(
        &mut self,
        res: DefId,
        name: Symbol,
        ty: &ty::FnTy,
        inputs: &Vec<Param>,
        body: &Block,
    ) {
        self.p.word("fn ");
        self.print_ident(res, name);

        let inputs = ty.inputs.iter().zip(inputs).collect();
        self.print_fun_inputs(&inputs);

        if let Some(output) = ty.output.as_ref() {
            self.p.word(" -> ");
            self.print_ty(output);
        }
        self.p.space();
        self.print_block(body);
    }

    fn print_fun_inputs(&mut self, inputs: &Vec<(&ty::Ty, &Param)>) {
        let mut inputs = inputs.as_slice();

        self.p.popen(Delim::Paren);

        while let [(ty, param), tail @ ..] = inputs {
            self.print_ident(param.res, param.name);
            self.p.word(":");
            self.print_ty(ty);

            if !tail.is_empty() {
                self.p.word(", ");
            }

            inputs = &inputs[1..];
        }

        self.p.pclose(Delim::Paren);
    }
}
