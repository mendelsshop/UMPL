use std::{
    collections::HashMap,
    ffi::{c_char, c_void},
    fmt,
};

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
    module::{Linkage, Module},
    passes::PassManager,
    types::{AnyType, BasicType, FloatType, FunctionType, IntType, StructType},
    values::{
        BasicValue, BasicValueEnum, FloatValue, FunctionValue, GlobalValue, IntValue, PointerValue,
        StructValue,
    },
    AddressSpace,
};

use crate::{
    ast::{Boolean, FnKeyword, UMPL2Expr},
    // exceptions::{raise_error, gen_defs, self},
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
// use std::ffi::CStr;

// #[repr(C)]
// #[derive(Clone, Copy)]
// pub struct Object {
//     kind: TyprIndex,
//     object: UntaggedObject,
// }

// impl Object {
//     pub extern "C" fn print(self) -> Self {
//         println!("{self}");
//         self
//     }

//     pub extern "C" fn extract_num(self) -> f64 {
//         if self.kind != TyprIndex::Number {
//             unsafe { raise_error("type mimsatch".to_string()) }
//         }

//         unsafe { self.object.number }
//     }

//     pub extern "C" fn from_number(number: f64) -> Self {
//         Self {
//             kind: TyprIndex::Number,
//             object: UntaggedObject { number },
//         }
//     }

//     pub extern "C" fn extract_str(self) -> *const i8 {
//         if self.kind != TyprIndex::String {
//             unsafe { raise_error("type mimsatch".to_string()) }
//         }

//         unsafe { self.object.string }
//     }

//     pub extern "C" fn from_str(string: *const i8) -> Self {
//         Self {
//             kind: TyprIndex::String,
//             object: UntaggedObject { string },
//         }
//     }

//     pub extern "C" fn extract_bool(self) -> bool {
//         if self.kind != TyprIndex::Boolean {
//             unsafe { raise_error("type mimsatch".to_string()) }
//         }

//         unsafe { self.object.bool }
//     }

//     pub extern "C" fn from_bool(bool: bool) -> Self {
//         Self {
//             kind: TyprIndex::Boolean,
//             object: UntaggedObject { bool },
//         }
//     }

//     pub extern "C" fn extract_error(self) -> *const i8 {
//         // self.object.error
//         todo!()
//     }

//     pub extern "C" fn extract_type(self) -> i32 {
//         self.kind as i32
//     }
// }

// impl fmt::Display for Object {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         unsafe {
//             match self.kind {
//                 TyprIndex::String => write!(
//                     f,
//                     "{}",
//                     CStr::from_ptr(self.object.string).to_str().unwrap()
//                 ),
//                 TyprIndex::Number => write!(f, "{}", self.object.number),
//                 TyprIndex::Boolean => write!(f, "{}", self.object.bool),
//                 TyprIndex::Lambda => todo!(),
//             }
//         }
//     }
// }

// #[repr(C)]
// #[derive(Clone, Copy)]
// pub struct Function {
//     pub ty: FunctionType<'static>,
//     pub name: *const c_char,
//     pub arglist: *const *const c_char,
//     pub arg_count: u64,
//     pub is_macro: bool,
//     pub invoke_f_ptr: *const c_void,
//     pub apply_to_f_ptr: *const c_void,
//     pub has_restarg: bool,
// }
// #[derive(Clone, Copy)]
// #[repr(C)]
// pub union UntaggedObject {
//     string: *const i8,
//     number: f64,
//     bool: bool,
//     lambda: *mut Function,
//     error: *mut i8,
// }
pub struct Compiler<'a, 'ctx> {
    context: &'ctx Context,
    module: &'a Module<'ctx>,
    variables: Vec<HashMap<RC<str>, PointerValue<'ctx>>>,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    string: HashMap<RC<str>, GlobalValue<'ctx>>,
    kind: StructType<'ctx>,
    vec_type: StructType<'ctx>,
    fn_type: FunctionType<'ctx>,
    fn_value: Option<FunctionValue<'ctx>>,
    error_block: Option<BasicBlock<'ctx>>,
    jit: ExecutionEngine<'ctx>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum TyprIndex {
    String = 2,
    Number = 1,
    Boolean = 0,
    Cons = 3,
    Lambda = 4,
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
        let kind = context.opaque_struct_type("object");
        let vec_type = context.opaque_struct_type("vec");
        let fn_type = kind.fn_type(&[vec_type.into()], false);
        let i8_type = context.i8_type();
        let str_type = i8_type.ptr_type(AddressSpace::default());
        let num_type = context.f64_type();
        let cons_type = kind.array_type(2).ptr_type(AddressSpace::default());
        let bool_type = context.bool_type();
        module.add_function(
            "printf",
            context.i32_type().fn_type(
                &[context.i8_type().ptr_type(AddressSpace::default()).into()],
                true,
            ),
            Some(Linkage::External),
        );
        kind.set_body(
            &[
                i8_type.as_basic_type_enum(),
                bool_type.as_basic_type_enum(),
                num_type.as_basic_type_enum(),
                str_type.as_basic_type_enum(),
                cons_type.as_basic_type_enum(),
                fn_type
                    .ptr_type(AddressSpace::default())
                    .as_basic_type_enum(),
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
            kind,
            fn_value: None,
            error_block: None,
            jit,
            fn_type,
            vec_type,
        }
    }

    fn value(
        &self,
        ty: TyprIndex,
        bool: Option<IntValue<'ctx>>,
        number: Option<FloatValue<'ctx>>,
        string: Option<PointerValue<'ctx>>,
        cons: Option<PointerValue<'ctx>>,
        lambda: Option<PointerValue<'ctx>>,
    ) -> StructValue<'ctx> {
        self.kind.const_named_struct(&[
            self.context.i8_type().const_int(ty as u64, false).into(),
            bool.unwrap_or(self.context.bool_type().const_zero()).into(),
            number
                .unwrap_or(self.context.f64_type().const_zero())
                .into(),
            string
                .unwrap_or(
                    self.context
                        .i8_type()
                        .ptr_type(AddressSpace::default())
                        .const_null(),
                )
                .into(),
            cons.unwrap_or(
                self.context
                    .i8_type()
                    .ptr_type(AddressSpace::default())
                    .const_null(),
            )
            .into(),
            lambda
                .unwrap_or(
                    self.context
                        .i8_type()
                        .ptr_type(AddressSpace::default())
                        .const_null(),
                )
                .into(),
        ])
    }

    fn number(&self, value: FloatValue<'ctx>) -> StructValue<'ctx> {
        self.value(TyprIndex::Number, None, Some(value), None, None, None)
    }

    fn const_number(&self, value: f64) -> StructValue<'ctx> {
        self.number(self.context.f64_type().const_float(value))
    }

    fn bool(&self, value: IntValue<'ctx>) -> StructValue<'ctx> {
        self.value(TyprIndex::Boolean, Some(value), None, None, None, None)
    }

    fn const_bool(&self, value: Boolean) -> StructValue<'ctx> {
        self.bool(self.context.bool_type().const_int(value as u64, false))
    }

    fn string(&self, value: PointerValue<'ctx>) -> StructValue<'ctx> {
        self.value(TyprIndex::String, None, None, Some(value), None, None)
    }

    fn const_string(&mut self, value: RC<str>) -> StructValue<'ctx> {
        #[allow(clippy::map_unwrap_or)]
        // allowing this lint b/c we insert in self.string in None case and rust doesn't like that after trying to get from self.string
        let str = self
            .string
            .get(&value)
            .map(BasicValue::as_basic_value_enum)
            .unwrap_or_else(|| {
                let str_ptr = &self.builder.build_global_string_ptr(&value, &value);
                self.string.insert(value, *str_ptr);
                str_ptr.as_basic_value_enum()
            })
            .into_pointer_value();
        self.string(str)
    }

    #[inline]
    fn current_fn_value(&self) -> Result<FunctionValue<'ctx>, String> {
        self.fn_value
            .ok_or("could not find current function".to_string())
    }
    // / Creates a new stack allocation instruction in the entry block of the function.
    fn create_entry_block_alloca(&self, name: &str) -> Result<PointerValue<'ctx>, String> {
        let fn_value = self.current_fn_value()?;
        // if a function is already allocated it will have an entry block so its fine to unwrap
        let entry = fn_value.get_first_basic_block().unwrap();

        entry.get_first_instruction().map_or_else(
            || self.builder.position_at_end(entry),
            |first_instr| self.builder.position_before(&first_instr),
        );

        Ok(self.builder.build_alloca(self.kind, name))
    }

    fn new_env(&mut self) {
        self.variables.push(HashMap::new());
    }

    fn pop_env(&mut self) {
        self.variables.pop();
    }

    fn insert_variable(&mut self, name: RC<str>, value: PointerValue<'ctx>) {
        if let Some(scope) = self.variables.last_mut() {
            scope.insert(name, value);
        }
    }

    fn get_variable(&self, name: &RC<str>) -> Option<PointerValue<'ctx>> {
        self.variables
            .iter()
            .rev()
            .flatten()
            .find(|v| v.0 == name)
            .map(|v| v.1.clone())
    }

    fn compile_expr(&mut self, expr: &UMPL2Expr) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        match expr {
            UMPL2Expr::Number(value) => Ok(Some(self.const_number(*value).as_basic_value_enum())),
            UMPL2Expr::Bool(value) => Ok(Some(self.const_bool(*value).as_basic_value_enum())),
            UMPL2Expr::String(value) => {
                Ok(Some(self.const_string(value.clone()).as_basic_value_enum()))
            }
            UMPL2Expr::Fanction(r#fn) => {
                let old_fn = self.fn_value;
                let old_block = self.builder.get_insert_block();
                let body = r#fn.scope();
                let name = r#fn
                    .name()
                    .map_or("lambda".to_string(), |name| name.to_string());
                let arg_types: Vec<_> = std::iter::repeat(self.kind)
                    .take(r#fn.param_count())
                    .map(std::convert::Into::into)
                    .collect();
                let ret_type = self.kind;
                let fn_type = ret_type.fn_type(&arg_types, false);
                let fn_value = self.module.add_function(&name, fn_type, None);

                for (name, arg) in fn_value.get_param_iter().enumerate() {
                    arg.set_name(&name.to_string());
                }
                let entry = self.context.append_basic_block(fn_value, "entry");
                self.fn_value = Some(fn_value);
                self.builder.position_at_end(entry);
                self.new_env();
                for (i, arg) in fn_value.get_param_iter().enumerate() {
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
                if fn_value.verify(true) {
                    self.fpm.run_on(&fn_value);
                    self.pop_env();

                    // Ok(Some(self.function(fn_value).as_basic_value_enum()))
                    todo!()
                } else {
                    println!();
                    fn_value.print_to_stderr();
                    unsafe {
                        fn_value.delete();
                    }

                    Err("Invalid generated function.".to_string())
                }
            }
            UMPL2Expr::Ident(s) => self.get_var(s).map(Some),
            UMPL2Expr::Scope(_) => todo!(),
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
                    self.context.i8_type().const_int(2, false),
                    "ifcond",
                );

                // conditinal: either not bool or true
                self.builder.build_or(bool_val, cond, "ifcond");
                let then_bb = self.context.append_basic_block(parent, "then");
                let else_bb = self.context.append_basic_block(parent, "else");
                let cont_bb = self.context.append_basic_block(parent, "ifcont");
                self.builder
                    .build_conditional_branch(cond, then_bb, else_bb);
                self.builder.position_at_end(then_bb);
                let then_val = self.compile_scope(if_stmt.cons())?;
                if then_val.is_some() {
                    self.builder.build_unconditional_branch(cont_bb);
                }
                let then_bb = self.builder.get_insert_block().unwrap();

                // build else block
                self.builder.position_at_end(else_bb);
                let else_val = self.compile_scope(if_stmt.alt())?;
                if else_val.is_some() {
                    self.builder.build_unconditional_branch(cont_bb);
                }
                let else_bb = self.builder.get_insert_block().unwrap();

                // emit merge block
                self.builder.position_at_end(cont_bb);

                let phi = self.builder.build_phi(self.kind, "iftmp");
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
                println!("{:?}", application);
                let op = match &application.args()[0] {
                    UMPL2Expr::Bool(_) => todo!(),
                    UMPL2Expr::Number(_) => todo!(),
                    UMPL2Expr::String(_) => todo!(),
                    UMPL2Expr::Scope(_) => todo!(),
                    UMPL2Expr::Ident(_) => todo!(),
                    UMPL2Expr::If(_) => todo!(),
                    UMPL2Expr::Unless(_) => todo!(),
                    UMPL2Expr::Stop(_) => todo!(),
                    UMPL2Expr::Skip => todo!(),
                    UMPL2Expr::Until(_) => todo!(),
                    UMPL2Expr::GoThrough(_) => todo!(),
                    UMPL2Expr::ContiueDoing(_) => todo!(),
                    UMPL2Expr::Fanction(_) => todo!(),
                    UMPL2Expr::Application(_) => todo!(),
                    UMPL2Expr::Quoted(_) => todo!(),
                    UMPL2Expr::Label(_) => todo!(),
                    UMPL2Expr::FnParam(_) => todo!(),
                    UMPL2Expr::Hempty => todo!(),
                    UMPL2Expr::Link(_, _) => todo!(),
                    UMPL2Expr::Tree(_) => todo!(),
                    UMPL2Expr::FnKW(k) => k,
                    UMPL2Expr::Let(_, _) => todo!(),
                };
                let args = return_none!(application
                    .args()
                    .iter()
                    .skip(1)
                    .map(|expr| self.compile_expr(expr))
                    .collect::<Result<Option<Vec<BasicValueEnum<'_>>>, _>>()?);
                Ok(Some(
                    match op {
                        // TODO shortent these
                        FnKeyword::Add => self.number(
                            args.into_iter()
                                .map(|arg| {
                                    self.extract_number(arg.into_struct_value())
                                        .unwrap()
                                        .into_float_value()
                                })
                                .fold(self.context.f64_type().const_zero(), |a, b| {
                                    self.builder.build_float_add(a, b, "number add")
                                }),
                        ),
                        FnKeyword::Sub => self.number(if args.len() == 1 {
                            self.builder.build_float_neg(
                                self.extract_number(args[0].into_struct_value())
                                    .unwrap()
                                    .into_float_value(),
                                "float neg",
                            )
                        } else {
                            args.into_iter()
                                .map(|arg| {
                                    self.extract_number(arg.into_struct_value())
                                        .unwrap()
                                        .into_float_value()
                                })
                                .reduce(|a, b| self.builder.build_float_sub(a, b, "number add"))
                                .unwrap_or(self.context.f64_type().const_zero())
                        }),
                        FnKeyword::Mul => self.number(
                            args.into_iter()
                                .map(|arg| {
                                    self.extract_number(arg.into_struct_value())
                                        .unwrap()
                                        .into_float_value()
                                })
                                .fold(self.context.f64_type().const_float(1.0), |a, b| {
                                    self.builder.build_float_mul(a, b, "number add")
                                }),
                        ),
                        FnKeyword::Div => self.number(
                            args.into_iter()
                                .map(|arg| {
                                    self.extract_number(arg.into_struct_value())
                                        .unwrap()
                                        .into_float_value()
                                })
                                .reduce(|a, b| self.builder.build_float_div(a, b, "number add"))
                                .unwrap_or(self.context.f64_type().const_float(1.0)),
                        ),
                        FnKeyword::Mod => self.number(
                            args.into_iter()
                                .map(|arg| {
                                    self.extract_number(arg.into_struct_value())
                                        .unwrap()
                                        .into_float_value()
                                })
                                .reduce(|a, b| self.builder.build_float_rem(a, b, "number add"))
                                .unwrap_or(self.context.f64_type().const_float(1.0)),
                        ),
                        FnKeyword::Print => {
                            // self.print_ir();
                            let r = self.print(args[0]).into_struct_value();
                            // self.print_ir();
                            r
                        }
                    }
                    .as_basic_value_enum(),
                ))
            }
            UMPL2Expr::Quoted(_) => todo!(),
            UMPL2Expr::Label(_) => todo!(),
            UMPL2Expr::FnParam(s) => self.get_var(&s.to_string().into()).map(Some),
            UMPL2Expr::Hempty => todo!(),
            UMPL2Expr::Link(_, _) => todo!(),
            UMPL2Expr::Tree(_) => todo!(),
            UMPL2Expr::FnKW(_) => todo!(),
            UMPL2Expr::Let(i, v) => {
                let v = return_none!(self.compile_expr(v)?);
                let ty = self.kind;
                let ptr = self.builder.build_alloca(ty, i);
                self.builder.build_store(ptr, v);
                self.insert_variable(i.clone(), ptr);
                return Ok(Some(
                    self.context.bool_type().const_zero().as_basic_value_enum(),
                ));
            }
        }
    }

    fn extract_type(&self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        self.builder.build_extract_value(cond_struct, 0, "get_type")
    }

    // TODO: for all extract_* methods have checked variants that check that what is trying to be obtained is in fact the type of the object
    fn make_extract_bool(&self) {
        let print_fn_ty: FunctionType<'_> =
            self.context.bool_type().fn_type(&[self.kind.into()], false);
        let print_fn = self.module.add_function("extract_bool", print_fn_ty, None);
        let entry_block = self.context.append_basic_block(print_fn, "entry");
        let ret_block = self.context.append_basic_block(print_fn, "ret");
        let args = print_fn.get_first_param().unwrap();
        self.builder.position_at_end(entry_block);
        let ty = self
            .extract_type(args.into_struct_value())
            .unwrap()
            .into_int_value();
        let condition = self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            ty,
            self.context
                .i8_type()
                .const_int(TyprIndex::Boolean as u64, false),
            "cmp",
        );

        self.builder
            .build_conditional_branch(condition, ret_block, self.error_block.unwrap());
        self.builder.position_at_end(ret_block);

        self.builder.build_return(Some(
            &self
                .builder
                .build_extract_value(args.into_struct_value(), TyprIndex::Boolean as u32 + 1, "extract bool return")
                .unwrap(),
        ));
        print_fn.verify(true);
    }

    fn extract_bool(&self, val: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        let print = self.module.get_function("extract_bool").unwrap();
        self.builder
            .build_call(print, &[val.into()], "print")
            .try_as_basic_value()
            .left()
    }


    fn make_extract_number(&self) {
        let print_fn_ty: FunctionType<'_> =
            self.context.f64_type().fn_type(&[self.kind.into()], false);
        let print_fn = self.module.add_function("extract_number", print_fn_ty, None);
        let entry_block = self.context.append_basic_block(print_fn, "entry");
        let ret_block = self.context.append_basic_block(print_fn, "ret");
        let args = print_fn.get_first_param().unwrap();
        self.builder.position_at_end(entry_block);
        let ty = self
            .extract_type(args.into_struct_value())
            .unwrap()
            .into_int_value();
        let condition = self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            ty,
            self.context
                .i8_type()
                .const_int(TyprIndex::Number as u64, false),
            "cmp",
        );

        self.builder
            .build_conditional_branch(condition, ret_block, self.error_block.unwrap());
        self.builder.position_at_end(ret_block);

        self.builder.build_return(Some(
            &self
                .builder
                .build_extract_value(args.into_struct_value(), TyprIndex::Number as u32 + 1, "extract nmber return")
                .unwrap(),
        ));
        print_fn.verify(true);
    }

    fn extract_number(&self, val: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        let print = self.module.get_function("extract_number").unwrap();
        self.builder
            .build_call(print, &[val.into()], "print")
            .try_as_basic_value()
            .left()
    }

    fn make_extract_string(&self) {
        let print_fn_ty: FunctionType<'_> =
            self.context.i8_type().ptr_type(AddressSpace::default()).fn_type(&[self.kind.into()], false);
        let print_fn = self.module.add_function("extract_string", print_fn_ty, None);
        let entry_block = self.context.append_basic_block(print_fn, "entry");
        let ret_block = self.context.append_basic_block(print_fn, "ret");
        let args = print_fn.get_first_param().unwrap();
        self.builder.position_at_end(entry_block);
        let ty = self
            .extract_type(args.into_struct_value())
            .unwrap()
            .into_int_value();
        let condition = self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            ty,
            self.context
                .i8_type()
                .const_int(TyprIndex::String as u64, false),
            "cmp",
        );

        self.builder
            .build_conditional_branch(condition, ret_block, self.error_block.unwrap());
        self.builder.position_at_end(ret_block);

        self.builder.build_return(Some(
            &self
                .builder
                .build_extract_value(args.into_struct_value(), TyprIndex::String as u32 + 1, "extract string return")
                .unwrap(),
        ));
        print_fn.verify(true);
    }

    fn extract_string(&self, val: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        let print = self.module.get_function("extract_string").unwrap();
        self.builder
            .build_call(print, &[val.into()], "print")
            .try_as_basic_value()
            .left()
    }

    fn extract_function(&mut self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        let ty = self.extract_type(cond_struct).unwrap().into_int_value();
        let condition = self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            ty,
            self.context
                .i8_type()
                .const_int(TyprIndex::Lambda as u64, false),
            "cmp",
        );
        let bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "them");

        // {
        //     self.builder
        //     .build_unconditional_branch(self.error_block.unwrap());
        //     self.context.i8_type().const_zero().as_basic_value_enum()
        // },

        self.builder
            .build_conditional_branch(condition, bb, self.error_block.unwrap());
        self.builder.position_at_end(bb);
        self.builder
            .build_extract_value(cond_struct, TyprIndex::Lambda as u32 + 1, "e")
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

    fn get_var(&mut self, s: &std::rc::Rc<str>) -> Result<BasicValueEnum<'ctx>, String> {
        Ok(self
            .builder
            .build_load(self.get_variable(s).ok_or(format!("{s} not found"))?, s))
    }

    pub fn compile_program(&mut self, program: &[UMPL2Expr]) -> Option<String> {
        let main_fn_type = self.context.i32_type().fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_fn_type, None);
        let main_block = self.context.append_basic_block(main_fn, "entry");
        self.error_block = Some(self.context.append_basic_block(main_fn, "error"));
        self.make_extract_bool();
        self.make_extract_number();
        self.make_extract_string();
        self.fn_value = Some(main_fn);
        self.make_print();

        self.builder.position_at_end(self.error_block.unwrap());

        self.builder
            .build_return(Some((&self.context.i32_type().const_int(1, false))));
        self.builder.position_at_end(main_block);

        // ctx.blocks_stack.push(Rc::new(ok_block));
        // self.builder.position_at_end(err_block);
        self.new_env();
        for expr in program {
            match self.compile_expr(expr) {
                Ok(_) => continue,
                Err(e) => return Some(e),
            }
        }

        // let obj = self
        //     .module
        //     .get_struct_type("object")
        //     .unwrap()
        //     .const_named_struct(&[
        //         self.context.i8_type().const_int(1, false).into(),
        //         self.context.bool_type().const_int(1, false).into(),
        //     ]);

        // self.print(obj.as_basic_value_enum());
        self.builder
            .build_return(Some(&self.context.i32_type().const_zero()));
        self.pop_env();
        // self.error_block.unwrap().get_terminator().unwrap_or_else(||
        // //  self.builder.p
        // );
        if main_fn.verify(true) {
            // self.fpm.run_on(&main_fn);
            // let main = self.module.add_function("main", main_fn_type, None);
            // let main_block = self.context.append_basic_block(main, "entry");
            // self.builder.position_at_end(main_block);
            // self.module.print_to_stderr();
            // let error_handling = self.module.get_function("global_error_handling").unwrap();
            // let result = self.builder.build_call(error_handling, &[], "error handling").try_as_basic_value().expect_left("no error handling").into_int_value();
            // self.builder.build_return(Some(&result));
            None
        } else {
            println!("without optimized");
            main_fn.print_to_stderr();
            // self.fpm.run_on(&main_fn);
            println!("with optimized");
            main_fn.print_to_stderr();

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
                .run_function_as_main(self.module.get_function("main").unwrap(), &[])
        }
    }


    fn make_print(&mut self) {
        let old = self.fn_value;
        let print_fn_ty: FunctionType<'_> = self.kind.fn_type(&[self.kind.into()], false);
        let print_fn = self.module.add_function("print", print_fn_ty, None);
        self.fn_value = Some(print_fn);
        let entry_block = self.context.append_basic_block(print_fn, "entry");
        let bool_block = self.context.append_basic_block(print_fn, "bool");
        let number_block = self.context.append_basic_block(print_fn, "number");
        let string_block = self.context.append_basic_block(print_fn, "string");
        // let cons_block = self.context.append_basic_block(print_fn, "cons");
        // let lambda_block = self.context.append_basic_block(print_fn, "lambda");
        let ret_block = self.context.append_basic_block(print_fn, "ret");
        let args = print_fn.get_first_param().unwrap();
        self.builder.position_at_end(entry_block);
        let ty = self
            .extract_type(args.into_struct_value())
            .unwrap()
            .into_int_value();
        self.builder.build_switch(
            ty,
            self.error_block.unwrap(),
            &[
                (
                    self.context
                        .i8_type()
                        .const_int(TyprIndex::Boolean as u64, false),
                    bool_block,
                ),
                (
                    self.context
                        .i8_type()
                        .const_int(TyprIndex::Number as u64, false),
                    number_block,
                ),
                (
                    self.context
                        .i8_type()
                        .const_int(TyprIndex::String as u64, false),
                    string_block,
                ),
            ],
        );
        self.builder.position_at_end(bool_block);
        let val = self.extract_bool(args.into_struct_value()).unwrap();
                let print = self.module.get_function("printf").unwrap();

        let call = self
            .builder
            .build_call(print, &[self.builder.build_global_string_ptr("%s", "bool fmt specifier").as_basic_value_enum().into(), val.into()], "print");
        self.builder.build_unconditional_branch(ret_block);

        self.builder.position_at_end(number_block);
        let val = self.extract_number(args.into_struct_value()).unwrap();
        self.builder.build_unconditional_branch(ret_block);
        self.builder.position_at_end(string_block);
        let val = self.extract_string(args.into_struct_value()).unwrap();
        self.builder.build_unconditional_branch(ret_block);
        self.builder.position_at_end(ret_block);
        let phi = self.builder.build_phi(self.kind, "print return");
        phi.add_incoming(&[
            (&args, bool_block),
            (&args, number_block),
            (&args, string_block),
        ]);
        self.builder.build_return(Some(&phi.as_basic_value()));
        self.fn_value = old;
        print_fn.verify(true);
    }

    fn print(&self, val: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        let print = self.module.get_function("print").unwrap();
        self.builder
            .build_call(print, &[val.into()], "print")
            .try_as_basic_value()
            .unwrap_left()
    }
}
