use std::{collections::HashMap, hash::Hash, ptr::NonNull};

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    execution_engine::ExecutionEngine,
    intrinsics::Intrinsic,
    module::{Linkage, Module},
    passes::PassManager,
    types::{BasicType, FloatType, FunctionType, IntType, PointerType, StructType},
    values::{
        BasicValue, BasicValueEnum, FunctionValue, GlobalValue, InstructionValue, IntValue,
        PhiValue, PointerValue, StructValue,
    },
    AddressSpace,
};

use crate::{ast::UMPL2Expr, interior_mut::RC};

use self::env::VarType;
macro_rules! return_none {
    ($expr:expr) => {
        match $expr {
            Some(e) => e,
            _ => return Ok(None),
        }
    };
}
mod conditionals;
mod env;
mod export_code;
mod extract_object;
mod functions;
mod labels;
mod loops;
mod object;
mod quotation;
mod stdlib;

/// needed for when we reach stoppers like stop or skip
/// to tell us what type of code to generate ie, br or return
#[derive(Clone, Copy, Debug)]
pub enum EvalType<'ctx> {
    // for a function since it's just build return we dont need to
    // keep any state from the function function
    Function,
    // for a loop we in case of a stop we need to know which block to branch too
    // and in case of a skip what was the start of a loop (block)
    // probably also need to keep track of function it was created in just in case we have the stopper being called from an inner function/thunk
    Loop {
        loop_bb: BasicBlock<'ctx>,
        done_loop_bb: BasicBlock<'ctx>,
        connection: PhiValue<'ctx>,
    },
}

/// a `HashMap` like thing that has two types of indexes
/// 1) list of indices that can get the value
/// 2) a single index that can set the value
// SAFETY:
// 1) All the NonNull stored in the map are garunteed to be vaild because they are obtained from [Box::into_raw].
// 2) The get and set methods with rust ownership rules garuntee that NonNulls are either have a lot of shared references
// or a single exlusive reference
// in other words this map can be implemented without the types that are generally used for interior mutablity and without unsafe
// by just having the keys map be a map of key to key and then get would first lookup in keys and then the key from keys looks up in the map for values
pub struct MultiMap<K: Hash + Eq, V> {
    keys: HashMap<K, NonNull<V>>,
    values: HashMap<K, NonNull<V>>,
}

impl<K: Hash + Eq, V> MultiMap<K, V> {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            values: HashMap::new(),
        }
    }
    /// allows you te get from on of the multiple keys
    pub fn get(&self, key: &K) -> Option<&V> {
        // SAFETY: were allowed to obtain a shared reference to the value because you can only obtain an exclusive reference to the value
        // if you call set, which we requires an exclusive reference to the MultiMap, which means you cannot also obtain shared references to the value
        // so in short the method signatures of the MultiMap garuntee the safety of using unsafe
        self.keys.get(key).map(|v| unsafe { v.as_ref() })
    }

    /// allows you to mutate a value based on its mutatable key
    pub fn set(&mut self, key: &K, setter: impl FnOnce(&V) -> V) -> Option<()> {
        // SAFETY: were allowed to obtain an exlusive reference to the value because you can only obtain an exclusive reference to the value
        // if you call set, which we requires an exclusive reference to the MultiMap, which means you cannot also obtain shared references to the value or another exlusive reference to the value
        // so in short the method signatures of the MultiMap garuntee the safety of using unsafe
        self.values.get(key).map(|v| unsafe {
            *v.as_ptr() = setter(&*v.as_ptr());
        })
    }

    pub fn get_or_set(
        &mut self,
        key: &K,
        getter: impl FnOnce(&V),
        setter: impl FnOnce(&V) -> V,
    ) -> Option<()> {
        self.get(key)
            .map(|v| {
                getter(v);
            })
            .or_else(|| self.set(key, setter))
    }
}

