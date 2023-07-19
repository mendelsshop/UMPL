use std::{
    collections::HashMap,
    ffi::{c_char, c_void},
    fmt,
};

use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::ExecutionEngine,
    module::Module,
    passes::PassManager,
    types::{BasicType, FunctionType, StructType},
    values::{
        BasicValue, BasicValueEnum, FloatValue, FunctionValue, GlobalValue, IntValue, PointerValue,
        StructValue,
    },
    AddressSpace,
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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Object {
    kind: TyprIndex,
    object: UntaggedObject,
}

impl Object {
    pub extern "C" fn print(self) -> Self {
        println!("{self}");
        self
    }

    pub extern "C" fn extract_num(self) -> f64 {
        if self.kind != TyprIndex::Number {
            panic!()
        }

        unsafe { self.object.number }
    }

    pub extern "C" fn from_number(number: f64) -> Self {
        Self {
            kind: TyprIndex::Number,
            object: UntaggedObject { number },
        }
    }

    pub extern "C" fn extract_str(self) -> *const i8 {
        if self.kind != TyprIndex::String {
            panic!()
        }

        unsafe { self.object.string }
    }

    pub extern "C" fn from_str(string: *const i8) -> Self {
        Self {
            kind: TyprIndex::String,
            object: UntaggedObject { string },
        }
    }

    pub extern "C" fn extract_bool(self) -> bool {
        if self.kind != TyprIndex::Boolean {
            panic!()
        }

        unsafe { self.object.bool }
    }

    pub extern "C" fn from_bool(bool: bool) -> Self {
        Self {
            kind: TyprIndex::Boolean,
            object: UntaggedObject { bool },
        }
    }

