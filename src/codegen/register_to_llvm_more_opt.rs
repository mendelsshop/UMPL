//! trying to do more inlining by computing more at compile time
pub(crate) use inkwell::module::Linkage;
use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::Module,
    types::{PointerType, StructType},
    values::FunctionValue,
    AddressSpace,
};
use std::collections::HashMap;

use super::sicp::{Const, Expr, Goto, Instruction, Register};

macro_rules! fixed_map {
    (@inner $(#[$attrs:meta])* $struct:ident($($gen:tt),*), $type:ty, $index:ty {$($fields:ident)*} fn $new:ident($($param:ident: $param_type:ty),*) -> $ret:ty $new_block:block) => {
        $(#[$attrs])*
        pub struct $struct<$($gen),*> {
            $(
                $fields: $type,
            )*
        }

        impl <$($gen),*> $struct<$($gen),*> {
            pub fn $new($($param: $param_type),*) -> $ret $new_block
            pub const fn get(&self, index: $index) -> &$type {
                match index {
                    $(
                        <$index>::$fields => &self.$fields,
                    )*
                }
            }
            pub fn get_mut(&mut self, index: $index) -> &mut $type {
                match index {
                    $(
                        <$index>::$fields => &mut self.$fields,
                    )*
                }
            }
        }
    };
    ($(#[$attrs:meta])* $struct:ident,$type:ident, $index:ty {$($fields:ident)*} fn $new:ident($($param:ident: $param_type:ty),*) -> $ret:ty $new_block:block ) => {
        fixed_map!(@inner $(#[$attrs])* $struct() , $type<>, $index {$($fields)*} fn $new($($param: $param_type),*) -> $ret $new_block);
    };
    ($(#[$attrs:meta])* $struct:ident,$type:ident<$($gen:tt),*>, $index:ty {$($fields:ident)*} fn $new:ident($($param:ident: $param_type:ty),*) -> $ret:ty $new_block:block ) => {
        fixed_map!(@inner $(#[$attrs])* $struct($($gen)*) , $type<$($gen),*>, $index {$($fields)*} fn $new($($param: $param_type),*) -> $ret $new_block);
    };


}

// fixed_map!(TypeMap, BasicTypeEnum<'ctx>,TypeIndex {empty bool number string symbol label cons}
//       fn new(
//         empty: BasicTypeEnum<'ctx>,
//         bool: BasicTypeEnum<'ctx>,
//         number: BasicTypeEnum<'ctx>,
//         string: BasicTypeEnum<'ctx>,
//         symbol: BasicTypeEnum<'ctx>,
//         label: BasicTypeEnum<'ctx>,
//         cons: BasicTypeEnum<'ctx>
//     ) -> Self {
//         Self {
//             empty,
//             bool,
//             number,
//             string,
//             symbol,
//             label,
//             cons,
//         }
//     }
// );

fixed_map!(#[allow(non_snake_case)]RegiMap, NewExpr, Register {Env Argl Val Proc Continue Thunk}
    fn new() -> Self {
        let create_register = |name| NewExpr::Empty;
        Self {
            Env: create_register("env"),
            Thunk: create_register("thunk"),
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
    // types: TypeMap<'ctx>,
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

#[derive(Clone)]
pub enum NewExpr {
    Empty,
    Label(String),
    String(String),
    Symbol(String),
    Number(f64),
    Boolean(bool),
    List(Box<NewExpr>, Box<NewExpr>),
}

pub struct CodeGen<'a, 'ctx> {
    context: &'ctx Context,
    builder: &'a Builder<'ctx>,
    module: &'a Module<'ctx>,
    main: FunctionValue<'ctx>,
    labels: HashMap<String, BasicBlock<'ctx>>,
    registers: RegiMap,
    types: Types<'ctx>,
    functions: Functions<'ctx>,
    stack: Vec<NewExpr>,
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
            // types: TypeMap::new(
            //     context.struct_type(&[], false).into(),
            //     context.bool_type().into(),
            //     context.f64_type().into(),
            //     string.into(),
            //     string.into(),
            //     pointer.into(),
            //     cons.into(),
            // ),
        };
        Self {
            context,
            builder,
            module,
            main,
            labels: HashMap::new(),
            registers: RegiMap::new(),
            types,
            functions: Functions::new(module, context),
            stack: vec![],
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
                let expr = self.compile_expr(e);
                let register = self.registers.get_mut(r);
                *register = expr;
            }
            Instruction::Test(_) => {}
            Instruction::Branch(_) => {}
            Instruction::Goto(g) => match g {
                Goto::Label(l) => {
                    self.builder
                        .build_unconditional_branch(*self.labels.get(&l).unwrap());
                }
                Goto::Register(r) => {
                    let register = self.registers.get(r);
                    match register {
                        NewExpr::Label(l) => {
                            self.builder
                                .build_unconditional_branch(*self.labels.get(l).unwrap());
                        }
                        _ => panic!("not label"),
                    }
                }
            },
            Instruction::Save(reg) => self.stack.push(self.registers.get(reg).clone()),
            Instruction::Restore(reg) => *self.registers.get_mut(reg) = self.stack.pop().unwrap(),
            Instruction::Perform(_) => {}
            Instruction::Label(_) => unreachable!(),
        }
    }

    fn compile_expr(&mut self, expr: Expr) -> NewExpr {
        match expr {
            Expr::Const(c) => self.compile_const(c),
            Expr::Label(l) => NewExpr::Label(l),
            Expr::Register(r) => self.registers.get(r).clone(),
            Expr::Op(_) => todo!(),
        }
    }

    fn compile_const(&mut self, constant: Const) -> NewExpr {
        match constant {
            Const::Empty => NewExpr::Empty,
            Const::String(s) => NewExpr::String(s),
            Const::Symbol(s) => NewExpr::Symbol(s),
            Const::Number(n) => NewExpr::Number(n),
            Const::Boolean(b) => NewExpr::Boolean(b),
            Const::List(car, cdr) => {
                let car = self.compile_expr(*car);
                let cdr = self.compile_expr(*cdr);
                NewExpr::List(Box::new(car), Box::new(cdr))
            }
        }
    }
}
