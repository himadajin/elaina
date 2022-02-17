use ir::{constant::*, stmt::*, *};

use inkwell::{builder::Builder, context::Context, module::Module, values::*, AddressSpace};
use typed_index_collections::TiVec;

#[allow(dead_code)]
pub struct CodegenContext<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    local_values: TiVec<LocalId, PointerValue<'ctx>>,
}

#[allow(dead_code)]
impl<'ctx> CodegenContext<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        CodegenContext {
            context: context,
            builder: context.create_builder(),
            local_values: TiVec::new(),
        }
    }

    pub fn codegen(mut self, body: Body) -> String {
        let module = self.context.create_module("main");

        // declare main function
        let i32_type = self.context.i32_type();
        let main_type = i32_type.fn_type(&[], false);

        // declare printf function
        self.declare_printf_int(&module);

        // add main function
        let function = module.add_function("main", main_type, None);
        self.codegen_body(function, body);

        let ret_ptr = self.local_values.first().unwrap().clone();
        let ret_val = self.builder.build_load(ret_ptr, "").into_int_value();

        // print return value
        self.call_printf_int(&module, ret_val);
        self.builder.build_return(Some(&i32_type.const_int(0, false)));

        module.print_to_string().to_string()
    }

    pub fn codegen_body(&mut self, function: FunctionValue, body: Body) {
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);
        // let block = self.context.append_basic_block(function, name)

        // allocate local values
        for local_id in body.local_decls.keys() {
            let local = &body.local_decls[local_id];
            let name = match &local.name {
                Some(name) => name,
                None => "",
            };

            let local_ptr = self.builder.build_alloca(self.context.i32_type(), name);
            self.local_values.push(local_ptr);
        }

        // codegen bodies
        for block_id in body.blocks.keys() {
            let block = &body.blocks[block_id];
            self.codegen_block(function, block);
        }
    }

    fn codegen_block(&self, _function: FunctionValue, block: &Block) {
        // let basic_block = self.context.append_basic_block(function, "");
        // self.builder.position_at_end(basic_block);

        for stmt in &block.stmts {
            self.codegen_stmt(stmt);
        }
    }

    fn codegen_stmt(&self, stmt: &Statement) {
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
                            BinOp::Div => todo!(),
                        };

                        result
                    }
                };

                self.builder.build_store(place_ptr, value);
            }
        }
    }

    fn int_value(&self, operand: &Operand) -> IntValue {
        match operand {
            Operand::Copy(place) => {
                let ptr = self.pointer_value(place);
                self.builder.build_load(ptr, "").into_int_value()
            }
            Operand::Constant(constant) => match constant.as_ref() {
                Constant::Scalar(scalar) => self.scalar_int(scalar),
            },
        }
    }

    fn scalar_int(&self, scalar: &ScalarInt) -> IntValue {
        let data = scalar.data as u64;
        self.context.i32_type().const_int(data, false)
    }

    fn pointer_value(&self, place: &Place) -> PointerValue {
        self.local_values[place.local]
    }

    fn declare_printf_int(&self, module: &Module<'ctx>) {
        let i32_type = self.context.i32_type();
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
        let printf_type = i32_type.fn_type(&[i8_ptr_type.into()], true);

        module.add_function("printf", printf_type, None);
    }

    fn call_printf_int(&self, module: &Module<'ctx>, value: IntValue) {
        let printf_fn = module.get_function("printf").unwrap();
        let text = self.builder.build_global_string_ptr("%d\n", ".str");
        self.builder.build_call(
            printf_fn,
            &[text.as_pointer_value().into(), value.into()],
            "printf",
        );
    }
}

pub fn codegen_ir_body(body: Body) -> String {
    let context = Context::create();
    CodegenContext::new(&context).codegen(body)
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
