use mir::stmt::*;

use inkwell::{module::Module, values::*, IntPredicate};

use crate::CodegenContext;

impl<'ctx, 'a> CodegenContext<'ctx, 'a> {
    pub(crate) fn codegen_stmt(
        &self,
        module: &Module<'ctx>,
        function: FunctionValue,
        stmt: &Statement,
    ) {
        match stmt {
            Statement::Assign(stmt) => {
                let (place, rvalue) = stmt.as_ref();

                let place_ptr = self.local_values[place.local];

                let value = match rvalue {
                    RValue::Use(operand) => self.int_value(operand),
                    RValue::BinaryOp(bin, operands) => {
                        let (lhs, rhs) = operands.as_ref();
                        let lhs_val = self.int_value(lhs);
                        let rhs_val = self.int_value(rhs);

                        let result = match bin {
                            BinOp::Add => self.builder.build_int_nsw_add(lhs_val, rhs_val, ""),
                            BinOp::Sub => self.builder.build_int_nsw_sub(lhs_val, rhs_val, ""),
                            BinOp::Mul => self.builder.build_int_nsw_mul(lhs_val, rhs_val, ""),
                            BinOp::Div => self.builder.build_int_signed_div(lhs_val, rhs_val, ""),
                            BinOp::Eq => self.builder.build_int_compare(
                                IntPredicate::EQ,
                                lhs_val,
                                rhs_val,
                                "",
                            ),
                            BinOp::Lt => self.builder.build_int_compare(
                                IntPredicate::SLT,
                                lhs_val,
                                rhs_val,
                                "",
                            ),
                            BinOp::Le => self.builder.build_int_compare(
                                IntPredicate::SLE,
                                lhs_val,
                                rhs_val,
                                "",
                            ),
                            BinOp::Ne => self.builder.build_int_compare(
                                IntPredicate::NE,
                                lhs_val,
                                rhs_val,
                                "",
                            ),
                            BinOp::Ge => self.builder.build_int_compare(
                                IntPredicate::SGE,
                                lhs_val,
                                rhs_val,
                                "",
                            ),
                            BinOp::Gt => self.builder.build_int_compare(
                                IntPredicate::SGT,
                                lhs_val,
                                rhs_val,
                                "",
                            ),
                        };

                        result
                    }
                    RValue::UnaryOp(op, operand) => {
                        let operand_val = self.int_value(operand);

                        let result = match op {
                            UnOp::Neg => self.builder.build_int_nsw_neg(operand_val, ""),
                        };

                        result
                    }
                };

                self.builder.build_store(place_ptr, value);
            }
            Statement::Println(operand) => {
                let operand_val = self.int_value(operand);
                self.call_buildint_print(&module, function, operand_val);
            }
        }
    }
}
