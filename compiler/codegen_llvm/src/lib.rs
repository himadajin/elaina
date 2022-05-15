pub mod stmt;
pub mod value;

use mir::*;
use span::SymbolMap;
use ty::{res::DefId, *};

use anyhow;
use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicMetadataTypeEnum, BasicType},
    values::*,
    AddressSpace, IntPredicate,
};
use typed_index_collections::TiVec;

use std::collections::HashMap;

pub struct CodegenContext<'ctx, 'a> {
    context: &'ctx Context,
    builder: Builder<'ctx>,

    symbol_map: &'a SymbolMap<'a>,
    local_values: TiVec<LocalId, PointerValue<'ctx>>,
    functions: HashMap<DefId, FunctionValue<'ctx>>,
    blocks: HashMap<BlockId, BasicBlock<'ctx>>,
}

impl<'ctx, 'a> CodegenContext<'ctx, 'a> {
    pub fn new(context: &'ctx Context, symbol_map: &'a SymbolMap<'a>) -> Self {
        CodegenContext {
            context: context,
            builder: context.create_builder(),
            local_values: TiVec::new(),
            symbol_map,
            functions: HashMap::new(),
            blocks: HashMap::new(),
        }
    }

    pub fn codegen(&mut self, bodies: &[Body]) -> Module<'ctx> {
        let module = self.context.create_module("main");

        // declare functions
        for body in bodies {
            let ret_ty = {
                let ty = &body.local_decls[body.id_return()].ty;
                self.basic_type(ty)
            };
            let input_types: Vec<_> = body
                .id_args()
                .map(|id| &body.local_decls[id].ty)
                .map(|ty| self.basic_meta_data_type(ty))
                .collect();
            let fn_type = ret_ty.fn_type(input_types.as_slice(), false);
            let fn_name = self.symbol_map.get(body.name);
            let function = module.add_function(fn_name, fn_type, None);
            self.functions.insert(body.def, function);
        }

        // declare buildin functions
        self.declare_builtin_print(&module);

        // codegen bodies
        for body in bodies {
            let function = self.functions[&body.def];
            self.codegen_body(&module, function, body);
        }

        module
    }

    pub fn codegen_body(
        &mut self,
        module: &Module<'ctx>,
        function: FunctionValue<'ctx>,
        body: &Body,
    ) {
        for block_key in body.blocks.keys() {
            let block = self.context.append_basic_block(function, "");
            self.blocks.insert(block_key, block);
        }

        self.builder
            .position_at_end(self.blocks[&body.blocks.first_key().unwrap()]);

        // allocate local values
        for id in body.local_decls.keys() {
            let decl = &body.local_decls[id];
            self.declare_local(id, decl);
        }

        // store args.
        for (nth, id) in body.id_args().enumerate() {
            let ptr = self.local_values[id];
            let value = function.get_nth_param(nth as u32).unwrap();
            self.builder.build_store(ptr, value);
        }

        // codegen bodies
        for block_id in body.blocks.keys() {
            let block = &body.blocks[block_id];

            self.builder.position_at_end(self.blocks[&block_id]);
            self.codegen_block(module, function, block);
        }
    }

    fn declare_local(&mut self, id: LocalId, decl: &LocalDecl) {
        let name = match &decl.name {
            Some(name) => name,
            None => "",
        };

        let basic_type = self.basic_type(&decl.ty);
        let local_ptr = self.builder.build_alloca(basic_type, name);
        self.local_values.insert(id, local_ptr);
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
            terminator::Terminator::Call {
                fun,
                args,
                destination,
            } => {
                let (dest_place, dest_block) = destination.as_ref().unwrap();
                let function = self.as_function_value(fun);
                let args: Vec<_> = args
                    .iter()
                    .map(|arg| self.basic_metadata_value(arg))
                    .collect();
                let call = self
                    .builder
                    .build_call(function, args.as_slice(), "")
                    .try_as_basic_value()
                    .left()
                    .unwrap();
                let dest_ptr = self.pointer_value(dest_place);
                self.builder.build_store(dest_ptr, call);

                let target = self.blocks[dest_block];
                self.builder.build_unconditional_branch(target);
            }
            terminator::Terminator::Return => {
                let ret_ptr = self.local_values.first().unwrap().clone();
                let ret_val = self.builder.build_load(ret_ptr, "").into_int_value();
                self.builder.build_return(Some(&ret_val));
            }
        }
    }

    fn basic_type(&self, ty: &Ty) -> impl BasicType<'ctx> {
        match ty.kind() {
            TyKind::Bool => self.context.bool_type(),
            TyKind::Int(int_ty) => match int_ty {
                IntTy::I32 => self.context.i32_type(),
            },
            _ => {
                panic!("error: tried to convert {:?} to BasicType", &ty.kind())
            }
        }
    }

    fn basic_meta_data_type(&self, ty: &Ty) -> BasicMetadataTypeEnum<'ctx> {
        match ty.kind() {
            TyKind::Bool => self.context.bool_type().into(),
            TyKind::Int(int_ty) => match int_ty {
                IntTy::I32 => self.context.i32_type().into(),
            },
            _ => {
                panic!(
                    "error: tried to convert {:?} to BasicMetadataTypeEnum",
                    ty.kind()
                )
            }
        }
    }

    fn declare_builtin_print(&self, module: &Module<'ctx>) {
        let i32_type = self.context.i32_type();
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::Generic);
        let printf_type = i32_type.fn_type(&[i8_ptr_type.into()], true);

        module.add_function("printf", printf_type, None);
    }

    pub(crate) fn call_buildin_print(
        &self,
        module: &Module<'ctx>,
        function: FunctionValue,
        value: IntValue,
    ) {
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

fn with_context<F, V>(symbol_map: &SymbolMap, f: F) -> V
where
    F: FnOnce(&mut CodegenContext) -> V,
{
    let context = Context::create();
    return f(&mut CodegenContext::new(&context, symbol_map));
}

pub fn codegen_string(bodies: &[Body], symbol_map: &SymbolMap) -> String {
    with_context(symbol_map, |context| {
        let module = context.codegen(bodies);
        module.print_to_string().to_string()
    })
}

pub fn codegen_and_execute(bodies: &[Body], symbol_map: &SymbolMap) -> anyhow::Result<i32> {
    with_context(symbol_map, |context| {
        let module = context.codegen(bodies);
        let engine = module
            .create_jit_execution_engine(inkwell::OptimizationLevel::None)
            .map_err(|err| anyhow::anyhow!("{}", err))?;

        let main_fn = unsafe { engine.get_function::<unsafe extern "C" fn() -> i32>("main") }?;
        let result = unsafe { main_fn.call() };

        Ok(result)
    })
}
