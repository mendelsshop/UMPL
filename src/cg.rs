use std::collections::HashMap;

use inkwell::{
    builder::Builder,
    context::Context,
    passes::PassManager,
    types::BasicType,
    values::{AnyValue, BasicValue, FunctionValue, GlobalValue, PointerValue},
    AddressSpace, module::Module,
};

use crate::{ast::UMPL2Expr, lexer::umpl_parse};

struct Compiler<'a, 'ctx> {
    context: &'ctx inkwell::context::Context,
    module: &'a inkwell::module::Module<'ctx>,
    variables: HashMap<String, PointerValue<'ctx>>,
    pub builder: &'a Builder<'ctx>,
    fn_value_opt: Vec<Module<'ctx>>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    string: HashMap<String, GlobalValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    fn new(
        context: &'ctx inkwell::context::Context,
        module: &'a inkwell::module::Module<'ctx>,
        builder: &'a Builder<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
    ) -> Self {
        Self {
            context,
            module,
            variables: HashMap::new(),
            builder,
            fn_value_opt: vec![],
            fpm,
            string: HashMap::new(),
        }
    }

    #[inline]
    fn fn_value(&self, name: &str, module: &'a inkwell::module::Module<'ctx>) -> FunctionValue<'ctx> {
    self.module.get_function(name).unwrap()
    }
    // / Creates a new stack allocation instruction in the entry block of the function.
    fn create_entry_block_alloca(&self, fn_name: &str, name: &str, module: &'a inkwell::module::Module<'ctx>) -> PointerValue<'ctx> {
        let fn_value = self.fn_value(fn_name, module);
        let entry = fn_value.get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => self.builder.position_before(&first_instr),
            None => self.builder.position_at_end(entry),
        }

        self.builder
            .build_alloca(fn_value.get_type().get_return_type().unwrap(), name)
    }

    fn get_module(&self) -> &Module<'ctx> {
        self.fn_value_opt.last().unwrap_or(self.module)
    }
    fn compile_expr(&mut self, expr: &UMPL2Expr) -> inkwell::values::BasicValueEnum<'ctx> {
        match expr {
            UMPL2Expr::Number(value) => {
                println!("{value}");
                self.context
                    .f64_type()
                    .const_float(*value)
                    .as_basic_value_enum()
                // self.context.i8_type().ptr_type(AddressSpace::default())
            }
            UMPL2Expr::Bool(value) => {
                inkwell::values::BasicValueEnum::IntValue(self.context.bool_type().const_int(
                    match value {
                        crate::ast::Boolean::True => 1,
                        crate::ast::Boolean::False => 0,
                        crate::ast::Boolean::Maybee => todo!(),
                    },
                    false,
                ))
            }
            UMPL2Expr::String(value) => {
                let res = self.string.get(&value.to_string());
                if let Some(v) = res {
                    return v.as_basic_value_enum();
                }
                let string = self.builder.build_global_string_ptr(&value, &value);
                self.string.insert(value.to_string(), string.clone());
                string.as_basic_value_enum()
                // let string_ptr = self.module.add_global(
                //     self.context.i8_type().ptr_type(AddressSpace::default()),
                //     None,
                //     "string_ptr",
                // );
                // string_ptr.set_initializer(&self.context.const_string(value.as_bytes(), false));

                // unsafe {
                //     inkwell::values::BasicValueEnum::PointerValue(
                //         self.context
                //             .i8_type()
                //             .ptr_type(AddressSpace::default())
                //             .const_null()
                //             .const_gep(&[
                //                 self.context.i32_type().const_zero(),
                //                 self.context.i32_type().const_zero(),
                //             ]),
                //     )
                // }
            }
            UMPL2Expr::Fanction(r#fn) => {
                let old_mod = self.module;

                let module = self.context.create_module("fn");
                self.fn_value_opt.push(module);
                // std::mem::swap(&mut self.module, &mut  &create_module);
                let body = r#fn.scope();
                let name = r#fn.name().to_string();
                let arg_types: Vec<_> =
                    std::iter::repeat(self.context.i8_type().ptr_type(AddressSpace::default()))
                        .take(r#fn.param_count())
                        .map(|a| a.into())
                        .collect();
                let ret_type = self.context.i8_type().ptr_type(AddressSpace::default());
                let fn_type = ret_type.fn_type(&arg_types, false);
                let fn_value = self.get_module().add_function(&name, fn_type, None);

                for (name, arg) in fn_value.get_param_iter().enumerate() {
                    arg.set_name(&name.to_string());
                }
                let entry = self.context.append_basic_block(fn_value, "entry");
                self.builder.position_at_end(entry);
                for (i, arg) in fn_value.get_param_iter().enumerate() {
                    let arg_name = i.to_string();

                    // let alloca = self.create_entry_block_alloca(&name, &arg_name);

                    // self.builder.build_store(alloca, arg);

                    // self.variables.insert(arg_name.clone(), alloca);
                    self.variables
                        .insert(arg_name.clone(), arg.into_pointer_value());
                }
                fn_value.print_to_stderr();
                let mut ret_value = None;
                for expr in body {
                    ret_value = Some(self.compile_expr(expr))
                }
                let ret_value = ret_value.unwrap();
                let ret = match ret_value.get_type() {
                    inkwell::types::BasicTypeEnum::ArrayType(_) => todo!(),
                    inkwell::types::BasicTypeEnum::FloatType(v) => {
                        let ptr = self.get_module().add_global(self.context.i8_type(), None, "ret");
                        ptr.set_initializer(&ret_value);
                        ptr.as_basic_value_enum()
                    }
                    inkwell::types::BasicTypeEnum::IntType(i) => {
                        let ptr = self.get_module().add_global(self.context.i8_type(), None, "ret");
                        ptr.set_initializer(&ret_value);
                        ptr.as_basic_value_enum()
                    }
                    inkwell::types::BasicTypeEnum::PointerType(p) => ret_value,
                    inkwell::types::BasicTypeEnum::StructType(_) => todo!(),
                    inkwell::types::BasicTypeEnum::VectorType(_) => todo!(),
                };

                self.builder.build_return(Some(&ret));
                println!();
                self.fn_value_opt.pop();
                // return the whole thing after verification and optimization
                if fn_value.verify(true) {
                    // self.fpm.run_on(&fn_value);

                    fn_value
                        .as_global_value()
                        .as_pointer_value()
                        .as_basic_value_enum()
                } else {
                    println!();
                    println!();
                    self.module.print_to_stderr();
                    unsafe {
                        fn_value.delete();
                    }

                    panic!("Invalid generated function.")
                }
            }
            UMPL2Expr::Ident(s) => self
                .builder
                .build_load(*self.variables.get(&s.to_string()).unwrap(), &s),
            UMPL2Expr::Scope(_) => todo!(),
            UMPL2Expr::If(_) => todo!(),
            UMPL2Expr::Unless(_) => todo!(),
            UMPL2Expr::Stop(_) => todo!(),
            UMPL2Expr::Skip => todo!(),
            UMPL2Expr::Until(_) => todo!(),
            UMPL2Expr::GoThrough(_) => todo!(),
            UMPL2Expr::ContiueDoing(_) => todo!(),
            UMPL2Expr::Application(_) => todo!(),
            UMPL2Expr::Quoted(_) => todo!(),
            UMPL2Expr::Label(_) => todo!(),
            UMPL2Expr::FnParam(s) => self
                .builder
                .build_load(*self.variables.get(&s.to_string()).unwrap(), &s.to_string()),
            UMPL2Expr::Hempty => todo!(),
            UMPL2Expr::Link(_, _) => todo!(),
            UMPL2Expr::Tree(_) => todo!(),
            UMPL2Expr::FnKW(_) => todo!(),
            UMPL2Expr::Let(_, _) => todo!(),
        }
    }

    fn compile_program(&mut self, program: &[UMPL2Expr]) {
        let main_fn_type = self.context.i32_type().fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_fn_type, None);
        let main_block = self.context.append_basic_block(main_fn, "entry");
        let builder = self.context.create_builder();
        builder.position_at_end(main_block);

        for expr in program {
            self.compile_expr(expr);
        }

        builder.build_return(Some(&self.context.i32_type().const_zero()));
    }

    fn print_ir(&self) {
        self.module.print_to_stderr();
    }
}
#[test]
fn main() {
    let program = umpl_parse("fanction 🚗  2 ᚜ .a. ᚛  fanction  🚆 2 ᚜ 1 &  fanction 🔥  2 ᚜ .a. ᚛  ᚛ ").unwrap();
    println!("{program:?}");
    let context = Context::create();
    let module = context.create_module("repl");
    let builder = context.create_builder();
    // Create FPM
    let fpm = PassManager::create(&module);

    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();

    fpm.initialize();
    let mut compiler = Compiler::new(&context, &module, &builder, &fpm);
    compiler.compile_program(&program);
    compiler.print_ir();
}
