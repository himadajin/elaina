use mir::{stmt::*, *};
use ty;

use inkwell::values::*;

use crate::CodegenContext;

impl<'ctx, 'a> CodegenContext<'ctx, 'a> {
    pub(crate) fn int_value(&self, operand: &Operand) -> IntValue {
        match operand {
            Operand::Copy(place) => {
                let ptr = self.pointer_value(place);
                self.builder.build_load(ptr, "").into_int_value()
            }
            Operand::Constant(constant) => {
                let _ty = constant.ty.clone();
                match &constant.literal {
                    ty::ConstLit::Scalar(s) => self.scalar_int(s),
                }
            }
        }
    }

    pub(crate) fn scalar_int(&self, scalar: &ty::ScalarInt) -> IntValue {
        let data = scalar.data as u64;
        match scalar.size {
            1 => self.context.bool_type().const_int(data, false),
            32 => self.context.i32_type().const_int(data, false),
            _ => panic!("Invalid data size of ScalarInt"),
        }
    }

    pub(crate) fn as_function_value(&self, operand: &Operand) -> FunctionValue {
        match operand {
            Operand::Copy(_) => todo!(),
            Operand::Constant(constant) => match &constant.ty.kind() {
                ty::TyKind::FnDef(def) => self.functions[def],
                _ => panic!(
                    "Tried to convert constant of {:?} to function value",
                    &constant
                ),
            },
        }
    }

    pub(crate) fn pointer_value(&self, place: &Place) -> PointerValue {
        self.local_values[place.local]
    }

    pub(crate) fn basic_metadata_value(&self, operand: &Operand) -> BasicMetadataValueEnum {
        match operand {
            Operand::Copy(place) => {
                let ptr = self.pointer_value(place);
                self.builder.build_load(ptr, "").into()
            }
            Operand::Constant(constant) => match &constant.ty.kind() {
                ty::TyKind::Bool | ty::TyKind::Int(_) => {
                    let ty::ConstLit::Scalar(scalar) = &constant.literal;
                    self.scalar_int(scalar).into()
                }
                ty::TyKind::Tuple(_) | ty::TyKind::FnDef(_) | ty::TyKind::Never => {
                    panic!(
                        "Tried to convert type of {:?} to BasicMetadataValueEnum",
                        &constant.ty
                    );
                }
            },
        }
    }
}
