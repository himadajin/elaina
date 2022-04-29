use mir::{constant::*, stmt::*, *};
use ty::*;

use anyhow;
use inkwell::{
    basic_block::BasicBlock, builder::Builder, context::Context, module::Module, values::*,
    AddressSpace, IntPredicate,
};
use typed_index_collections::TiVec;

use std::collections::HashMap;

pub struct CodegenContext<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,

    local_values: TiVec<LocalId, PointerValue<'ctx>>,
    blocks: HashMap<BlockId, BasicBlock<'ctx>>,
}

impl<'ctx> CodegenContext<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        CodegenContext {
            context: context,
            builder: context.create_builder(),
            local_values: TiVec::new(),

            blocks: HashMap::new(),
        }
    }

    pub fn codegen(mut self, body: Body) -> Module<'ctx> {
        let module = self.context.create_module("main");

        // declare main function
        let i32_type = self.context.i32_type();
        let main_type = i32_type.fn_type(&[], false);

        // declare print function
        self.declare_builtin_print(&module);

        // add main function
        let function = module.add_function("main", main_type, None);
        self.codegen_body(&module, function, body);

        module
    }

    pub fn codegen_body(&mut self, module: &Module<'ctx>, function: FunctionValue, body: Body) {
        for block_key in body.blocks.keys() {
            let block = self.context.append_basic_block(function, "");
            self.blocks.insert(block_key, block);
        }

        self.builder
            .position_at_end(self.blocks[&body.blocks.first_key().unwrap()]);

        // allocate local values
        for local_id in body.local_decls.keys() {
            let local = &body.local_decls[local_id];
            let name = match &local.name {
                Some(name) => name,
                None => "",
            };

            let ty_llvm = match local.ty.kind {
                TyKind::Bool => self.context.bool_type(),

                TyKind::Int(ty) => match ty {
                    IntTy::I32 => self.context.i32_type(),
                },
                TyKind::Tuple(_) => todo!(),
                TyKind::Fn(_) => todo!(),
                TyKind::FnDef(_) => todo!(),
                TyKind::FnPtr(_) => todo!(),
                TyKind::Never => todo!(),
            };
            let local_ptr = self.builder.build_alloca(ty_llvm, name);
            self.local_values.push(local_ptr);
        }

        // codegen bodies
        for block_id in body.blocks.keys() {
            let block = &body.blocks[block_id];

            self.builder.position_at_end(self.blocks[&block_id]);
            self.codegen_block(module, function, block);
        }
    }

    fn codegen_block(&self, module: &Module<'ctx>, function: FunctionValue, block: &Block) {
        // let basic_block = self.context.append_basic_block(function, "");
        // self.builder.position_at_end(basic_block);

        for stmt in &block.stmts {
            self.codegen_stmt(module, function, stmt);
        }

        match block
            .terminator
            .as_ref()
            .expect("The Terminator in the Block is None.")
        {
            terminator::Terminator::Goto { target } => {
                let target = self.blocks[target];
                self.builder.build_unconditional_branch(target);
            }
            terminator::Terminator::SwitchInt {
                discr,
                switch_ty: _,
                targets,
            } => {
                if targets.values.len() == 2 {
                    let comp = self.int_value(discr);
                    let else_block = self.blocks[&targets.targets[0]];
                    let then_block = self.blocks[&targets.targets[1]];

                    self.builder
                        .build_conditional_branch(comp, then_block, else_block);
                } else {
                    todo!();
                }
            }
            terminator::Terminator::Call { .. } => todo!(),
            terminator::Terminator::Return => {
                let ret_ptr = self.local_values.first().unwrap().clone();
                let ret_val = self.builder.build_load(ret_ptr, "").into_int_value();
                self.builder.build_return(Some(&ret_val));
            }
        }
    }

    fn codegen_stmt(&self, module: &Module<'ctx>, function: FunctionValue, stmt: &Statement) {
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

    fn int_value(&self, operand: &Operand) -> IntValue {
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

    fn scalar_int(&self, scalar: &ScalarInt) -> IntValue {
        let data = scalar.data as u64;
        match scalar.size {
            1 => self.context.bool_type().const_int(data, false),
            32 => self.context.i32_type().const_int(data, false),
            _ => panic!("Invalid data size of ScalarInt"),
        }
    }

    fn pointer_value(&self, place: &Place) -> PointerValue {
        self.local_values[place.local]
    }

    fn declare_builtin_print(&self, module: &Module<'ctx>) {
        let i32_type = self.context.i32_type();
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
        let printf_type = i32_type.fn_type(&[i8_ptr_type.into()], true);

        module.add_function("printf", printf_type, None);
    }

    fn call_buildint_print(&self, module: &Module<'ctx>, function: FunctionValue, value: IntValue) {
        let printf_fn = module.get_function("printf").unwrap();
        match value.get_type().get_bit_width() {
            1 => {
                let true_value = self.context.bool_type().const_int(1, false);
                let cmp =
                    self.builder
                        .build_int_compare(IntPredicate::EQ, value, true_value, "cmp");
                let then_block = self.context.append_basic_block(function, "if.then");
                let else_block = self.context.append_basic_block(function, "if.else");
                let end_block = self.context.append_basic_block(function, "if.end");
                self.builder
                    .build_conditional_branch(cmp, then_block, else_block);

                // if.then
                self.builder.position_at_end(then_block);
                let text = self.builder.build_global_string_ptr("true\n", ".str");
                self.builder.build_call(
                    printf_fn,
                    &[text.as_pointer_value().into(), value.into()],
                    "printf",
                );
                self.builder.build_unconditional_branch(end_block);

                // if.else
                self.builder.position_at_end(else_block);
                let text = self.builder.build_global_string_ptr("false\n", ".str");
                self.builder.build_call(
                    printf_fn,
                    &[text.as_pointer_value().into(), value.into()],
                    "printf",
                );
                self.builder.build_unconditional_branch(end_block);

                // if.end
                self.builder.position_at_end(end_block);
            }
            _ => {
                let text = self.builder.build_global_string_ptr("%d\n", ".str");
                self.builder.build_call(
                    printf_fn,
                    &[text.as_pointer_value().into(), value.into()],
                    "printf",
                );
            }
        }
    }
}

pub fn codegen_string(body: Body) -> String {
    let context = Context::create();
    let module = CodegenContext::new(&context).codegen(body);
    module.print_to_string().to_string()
}

pub fn codegen_and_execute(body: Body) -> anyhow::Result<i32> {
    let context = Context::create();
    let module = CodegenContext::new(&context).codegen(body);
    let engine = module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .map_err(|err| anyhow::anyhow!("{}", err))?;

    let main_fn = unsafe { engine.get_function::<unsafe extern "C" fn() -> i32>("main") }?;
    let result = unsafe { main_fn.call() };

    Ok(result)
}

/// codegen LLVM IR that print `a`
pub fn codegen_ir() -> String {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let i32_type = context.i32_type();

    // decalre i32 @putchar(i32)
    let putchar_type = i32_type.fn_type(&[i32_type.into()], false);
    module.add_function("putchar", putchar_type, None);

    // define i32 @main()
    let main_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", main_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    // call i32 @putchar (i32 72)
    let func_putchar = module.get_function("putchar").unwrap();
    builder.build_call(
        func_putchar,
        &[i32_type.const_int('a'.into(), false).into()],
        "putchar",
    );
    builder.build_call(
        func_putchar,
        &[i32_type.const_int('\n'.into(), false).into()],
        "putchar",
    );

    // ret i32 0
    builder.build_return(Some(&i32_type.const_int(0, false)));

    // output LLVM-IR
    module.print_to_string().to_string()
}
