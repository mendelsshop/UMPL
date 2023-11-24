use std::collections::HashMap;

use inkwell::types::BasicTypeEnum;
use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::Module,
    types::{PointerType, StructType},
    values::{AggregateValue, BasicValue, FunctionValue, PointerValue, StructValue},
    AddressSpace,
};

use super::sicp::{Const, Expr, Instruction, Register};

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

pub struct CodeGen<'a, 'ctx> {
    context: &'ctx Context,
    builder: &'a Builder<'ctx>,
    module: &'a Module<'ctx>,
    main: FunctionValue<'ctx>,
    labels: HashMap<String, BasicBlock<'ctx>>,
    registers: RegiMap<'ctx>,
    types: Types<'ctx>,
}

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
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
            builder,
            module,
            main,
            labels: HashMap::new(),
            registers: RegiMap::new(builder, object),
            types,
        }
    }

    pub fn export_ir(&self) -> String {
        self.module.to_string()
    }

    pub fn compile(&mut self, instructions: Vec<Instruction>) {
        for inst in instructions
            .into_iter()
            .filter_map(|x| match x {
                Instruction::Label(l) => {
                    let v = self.context.append_basic_block(self.main, &l);
                    self.labels.insert(l, v);
                    None
                }
                _ => Some(x),
            })
            .collect::<Vec<_>>()
        {
            self.compile_instructions(inst);
        }
    }

    fn compile_instructions(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Assign(r, e) => {
                let register = self.registers.get(r);
                let expr = self.compile_expr(e);
                self.builder.build_store(register, expr);
            }
            Instruction::Test(_) => {}
            Instruction::Branch(_) => {}
            Instruction::Goto(_) => {}
            Instruction::Save(_) => {}
            Instruction::Restore(_) => {}
            Instruction::Perform(_) => {}
            Instruction::Label(_) => unreachable!(),
        }
    }

    fn compile_expr(&mut self, expr: Expr) -> StructValue<'ctx> {
        match expr {
            Expr::Const(c) => self.compile_const(c),
            Expr::Label(_) => todo!(),
            Expr::Register(_) => todo!(),
            Expr::Op(_) => todo!(),
        }
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
            Const::Empty => self.types.object.const_zero(),
            Const::String(_) => todo!(),
            Const::Symbol(_) => todo!(),
            Const::Number(n) => {
                let nubmer = self.context.f64_type().const_float(n);
                self.make_object(&nubmer, TypeIndex::number)
            }
            Const::Boolean(_) => todo!(),
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
