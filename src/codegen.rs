use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    execution_engine::ExecutionEngine,
    module::{Linkage, Module},
    passes::PassManager,
    targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine},
    types::{BasicType, FloatType, FunctionType, IntType, PointerType, StructType},
    values::{
        AnyValue, BasicMetadataValueEnum, BasicValue, BasicValueEnum, FloatValue, FunctionValue,
        GlobalValue, IntValue, PointerValue, StructValue,
    },
    AddressSpace, OptimizationLevel, intrinsics::Intrinsic,
};

use crate::{
    ast::{Boolean, FlattenAst, UMPL2Expr},
    interior_mut::RC,
};
macro_rules! return_none {
    ($expr:expr) => {
        match $expr {
            Some(e) => e,
            _ => return Ok(None),
        }
    };
}
#[derive(Clone, Debug)]
pub struct Compiler<'a, 'ctx> {
    context: &'ctx Context,
    pub(crate) module: &'a Module<'ctx>,
    variables: Vec<(HashMap<RC<str>, PointerValue<'ctx>>, Vec<RC<str>>)>,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub(crate) string: HashMap<RC<str>, GlobalValue<'ctx>>,
    // ident stores all used identifiers that were turned in a llvm string literal
    // so we don't store multiple sof the same identifiers
    pub(crate) ident: HashMap<RC<str>, GlobalValue<'ctx>>,
    fn_value: Option<FunctionValue<'ctx>>,
    jit: ExecutionEngine<'ctx>,
    links: HashMap<RC<str>, BasicBlock<'ctx>>,
    pub(crate) types: Types<'ctx>,
    // not were umpl functions are stored
    functions: Functions<'ctx>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
#[allow(non_camel_case_types)]
/// when updating anything in this enum, remember to update the how object is set in [Compiler::new] as it is the only thing that won't automatically reflect changes made here
pub enum TyprIndex {
    #[default]
    Unkown = 100,
    boolean = 0,
    cons = 3,
    // TODO: make hempty be 0 so object will be zeroinitilizer if its hempty
    hempty = 7,
    lambda = 4,
    number = 1,
    string = 2,
    symbol = 5,
    thunk = 6,
}

macro_rules! buider_object {
    ($value:ident, $type:ty) => {
        pub fn $value(&self, value: $type) -> StructValue<'ctx> {
            let ty = TyprIndex::$value;
            let obj = self.types.object.const_zero();
            let obj = self
                .builder
                .build_insert_value(obj, self.types.ty.const_int(ty as u64, false), 0, "type")
                .unwrap();
            let obj = self
                .builder
                .build_insert_value(obj, value, ty as u32 + 1, "value")
                .unwrap();
            obj.into_struct_value()
        }
    };
}

#[derive(Clone, Copy, Debug)]
pub struct Types<'ctx> {
    pub object: StructType<'ctx>,
    pub ty: IntType<'ctx>,
    pub boolean: IntType<'ctx>,
    pub number: FloatType<'ctx>,
    pub string: PointerType<'ctx>,
    pub cons: StructType<'ctx>,
    pub lambda: StructType<'ctx>,
    pub lambda_ptr: PointerType<'ctx>,
    pub lambda_ty: FunctionType<'ctx>,
    pub symbol_type: PointerType<'ctx>,
    pub generic_pointer: PointerType<'ctx>,
    pub hempty: StructType<'ctx>,
}

#[derive(Clone, Copy, Debug)]
/// Important function that the compiler needs to access
pub struct Functions<'ctx> {
    pub va_start: FunctionValue<'ctx>,
    pub va_end: FunctionValue<'ctx>,
}


macro_rules! make_extract {
    ($self:expr, $type:ident, $o_type: ident, $name:literal) => {{
        let extract_fn_ty: FunctionType<'_> = $self
            .types
            .$type
            .fn_type(&[$self.types.object.into()], false);
        let extract_fn =
            $self
                .module
                .add_function(&format!("extract_{}", $name), extract_fn_ty, None);
        let prefix = |end| format!("extract-{}:{end}", $name);
        let entry_block = $self
            .context
            .append_basic_block(extract_fn, &prefix("entry"));
        let ret_block = $self
            .context
            .append_basic_block(extract_fn, &prefix("return"));
        let args = extract_fn.get_first_param().unwrap();
        let error_block = $self
            .context
            .append_basic_block(extract_fn, &prefix("error"));
        $self.builder.position_at_end(error_block);
        $self.exit(&format!("not a {}\n", $name), 1);
        $self.builder.position_at_end(entry_block);

        let ty = $self
            .extract_type(args.into_struct_value())
            .unwrap()
            .into_int_value();
        let condition = $self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            ty,
            $self.types.ty.const_int(TyprIndex::$o_type as u64, false),
            &prefix("cmp-type"),
        );

        $self
            .builder
            .build_conditional_branch(condition, ret_block, error_block);
        $self.builder.position_at_end(ret_block);

        $self.builder.build_return(Some(
            &$self
                .builder
                .build_extract_value(
                    args.into_struct_value(),
                    TyprIndex::$o_type as u32 + 1,
                    &prefix("return"),
                )
                .unwrap(),
        ));
        extract_fn.verify(true);
        $self.fpm.run_on(&extract_fn);
    }};
}

