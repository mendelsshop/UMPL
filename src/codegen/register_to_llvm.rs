use std::collections::HashMap;

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

use super::sicp::{Const, Expr, Goto, Instruction, Perform, Register};

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
}

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    fn extract_type(&self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        self.builder.build_extract_value(cond_struct, 0, "get_type")
    }
    extract!(get_label, label, "label");
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
        let types = Types {
            object,
            string,
            cons,
            pointer,
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
            Instruction::Save(_) => {}
            Instruction::Restore(_) => {}
            Instruction::Perform(p) => {
                self.compile_perform(p);
            }
            Instruction::Label(l) => {
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
        self.empty()
    }

    fn empty(&mut self) -> StructValue<'ctx> {
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
            Const::String(_) => self.empty(),
            Const::Symbol(_) => self.empty(),
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
}
