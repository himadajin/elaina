use crate::*;

use super::MIRPrinter;

use printer::*;
use ty::TyKind;

impl MIRPrinter<'_> {
    pub fn print_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Assign(assign) => {
                let (lhs, rhs) = assign.as_ref();
                self.print_place(lhs.clone());
                self.space();
                self.eq();
                self.space();
                self.print_rvalue(rhs);
            }
            Statement::Println(op) => {
                self.print("println");
                self.with_delim(Delim::Paren, false, |this| {
                    this.print_operand(op);
                });
            }
        }
    }

    fn print_rvalue(&mut self, rvalue: &RValue) {
        match rvalue {
            RValue::Use(op) => self.print_operand(op),
            RValue::BinaryOp(bin, operand) => {
                self.print(bin);
                self.list(
                    [&operand.0, &operand.1].iter(),
                    Delim::Paren,
                    |this, item| {
                        this.print_operand(item);
                    },
                );
            }
            RValue::UnaryOp(un, operand) => {
                self.print(un);
                self.with_delim(Delim::Paren, false, |this| {
                    this.print_operand(operand);
                });
            }
        }
    }

    pub(crate) fn print_operand(&mut self, operand: &Operand) {
        match operand {
            Operand::Copy(place) => {
                self.print_place(place.clone());
            }
            Operand::Constant(constant) => {
                if let TyKind::FnDef(def) = constant.ty.kind {
                    self.print("%");
                    self.print(def);
                } else {
                    match &constant.literal {
                        constant::ConstValue::Scalar(scalar) => {
                            self.print(scalar.data);
                        }
                    }
                }
            }
        }
    }
}
