use std::{collections::HashMap, hash::BuildHasher};

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    types::{FunctionType, PointerType, StructType},
    values::{
        AggregateValue, BasicValue, BasicValueEnum, FunctionValue, IntValue, PointerValue,
        StructValue,
    },
    AddressSpace, IntPredicate,
};
use inkwell::{module::Linkage, types::BasicTypeEnum};
use itertools::Itertools;

use super::sicp::{Const, Expr, Goto, Instruction, Operation, Perform, Register};

macro_rules! fixed_map {
    (@inner $(#[$attrs:meta])* $struct:ident, <$($gen:tt),*>, $type:ty, $index:ty {$($fields:ident)*} fn $new:ident($($param:ident: $param_type:ty),*) -> $ret:ty $new_block:block) => {
        $(#[$attrs])*
        pub struct $struct<$($gen),*> {
            $(
                $fields: $type,
            )*
        }

        impl <$($gen),*> $struct<$($gen),*> {
            pub fn $new($($param: $param_type),*) -> $ret $new_block
            pub const fn get(&self, index: $index) -> $type {
                match index {
                    $(
                        <$index>::$fields => self.$fields,
                    )*
                }
            }
        }
    };

($(#[$attrs:meta])* $struct:ident,$type:ident<$($gen:tt),*>, $index:ty {$($fields:ident)*} fn $new:ident($($param:ident: $param_type:ty),*) -> $ret:ty $new_block:block ) => {
        fixed_map!(@inner $(#[$attrs])* $struct,<$($gen),*> , $type<$($gen),*>, $index {$($fields)*} fn $new($($param: $param_type),*) -> $ret $new_block);
    };
    ($(#[$attrs:meta])* $struct:ident,$type:ident $index:ty {$($fields:ident)*} fn $new:ident($($param:ident: $param_type:ty),*) -> $ret:ty $new_block:block ) => {
        fixed_map!(@inner $(#[$attrs])* $struct,<> , $type<>, $index {$($fields)*} fn $new($($param: $param_type),*) -> $ret $new_block);
    };

}

macro_rules! extract {
    ($fn_name:ident, $unchecked:ident, $type:ident, $name:literal) => {
        pub(super) fn $fn_name(&self, val: StructValue<'ctx>) -> BasicValueEnum<'ctx> {
            let current_fn = self.main;
            let prefix = |end| format!("extract-{}:{end}", $name);
            let ret_block = self
                .context
                .append_basic_block(current_fn, &prefix("return"));

            let ty = self.extract_type(val).unwrap().into_int_value();
            let condition = self.builder.build_int_compare(
                inkwell::IntPredicate::EQ,
                ty,
                self.context
                    .i32_type()
                    .const_int(TypeIndex::$type as u64, false),
                &prefix("cmp-type"),
            );
            self.set_error(&format!("type mismtatch expected {}\n", $name), 1);
            self.builder
                .build_conditional_branch(condition, ret_block, self.error_block);

            self.builder.position_at_end(ret_block);
            self.$unchecked(val)
        }
        pub(super) fn $unchecked(&self, val: StructValue<'ctx>) -> BasicValueEnum<'ctx> {
            let current_fn = self.main;
            let prefix = |end| format!("extract-{}:{end}", $name);
            let pointer = self
                .builder
                .build_extract_value(val, 1, &prefix("return"))
                .unwrap();
            (self.builder.build_load(
                self.types.types.get(TypeIndex::$type),
                pointer.into_pointer_value(),
                &prefix(""),
            ))
        }
    };
}

fixed_map!(TypeMap, BasicTypeEnum<'ctx>,TypeIndex {empty bool number string symbol label cons primitive}
      fn new(
        empty: BasicTypeEnum<'ctx>,
        bool: BasicTypeEnum<'ctx>,
        number: BasicTypeEnum<'ctx>,
        string: BasicTypeEnum<'ctx>,
        symbol: BasicTypeEnum<'ctx>,
        label: BasicTypeEnum<'ctx>,
        cons: BasicTypeEnum<'ctx>,
        primitive: BasicTypeEnum<'ctx>
    ) -> Self {
        Self {
            empty,
            bool,
            number,
            string,
            symbol,
            label,
            cons,
            primitive,
        }
    }
);

fixed_map!(#[allow(non_snake_case)]RegiMap, PointerValue<'ctx>,Register {Env Argl Val Proc Continue}
    fn new(builder: &Builder<'ctx>, ty: StructType<'ctx>) -> Self {
        let create_register = |name| builder.build_alloca(ty, name);
        Self {
            Env: create_register("env"),
            Argl: create_register("argl"),
            Val: create_register("val"),
            Proc: create_register("proc"),
            Continue: create_register("continue"),
        }
    }

);

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum TypeIndex {
    empty = 0,
    bool = 1,
    number = 2,
    string = 3,
    symbol = 4,
    label = 5,
    cons = 6,
    primitive = 7,
}

pub struct Types<'ctx> {
    object: StructType<'ctx>,
    string: StructType<'ctx>,
    cons: StructType<'ctx>,
    pointer: PointerType<'ctx>,
    types: TypeMap<'ctx>,
    stack: StructType<'ctx>,
    primitive: FunctionType<'ctx>,
    error: StructType<'ctx>,
}
/// Important function that the compiler needs to access
pub struct Functions<'ctx> {
    exit: FunctionValue<'ctx>,
    strncmp: FunctionValue<'ctx>,
    printf: FunctionValue<'ctx>,
    rand: FunctionValue<'ctx>,
}

impl<'ctx> Functions<'ctx> {
    pub fn new(module: &Module<'ctx>, context: &'ctx Context) -> Self {
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
            context.i32_type().fn_type(
                &[context.bool_type().ptr_type(AddressSpace::default()).into()],
                true,
            ),
            Some(Linkage::External),
        );
        let strncmp = module.add_function(
            "strncmp",
            context.i32_type().fn_type(
                &[
                    context.i8_type().ptr_type(AddressSpace::default()).into(),
                    context.i8_type().ptr_type(AddressSpace::default()).into(),
                    context.i32_type().into(),
                ],
                false,
            ),
            Some(Linkage::External),
        );
        Self {
            exit,
            printf,
            rand,
            strncmp,
        }
    }
}

macro_rules! make_accessors {
    ($name:ident $outer:ident $inner:ident) => {
        pub fn $name(&self, cons: StructValue<'ctx>) -> StructValue<'ctx> {
            self.$outer(self.$inner(cons))
        }
    };
}

macro_rules! is_type {
    ($name:ident,$type:literal,$typeindex:ident) => {
        fn $name(&self, obj: StructValue<'ctx>) -> IntValue<'ctx> {
            let arg_type = self.extract_type(obj).unwrap();
            self.builder.build_int_compare(
                inkwell::IntPredicate::EQ,
                arg_type.into_int_value(),
                self.context
                    .i32_type()
                    .const_int(TypeIndex::$typeindex as u64, false),
                "is hempty",
            )
        }
    };
}
pub struct CodeGen<'a, 'ctx> {
    context: &'ctx Context,
    builder: &'a Builder<'ctx>,
    module: &'a Module<'ctx>,
    main: FunctionValue<'ctx>,
    labels: HashMap<String, BasicBlock<'ctx>>,
    registers: RegiMap<'ctx>,
    types: Types<'ctx>,
    functions: Functions<'ctx>,
    flag: PointerValue<'ctx>,
    fpm: &'a PassManager<FunctionValue<'ctx>>,
    stack: PointerValue<'ctx>,
    error_block: BasicBlock<'ctx>,
    error_phi: inkwell::values::PhiValue<'ctx>,
}

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    fn extract_type(&self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        self.builder.build_extract_value(cond_struct, 0, "get_type")
    }
    is_type!(is_hempty, "hempty", empty);
    is_type!(is_number, "number", number);
    is_type!(is_primitive, "primitive", primitive);
    is_type!(is_boolean, "boolean", bool);
    is_type!(is_string, "string", string);
    is_type!(is_symbol, "symbol", symbol);
    is_type!(is_cons, "cons", cons);
    is_type!(is_label, "label", label);
    extract!(
        get_primitive,
        unchecked_get_primitive,
        primitive,
        "primitive"
    );
    extract!(get_label, unchecked_get_label, label, "label");
    extract!(get_cons, unchecked_get_cons, cons, "cons");
    extract!(get_symbol, unchecked_get_symbol, symbol, "symbol");
    pub fn new(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
    ) -> Self {
        let pointer = context.bool_type().ptr_type(AddressSpace::default());
        let object = context.struct_type(&[context.i32_type().into(), pointer.into()], false);
        let string = context.struct_type(&[context.i32_type().into(), pointer.into()], false);
        let cons = context.struct_type(&[pointer.into(), pointer.into()], false);
        let stack = context.struct_type(&[object.into(), pointer.into()], false);
        let primitive_type = object.fn_type(&[object.into()], false);
        let error = context.struct_type(&[pointer.into(), context.i32_type().into()], false);
        let types = Types {
            object,
            string,
            cons,
            pointer,
            stack,
            primitive: primitive_type,
            error,
            types: TypeMap::new(
                context.struct_type(&[], false).into(),
                context.bool_type().into(),
                context.f64_type().into(),
                string.into(),
                string.into(),
                pointer.into(),
                cons.into(),
                primitive_type.ptr_type(AddressSpace::default()).into(),
            ),
        };
        let functions = Functions::new(module, context);
        // simulating a primitive function until i define the actual primitives
        let primitive_newline = module.add_function("newline", primitive_type, None);
        {
            let entry = context.append_basic_block(primitive_newline, "entry");
            builder.position_at_end(entry);
            builder.build_call(
                functions.printf,
                &[builder
                    .build_global_string_ptr("\n", "\n")
                    .as_pointer_value()
                    .into()],
                "call newline",
            );
            builder.build_return(Some(&primitive_newline.get_first_param().unwrap()));
        }
        // the empty environment
        let main = module.add_function("main", context.i32_type().fn_type(&[], false), None);
        let entry_bb = context.append_basic_block(main, "entry");
        let error_block = context.append_basic_block(main, "error");
        builder.position_at_end(error_block);
        // error phi
        let error_phi = builder.build_phi(error, "error phi");
        {
            let error_msg = builder
                .build_extract_value(
                    error_phi.as_basic_value().into_struct_value(),
                    0,
                    "error_msg",
                )
                .unwrap();
            let error_code = builder
                .build_extract_value(
                    error_phi.as_basic_value().into_struct_value(),
                    1,
                    "error_code",
                )
                .unwrap();
            builder.build_call(functions.printf, &[error_msg.into()], "print");
            builder.build_return(Some(&error_code));
        }

        builder.position_at_end(entry_bb);
        let registers = RegiMap::new(builder, object);
        let this = Self {
            stack: builder.build_alloca(stack, "stack"),
            context,
            flag: builder.build_alloca(types.object, "flag"),
            builder,
            module,
            main,
            labels: HashMap::new(),
            registers,
            types,
            fpm,
            functions,
            error_phi,
            error_block,
        };
        // adding a dummy primitive to the environment so defining variable (define ...) dont freak out the environment starts off as ()
        let primitive_print_ptr = primitive_newline.as_global_value().as_pointer_value();
        let primitive_newline = this.make_object(&primitive_print_ptr, TypeIndex::primitive);
        let primiitve_env = this.make_cons(
            this.make_cons(this.create_string("newline".to_string()), this.empty()),
            this.make_cons(primitive_newline, this.empty()),
        );
        let env = this.make_cons(primiitve_env, this.empty());

        builder.build_store(this.registers.get(Register::Env), env);
        this
    }

    // TODO: maybe make primitives be a block rather that a function and instead of doing an exit + unreachable with errors we could have an error block with a phi for for error string and ret code,
    // or maybe still use functiions for primitives but try to understand trampolines
    // the advantage of this is that the optimizer wouldn't get sometimes confused by unreacahbles, the disadvantage is if we go the primitive as block way is that  we have to use indirectbr for going to the primitive
    // currentyl we use error block approach with primitives being functions, and that just means that primitives must do exit + uncreachable on their own

    pub fn set_error(&self, reason: &str, code: i32) {
        self.error_phi.add_incoming(&[(
            &self.types.error.const_named_struct(&[
                self.builder
                    .build_global_string_ptr(reason, "error exit")
                    .as_basic_value_enum()
                    .into(),
                self.context.i32_type().const_int(code as u64, false).into(),
            ]),
            self.builder.get_insert_block().unwrap(),
        )]);
    }
    pub fn export_ir(&self) -> String {
        self.module.to_string()
    }

    pub fn compile(&mut self, instructions: Vec<Instruction>) {
        instructions
            .into_iter()
            // we scan looking for the labels and create new blocks for each label
            // we do not remove the label instruction b/c when we compile each instruction
            // we need to know when to start compiling un der a new block
            .inspect(|x| {
                if let Instruction::Label(l) = x {
                    let v = self.context.append_basic_block(self.main, l);
                    self.labels.insert(l.clone(), v);
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|inst| {
                self.compile_instructions(inst);
            });
        self.builder
            .build_return(Some(&self.context.i32_type().const_zero()));
        println!("{}", self.export_ir());
        self.fpm.run_on(&self.main);
        let fpm = PassManager::create(());
        // TODO: more optimizations
        // fpm.add_function_inlining_pass();
        // fpm.add_merge_functions_pass();
        fpm.add_global_dce_pass();
        fpm.add_ipsccp_pass();
        // makes hard to debug llvm ir
        // fpm.add_strip_symbol_pass();
        fpm.add_constant_merge_pass();

        fpm.add_new_gvn_pass();
        // fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();
        fpm.add_gvn_pass();
        fpm.add_basic_alias_analysis_pass();
        fpm.add_promote_memory_to_register_pass();
        fpm.add_aggressive_inst_combiner_pass();
        // // doesn't work with current goto implementation
        // fpm.add_cfg_simplification_pass();
        // fpm.add_aggressive_dce_pass();
        // fpm.add_function_inlining_pass();
        fpm.add_strip_dead_prototypes_pass();

        fpm.run_on(self.module);
    }

    fn truthy(&self, val: StructValue<'ctx>) -> IntValue<'ctx> {
        let ty = self.extract_type(val).unwrap().into_int_value();
        let is_not_bool = self.builder.build_int_compare(
            IntPredicate::NE,
            ty,
            self.context
                .i32_type()
                .const_int(TypeIndex::bool as u64, false),
            "not bool check",
        );
        let value = self
            .builder
            .build_extract_value(val, 1, "get object context")
            .unwrap();
        let value = self.builder.build_load(
            self.types.types.get(TypeIndex::bool),
            value.into_pointer_value(),
            "get bool value",
        );
        self.builder
            .build_or(is_not_bool, value.into_int_value(), "non bool or true")
    }

    fn compile_instructions(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Assign(r, e) => {
                let register = self.registers.get(r);
                let expr = self.compile_expr(e);
                self.builder.build_store(register, expr);
            }
            Instruction::Test(p) => {
                self.builder.build_store(self.flag, self.compile_perform(p));
            }
            Instruction::Branch(l) => {
                let flag = self
                    .builder
                    .build_load(self.types.object, self.flag, "load flag");
                // TODO: hack technically (in most cases) if we have a branch, the next instruction after it will be a label so we dont need a new block for each branch but rather we should either "peek" ahead to the next instruction to obtain the label
                // or encode the label as part of the branch variant
                let next_label = self.context.append_basic_block(self.main, "next-block");
                self.builder.build_conditional_branch(
                    self.truthy(flag.into_struct_value()),
                    *self.labels.get(&l).unwrap(),
                    next_label,
                );
                self.builder.position_at_end(next_label);
            }
            Instruction::Goto(g) => {
                match g {
                    Goto::Label(l) => {
                        self.builder
                            .build_unconditional_branch(*self.labels.get(&l).unwrap());
                    }
                    Goto::Register(r) => {
                        let register = self.registers.get(r);
                        let register =
                            self.builder
                                .build_load(self.types.object, register, "load register");
                        let label = self
                            .get_label(register.into_struct_value())
                            .into_pointer_value();
                        self.builder
                            // we need all possible labels as destinations b/c indirect br requires a destination but we dont which one at compile time so we use all of them - maybe fixed with register_to_llvm_more_opt
                            .build_indirect_branch(
                                label,
                                &self.labels.values().copied().collect_vec(),
                            );
                    }
                }
                // we create new block/label b/c goto should be last instruction for a block so this "dummy" label acts as a catch all for anything afterwords
                // realy only needed b/c for label instruction we assume we should just br to the label, but if we digit goto followed by new label we would have double br
                // note wwe mahe similiar problem with branch
                let next_label = self.context.append_basic_block(self.main, "next-block");
                self.builder.position_at_end(next_label)
            }
            Instruction::Save(reg) => {
                let prev_stack =
                    self.builder
                        .build_load(self.types.stack, self.stack, "load stack");
                let prev_stack_ptr = self
                    .builder
                    .build_alloca(self.types.stack, "previous stack");
                self.builder.build_store(prev_stack_ptr, prev_stack);
                let new_stack = self.types.stack.const_zero();
                let new_stack = self
                    .builder
                    .build_insert_value(new_stack, self.registers.get(reg), 0, "save register")
                    .unwrap();

                let new_stack = self
                    .builder
                    .build_insert_value(new_stack, prev_stack_ptr, 1, "save previous stack")
                    .unwrap();
                self.builder.build_store(self.stack, new_stack);
            }
            Instruction::Restore(reg) => {
                let stack = self
                    .builder
                    .build_load(self.types.stack, self.stack, "stack");
                let old_stack = self
                    .builder
                    .build_extract_value(stack.into_struct_value(), 1, "old stack")
                    .unwrap();
                let current = self
                    .builder
                    .build_extract_value(stack.into_struct_value(), 0, "current stack")
                    .unwrap();
                self.builder.build_store(self.registers.get(reg), current);
                let old_stack = self.builder.build_load(
                    self.types.stack,
                    old_stack.into_pointer_value(),
                    "load previous stack",
                );
                self.builder.build_store(self.stack, old_stack);
            }
            Instruction::Perform(p) => {
                self.compile_perform(p);
            }
            Instruction::Label(l) => {
                let label = self.labels.get(&l);
                self.builder.build_unconditional_branch(*label.unwrap());
                self.builder.position_at_end(*self.labels.get(&l).unwrap());
            }
        }
    }

    fn compile_expr(&mut self, expr: Expr) -> StructValue<'ctx> {
        match expr {
            Expr::Const(c) => self.compile_const(c),
            Expr::Label(l) => {
                let label_address = unsafe { self.labels.get(&l).unwrap().get_address() }.unwrap();
                self.make_object(&label_address, TypeIndex::label)
            }
            Expr::Register(r) => {
                let reg = self.registers.get(r);
                self.builder
                    .build_load(self.types.object, reg, "load register")
                    .into_struct_value()
            }
            Expr::Op(p) => self.compile_perform(p),
        }
    }

    // lookup variable for set! and plain variable lookup
    // returns a tuple contatining the cons of the found variable and the rest of the vars in the frame
    fn lookup_variable(&self, var: StructValue<'ctx>, env: StructValue<'ctx>) -> StructValue<'ctx> {
        let lookup_entry_bb = self.context.append_basic_block(self.main, "lookup-entry");
        let lookup_bb = self.context.append_basic_block(self.main, "lookup");
        let scan_bb = self.context.append_basic_block(self.main, "scan");
        let next_env_bb = self.context.append_basic_block(self.main, "next-env");
        let check_bb = self.context.append_basic_block(self.main, "scan");
        let found_bb = self.context.append_basic_block(self.main, "found");
        let scan_next_bb = self.context.append_basic_block(self.main, "scan-next");

        let env_ptr = self.builder.build_alloca(self.types.object, "env");
        self.builder.build_store(env_ptr, env);
        self.builder.build_unconditional_branch(lookup_entry_bb);

        self.builder.position_at_end(lookup_entry_bb);
        self.set_error("unbound variable", 1);
        let env_load = self
            .builder
            .build_load(self.types.object, env_ptr, "load env");
        self.builder.build_conditional_branch(
            self.is_hempty(env_load.into_struct_value()),
            self.error_block,
            lookup_bb,
        );

        self.builder.position_at_end(lookup_bb);
        let frame = self.make_unchecked_car(env_load.into_struct_value());
        let vars_pointer = self.builder.build_alloca(self.types.object, "vars");
        let vals_pointer = self.builder.build_alloca(self.types.object, "vals");
        let vars = self.make_unchecked_car(frame);
        self.builder
            .build_store(vars_pointer, self.make_unchecked_car(frame));
        self.builder
            .build_store(vals_pointer, self.make_unchecked_cdr(frame));
        self.builder.build_unconditional_branch(scan_bb);

        self.builder.position_at_end(scan_bb);
        self.builder
            .build_conditional_branch(self.is_hempty(vars), next_env_bb, check_bb);

        self.builder.position_at_end(next_env_bb);
        let new_env = self.make_unchecked_cdr(env_load.into_struct_value());
        self.builder.build_store(env_ptr, new_env);
        self.builder.build_unconditional_branch(lookup_entry_bb);

        self.builder.position_at_end(check_bb);
        let vars_load = self
            .builder
            .build_load(self.types.object, vars_pointer, "load vars")
            .into_struct_value();
        let vars_car = self.make_unchecked_car(vars_load);
        self.builder.build_conditional_branch(
            self.compare_symbol(var, vars_car),
            found_bb,
            scan_next_bb,
        );

        self.builder.position_at_end(scan_next_bb);
        let vals_load = self
            .builder
            .build_load(self.types.object, vals_pointer, "load vals")
            .into_struct_value();
        self.builder
            .build_store(vars_pointer, self.make_unchecked_cdr(vars_load));
        self.builder
            .build_store(vals_pointer, self.make_unchecked_cdr(vals_load));
        self.builder.build_unconditional_branch(scan_bb);

        self.builder.position_at_end(found_bb);
        vals_load
    }

    fn compare_symbol(&self, s1: StructValue<'ctx>, s2: StructValue<'ctx>) -> IntValue<'ctx> {
        let s1 = self.unchecked_get_symbol(s1).into_struct_value();
        let s2 = self.unchecked_get_symbol(s2).into_struct_value();
        let len1 = self
            .builder
            .build_extract_value(s1, 0, "get str length")
            .unwrap()
            .into_int_value();
        let len2 = self
            .builder
            .build_extract_value(s2, 0, "get str length")
            .unwrap()
            .into_int_value();
        let s1 = self.builder.build_extract_value(s1, 1, "get str").unwrap();
        let s2 = self.builder.build_extract_value(s2, 1, "get str").unwrap();
        let str_len_matches =
            self.builder
                .build_int_compare(IntPredicate::EQ, len1, len2, "len matches");
        let str_cmp = self
            .builder
            .build_call(
                self.functions.strncmp,
                &[s1.into(), s2.into(), len1.into()],
                "strcmp",
            )
            .try_as_basic_value()
            .unwrap_left()
            .into_int_value();
        self.builder.build_and(
            str_len_matches,
            self.builder
                .build_int_cast(str_cmp, self.context.bool_type(), ""),
            "eq?",
        )
    }

    fn compile_perform(&mut self, action: Perform) -> StructValue<'ctx> {
        // TODO: dont compile all the args before the match on operation b/c some of them could be better written or something if we had the actual types at compile time
        // similiar to the idea of combining the operastion and its args
        // TODO: each operation could be function or block(s) + phi so instead of compiling the following code each time we find a perform we would do it only once
        let args: Vec<_> = action
            .args()
            .to_vec()
            .into_iter()
            .map(|e| self.compile_expr(e))
            .collect();
        match action.op() {
            Operation::LookupVariableValue => {
                let var = args[0];
                let env = args[1];
                self.make_unchecked_car(self.lookup_variable(var, env))
            }
            Operation::CompiledProcedureEnv => {
                let proc = args[0];
                self.make_caddr(proc)
            }
            Operation::CompiledProcedureEntry => {
                let proc = args[0];
                self.make_cadr(proc)
            }
            Operation::DefineVariable => {
                let var = args[0];
                let val = args[1];
                let env = args[2];
                let frame = self.make_unchecked_car(env);
                // set the vars part of the frame
                self.make_unchecked_set_car(
                    frame,
                    self.make_cons(var, self.make_unchecked_car(frame)),
                );
                // set the vals part of the frame
                self.make_unchecked_set_cdr(
                    frame,
                    self.make_cons(val, self.make_unchecked_cdr(frame)),
                );
                self.empty()
            }
            Operation::ApplyPrimitiveProcedure => {
                let proc = args[0];
                let argl = args[1];
                let proc = self.unchecked_get_primitive(proc);
                self.builder
                    .build_indirect_call(
                        self.types.primitive,
                        proc.into_pointer_value(),
                        &[argl.into()],
                        "call primitive",
                    )
                    .try_as_basic_value()
                    .unwrap_left()
                    .into_struct_value()
            }
            Operation::ExtendEnvoirnment => {
                let vars = args[0];
                let vals = args[1];
                let env = args[2];

                let frame = self.make_cons(vals, vals);
                self.make_cons(frame, env)
            }
            Operation::Cons => {
                let car = *args.first().unwrap();
                let cdr = *args.get(1).unwrap();
                self.make_cons(car, cdr)
            }
            Operation::SetVariableValue => {
                let var = args[0];
                let new_val = args[1];
                let env = args[2];
                self.make_unchecked_set_car(self.lookup_variable(var, env), new_val);
                self.empty()
            }
            Operation::False => {
                let boolean = self
                    .builder
                    .build_not(self.truthy(*args.first().unwrap()), "not truthy");
                self.make_object(&boolean, TypeIndex::number)
            }
            Operation::RandomBool => {
                let bool = self
                    .builder
                    .build_call(self.functions.rand, &[], "random bool")
                    .try_as_basic_value()
                    .unwrap_left();
                let bool = self.builder.build_int_signed_rem(
                    bool.into_int_value(),
                    self.context.i32_type().const_int(2, false),
                    "truncate to bool",
                );

                self.make_object(&bool, TypeIndex::number)
            }
            Operation::MakeCompiledProcedure => {
                let compiled_procedure_string =
                    self.create_string("compiled-procedure".to_string());
                let compiled_procedure_string =
                    self.make_object(&compiled_procedure_string, TypeIndex::symbol);
                let compiled_procedure_entry = args.first().unwrap();
                let compiled_procedure_env = args.get(1).unwrap();
                let tail = self.empty();
                let tail = self.make_cons(*compiled_procedure_env, tail);
                let tail = self.make_cons(*compiled_procedure_entry, tail);
                self.make_cons(compiled_procedure_string, tail)
            }
            Operation::PrimitiveProcedure => {
                self.make_object(&self.is_primitive(args[0]), TypeIndex::bool)
            }
        }
    }

    // TODO: make unchecked versions of *car and *cdr function to avoid dealing with br(s) added self.get_cons (which is causing problems with multiple br(s) per block in variable lookup and other places)
    // or turn car,cdr into llvm functions
    fn car(&self, cons: StructValue<'ctx>) -> PointerValue<'ctx> {
        let cons = self.get_cons(cons).into_struct_value();
        self.builder
            .build_extract_value(cons, 0, "get car")
            .unwrap()
            .into_pointer_value()
    }

    fn cdr(&self, cons: StructValue<'ctx>) -> PointerValue<'ctx> {
        let cons = self.get_cons(cons).into_struct_value();
        self.builder
            .build_extract_value(cons, 1, "get cdr")
            .unwrap()
            .into_pointer_value()
    }
    fn make_car(&self, cons: StructValue<'ctx>) -> StructValue<'ctx> {
        self.builder
            .build_load(self.types.object, self.car(cons), "load car")
            .into_struct_value()
    }

    fn make_cdr(&self, cons: StructValue<'ctx>) -> StructValue<'ctx> {
        self.builder
            .build_load(self.types.object, self.cdr(cons), "load cdr")
            .into_struct_value()
    }

    fn make_set_car(
        &self,
        cons: StructValue<'ctx>,
        new_value: StructValue<'ctx>,
    ) -> StructValue<'ctx> {
        self.builder.build_store(self.car(cons), new_value);
        self.empty()
    }

    fn make_set_cdr(
        &self,
        cons: StructValue<'ctx>,
        new_value: StructValue<'ctx>,
    ) -> StructValue<'ctx> {
        self.builder.build_store(self.cdr(cons), new_value);
        self.empty()
    }
    fn unchecked_car(&self, cons: StructValue<'ctx>) -> PointerValue<'ctx> {
        let cons = self.unchecked_get_cons(cons).into_struct_value();
        self.builder
            .build_extract_value(cons, 0, "get car")
            .unwrap()
            .into_pointer_value()
    }

    fn unchecked_cdr(&self, cons: StructValue<'ctx>) -> PointerValue<'ctx> {
        let cons = self.unchecked_get_cons(cons).into_struct_value();
        self.builder
            .build_extract_value(cons, 1, "get cdr")
            .unwrap()
            .into_pointer_value()
    }
    fn make_unchecked_car(&self, cons: StructValue<'ctx>) -> StructValue<'ctx> {
        self.builder
            .build_load(self.types.object, self.unchecked_car(cons), "load car")
            .into_struct_value()
    }

    fn make_unchecked_cdr(&self, cons: StructValue<'ctx>) -> StructValue<'ctx> {
        self.builder
            .build_load(self.types.object, self.unchecked_cdr(cons), "load cdr")
            .into_struct_value()
    }

    fn make_unchecked_set_car(
        &self,
        cons: StructValue<'ctx>,
        new_value: StructValue<'ctx>,
    ) -> StructValue<'ctx> {
        self.builder
            .build_store(self.unchecked_car(cons), new_value);
        self.empty()
    }

    fn make_unchecked_set_cdr(
        &self,
        cons: StructValue<'ctx>,
        new_value: StructValue<'ctx>,
    ) -> StructValue<'ctx> {
        self.builder
            .build_store(self.unchecked_cdr(cons), new_value);
        self.empty()
    }

    make_accessors!(make_caar make_car make_car);
    make_accessors!(make_cadr make_car make_cdr);
    make_accessors!(make_cdar make_cdr make_car);
    make_accessors!(make_cddr make_cdr make_cdr);
    make_accessors!(make_caaar make_car make_caar);
    make_accessors!(make_caadr make_car make_cadr);
    make_accessors!(make_cadar make_car make_cdar);
    make_accessors!(make_caddr make_car make_cddr);
    make_accessors!(make_cdaar make_cdr make_caar);
    make_accessors!(make_cdadr make_cdr make_cadr);
    make_accessors!(make_cddar make_cdr make_cdar);
    make_accessors!(make_cdddr make_cdr make_cddr);
    make_accessors!(make_caaaar make_car make_caaar);
    make_accessors!(make_caaadr make_car make_caadr);
    make_accessors!(make_caadar make_car make_cadar);
    make_accessors!(make_caaddr make_car make_caddr);
    make_accessors!(make_cadaar make_car make_cdaar);
    make_accessors!(make_cadadr make_car make_cdadr);
    make_accessors!(make_caddar make_car make_cddar);
    make_accessors!(make_cadddr make_car make_cdddr);
    make_accessors!(make_cdaaar make_cdr make_caaar);
    make_accessors!(make_cdaadr make_cdr make_caadr);
    make_accessors!(make_cdadar make_cdr make_cadar);
    make_accessors!(make_cdaddr make_cdr make_caddr);
    make_accessors!(make_cddaar make_cdr make_cdaar);
    make_accessors!(make_cddadr make_cdr make_cdadr);
    make_accessors!(make_cdddar make_cdr make_cddar);
    make_accessors!(make_cddddr make_cdr make_cdddr);

    fn make_cons(&self, car: StructValue<'ctx>, cdr: StructValue<'ctx>) -> StructValue<'ctx> {
        let cons = self.types.cons.const_zero();
        let car_ptr = self.builder.build_alloca(self.types.object, "car ptr");
        let cdr_ptr = self.builder.build_alloca(self.types.object, "cdr ptr");
        self.builder.build_store(car_ptr, car);
        self.builder.build_store(cdr_ptr, cdr);
        let cons = self
            .builder
            .build_insert_value(cons, car_ptr, 0, "insert car - cons")
            .unwrap()
            .into_struct_value();
        let cons = self
            .builder
            .build_insert_value(cons, cdr_ptr, 1, "insert cdr - cons")
            .unwrap()
            .into_struct_value();
        self.make_object(&cons, TypeIndex::cons)
    }

    fn empty(&self) -> StructValue<'ctx> {
        self.types.object.const_zero()
    }

    fn make_object(&self, obj: &dyn BasicValue<'ctx>, index: TypeIndex) -> StructValue<'ctx> {
        let value_ptr = self
            .builder
            .build_alloca(self.types.types.get(index), "object value");
        self.builder
            .build_store(value_ptr, obj.as_basic_value_enum());
        let obj = self.types.object.const_zero();
        let obj = self
            .builder
            .build_insert_value(
                obj,
                self.context.i32_type().const_int(index as u64, false),
                0,
                "insert type object",
            )
            .unwrap();
        let obj = self
            .builder
            .build_insert_value(obj, value_ptr, 1, "insert value object")
            .unwrap();
        obj.into_struct_value()
    }

    fn compile_const(&mut self, constant: Const) -> StructValue<'ctx> {
        match constant {
            Const::Empty => self.empty(),
            Const::String(s) => self.create_string(s),
            Const::Symbol(s) => self.create_string(s), // TODO: intern the symbol
            Const::Number(n) => {
                let number = self.context.f64_type().const_float(n);
                self.make_object(&number, TypeIndex::number)
            }
            Const::Boolean(b) => {
                let boolean = self.context.bool_type().const_int(u64::from(b), false);
                self.make_object(&boolean, TypeIndex::number)
            }
            Const::List(car, cdr) => {
                let cons: StructValue<'_> = self.types.cons.const_zero();
                let mut compile_and_add = |expr, name, cons, index| {
                    let expr_compiled = self.compile_expr(expr);
                    let expr = self.builder.build_alloca(self.types.object, name);
                    self.builder.build_store(expr, expr_compiled);
                    self.builder
                        .build_insert_value(cons, expr, index, &format!("insert {name}"))
                        .unwrap()
                };
                let cons = compile_and_add(*car, "car", cons.as_aggregate_value_enum(), 0);
                let cons = compile_and_add(*cdr, "cdr", cons, 1);
                self.make_object(&cons, TypeIndex::cons)
            }
        }
    }

    fn create_string(&self, s: String) -> StructValue<'ctx> {
        let strlen = s.chars().count();
        let global_str = self
            .builder
            .build_global_string_ptr(&s, &s)
            .as_pointer_value();
        let obj = self.types.string.const_zero();
        let mut add_to_string = |string_object, name, expr, index| {
            self.builder
                .build_insert_value(string_object, expr, index, &format!("insert {name}"))
                .unwrap()
        };
        add_to_string(
            add_to_string(
                obj,
                "string length",
                self.context
                    .i32_type()
                    .const_int(strlen as u64, false)
                    .as_basic_value_enum(),
                0,
            )
            .into_struct_value(),
            "string data",
            global_str.as_basic_value_enum(),
            1,
        )
        .into_struct_value()
    }
}
