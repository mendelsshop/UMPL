use std::{collections::HashMap, hash::BuildHasher};

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::Module,
    types::{PointerType, StructType},
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
    ($fn_name:ident, $type:ident, $name:literal) => {
        pub(super) fn $fn_name(&self, val: StructValue<'ctx>) -> BasicValueEnum<'ctx> {
            let current_fn = self.main;
            let prefix = |end| format!("extract-{}:{end}", $name);
            let ret_block = self
                .context
                .append_basic_block(current_fn, &prefix("return"));
            let error_block = self
                .context
                .append_basic_block(current_fn, &prefix("error"));

            let ty = self.extract_type(val).unwrap().into_int_value();
            let condition = self.builder.build_int_compare(
                inkwell::IntPredicate::EQ,
                ty,
                self.context
                    .i32_type()
                    .const_int(TypeIndex::$type as u64, false),
                &prefix("cmp-type"),
            );
            self.builder
                .build_conditional_branch(condition, ret_block, error_block);
            self.builder.position_at_end(error_block);
            self.exit(&format!("type mismtatch expected {}\n", $name), 1);

            self.builder.position_at_end(ret_block);
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

fixed_map!(TypeMap, BasicTypeEnum<'ctx>,TypeIndex {empty bool number string symbol label cons}
      fn new(
        empty: BasicTypeEnum<'ctx>,
        bool: BasicTypeEnum<'ctx>,
        number: BasicTypeEnum<'ctx>,
        string: BasicTypeEnum<'ctx>,
        symbol: BasicTypeEnum<'ctx>,
        label: BasicTypeEnum<'ctx>,
        cons: BasicTypeEnum<'ctx>
    ) -> Self {
        Self {
            empty,
            bool,
            number,
            string,
            symbol,
            label,
            cons,
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
}

pub struct Types<'ctx> {
    object: StructType<'ctx>,
    string: StructType<'ctx>,
    cons: StructType<'ctx>,
    pointer: PointerType<'ctx>,
    types: TypeMap<'ctx>,
    stack: StructType<'ctx>,
}
/// Important function that the compiler needs to access
pub struct Functions<'ctx> {
    exit: FunctionValue<'ctx>,
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
        Self { exit, printf, rand }
    }
}

macro_rules! make_accessors {
    ($name:ident $outer:ident $inner:ident) => {
        pub fn $name(&self, cons: StructValue<'ctx>) -> StructValue<'ctx> {
            self.$outer(self.$inner(cons))
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
    stack: PointerValue<'ctx>,
}

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    fn extract_type(&self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        self.builder.build_extract_value(cond_struct, 0, "get_type")
    }
    extract!(get_label, label, "label");
    extract!(get_cons, cons, "cons");
    pub fn new(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
    ) -> Self {
        let main = module.add_function("main", context.i32_type().fn_type(&[], false), None);
        builder.position_at_end(context.append_basic_block(main, "entry"));
        let pointer = context.bool_type().ptr_type(AddressSpace::default());
        let object = context.struct_type(&[context.i32_type().into(), pointer.into()], false);
        let string = context.struct_type(&[context.i32_type().into(), pointer.into()], false);
        let cons = context.struct_type(&[pointer.into(), pointer.into()], false);
        let stack = context.struct_type(&[object.into(), pointer.into()], false);
        let types = Types {
            object,
            string,
            cons,
            pointer,
            stack,
            types: TypeMap::new(
                context.struct_type(&[], false).into(),
                context.bool_type().into(),
                context.f64_type().into(),
                string.into(),
                string.into(),
                pointer.into(),
                cons.into(),
            ),
        };
        Self {
            stack: builder.build_alloca(stack, "stack"),
            context,
            flag: builder.build_alloca(types.object, "flag"),
            builder,
            module,
            main,
            labels: HashMap::new(),
            registers: RegiMap::new(builder, object),
            types,
            functions: Functions::new(module, context),
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
                let next_label = self.context.append_basic_block(self.main, "next-block");
                self.builder.build_conditional_branch(
                    self.truthy(flag.into_struct_value()),
                    *self.labels.get(&l).unwrap(),
                    next_label,
                );
                self.builder.position_at_end(next_label);
            }
            Instruction::Goto(g) => match g {
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
                        .build_indirect_branch(label, &self.labels.values().copied().collect_vec());
                }
            },
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

    fn compile_perform(&mut self, action: Perform) -> StructValue<'ctx> {
        let args: Vec<_> = action
            .args()
            .to_vec()
            .into_iter()
            .map(|e| self.compile_expr(e))
            .collect();
        match action.op() {
            Operation::LookupVariableValue => self.empty(),
            Operation::CompiledProcedureEnv => {
                let proc = args[0];
                self.make_caddr(proc)
            }
            Operation::CompiledProcedureEntry => {
                let proc = args[0];
                self.make_cadr(proc)
            }
            Operation::DefineVariable => self.empty(),
            Operation::ApplyPrimitiveProcedure => self.empty(),
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
            Operation::SetVariableValue => self.empty(),
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
            Operation::PrimitiveProcedure => self.empty(),
        }
    }

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

    fn make_cons(&mut self, car: StructValue<'ctx>, cdr: StructValue<'ctx>) -> StructValue<'ctx> {
        let cons = self.types.cons.const_zero();
        let cons = self
            .builder
            .build_insert_value(cons, car, 0, "insert car - cons")
            .unwrap()
            .into_struct_value();
        let cons = self
            .builder
            .build_insert_value(cons, cdr, 1, "insert cdr - cons")
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

    fn create_string(&mut self, s: String) -> StructValue<'ctx> {
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
