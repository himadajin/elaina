use mir::{constant::*, stmt::*, *};

use inkwell::values::*;

use crate::CodegenContext;

impl CodegenContext<'_> {
    pub(crate) fn int_value(&self, operand: &Operand) -> IntValue {
        match operand {
            Operand::Copy(place) => {
                let ptr = self.pointer_value(place);
                self.builder.build_load(ptr, "").into_int_value()
            }
            Operand::Constant(constant) => {
                let _ty = constant.ty.clone();
                match &constant.literal {
                    ConstValue::Scalar(s) => self.scalar_int(s),
                }
            }
        }
    }

    pub(crate) fn scalar_int(&self, scalar: &ScalarInt) -> IntValue {
        let data = scalar.data as u64;
        match scalar.size {
            1 => self.context.bool_type().const_int(data, false),
            32 => self.context.i32_type().const_int(data, false),
            _ => panic!("Invalid data size of ScalarInt"),
        }
    }

    pub(crate) fn pointer_value(&self, place: &Place) -> PointerValue {
        self.local_values[place.local]
    }
}