enum TableAccess {
    String,
    Symbol,
}

/// Seperate impl for the [Compiler] for making new objects
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    buider_object!(boolean, IntValue<'ctx>);
    buider_object!(number, FloatValue<'ctx>);
    buider_object!(string, PointerValue<'ctx>);
    buider_object!(cons, PointerValue<'ctx>);
    buider_object!(lambda, StructValue<'ctx>);
    buider_object!(thunk, PointerValue<'ctx>);
    buider_object!(symbol, PointerValue<'ctx>);
    pub fn hempty(&self) -> StructValue<'ctx> {
        let ty = TyprIndex::hempty;
        let obj = self.types.object.const_zero();
        let obj = self
            .builder
            .build_insert_value(obj, self.types.ty.const_int(ty as u64, false), 0, "type")
            .unwrap();
        obj.into_struct_value()
    }
    pub(crate) fn const_number(&self, value: f64) -> StructValue<'ctx> {
        self.number(self.types.number.const_float(value))
    }
    pub(crate) fn const_boolean(&self, value: Boolean) -> StructValue<'ctx> {
        self.boolean(self.types.boolean.const_int(
            match value {
                Boolean::False => 0,
                Boolean::True => 1,
                Boolean::Maybee => todo!(),
            },
            false,
        ))
    }

    pub(crate) fn const_string(&mut self, value: &RC<str>) -> StructValue<'ctx> {
        let str = self.make_string(Some(TableAccess::String), value);
        self.string(str)
    }

    pub(crate) fn const_symbol(&mut self, value: &RC<str>) -> StructValue<'ctx> {
        let str = self.make_string(Some(TableAccess::Symbol), value);
        self.symbol(str)
    }

    fn make_string(
        &mut self,
        kind: Option<TableAccess>,
        value: &std::rc::Rc<str>,
    ) -> PointerValue<'ctx> {
        #[allow(clippy::map_unwrap_or)]
        // allowing this lint b/c we insert in self.string in None case and rust doesn't like that after trying to get from self.string
        kind.map_or_else(
            || {
                self.builder
                    .build_global_string_ptr(value, value)
                    .as_pointer_value()
            },
            |acces| {
                let string_map = match acces {
                    TableAccess::String => &mut self.string,
                    TableAccess::Symbol => &mut self.ident,
                };

                if let Some(str) = string_map.get(value) {
                    str.as_pointer_value()
                } else {
                    let str = self.builder.build_global_string_ptr(value, value);
                    string_map.insert(value.clone(), str);
                    str.as_pointer_value()
                }
            },
        )
    }

    pub(crate) fn const_cons(
        &self,
        left_tree: StructValue<'ctx>,
        this: StructValue<'ctx>,
        right_tree: StructValue<'ctx>,
    ) -> StructValue<'ctx> {
        // TODO: try to not use globals
        // let left_ptr = builder.build_alloca(types.object, "cdr");
        // builder.build_store(left_ptr, left_tree);
        // let this_ptr = builder.build_alloca(types.object, "car");
        // builder.build_store(this_ptr, this);
        // let right_ptr = builder.build_alloca(types.object, "cgr");
        // builder.build_store(right_ptr, right_tree);
        let tree_type =
            self.types
                .cons
                .const_named_struct(&[left_tree.into(), this.into(), right_tree.into()]);

        let tree_ptr = self.module.add_global(tree_type.get_type(), None, "cons");
        tree_ptr.set_initializer(&self.types.cons.const_zero());
        self.builder
            .build_store(tree_ptr.as_pointer_value(), tree_type);
        self.cons(tree_ptr.as_pointer_value())
    }

    fn const_lambda(
        &self,
        function: FunctionValue<'ctx>,
        env: PointerValue<'ctx>,
    ) -> Result<StructValue<'ctx>, &str> {
        if function.verify(true) {
            self.fpm.run_on(&function);
            Ok(self.lambda(self.types.lambda.const_named_struct(&[
                function.as_global_value().as_pointer_value().into(),
                env.into(),
            ])))
        } else {
            unsafe { function.delete() }
            Err("function defined incorrectly")
        }
    }
}

