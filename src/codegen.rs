use std::collections::HashMap;

use inkwell::{
    builder::Builder,
    passes::PassManager,
    types::{BasicType, StructType},
    values::{BasicValue, FunctionValue, GlobalValue, PointerValue, StructValue},
    AddressSpace,
};

use crate::{
    ast::{Boolean, UMPL2Expr},
    interior_mut::RC,
};

pub(crate) struct Compiler<'a, 'ctx> {
    context: &'ctx inkwell::context::Context,
    module: &'a inkwell::module::Module<'ctx>,
    variables: HashMap<RC<str>, PointerValue<'ctx>>,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    string: HashMap<RC<str>, GlobalValue<'ctx>>,
    kind: StructType<'ctx>,
}
pub enum TyprIndex {
    String = 0,
    Number = 1,
    Boolean = 2,
    Lambda = 3,
}
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
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
            fpm,
            string: HashMap::new(),
            kind: context.struct_type(
                &[
                    context.i8_type().as_basic_type_enum(),
                    context
                        .i8_type()
                        .ptr_type(AddressSpace::default())
                        .as_basic_type_enum(),
                    context.bool_type().as_basic_type_enum(),
                ],
                false,
            ),
        }
    }

    fn value(
        &mut self,
        ty: TyprIndex,
        string: Option<RC<str>>,
        number: Option<f64>,
        bool: Option<bool>,
        fn_ty: Option<FunctionValue<'ctx>>,
    ) -> StructValue<'ctx> {
        // value is a llvm struct the first field tell you the type ie 0 means string 1 mean number ...
        // to get a value out find the field asscoited with the type number
        self.kind.const_named_struct(&[
            self.context
                .i8_type()
                .const_int(ty as u64, false)
                .as_basic_value_enum(),
            if let Some(s) = string {
                // making sure same string isnt saved more than once
                self.string
                    .get(&s)
                    .map(|s| s.as_basic_value_enum())
                    .unwrap_or_else(|| {
                        let str_ptr = &self.builder.build_global_string_ptr(&s, &s);
                        self.string.insert(s, *str_ptr);
                        str_ptr.as_basic_value_enum()
                    })
            } else {
                self.context
                    .i8_type()
                    .ptr_type(AddressSpace::default())
                    .const_null()
                    .as_basic_value_enum()
            },
            self.context
                .f64_type()
                .const_float(number.unwrap_or_default())
                .as_basic_value_enum(),
            self.context
                .bool_type()
                .const_int(bool.unwrap_or_default() as u64, false)
                .as_basic_value_enum(),
            fn_ty
                .map(|f| f.as_global_value().as_pointer_value())
                .unwrap_or(
                    self.context
                        .i8_type()
                        .ptr_type(AddressSpace::default())
                        .const_null(),
                )
                .as_basic_value_enum(),
        ])
    }

    fn string(&mut self, string: RC<str>) -> StructValue<'ctx> {
        self.value(TyprIndex::String, Some(string), None, None, None)
    }
    fn number(&mut self, number: f64) -> StructValue<'ctx> {
        self.value(TyprIndex::Number, None, Some(number), None, None)
    }
    fn bool(&mut self, bool: Boolean) -> StructValue<'ctx> {
        self.value(
            TyprIndex::Boolean,
            None,
            None,
            Some(match bool {
                Boolean::True => true,
                Boolean::False => false,
                Boolean::Maybee => todo!(),
            }),
            None,
        )
    }

    fn function(&mut self, fn_value: FunctionValue<'ctx>) -> StructValue<'ctx> {
        self.value(TyprIndex::Lambda, None, None, None, Some(fn_value))
    }
    #[inline]
    fn fn_value(&self, name: &str) -> FunctionValue<'ctx> {
        self.module.get_function(name).unwrap()
    }
    // / Creates a new stack allocation instruction in the entry block of the function.
    fn create_entry_block_alloca(&self, fn_name: &str, name: &str) -> PointerValue<'ctx> {
        let fn_value = self.fn_value(fn_name);
        let entry = fn_value.get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => self.builder.position_before(&first_instr),
            None => self.builder.position_at_end(entry),
        }

        self.builder.build_alloca(self.kind, name)
    }

    fn compile_expr(&mut self, expr: &UMPL2Expr) -> inkwell::values::BasicValueEnum<'ctx> {
        match expr {
            UMPL2Expr::Number(value) => self.number(*value).as_basic_value_enum(),
            UMPL2Expr::Bool(value) => self.bool(*value).as_basic_value_enum(),
            UMPL2Expr::String(value) => self.string(value.clone()).as_basic_value_enum(),
            UMPL2Expr::Fanction(r#fn) => {
                let old_block = self.builder.get_insert_block();
                let body = r#fn.scope();
                let name = r#fn.name().to_string();
                let arg_types: Vec<_> = std::iter::repeat(self.kind)
                    .take(r#fn.param_count())
                    .map(|a| a.into())
                    .collect();
                let ret_type = self.kind;
                let fn_type = ret_type.fn_type(&arg_types, false);
                let fn_value = self.module.add_function(&name, fn_type, None);

                for (name, arg) in fn_value.get_param_iter().enumerate() {
                    arg.set_name(&name.to_string());
                }
                let entry = self.context.append_basic_block(fn_value, "entry");
                self.builder.position_at_end(entry);
                for (i, arg) in fn_value.get_param_iter().enumerate() {
                    let arg_name: RC<str> = i.to_string().into();

                    let alloca = self.create_entry_block_alloca(&name, &arg_name);

                    self.builder.build_store(alloca, arg);

                    self.variables.insert(arg_name, alloca);
                }
                self.builder
                    .position_at_end(fn_value.get_last_basic_block().unwrap());
                let mut ret_value = None;
                for expr in body {
                    ret_value = Some(self.compile_expr(expr))
                }
                let ret_value = ret_value.unwrap();

                self.builder.build_return(Some(&ret_value));
                if let Some(end) = old_block {
                    self.builder.position_at_end(end);
                }

                // return the whole thing after verification and optimization
                if fn_value.verify(true) {
                    self.fpm.run_on(&fn_value);

                    self.function(fn_value).as_basic_value_enum()
                } else {
                    println!();
                    println!();
                    fn_value.print_to_stderr();
                    unsafe {
                        fn_value.delete();
                    }

                    panic!("Invalid generated function.")
                }
            }
            UMPL2Expr::Ident(s) => self.builder.build_load(*self.variables.get(s).unwrap(), s),
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
            UMPL2Expr::FnParam(s) => self.builder.build_load(
                *self
                    .variables
                    .get::<RC<str>>(&(s.to_string().into()))
                    .unwrap(),
                &s.to_string(),
            ),
            UMPL2Expr::Hempty => todo!(),
            UMPL2Expr::Link(_, _) => todo!(),
            UMPL2Expr::Tree(_) => todo!(),
            UMPL2Expr::FnKW(_) => todo!(),
            UMPL2Expr::Let(_, _) => todo!(),
        }
    }

    pub fn compile_program(&mut self, program: &[UMPL2Expr]) {
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

    pub fn print_ir(&self) {
        self.module.print_to_stderr();
    }
}
