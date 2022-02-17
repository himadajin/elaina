use inkwell::context::{Context};

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
