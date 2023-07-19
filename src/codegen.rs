use std::{
    collections::HashMap,
    ffi::{c_char, c_void},
};

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    types::{BasicType, FunctionType, StructType},
    values::{
        BasicValue, BasicValueEnum, FloatValue, FunctionValue, GlobalValue, PointerValue,
        StructValue,
    },
    AddressSpace, OptimizationLevel,
};

use crate::{
    ast::{Boolean, FnKeyword, UMPL2Expr},
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
use std::ffi::CStr;
#[inline(never)]
#[no_mangle]
pub extern "C" fn extract_num(object: Object) -> i32 {
    println!("extract_num");
    // unsafe {
    println!("extract_num:kind {:?}", object.kind);
    // println!("extract_num::num {:?}", object.number);
    unsafe {
    println!("extract_num::cstr pointer {:?}", object.object.string);
    }
    if object.kind == 0 {
        unsafe {
            println!(
                "extract_num::str {:?}",
                CStr::from_ptr(object.object.string)
            )
        }
    } else {
        unsafe {
            println!("extract_num::str {:?}", object.object.string.is_null());
        }
    }
    // println!("extract_num {:?} ", object.object.string);
    // // println!("extract_num {:?} ", object.object.bool);
    unsafe {
        println!("extract_num::num {:?} ", object.object.number);
    }
    // unsafe { println!("extract_num::lam {:?} ", (object.object.lambda.is_null())); }
    // }
    // // panic!();
    // // println!("object: {:?}", object.kind);
    // if object.kind != TyprIndex::Number as i8 {
    //     panic!("type mismatch");
    // }unsafe {

    return 1;
    // }
}
#[used]
static ___USED_UNLISP_RT_INT_FROM_OBJ: extern "C" fn(o: Object) -> i32 = extract_num;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Object {
    kind: i8,
    object: UntaggedObject,
    //     bool: bool,
    // string: *const i8,
    // number: f64
    // lambda: *const Function
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Function {
    pub ty: FunctionType<'static>,
    pub name: *const c_char,
    pub arglist: *const *const c_char,
    pub arg_count: u64,
    pub is_macro: bool,
    pub invoke_f_ptr: *const c_void,
    pub apply_to_f_ptr: *const c_void,
    pub has_restarg: bool,
}
#[derive(Clone, Copy)]
#[repr(C)]
pub union UntaggedObject {
    
    string: *const i8,
    number: f64,
    // bool: bool,
    // lambda: *mut Function,
    // lambda: *const i8,
}
pub struct Compiler<'a, 'ctx> {
    context: &'ctx Context,
    module: &'a Module<'ctx>,
    variables: HashMap<RC<str>, PointerValue<'ctx>>,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    string: HashMap<RC<str>, GlobalValue<'ctx>>,
    kind: StructType<'ctx>,
    fn_value: Option<FunctionValue<'ctx>>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum TyprIndex {
    String = 0,
    Number = 1,
    Boolean = 2,
    Lambda = 3,
}
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        context: &'ctx Context,
        module: &'a Module<'ctx>,
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
                        //          context
                        // .i8_type()
                        // .ptr_type(AddressSpace::default())
                        // .as_basic_type_enum(),
                    context
                        .i8_type()
                        .ptr_type(AddressSpace::default())
                        .as_basic_type_enum(),
                    context.f64_type().as_basic_type_enum(),
                            //  context.bool_type().as_basic_type_enum(),

                ],
                false,
            ),
            fn_value: None,
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
        if bool.is_some() && number.is_some() {
            panic!()
        }
        // value is a llvm struct the first field tell you the type ie 0 means string 1 mean number ...
        // to get a value out find the field asscoited with the type number
        
        self.kind.const_named_struct(&[
// kind
            self.context
                .i8_type()
                .const_int(ty as u64, false)
                .as_basic_value_enum(),
   
            if let Some(s) = string {
                // making sure same string isnt saved more than once
                #[allow(clippy::map_unwrap_or)]
                // allowing this lint b/c we insert in self.string in None case and rust doesn't like that after trying to get from self.string
                self.string
                    .get(&s)
                    .map(BasicValue::as_basic_value_enum)
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
                            // string
                // lambda
            // fn_ty
            //     .map_or(
            //         self.context
            //             .i8_type()
            //             .ptr_type(AddressSpace::default())
            //             .const_null(),
            //         |f| f.as_global_value().as_pointer_value(),
            //     )
            //     .as_basic_value_enum(),
// number
            self.context
                .f64_type()
                .const_float(number.unwrap_or_default())
                .as_basic_value_enum(),
                        //   bool   
                // self.context
                // .bool_type()
                // .const_int(u64::from(bool.unwrap_or_default()), false)
                // .as_basic_value_enum(),

        ])
    }

    fn string(&mut self, string: RC<str>) -> StructValue<'ctx> {
        self.value(TyprIndex::String, Some(string), None, None, None)
    }
    fn const_number(&mut self, number: f64) -> StructValue<'ctx> {
        self.value(TyprIndex::Number, None, Some(number), None, None)
    }

    fn number(&mut self, number: FloatValue<'ctx>) -> StructValue<'ctx> {
        // we first create an empty object because if we just create the struct with number llvm complains about returning instructions
        let num = self.value(TyprIndex::Number, None, Some(0.0), None, None);
        // after creating object set the number field to the value
        self.builder
            .build_insert_value(num, number, 2, "number")
            .unwrap()
            .into_struct_value()
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
        let ret = self.value(TyprIndex::Lambda, None, None, None, None);
        self.builder
            .build_insert_value(
                ret,
                fn_value.as_global_value().as_pointer_value(),
                3,
                "loadfn",
            )
            .unwrap()
            .into_struct_value()
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

    fn compile_expr(&mut self, expr: &UMPL2Expr) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        match expr {
            UMPL2Expr::Number(value) => Ok(Some(self.const_number(*value).as_basic_value_enum())),
            UMPL2Expr::Bool(value) => Ok(Some(self.bool(*value).as_basic_value_enum())),
            UMPL2Expr::String(value) => Ok(Some(self.string(value.clone()).as_basic_value_enum())),
            UMPL2Expr::Fanction(r#fn) => {
                let old_fn = self.fn_value;
                let old_block = self.builder.get_insert_block();
                let body = r#fn.scope();
                let name = r#fn.name().to_string();
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
                for (i, arg) in fn_value.get_param_iter().enumerate() {
                    let arg_name: RC<str> = i.to_string().into();
                    let alloca = self.create_entry_block_alloca(&arg_name)?;
                    self.builder.build_store(alloca, arg);
                    self.variables.insert(arg_name, alloca);
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

                    Ok(Some(self.function(fn_value).as_basic_value_enum()))
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
                self.module.print_to_stderr();

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
                let lhs = return_none!(self.compile_expr(&application.args()[1])?);
                let f = self
                    .extract_number(lhs.into_struct_value())
                    .unwrap()
                    .into_float_value();
                let rhs = return_none!(self.compile_expr(&application.args()[2])?);
                let l = self
                    .extract_number(rhs.into_struct_value())
                    .unwrap()
                    .into_float_value();
                Ok(Some(
                    self.number(match op {
                        FnKeyword::Add => self.builder.build_float_add(f, l, "tmpadd"),
                        FnKeyword::Sub => self.builder.build_float_sub(f, l, "tmpsub"),
                        FnKeyword::Mul => self.builder.build_float_mul(f, l, "tmpmul"),
                        FnKeyword::Div => self.builder.build_float_div(f, l, "tmpdiv"),
                        FnKeyword::Mod => todo!(),
                    })
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
                self.variables.insert(i.clone(), ptr);
                return Ok(Some(
                    self.context.bool_type().const_zero().as_basic_value_enum(),
                ));
            }
        }
    }

    fn extract_type(&mut self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        self.builder.build_extract_value(cond_struct, 0, "load")
    }

    // TODO: for all extract_* methods have checked variants that check that what is trying to be obtained is in fact the type of the object
    fn extract_bool(&mut self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        self.builder.build_extract_value(cond_struct, 3, "load")
    }

    fn extract_string(&mut self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        self.builder.build_extract_value(cond_struct, 1, "load")
    }

    fn extract_number(&mut self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        self.builder.build_extract_value(cond_struct, 2, "load")
    }

    fn extract_function(&mut self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        self.builder.build_extract_value(cond_struct, 4, "load")
    }

    // fn extract_number_checked(&mut self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
    //     let ty = self.extract_type(cond_struct).unwrap().into_int_value();

    //     if val != TyprIndex::Number as u64 {
    //         println!("type mismatch");
    //         return None;
    //     }
    //     self.builder.build_extract_value(cond_struct, 2, "load")
    // }

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
            .build_load(*self.variables.get(s).ok_or(format!("{s} not found"))?, s))
    }

    pub fn compile_program(&mut self, program: &[UMPL2Expr]) -> Option<String> {
        let main_fn_type = self.context.i32_type().fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_fn_type, None);
        let main_block = self.context.append_basic_block(main_fn, "entry");
        // let builder = self.context.create_builder();
        self.builder.position_at_end(main_block);

        // let other = self.kind.fn_type(&[], false);
        // let connst_fn = self.module.add_function("hi", other, None);
        let fun = self.module.add_function(
            "extract_num",
            self.context
                .i32_type()
                .fn_type(vec![self.kind.into()].as_slice(), false),
            None,
        );
        fun.set_call_conventions(8);
        // ee.add_global_mapping(&extf, sumf as usize);

        let const_float = self.string("const_float".into());
        // let const_float = self.const_number(441.0);
        // let const_float = self.function(connst_fn);
        const_float.print_to_stderr();
        // connst_fn.print_to_stderr();
        let ret = self.builder.build_call(fun, &[const_float.into()], "retv");
        // for expr in program {
        //     match self.compile_expr(expr) {
        //         Ok(_) => continue,
        //         Err(e) => return Some(e),
        //     }
        // }

        let basic_value_enum = ret.try_as_basic_value().left().unwrap().into_int_value();

        self.builder.build_return(Some(&basic_value_enum));
        let execution_engine = self
            .module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();
        execution_engine.add_global_mapping(&fun, ___USED_UNLISP_RT_INT_FROM_OBJ as usize);

        let re = unsafe { execution_engine.run_function(main_fn, &[]) }.as_int(false);
        println!("err {}", re);
        None
    }

    pub fn print_ir(&self) {
        self.module.print_to_stderr();
    }
}
#[test]
fn t() {
    use inkwell::context::Context;
    use inkwell::targets::{InitializationConfig, Target};
    use inkwell::OptimizationLevel;

    Target::initialize_native(&InitializationConfig::default()).unwrap();

    extern "C" fn sumf(a: f64, b: f64) -> f64 {
        println!("a");
        3.0
    }

    let context = Context::create();
    let module = context.create_module("test");
    let builder = context.create_builder();

    let ft = context.f64_type();
    let fnt = ft.fn_type(&[], false);

    let f = module.add_function("test_fn", fnt, None);
    let b = context.append_basic_block(f, "entry");

    builder.position_at_end(b);

    let extf = module.add_function("sumf", ft.fn_type(&[ft.into(), ft.into()], false), None);

    let argf = ft.const_float(64.);
    let call_site_value = builder.build_call(extf, &[argf.into(), argf.into()], "retv");
    let retv = call_site_value
        .try_as_basic_value()
        .left()
        .unwrap()
        .into_float_value();

    builder.build_return(Some(&retv));

    let mut ee = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
    ee.add_global_mapping(&extf, sumf as usize);
    module.print_to_stderr();

    let result = unsafe { ee.run_function(f, &[]) }.as_float(&ft);
}