/// provides a standard library and adds the functions to the root envoirment
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    fn make_print(&mut self) {
        // maybe make print should turn into make string

        let print_fn_ty: FunctionType<'_> = self.types.object.fn_type(
            &[self.types.generic_pointer.into(), self.types.object.into()],
            false,
        );
        let print_fn = self.module.add_function("print", print_fn_ty, None);
        self.fn_value = Some(print_fn);
        let entry_block = self.context.append_basic_block(print_fn, "entry");
        let bool_block = self.context.append_basic_block(print_fn, "bool");
        let number_block = self.context.append_basic_block(print_fn, "number");
        let string_block = self.context.append_basic_block(print_fn, "string");
        let cons_block = self.context.append_basic_block(print_fn, "cons");
        // let lambda_block = self.context.append_basic_block(print_fn, "lambda");
        let symbol_block = self.context.append_basic_block(print_fn, "hempty");
        let hempty_block = self.context.append_basic_block(print_fn, "symbol");
        let ret_block = self.context.append_basic_block(print_fn, "return");
        let error_block = self.context.append_basic_block(print_fn, "error");
        self.builder.position_at_end(error_block);

        self.exit("not a valid type\n", 1);
        self.builder.position_at_end(entry_block);
        let args = print_fn.get_nth_param(1).unwrap().into_struct_value();

        let ty = self.extract_type(args).unwrap().into_int_value();
        self.builder.build_switch(
            ty,
            error_block,
            &[
                (
                    self.types.ty.const_int(TyprIndex::boolean as u64, false),
                    bool_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::number as u64, false),
                    number_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::string as u64, false),
                    string_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::cons as u64, false),
                    cons_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::hempty as u64, false),
                    hempty_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::symbol as u64, false),
                    symbol_block,
                ),
            ],
        );
        let print = self.module.get_function("printf").unwrap();
        let print_type = |block,
                          extracter: fn(
            &Compiler<'a, 'ctx>,
            StructValue<'ctx>,
        ) -> Option<BasicValueEnum<'ctx>>,
                          fmt_spec,
                          name| {
            self.builder.position_at_end(block);
            let val = extracter(self, args).unwrap();

            self.builder.build_call(
                print,
                &[
                    self.builder
                        .build_global_string_ptr(fmt_spec, &format!("{name} fmt specifier"))
                        .as_basic_value_enum()
                        .into(),
                    val.into(),
                ],
                &format!("print {name}"),
            );
            self.builder.build_unconditional_branch(ret_block);
        };
        print_type(bool_block, Self::extract_bool, "%i", "boolean");
        print_type(number_block, Self::extract_number, "%f", "number");
        print_type(string_block, Self::extract_string, "%s", "string");
        print_type(symbol_block, Self::extract_symbol, "%s", "symbol");
        self.builder.position_at_end(cons_block);
        // let val = self.extract_cons( args).unwrap();
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr("(", "open paren")
                .as_basic_value_enum()
                .into()],
            &format!("print open"),
        );
        let val = self
            .builder
            .build_call(
                self.module.get_function("car").unwrap(),
                &[self.types.generic_pointer.const_null().into(), args.into()],
                "getcar",
            )
            .try_as_basic_value()
            .unwrap_left();
        self.builder.build_call(
            self.module.get_function("print").unwrap(),
            &[self.types.generic_pointer.const_null().into(), val.into()],
            "printcar",
        );
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr(" ", "space")
                .as_basic_value_enum()
                .into()],
            &format!("print space"),
        );
        let val = self
            .builder
            .build_call(
                self.module.get_function("cdr").unwrap(),
                &[self.types.generic_pointer.const_null().into(), args.into()],
                "getcar",
            )
            .try_as_basic_value()
            .unwrap_left();
        self.builder.build_call(
            self.module.get_function("print").unwrap(),
            &[self.types.generic_pointer.const_null().into(), val.into()],
            "printcar",
        );
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr(" ", "space")
                .as_basic_value_enum()
                .into()],
            &format!("print space"),
        );
        let val = self
            .builder
            .build_call(
                self.module.get_function("cgr").unwrap(),
                &[self.types.generic_pointer.const_null().into(), args.into()],
                "getcar",
            )
            .try_as_basic_value()
            .unwrap_left();
        self.builder.build_call(
            self.module.get_function("print").unwrap(),
            &[self.types.generic_pointer.const_null().into(), val.into()],
            "printcar",
        );
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr(")", "open paren")
                .as_basic_value_enum()
                .into()],
            &format!("print open"),
        );
        self.builder.build_unconditional_branch(ret_block);
        self.builder.position_at_end(hempty_block);
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr("hempty", "hempty")
                .as_pointer_value()
                .into()],
            "printcar",
        );
        self.builder.build_unconditional_branch(ret_block);
        self.builder.position_at_end(ret_block);
        let phi = self.builder.build_phi(self.types.object, "print return");
        phi.add_incoming(&[
            (&args, bool_block),
            (&args, number_block),
            (&args, string_block),
            (&args, cons_block),
            (&args, hempty_block),
            (&args, symbol_block),
        ]);
        self.builder.build_return(Some(&phi.as_basic_value()));
        self.insert_function(
            "print".into(),
            print_fn,
            self.types.generic_pointer.const_null(),
        )
    }

    fn make_add(&mut self) {
        let fn_ty = self.types.lambda_ty;
        let func = self.module.add_function("add", fn_ty, None);
        let entry = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(entry);
        let va_list =self.builder.build_alloca(self.context.i8_type(), "va_list");
        self.builder.build_call(self.functions.va_start, &[va_list.into()], "init args");
        self.builder.build_call(self.functions.va_end, &[va_list.into()], "va end");
        self.builder.build_return(Some(&self.hempty()));
        self.insert_function("add".into(), func, self.types.generic_pointer.const_null());
    }

    fn make_accesors(&mut self) {
        let fn_ty = self.types.lambda_ty;
        let mut accesor = |idx, name| {
            let func = self.module.add_function(name, fn_ty, None);
            let entry = self.context.append_basic_block(func, name);
            self.builder.position_at_end(entry);
            // we can ignore envoirment, just needed for compatibility with other procedures
            let cons_object = func.get_nth_param(1).unwrap().into_struct_value();
            let cons_object = self.extract_cons(cons_object).unwrap().into_struct_value();
            let car = self
                .builder
                .build_extract_value(cons_object, idx, &format!("get-{name}"))
                .unwrap();
            self.builder.build_return(Some(&car));
            self.insert_function(name.into(), func, self.types.generic_pointer.const_null());
        };
        accesor(0, "car");
        accesor(1, "cdr");
        accesor(2, "cgr");
    }
}