impl<K: Hash + Eq, V> Drop for MultiMap<K, V> {
    fn drop(&mut self) {
        self.values
            .iter_mut()
            // SAFETY: see from impl
            .for_each(|(_, v)| drop(unsafe { Box::from_raw(v.as_ptr()) }));
    }
}
impl<T: IntoIterator<Item = (KS, K, V)>, K: Hash + Eq + Clone, V, KS: IntoIterator<Item = K>>
    From<T> for MultiMap<K, V>
{
    fn from(value: T) -> Self {
        let (values, keys): (HashMap<K, _>, Vec<_>) = value
            .into_iter()
            .map(|(keys, key, value): (KS, K, V)| {
                let value = Box::into_raw(Box::new(value));
                (
                    // SAFTEY: Box::from_raw does not give us a null pointer
                    (key, unsafe { NonNull::new_unchecked(value) }),
                    keys.into_iter()
                        // SAFTEY: Box::from_raw does not give us a null pointer
                        .map(|keys_outer: K| (keys_outer, unsafe { NonNull::new_unchecked(value) }))
                        .collect::<Vec<_>>(),
                )
            })
            .unzip();

        let keys: HashMap<_, _> = keys.into_iter().flatten().collect();
        Self { keys, values }
    }
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
    pub lambda_ty: FunctionType<'ctx>,
    pub symbol: PointerType<'ctx>,
    pub generic_pointer: PointerType<'ctx>,
    pub hempty: StructType<'ctx>,
    pub thunk: FunctionType<'ctx>,
    thunk_ty: StructType<'ctx>,
    primitive_ty: FunctionType<'ctx>,
    /// {param count, basicbock ptr}
    /// maintains information about a function calish
    /// It is a struct that keeps the number of arguments
    /// and also a pointer to a basic block which the function should jump too (if non null) for (gotos)
    call_info: StructType<'ctx>,
    args: StructType<'ctx>,
}

#[derive(Clone, Copy, Debug)]
/// Important function that the compiler needs to access
pub struct Functions<'ctx> {
    exit: FunctionValue<'ctx>,
    va_procces: FunctionValue<'ctx>,
    printf: FunctionValue<'ctx>,
    rand: FunctionValue<'ctx>,
}