    pub extern "C" fn extract_type(self) -> i32 {
        self.kind as i32
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            match self.kind {
                TyprIndex::String => write!(
                    f,
                    "{}",
                    CStr::from_ptr(self.object.string).to_str().unwrap()
                ),
                TyprIndex::Number => write!(f, "{}", self.object.number),
                TyprIndex::Boolean => write!(f, "{}", self.object.bool),
                TyprIndex::Lambda => todo!(),
            }
        }
    }
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
    bool: bool,
    lambda: *mut Function,
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
    jit: ExecutionEngine<'ctx>,
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
        let jit = module
            .create_jit_execution_engine(inkwell::OptimizationLevel::None)
            .unwrap();
        let kind = context.struct_type(
            &[
                context.i32_type().as_basic_type_enum(),
                context
                    .i8_type()
                    .ptr_type(AddressSpace::default())
                    .as_basic_type_enum(),
            ],
            false,
        );
        jit.add_global_mapping(
            &module.add_function(
                "extract_number",
                context
                    .f64_type()
                    .fn_type(vec![kind.into()].as_slice(), false),
                None,
            ),
            Object::extract_num as usize,
        );
        jit.add_global_mapping(
            &module.add_function(
                "from_number",
                kind.fn_type(vec![context.f64_type().into()].as_slice(), false),
                None,
            ),
            Object::from_number as usize,
        );
        jit.add_global_mapping(
            &module.add_function(
                "print",
                kind.fn_type(vec![kind.into()].as_slice(), false),
                None,
            ),
            Object::print as usize,
        );
        jit.add_global_mapping(
            &module.add_function(
                "extract_bool",
                context
                    .bool_type()
                    .fn_type(vec![kind.into()].as_slice(), false),
                None,
            ),
            Object::extract_bool as usize,
        );
        jit.add_global_mapping(
            &module.add_function(
                "from_bool",
                kind.fn_type(vec![context.bool_type().into()].as_slice(), false),
                None,
            ),
            Object::from_bool as usize,
        );
        jit.add_global_mapping(
            &module.add_function(
                "extract_string",
                context
                    .i8_type()
                    .ptr_type(AddressSpace::default())
                    .as_basic_type_enum()
                    .fn_type(vec![kind.into()].as_slice(), false),
                None,
            ),
            Object::extract_str as usize,
        );
        jit.add_global_mapping(
            &module.add_function(
                "from_string",
                kind.fn_type(
                    vec![context
                        .i8_type()
                        .ptr_type(AddressSpace::default())
                        .as_basic_type_enum()
                        .into()]
                    .as_slice(),
                    false,
                ),
                None,
            ),
            Object::from_str as usize,
        );
        jit.add_global_mapping(
            &module.add_function(
                "extract_type",
                context
                    .i32_type()
                    .fn_type(vec![kind.into()].as_slice(), false),
                None,
            ),
            Object::extract_type as usize,
        );
        Self {
            context,
            module,
            variables: HashMap::new(),
            builder,
            fpm,
            string: HashMap::new(),
            kind,
            fn_value: None,
            jit,
        }
    }

    fn number(&self, value: FloatValue<'ctx>) -> StructValue<'ctx> {
        let from_number = self.module.get_function("from_number").unwrap();
        let call = self
            .builder
            .build_call(from_number, &[value.into()], "to number");
        call.try_as_basic_value().unwrap_left().into_struct_value()
    }

    fn const_number(&self, value: f64) -> StructValue<'ctx> {
        self.number(self.context.f64_type().const_float(value))
    }

    fn bool(&self, value: IntValue<'ctx>) -> StructValue<'ctx> {
        let from_bool = self.module.get_function("from_bool").unwrap();
        let call = self
            .builder
            .build_call(from_bool, &[value.into()], "to bool");
        call.try_as_basic_value().unwrap_left().into_struct_value()
    }

    fn const_bool(&self, value: Boolean) -> StructValue<'ctx> {
        self.bool(self.context.bool_type().const_int(value as u64, false))
    }

    fn string(&self, value: PointerValue<'ctx>) -> StructValue<'ctx> {
        let from_string = self.module.get_function("from_string").unwrap();
        let call = self
            .builder
            .build_call(from_string, &[value.into()], "to string");
        call.try_as_basic_value().unwrap_left().into_struct_value()
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
                let args = return_none!(application
                    .args()
                    .iter()
                    .skip(1)
                    .map(|expr| self.compile_expr(expr))
                    .collect::<Result<Option<Vec<_>>, _>>()?);
                Ok(Some(
                    match op {
                        // TODO shortent these
                        FnKeyword::Add => self.number(
                            self.builder.build_float_add(
                                self.extract_number(args[0].into_struct_value())
                                    .unwrap()
                                    .into_float_value(),
                                self.extract_number(args[1].into_struct_value())
                                    .unwrap()
                                    .into_float_value(),
                                "tmpadd",
                            ),
                        ),
                        FnKeyword::Sub => self.number(
                            self.builder.build_float_sub(
                                self.extract_number(args[0].into_struct_value())
                                    .unwrap()
                                    .into_float_value(),
                                self.extract_number(args[1].into_struct_value())
                                    .unwrap()
                                    .into_float_value(),
                                "tmpsub",
                            ),
                        ),
                        FnKeyword::Mul => self.number(
                            self.builder.build_float_mul(
                                self.extract_number(args[0].into_struct_value())
                                    .unwrap()
                                    .into_float_value(),
                                self.extract_number(args[1].into_struct_value())
                                    .unwrap()
                                    .into_float_value(),
                                "tmpmul",
                            ),
                        ),
                        FnKeyword::Div => self.number(
                            self.builder.build_float_div(
                                self.extract_number(args[0].into_struct_value())
                                    .unwrap()
                                    .into_float_value(),
                                self.extract_number(args[1].into_struct_value())
                                    .unwrap()
                                    .into_float_value(),
                                "tmpdiv",
                            ),
                        ),
                        FnKeyword::Mod => todo!(),
                        FnKeyword::Print => self.print(args[0]).into_struct_value(),
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
                self.variables.insert(i.clone(), ptr);
                return Ok(Some(
                    self.context.bool_type().const_zero().as_basic_value_enum(),
                ));
            }
        }
    }

    fn extract_type(&mut self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        let get_type = self.module.get_function("extract_type").unwrap();
        let call = self
            .builder
            .build_call(get_type, &[cond_struct.into()], "to type");
        Some(call.try_as_basic_value().unwrap_left())
    }

    // TODO: for all extract_* methods have checked variants that check that what is trying to be obtained is in fact the type of the object
    fn extract_bool(&mut self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        let get_type = self.module.get_function("extract_bool").unwrap();
        let call = self
            .builder
            .build_call(get_type, &[cond_struct.into()], "to bool");
        Some(call.try_as_basic_value().unwrap_left())
    }

    fn extract_string(&self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        let get_type = self.module.get_function("extract_string").unwrap();
        let call = self
            .builder
            .build_call(get_type, &[cond_struct.into()], "to string");
        Some(call.try_as_basic_value().unwrap_left())
    }

    fn extract_number(&self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        let get_type = self.module.get_function("extract_number").unwrap();
        let call = self
            .builder
            .build_call(get_type, &[cond_struct.into()], "to number");
        Some(call.try_as_basic_value().unwrap_left())
    }

    fn extract_function(&mut self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        todo!()
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
            .build_load(*self.variables.get(s).ok_or(format!("{s} not found"))?, s))
    }

    pub fn compile_program(&mut self, program: &[UMPL2Expr]) -> Option<String> {
        let main_fn_type = self.context.i32_type().fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_fn_type, None);
        let main_block = self.context.append_basic_block(main_fn, "entry");

        self.builder.position_at_end(main_block);
        for expr in program {
            match self.compile_expr(expr) {
                Ok(_) => continue,
                Err(e) => return Some(e),
            }
        }

        self.builder
            .build_return(Some(&self.context.i32_type().const_zero()));
        None
    }

    pub fn print_ir(&self) {
        self.module.print_to_stderr();
    }

    pub fn run(&self) -> u64 {
        let main = self.module.get_function("main").unwrap();
        unsafe { self.jit.run_function(main, &[]) }.as_int(false)
    }

    fn print(&self, args: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        let print = self.module.get_function("print").unwrap();
        let call = self.builder.build_call(print, &[args.into()], "print");
        call.try_as_basic_value().unwrap_left()
    }
}