/// envoirnment/variable handling functions
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    fn insert_function(
        &mut self,
        name: RC<str>,
        function: FunctionValue<'ctx>,
        env: PointerValue<'ctx>,
    ) {
        if let Ok(lambda) = self.const_lambda(function, env) {
            self.insert_lambda(name, lambda)
        } else {
            println!("")
        }
    }

    fn insert_lambda(&mut self, name: RC<str>, lambda: StructValue<'ctx>) {
        let gloabl_lambda = self.module.add_global(lambda.get_type(), None, &name);
        gloabl_lambda.set_initializer(&lambda);
        self.insert_variable(name, gloabl_lambda.as_pointer_value())
    }

    fn new_env(&mut self) {
        self.variables.push((HashMap::new(), vec![]));
    }

    fn pop_env(&mut self) {
        self.variables.pop();
    }

    fn insert_variable(&mut self, name: RC<str>, value: PointerValue<'ctx>) {
        if let Some(scope) = self.variables.last_mut() {
            scope.0.insert(name.clone(), value);
            scope.1.push(name);
        }
    }

    pub fn get_scope(&self) -> (StructType<'ctx>, PointerValue<'ctx>) {
        let prev = self.get_current_env_name();

        let value: Vec<_> = prev.collect();
        let env_struct_type = self.context.struct_type(
            &std::iter::repeat(self.types.object)
                .take(value.len())
                .map(|s| s.into())
                .collect::<Vec<_>>(),
            false,
        );
        let global_save = self.module.add_global(env_struct_type, None, "");
        global_save.set_initializer(&env_struct_type.const_zero());
        let env_pointer = global_save.as_pointer_value();

        for (i, v) in value.iter().enumerate() {
            println!("{i} {v}");
            let value = self.get_var(*v).unwrap();
            let gep = self
                .builder
                .build_struct_gep(env_struct_type, env_pointer, i as u32, "save env")
                .unwrap();
            self.builder.build_store(gep, value);
        }
        (env_struct_type, env_pointer)
    }

    pub fn get_current_env_name(&self) -> impl Iterator<Item = &RC<str>> {
        self.variables.last().unwrap().1.iter()
    }

    fn get_variable(&self, name: &RC<str>) -> Option<PointerValue<'ctx>> {
        self.variables
            .iter()
            .rev()
            .cloned()
            .map(|v| v.0)
            .flatten()
            .find(|v| v.0 == name.clone())
            .map(|v| v.1)
    }

    fn get_var(&self, s: &std::rc::Rc<str>) -> Result<BasicValueEnum<'ctx>, String> {
        Ok(self.builder.build_load(
            self.types.object,
            self.get_variable(s).ok_or(format!("{s} not found"))?,
            s,
        ))
    }
}
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        builder: &'a Builder<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
    ) -> Self {

        let jit = module
            .create_jit_execution_engine(inkwell::OptimizationLevel::None)
            .unwrap();
        let env_ptr: PointerType<'ctx> = context
            .struct_type(&[], false)
            .ptr_type(AddressSpace::default())
            .into();
        let kind = context.opaque_struct_type("object");
        let fn_type = kind.fn_type(&[env_ptr.into(), kind.into()], true);
        let lambda = context.struct_type(
            &[
                fn_type.ptr_type(AddressSpace::default()).into(),
                env_ptr.into(),
            ],
            false,
        );
        let types = Types {
            object: kind,
            ty: context.i8_type(),
            boolean: context.bool_type(),
            number: context.f64_type(),
            string: context.i8_type().ptr_type(AddressSpace::default()),
            cons: context.struct_type(&[kind.into(), kind.into(), kind.into()], false),
            lambda,
            lambda_ptr: lambda.ptr_type(AddressSpace::default()),
            lambda_ty: fn_type,
            symbol_type: context.i8_type().ptr_type(AddressSpace::default()),
            generic_pointer: context.i8_type().ptr_type(AddressSpace::default()),
            hempty: context.struct_type(&[], false),
        };
        module.add_function(
            "exit",
            context
                .void_type()
                .fn_type(&[context.i32_type().into()], false),
            Some(Linkage::External),
        );
        module.add_function(
            "printf",
            context.i32_type().fn_type(&[types.string.into()], true),
            Some(Linkage::External),
        );
        let va_arg_start = Intrinsic::find("llvm.va_start").unwrap();
        let va_start=va_arg_start.get_declaration(module, &[]).unwrap();
        let va_arg_end = Intrinsic::find("llvm.va_end").unwrap();
        let va_end=va_arg_end.get_declaration(module, &[]).unwrap();
        let functions= Functions {
            va_end, va_start
        };
        kind.set_body(
            &[
                types.ty.as_basic_type_enum(),
                types.boolean.as_basic_type_enum(),
                types.number.as_basic_type_enum(),
                types.string.as_basic_type_enum(),
                types.generic_pointer.as_basic_type_enum(),
                types.lambda.as_basic_type_enum(),
                types.symbol_type.as_basic_type_enum(),
                types.hempty.as_basic_type_enum(),
            ],
            false,
        );
        Self {
            context,
            module,
            variables: vec![],
            builder,
            fpm,
            string: HashMap::new(),
            ident: HashMap::new(),
            fn_value: None,
            jit,
            types,
            links: HashMap::new(),
            functions,
        }
    }

    #[inline]
    fn current_fn_value(&self) -> Result<FunctionValue<'ctx>, &str> {
        self.fn_value.ok_or("could not find current function")
    }
    // / Creates a new stack allocation instruction in the entry block of the function.
    fn create_entry_block_alloca(&self, name: &str) -> Result<PointerValue<'ctx>, &str> {
        let fn_value = self.current_fn_value()?;
        // if a function is already allocated it will have an entry block so its fine to unwrap
        let entry = fn_value.get_first_basic_block().unwrap();

        entry.get_first_instruction().map_or_else(
            || self.builder.position_at_end(entry),
            |first_instr| self.builder.position_before(&first_instr),
        );

        let build_alloca = self.builder.build_alloca(self.types.object, name);
        Ok(build_alloca)
        // store everything as a global variable
        // Ok(self.module.add_global(self.types.object, Some(AddressSpace::default()), name).as_pointer_value())
    }

    fn compile_expr(&mut self, expr: &UMPL2Expr) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        match expr {
            UMPL2Expr::Number(value) => Ok(Some(self.const_number(*value).as_basic_value_enum())),
            UMPL2Expr::Bool(value) => Ok(Some(self.const_boolean(*value).as_basic_value_enum())),
            UMPL2Expr::String(value) => Ok(Some(self.const_string(value).as_basic_value_enum())),
            UMPL2Expr::Fanction(r#fn) => {
                // if its var arg dont make it var arg, just make it arg_count+1  number of parameters
                let env = self.get_scope();
                let old_fn = self.fn_value;
                let old_block = self.builder.get_insert_block();
                let body = r#fn.scope();
                let name = r#fn
                    .name()
                    .map_or("lambda".to_string(), |name| name.to_string());
                let mut arg_types: Vec<_> = std::iter::repeat(self.types.object)
                    .take(r#fn.param_count())
                    .map(std::convert::Into::into)
                    .collect();
                arg_types.insert(0, env.0.ptr_type(AddressSpace::default()).into());
                let ret_type = self.types.object;
                let fn_type = ret_type.fn_type(&arg_types, false);
                let fn_value = self.module.add_function(&name, fn_type, None);
                let envs = env.1;

                for (name, arg) in fn_value.get_param_iter().skip(1).enumerate() {
                    arg.set_name(&name.to_string());
                }
                let entry = self.context.append_basic_block(fn_value, "entry");
                self.fn_value = Some(fn_value);
                self.builder.position_at_end(entry);
                let env_iter = self.get_current_env_name().cloned().collect::<Vec<_>>();
                // right now even though we take the first parameter to be the "envoirnment" we don't actully use it, maybee remove that parameter
                let envs = self
                    .builder
                    .build_load(env.0, envs, "load env")
                    .into_struct_value();
                self.new_env();
                for i in 0..env.0.count_fields() {
                    let cn = env_iter[i as usize].clone();
                    // self.module.add_global(type_, address_space, name)
                    let alloca = self.builder.build_alloca(self.types.object, &cn);
                    let arg = self
                        .builder
                        .build_extract_value(envs, i.try_into().unwrap(), "load captured")
                        .unwrap();
                    self.builder.build_store(alloca, arg);
                    self.insert_variable(cn.clone(), alloca);
                }
                for (i, arg) in fn_value.get_param_iter().skip(1).enumerate() {
                    let arg_name: RC<str> = i.to_string().into();
                    let alloca = self.create_entry_block_alloca(&arg_name)?;
                    self.builder.build_store(alloca, arg);
                    self.insert_variable(arg_name, alloca);
                }
                self.builder
                    .position_at_end(fn_value.get_last_basic_block().unwrap());
                if let Some(ret) = self.compile_scope(body)? {
                    self.builder.build_return(Some(&ret));
                }

                // reset to previous state (before function) needed for functions in functions
                if let Some(end) = old_block {
                    self.builder.position_at_end(end);
                }
                self.fn_value = old_fn;

                // return the whole thing after verification and optimization
                if let Ok(lambda) =
                    self.const_lambda(fn_value, self.types.generic_pointer.const_null())
                {
                    self.pop_env();
                    let ret = if let Some(_) = r#fn.name() {
                        self.insert_lambda(name.into(), lambda);
                        self.hempty()
                    } else {
                        lambda
                    };
                    Ok(Some(ret.as_basic_value_enum()))
                } else {
                    println!();
                    self.print_ir();
                    unsafe {
                        fn_value.delete();
                    }

                    Err("Invalid generated function.".to_string())
                }
            }
            UMPL2Expr::Ident(s) => self.get_var(s).map(Some),
            UMPL2Expr::Scope(_) => unreachable!(),
            UMPL2Expr::If(if_stmt) => {
                let parent = self.current_fn_value()?;
                let cond_struct =
                    return_none!(self.compile_expr(if_stmt.cond())?).into_struct_value();
                let bool_val = self.extract_bool(cond_struct).unwrap().into_int_value();
                let object_type = self.extract_type(cond_struct).unwrap().into_int_value();
                // if its not a bool type
                let cond = self.builder.build_int_compare(
                    inkwell::IntPredicate::NE,
                    object_type,
                    self.types.ty.const_int(TyprIndex::boolean as u64, false),
                    "if:cond:boolean?",
                );

                // conditinal: either not bool or true
                let cond = self.builder.build_or(bool_val, cond, "if:cond:false?");
                let then_bb = self.context.append_basic_block(parent, "then");
                let else_bb = self.context.append_basic_block(parent, "else");
                let cont_bb = self.context.append_basic_block(parent, "ifcont");
                self.builder
                    .build_conditional_branch(cond, then_bb, else_bb);
                self.builder.position_at_end(then_bb);
                let then_val = self.compile_scope(if_stmt.alt())?;
                if then_val.is_some() {
                    self.builder.build_unconditional_branch(cont_bb);
                }
                let then_bb = self.builder.get_insert_block().unwrap();

                // build else block
                self.builder.position_at_end(else_bb);
                let else_val = self.compile_scope(if_stmt.cons())?;
                if else_val.is_some() {
                    self.builder.build_unconditional_branch(cont_bb);
                }
                let else_bb = self.builder.get_insert_block().unwrap();

                // emit merge block
                self.builder.position_at_end(cont_bb);

                let phi = self.builder.build_phi(self.types.object, "if:phi-cont");
                match (then_val, else_val) {
                    (None, None) => phi.add_incoming(&[]),
                    (None, Some(else_val)) => phi.add_incoming(&[(&else_val, else_bb)]),
                    (Some(then_val), None) => phi.add_incoming(&[(&then_val, then_bb)]),
                    (Some(then_val), Some(else_val)) => {
                        phi.add_incoming(&[(&then_val, then_bb), (&else_val, else_bb)]);
                    }
                }
                Ok(Some(phi.as_basic_value()))
            }
            UMPL2Expr::Unless(_) => todo!(),
            UMPL2Expr::Stop(s) => {
                let res = return_none!(self.compile_expr(s)?);
                self.builder.build_return(Some(&res));
                Ok(None)
            }
            UMPL2Expr::Skip => todo!(),
            UMPL2Expr::Until(_) => todo!(),
            UMPL2Expr::GoThrough(_) => todo!(),
            UMPL2Expr::ContiueDoing(_) => todo!(),
            UMPL2Expr::Application(application) => {
                // TODO
                let op = return_none!(self.compile_expr(&application.args()[0])?);
                let args = return_none!(application
                    .args()
                    .iter()
                    .skip(1)
                    .map(|expr| self.compile_expr(expr))
                    .collect::<Result<Option<Vec<BasicValueEnum<'_>>>, _>>()?);

                let val = op.into_struct_value();

                let op = self.extract_function(val).unwrap();
                let function_pointer = self
                    .builder
                    .build_extract_value(op.into_struct_value(), 0, "function load")
                    .unwrap()
                    .as_any_value_enum()
                    .into_pointer_value();
                let env_pointer = self
                    .builder
                    .build_extract_value(op.into_struct_value(), 1, "function env load")
                    .unwrap()
                    .as_any_value_enum()
                    .into_pointer_value();
                let mut args = args
                    .iter()
                    .map(|a| (*a).into())
                    .collect::<Vec<BasicMetadataValueEnum<'ctx>>>();
                args.insert(0, env_pointer.into());
                // should probavly figure out that actual param count of function cause supposedly tail calls dont work on varidiac aargument function
                let unwrap_left = self
                    .builder
                    .build_indirect_call(
                        self.types.lambda_ty,
                        function_pointer,
                        args.as_slice(),
                        "application:call",
                    )
                    .try_as_basic_value()
                    .unwrap_left();
                Ok(Some(unwrap_left))
            }
            // right now the approach for quotation is to codegen the expression and wrap it in a function which will be called with the to get the value of the expression
            // kinda of doesnt work because quotation should assume nothing about the environment, but since we do a full codegen if a ident is quoted it will attempt to lookup
            // the variable and error if it doesn't exist (not wanted behavior)
            // another approach would be to make codegen eitheer return and llvm value or a UMPl2expr

            // note the above comment and code below is wrong we just need to convert to appropriate literal types either tree, number, bool, string, or symbol (needs to be added to object struct)
            UMPL2Expr::Quoted(q) => Ok(Some(q.clone().flatten(self).as_basic_value_enum())),
            UMPL2Expr::Label(_s) => todo!(),
            UMPL2Expr::FnParam(s) => self.get_var(&s.to_string().into()).map(Some),
            UMPL2Expr::Hempty => Ok(Some(self.hempty().into())),
            UMPL2Expr::Link(_, _) => todo!(),
            // UMPL2Expr::Tree(_) => todo!(),
            UMPL2Expr::Let(i, v) => {
                let v = return_none!(self.compile_expr(v)?);
                let ty = self.types.object;
                let ptr = self.builder.build_alloca(ty, i);
                // let ptr = self.module.add_global(ty, Some(AddressSpace::default()), i).as_pointer_value();
                self.builder.build_store(ptr, v);
                self.insert_variable(i.clone(), ptr);
                // self.context.o
                return Ok(Some(self.types.boolean.const_zero().as_basic_value_enum()));
            }
            UMPL2Expr::ComeTo(_) => todo!(),
        }
    }

    fn extract_type(&self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        self.builder.build_extract_value(cond_struct, 0, "get_type")
    }

    fn extract_bool(&self, val: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        let print = self.module.get_function("extract_boolean").unwrap();
        self.builder
            .build_call(print, &[val.into()], "extract-bool")
            .try_as_basic_value()
            .left()
    }

    fn extract_number(&self, val: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        let extract_number_fn = self.module.get_function("extract_number").unwrap();
        self.builder
            .build_call(extract_number_fn, &[val.into()], "extract-number")
            .try_as_basic_value()
            .left()
    }

    fn extract_string(&self, val: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        let print = self.module.get_function("extract_string").unwrap();
        self.builder
            .build_call(print, &[val.into()], "extract-string")
            .try_as_basic_value()
            .left()
    }

    fn extract_symbol(&self, val: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        let print = self.module.get_function("extract_symbol").unwrap();
        self.builder
            .build_call(print, &[val.into()], "extract-symbol")
            .try_as_basic_value()
            .left()
    }

    fn extract_function(&self, val: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        let print = self.module.get_function("extract_function").unwrap();
        self.builder
            .build_call(print, &[val.into()], "extract-function")
            .try_as_basic_value()
            .left()
    }

    fn extract_cons(&self, val: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        let cons_extract = self.module.get_function("extract_cons").unwrap();
        self.builder
            .build_call(cons_extract, &[val.into()], "extract-cons")
            .try_as_basic_value()
            .left()
            .map(|pointer| {
                self.builder
                    .build_load(self.types.cons, pointer.into_pointer_value(), "loadcons")
            })
    }

    fn compile_scope(
        &mut self,
        body: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        let mut res = Err("scope does not have value".to_string());
        for expr in body {
            res = Ok(return_none!(self.compile_expr(expr)?));
        }
        res.map(Some)
    }

    pub fn make_extraction(&mut self) {
        make_extract!(self, lambda, lambda, "function");
        make_extract!(self, string, string, "string");
        make_extract!(self, boolean, boolean, "boolean");
        make_extract!(self, number, number, "number");
        make_extract!(self, string, symbol, "symbol");
        make_extract!(self, generic_pointer, cons, "cons");
    }

    pub fn compile_program(
        &mut self,
        program: &[UMPL2Expr],
        _links: HashSet<RC<str>>,
    ) -> Option<String> {
        self.new_env();
        self.make_extraction();
        self.make_accesors();
        self.make_print();
        self.make_add();
        self.new_env();
        let main_fn_type = self.context.i32_type().fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_fn_type, None);
        let main_block = self.context.append_basic_block(main_fn, "entry");
        // TODO: maybe dont optimize make_* functions b/c indirect call branches

        self.fn_value = Some(main_fn);

        self.builder.position_at_end(main_block);

        for expr in program {
            match self.compile_expr(expr) {
                Ok(_) => continue,
                Err(e) => return Some(e),
            }
        }
        self.builder
            .build_return(Some(&self.context.i32_type().const_zero()));
        self.pop_env();

        let verify = main_fn.verify(true);

        if verify {
            self.fpm.run_on(&main_fn);
            println!("done");
            None
        } else {
            println!("without optimized");
            self.print_ir();
            self.fpm.run_on(&main_fn);
            println!("with optimized");
            self.print_ir();

            unsafe {
                main_fn.delete();
            }

            Some("Invalid generated function.".to_string())
        }
    }

    pub fn print_ir(&self) {
        self.module.print_to_stderr();
    }
    pub fn run(&self) -> i32 {
        unsafe {
            self.jit
                .run_function(self.module.get_function("main").unwrap(), &[])
                .as_int(false) as i32
        }
    }

    pub fn export_ir(&self, path: impl Into<PathBuf>) {
        let mut path: PathBuf = path.into();
        path.set_extension("ll");
        self.module.print_to_file(&path).expect("couldn't export");
    }

    pub fn export_bc(&self, path: impl Into<PathBuf>) {
        let mut path: PathBuf = path.into();
        path.set_extension("bc");
        self.module.write_bitcode_to_path(&path);
    }

    // TODO: split into asm and object functions
    pub fn export_object_and_asm(&self, path: impl Into<PathBuf>) {
        let mut asm_path: PathBuf = path.into();
        let mut o_path: PathBuf = asm_path.clone();
        o_path.set_extension("o");
        asm_path.set_extension("as");

        let config = InitializationConfig {
            asm_parser: true,
            asm_printer: true,
            base: true,
            disassembler: true,
            info: true,
            machine_code: true,
        };

        Target::initialize_native(&config).unwrap();
        let triple = TargetMachine::get_default_triple();

        let target = Target::from_triple(&triple).unwrap();

        let tm = target
            .create_target_machine(
                &TargetMachine::get_default_triple(),
                &TargetMachine::get_host_cpu_name().to_string(),
                &TargetMachine::get_host_cpu_features().to_string(),
                OptimizationLevel::Aggressive,
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();
        tm.add_analysis_passes(self.fpm);

        tm.write_to_file(self.module, inkwell::targets::FileType::Object, &o_path)
            .expect(" writing to file ");

        tm.write_to_file(self.module, inkwell::targets::FileType::Assembly, &asm_path)
            .expect(" writing to file ");
    }

    fn print(&self, val: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        let print = self.module.get_function("print").unwrap();
        self.builder
            .build_call(print, &[val.into()], "print")
            .try_as_basic_value()
            .unwrap_left()
    }

    pub fn exit(&self, reason: &str, code: i32) {
        let print = self.module.get_function("printf").unwrap();
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr(reason, "error exit")
                .as_basic_value_enum()
                .into()],
            "print",
        );
        let exit = self.module.get_function("exit").unwrap();
        self.builder.build_call(
            exit,
            &[self.context.i32_type().const_int(code as u64, false).into()],
            "exit",
        );

        self.builder.build_unreachable();
    }
}