#[allow(missing_debug_implementations)]
pub struct Compiler<'a, 'ctx> {
    context: &'ctx Context,
    pub(crate) module: &'a Module<'ctx>,
    variables: Vec<HashMap<RC<str>, VarType<'a, 'ctx>>>,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub(crate) string: HashMap<RC<str>, GlobalValue<'ctx>>,
    // ident stores all used identifiers that were turned in a llvm string literal
    // so we don't store multiple sof the same identifiers
    pub(crate) ident: HashMap<RC<str>, GlobalValue<'ctx>>,
    fn_value: Option<FunctionValue<'ctx>>,
    links: MultiMap<RC<str>, Option<(PointerValue<'ctx>, FunctionValue<'ctx>)>>,
    pub(crate) types: Types<'ctx>,
    // not were umpl functions are stored
    functions: Functions<'ctx>,
    state: Vec<EvalType<'ctx>>,
    // used to recover where eval was in when evaling from repl
    main: Option<(FunctionValue<'ctx>, BasicBlock<'ctx>)>,
    non_found_links: Vec<(RC<str>, BasicBlock<'ctx>, Option<InstructionValue<'ctx>>)>,
    engine: Option<ExecutionEngine<'ctx>>,
    module_list: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
#[allow(non_camel_case_types)]
/// when updating anything in this enum, remember to update the how object is set in [`Compiler::new`] as it is the only thing that won't automatically reflect changes made here
pub enum TyprIndex {
    #[default]
    Unknown = 100,
    boolean = 0,
    number = 1,
    string = 2,
    cons = 3,
    lambda = 4,
    symbol = 5,
    thunk = 6,
    // TODO: make hempty be 0 so object will be zeroinitilizer if its hempty
    hempty = 7,
    // TODO make primitive things so function like print cons, car .. dont needt to unthunk or take env pointers
    primitive = 8,
}

#[derive(Debug)]
pub enum EngineType {
    // for running in repl mode
    Repl,
    // for when running code once
    Jit,
    // for plain compilation
    None,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        builder: &'a Builder<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
        ee_type: &EngineType,
    ) -> Self {
        let kind = context.opaque_struct_type("object");
        // TODO: make the generic lambda function type not explicitly take an object, and also it should take a number, which signify the amount actual arguments
        // and also it should take a pointer (that if non-null should indirect br to that ptr)
        let call_info = context.struct_type(
            &[
                context.i64_type().into(),
                context.i8_type().ptr_type(AddressSpace::default()).into(),
            ],
            false,
        );
        let generic_pointer = context.i8_type().ptr_type(AddressSpace::default());
        let fn_type = kind.fn_type(
            &[
                generic_pointer.into(),
                call_info.into(),
                generic_pointer.into(),
            ],
            true,
        );

        let lambda = context.struct_type(
            &[
                fn_type.ptr_type(AddressSpace::default()).into(),
                generic_pointer.into(),
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
            lambda_ty: fn_type,
            symbol: context.i8_type().ptr_type(AddressSpace::default()),
            generic_pointer,
            hempty: context.struct_type(&[], false),
            thunk_ty: context.struct_type(
                &[
                    kind.fn_type(&[generic_pointer.into()], false)
                        .ptr_type(AddressSpace::default())
                        .into(),
                    generic_pointer.into(),
                ],
                false,
            ),
            thunk: kind.fn_type(&[generic_pointer.into()], false),
            primitive_ty: kind.fn_type(&[call_info.into(), generic_pointer.into()], false),
            args: context.struct_type(&[kind.into(), generic_pointer.into()], false),
            call_info,
        };
        let exit = module.add_function(
            "exit",
            context
                .void_type()
                .fn_type(&[context.i32_type().into()], false),
            Some(Linkage::External),
        );
        let rand = module.add_function(
            "rand",
            context.i32_type().fn_type(&[], false),
            Some(Linkage::External),
        );
        let printf = module.add_function(
            "printf",
            context.i32_type().fn_type(&[types.string.into()], true),
            Some(Linkage::External),
        );

        let va_procces_type = kind.fn_type(
            &[types.generic_pointer.into(), context.i64_type().into()],
            false,
        );
        let va_procces = module.add_function("va_procces", va_procces_type, None);

        let functions = Functions {
            exit,
            printf,
            rand,
            va_procces,
        };
        kind.set_body(
            &[
                types.ty.as_basic_type_enum(),              //type
                types.boolean.as_basic_type_enum(),         // bool
                types.number.as_basic_type_enum(),          //number
                types.string.as_basic_type_enum(),          // string
                types.generic_pointer.as_basic_type_enum(), // cons (maybee turn it back to 3 elemement struct)
                types.lambda.as_basic_type_enum(),          // function
                types.symbol.as_basic_type_enum(),          // symbol
                types.thunk_ty.as_basic_type_enum(),        // thunk
                types.hempty.as_basic_type_enum(),          //hempty
                types.generic_pointer.as_basic_type_enum(), // primitive function
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
            types,
            links: MultiMap::new(),
            functions,
            state: vec![],
            main: None,
            non_found_links: vec![],
            engine: Self::create_engine(module, ee_type),
            module_list: vec![],
        }
    }

    // because we cannot have multiple engines per module
    fn create_engine(
        module: &'a Module<'ctx>,
        ee_type: &EngineType,
    ) -> Option<ExecutionEngine<'ctx>> {
        match ee_type {
            EngineType::Repl => Some(module.create_execution_engine().unwrap()),
            EngineType::Jit => Some(
                module
                    // optimaztion break goto
                    .create_jit_execution_engine(inkwell::OptimizationLevel::None)
                    .unwrap(),
            ),
            EngineType::None => None,
        }
    }

    pub(crate) fn build_n_select(
        &self,
        default: BasicValueEnum<'ctx>,
        choices: &[(IntValue<'ctx>, BasicValueEnum<'ctx>)],
    ) -> BasicValueEnum<'ctx> {
        choices.iter().fold(default, |prev, choice| {
            self.builder.build_select(choice.0, choice.1, prev, "")
        })
    }

    #[inline]
    fn current_fn_value(&self) -> Result<FunctionValue<'ctx>, &str> {
        self.fn_value.ok_or("could not find current function")
    }
    // / Creates a new stack allocation instruction in the entry block of the function.
    fn create_entry_block_alloca<T>(&self, ty: T, name: &str) -> Result<PointerValue<'ctx>, &str>
    where
        T: BasicType<'ctx>,
    {
        // self.engine.unwrap().
        let old_block = self.builder.get_insert_block();
        let fn_value = self.current_fn_value()?;
        // if a function is already allocated it will have an entry block so its fine to unwrap
        let entry = fn_value.get_first_basic_block().unwrap();

        entry.get_first_instruction().map_or_else(
            || self.builder.position_at_end(entry),
            |first_instr| self.builder.position_before(&first_instr),
        );

        let build_alloca = self.builder.build_alloca(ty, name);
        if let Some(bb) = old_block {
            self.builder.position_at_end(bb);
        }
        Ok(build_alloca)
    }

    fn compile_expr(&mut self, expr: &UMPL2Expr) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        match expr {
            UMPL2Expr::Number(value) => Ok(Some(self.const_number(*value).as_basic_value_enum())),
            UMPL2Expr::Bool(value) => Ok(Some(self.const_boolean(*value).as_basic_value_enum())),
            UMPL2Expr::String(value) => Ok(Some(self.const_string(value).as_basic_value_enum())),
            UMPL2Expr::Ident(s) => self.get_var(s).map(Some),

            UMPL2Expr::Application(application) => self.compile_application(application),

            UMPL2Expr::Label(s) => self.compile_label(s),
            UMPL2Expr::FnParam(s) => self.get_var(&s.to_string().into()).map(Some),
            UMPL2Expr::Hempty => Ok(Some(self.hempty().into())),
        }
    }

    // the reason this is not in loop.rs is because it could be in functions.rs too.
    // maybe we should make control_flow.rs for stop, skip, and labels

    // TODO: keep in mind the fact that the loop might be in outer function
    fn special_form_stop(
        &mut self,
        exprs: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        if exprs.len() != 1 {
            Err("this is an expression oreinted language stopping a loop or function requires a value")?;
        }
        let res = return_none!(self.compile_expr(&exprs[0])?);
        match self
            .state
            .last()
            .ok_or("a stop is found outside a funcion or loop")?
        {
            EvalType::Function => {
                self.builder.build_return(Some(&res));
            }
            EvalType::Loop {
                loop_bb: _,
                done_loop_bb,
                connection,
            } => {
                let cont_bb = self
                    .context
                    .append_basic_block(self.fn_value.unwrap(), "loop-continue");
                self.builder.build_conditional_branch(
                    self.context.bool_type().const_zero(),
                    cont_bb,
                    *done_loop_bb,
                );
                connection.add_incoming(&[(&res, self.builder.get_insert_block().unwrap())]);
                self.builder.position_at_end(cont_bb);
            }
        };
        Ok(None)
    }

    fn special_form_mod(
        &mut self,
        exprs: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        // we should probalby compile with root env as opposed to whatever env the compiler was in when it reached this mod
        // one way to do this is to keep a list of modules with thein envs including one for the root ...
        if exprs.is_empty() {
            Err("Mod requires either a name and scope")?;
        }
        let UMPL2Expr::Ident(module_name) = &exprs[0] else {
            return Err("Mod requires a module name as its first argument")?;
        };
        self.module_list.push(module_name.to_string());
        for expr in &exprs[1..] {
            // self.print_ir();
            self.compile_expr(expr)?;
        }
        self.module_list.pop();
        Ok(Some(self.hempty().into()))
    }

    fn insert_variable_new_ptr(
        &mut self,
        i: &RC<str>,
        v: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        let ty = self.types.object;
        let ptr = self.create_entry_block_alloca(ty, i).unwrap();
        self.builder.build_store(ptr, v);
        self.insert_new_variable(i.clone(), ptr)?;
        Ok(self.types.boolean.const_zero().as_basic_value_enum())
    }

    fn actual_value(&self, thunked: StructValue<'ctx>) -> StructValue<'ctx> {
        // needs entry /condin
        let current_fn = self.fn_value.unwrap();
        let current_bb = self.builder.get_insert_block().unwrap();
        let force = self.context.append_basic_block(current_fn, "force");
        let done_force = self.context.append_basic_block(current_fn, "done-force");

        let ty = self.extract_type(thunked).unwrap().into_int_value();
        let cond = self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            ty,
            self.types.ty.const_int(TyprIndex::thunk as u64, false),
            "is thunk",
        );
        self.builder
            .build_conditional_branch(cond, force, done_force);
        self.builder.position_at_end(force);
        let unthunked = self.extract_thunk(thunked).unwrap().into_struct_value();
        let thunked_fn = self
            .builder
            .build_extract_value(unthunked, 0, "thunk-fn")
            .unwrap();
        let unthunked_env = self
            .builder
            .build_extract_value(unthunked, 1, "thunk-env")
            .unwrap();
        let unthunked = self
            .builder
            .build_indirect_call(
                self.types.thunk,
                thunked_fn.into_pointer_value(),
                &[unthunked_env.into()],
                "unthunk",
            )
            .try_as_basic_value()
            .unwrap_left()
            .into_struct_value();
        self.builder.build_unconditional_branch(done_force);
        let force = self.builder.get_insert_block().unwrap();
        self.builder.position_at_end(done_force);
        // we dont need to reget the block for unthunking because we are only calling a function and nothing elsse that would make another block in between
        let object = self.builder.build_phi(self.types.object, "value");
        object.add_incoming(&[(&thunked, current_bb), (&unthunked, force)]);
        object.as_basic_value().into_struct_value()
    }

    pub fn compile_scope(
        &mut self,
        body: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        let mut res = Err("scope does not have value".to_string());
        for expr in body {
            res = Ok(return_none!(self.compile_expr(expr)?));
        }
        res.map(Some)
    }

    fn is_null(&self, pv: PointerValue<'ctx>) -> IntValue<'ctx> {
        self.builder.build_is_null(pv, "null check")
    }

    fn is_hempty(&self, arg: StructValue<'ctx>) -> inkwell::values::IntValue<'ctx> {
        let arg_type = self.extract_type(arg).unwrap();
        let is_hempty = self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            arg_type.into_int_value(),
            self.types.ty.const_int(TyprIndex::hempty as u64, false),
            "is hempty",
        );
        is_hempty
    }

    fn init_special_forms(&mut self) {
        self.insert_special_form("if".into(), Self::special_form_if);
        self.insert_special_form("loop".into(), Self::special_form_loop);
        self.insert_special_form("for".into(), Self::special_form_for_loop);
        self.insert_special_form("while".into(), Self::special_form_while_loop);
        self.insert_special_form("lambda".into(), Self::special_form_lambda);
        self.insert_special_form("define".into(), Self::special_form_define);
        self.insert_special_form("quote".into(), Self::special_form_quote);
        self.insert_special_form("unquote".into(), Self::special_form_unquote);
        self.insert_special_form("quasiquote".into(), Self::special_form_quasiquote);
        self.insert_special_form("skip".into(), Self::special_form_skip);
        self.insert_special_form("stop".into(), Self::special_form_stop);
        self.insert_special_form("mod".into(), Self::special_form_mod);
        self.insert_special_form("begin".into(), Self::compile_scope);
        self.insert_special_form("set!".into(), Self::special_form_set);
    }

    pub fn get_main(&mut self) -> (FunctionValue<'ctx>, BasicBlock<'ctx>) {
        if let Some((function, block)) = self.main {
            (function, block)
        } else {
            self.new_env();
            self.init_special_forms();
            self.init_stdlib();
            self.make_va_process();
            self.new_env();
            let main_fn_type = self.context.i32_type().fn_type(&[], false);
            let main_fn = self.module.add_function("main", main_fn_type, None);
            let main_block = self.context.append_basic_block(main_fn, "entry");
            let inner_main_fn_type = self.context.i32_type().fn_type(
                &[
                    self.types.generic_pointer.into(),
                    self.types.call_info.into(),
                    self.types.generic_pointer.into(),
                ],
                false,
            );
            let inner_main_fn = self.module.add_function("main", inner_main_fn_type, None);
            self.builder.position_at_end(main_block);
            self.builder.build_return(Some(
                &self
                    .builder
                    .build_call(
                        inner_main_fn,
                        &[
                            self.types.generic_pointer.const_null().into(),
                            self.types
                                .call_info
                                .const_named_struct(&[
                                    self.context.i64_type().const_zero().into(),
                                    self.types.generic_pointer.const_null().into(),
                                ])
                                .into(),
                            self.types.generic_pointer.const_null().into(),
                        ],
                        "main",
                    )
                    .try_as_basic_value()
                    .unwrap_left(),
            ));

            let main_block = self.context.append_basic_block(inner_main_fn, "entry");
            self.builder.position_at_end(main_block);
            let call_info = inner_main_fn.get_nth_param(1).unwrap().into_struct_value();
            let jmp_block = self
                .builder
                .build_extract_value(call_info, 1, "basic block address")
                .unwrap()
                .into_pointer_value();
            let jump_bb = self.context.append_basic_block(inner_main_fn, "not-jmp");
            let cont_bb = self
                .context
                .append_basic_block(inner_main_fn, "normal evaluation");
            let is_jmp = self.builder.build_int_compare(
                inkwell::IntPredicate::NE,
                jmp_block,
                self.types.generic_pointer.const_null(),
                "is null",
            );
            self.builder
                .build_conditional_branch(is_jmp, jump_bb, cont_bb);
            self.builder.position_at_end(jump_bb);
            self.builder.build_indirect_branch(jmp_block, &[]);
            self.builder.position_at_end(cont_bb);
            self.main = Some((inner_main_fn, cont_bb));
            (inner_main_fn, cont_bb)
        }
    }

    pub fn resolve_links(&mut self) {
        self.non_found_links = std::mem::take(&mut self.non_found_links)
            .into_iter()
            .filter(|link| {
                if let Some(Some(link_info)) = self.links.get(&link.0) {
                    // link.2 shouldnt be none b/c of place holder
                    if let Some(inst) = link.2 {
                        self.builder.position_at(link.1, &inst);
                    }
                    let call_info = self.types.call_info.const_named_struct(&[
                        self.context.i64_type().const_zero().into(),
                        link_info.0.into(),
                    ]);

                    self.builder.build_call(
                        link_info.1,
                        &[
                            self.types.generic_pointer.const_null().into(),
                            call_info.into(),
                            self.types.generic_pointer.const_null().into(),
                        ],
                        "jump",
                    );
                    false
                } else {
                    println!(
                        "Warning link {} not resolved bad behaviour may occur",
                        link.0
                    );
                    true
                }
            })
            .collect();
    }

    pub fn compile_program(
        &mut self,
        program: &[UMPL2Expr],
        links: HashMap<RC<str>, Vec<RC<str>>>,
    ) -> Option<String> {
        // TODO: dont reset links instead add to current (needed for repl to work)
        self.links = MultiMap::from(links.into_iter().map(|(k, ks)| {
            (
                ks,
                k,
                <Option<(PointerValue<'ctx>, FunctionValue<'ctx>)>>::None,
            )
        }));
        let (main_fn, main_block) = self.get_main();
        self.fn_value = Some(main_fn);

        self.builder.position_at_end(main_block);
        if let Some(term) = main_block.get_terminator() {
            term.erase_from_basic_block();
        }

        for expr in program {
            match self.compile_expr(expr) {
                Ok(_) => continue,
                Err(e) => return Some(e),
            }
        }
        let insert_bb = self.builder.get_insert_block().unwrap();
        self.resolve_links();
        self.builder.position_at_end(insert_bb);
        self.builder
            .build_return(Some(&self.context.i32_type().const_zero()));
        self.main = Some((main_fn, self.builder.get_insert_block().unwrap()));

        let verify = main_fn.verify(true);

        if verify {
            self.fpm.run_on(&main_fn);
            let fpm = PassManager::create(());
            // TODO: more optimizations
            fpm.add_function_inlining_pass();
            fpm.add_merge_functions_pass();
            fpm.add_global_dce_pass();
            fpm.add_ipsccp_pass();
            // makes hard to debug llvm ir
            // fpm.add_strip_symbol_pass();
            fpm.add_constant_merge_pass();

            fpm.add_new_gvn_pass();
            fpm.add_instruction_combining_pass();
            fpm.add_reassociate_pass();
            fpm.add_gvn_pass();
            fpm.add_basic_alias_analysis_pass();
            fpm.add_promote_memory_to_register_pass();
            fpm.add_aggressive_inst_combiner_pass();
            // // doesn't work with current goto implementation
            // // fpm.add_cfg_simplification_pass();
            fpm.add_aggressive_dce_pass();
            fpm.add_instruction_simplify_pass();
            fpm.add_function_inlining_pass();
            fpm.add_strip_dead_prototypes_pass();

            fpm.run_on(self.module);
            None
        } else {
            println!("error occurred");
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
    pub fn run(&self) -> Option<i32> {
        unsafe {
            self.engine.as_ref().map(|engine| {
                // still need better soloution for only executing only need code inr repl
                // remove module from ee so we can reuse
                engine.remove_module(self.module).unwrap();
                // add module back to ee
                engine.add_module(self.module).unwrap();
                engine.run_function_as_main(self.module.get_function("main").unwrap(), &[])
            })
        }
    }

    pub fn exit(&self, reason: &str, code: i32) {
        self.builder.build_call(
            self.functions.printf,
            &[self
                .builder
                .build_global_string_ptr(reason, "error exit")
                .as_basic_value_enum()
                .into()],
            "print",
        );
        self.builder.build_call(
            self.functions.exit,
            &[self.context.i32_type().const_int(code as u64, false).into()],
            "exit",
        );

        self.builder.build_unreachable();
    }
}
