#![allow(dead_code, unused_variables, unused_mut)]
use std::collections::HashMap;

use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{FunctionValue, PointerValue};

use inkwell::context::Context;
use inkwell::{
    builder::Builder,
    values::{BasicValue, BasicValueEnum},
};

use crate::ast::{Fanction, FnKeyword, UMPL2Expr};
macro_rules! return_none {
    ($expr:expr) => {
        match $expr {
            Some(e) => e,
            _ => return Ok(None),
        }
    };
}
/// Defines the `Expr` compiler.
pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub module: &'a Module<'ctx>,
    pub function: &'a Fanction,

    variables: HashMap<String, PointerValue<'ctx>>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
}
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Compiles the specified `Function` in the given `Context` and using the specified `Builder`, `PassManager`, and `Module`.
    pub fn compile(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        pass_manager: &'a PassManager<FunctionValue<'ctx>>,
        module: &'a Module<'ctx>,
        function: &Fanction,
    ) -> Result<FunctionValue<'ctx>, &'static str> {
        let mut compiler = Compiler {
            context,
            builder,
            fpm: pass_manager,
            module,
            function,
            fn_value_opt: None,
            variables: HashMap::new(),
        };

        compiler.compile_fn()
    }
    #[inline]
    fn fn_value(&self) -> FunctionValue<'ctx> {
        self.fn_value_opt.unwrap()
    }
    /// Creates a new stack allocation instruction in the entry block of the function.
    fn create_entry_block_alloca(&self, name: &str) -> PointerValue<'ctx> {
        let builder = self.context.create_builder();

        let entry = self.fn_value().get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        builder.build_alloca(self.context.struct_type(&[], false), name)
    }
    fn compile_fn(&mut self) -> Result<FunctionValue<'ctx>, &'static str> {
        let ret_type = self.context.struct_type(&[], false);
        let name = self.function.name().to_string();
        let args_types = std::iter::repeat(ret_type)
            .take(self.function.param_count())
            .map(|f| f.into())
            .collect::<Vec<BasicMetadataTypeEnum>>();
        let args_types = args_types.as_slice();
        let fn_type = self
            .context
            .void_type()
            .fn_type(args_types, self.function.optinal_params().is_some());
        println!("{}", fn_type);
        let fn_val = self.module.add_function(&name, fn_type, None);
        for (i, arg) in fn_val.get_param_iter().enumerate() {
            arg.set_name(&format!("{i}"));
        }
        let entry = self.context.append_basic_block(fn_val, "entry");
        self.builder.position_at_end(entry);
        self.fn_value_opt = Some(fn_val);
        self.variables.reserve(self.function.param_count());
        for (i, arg) in fn_val.get_param_iter().enumerate() {
            println!("{arg}");
            let arg_name = format!("{i}");
            let alloca = self.create_entry_block_alloca(&arg_name);

            self.builder.build_store(alloca, arg);

            self.variables.insert(arg_name.clone(), alloca);
        }

        let scope = &self.function.scope();
        let mut done = false;
        // let mut ret = None;
        // for expr in scope.iter() {

        //     let value = match self.compile_expr(expr)? {
        //         Some(v) => v,
        //         None => {
        //             done = true;

        //             self.builder.build_return(Some(&ret.unwrap()));
        //             break;
        //         }
        //     };

        //     println!("{}", value.to_string());
        //     ret = Some(value);
        // }
        if !done {
            // self.builder.build_return(Some(&ret.unwrap()));
            self.builder.build_return(None);
        }
        fn_val.print_to_stderr();
        println!("{}", fn_val);

        // return the whole thing after verification and optimization
        if fn_val.verify(true) {
            // self.fpm.run_on(&fn_val);

            Ok(fn_val)
        } else {
            unsafe {
                fn_val.delete();
            }

            Err("Invalid generated function.")
        }
    }

    fn compile_scope(
        &mut self,
        scope: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, &'static str> {
        let mut res = None;
        for expr in scope {
            res = Some(return_none!(self.compile_expr(expr)?));
        }
        Ok(Some(res.ok_or("")?))
    }

    fn compile_expr(
        &mut self,
        expr: &UMPL2Expr,
    ) -> Result<Option<BasicValueEnum<'ctx>>, &'static str> {
        match expr {
            UMPL2Expr::Bool(b) => Ok(Some(
                self.context
                    .bool_type()
                    .const_int(
                        match b {
                            crate::ast::Boolean::True => 1,
                            crate::ast::Boolean::False => 0,
                            crate::ast::Boolean::Maybee => todo!(),
                        },
                        false,
                    )
                    .as_basic_value_enum(),
            )),
            UMPL2Expr::Number(n) => Ok(Some(
                self.context
                    .bool_type()
                    .const_int(*n as u64, false)
                    .as_basic_value_enum(),
            )),
            UMPL2Expr::String(_) => todo!(),
            UMPL2Expr::Scope(_) => todo!(),
            UMPL2Expr::Ident(i) => match self.variables.get(i.to_string().as_str()) {
                Some(var) => Ok(Some(self.builder.build_load(*var, i.to_string().as_str()))),
                None => Err("Could not find a matching variable."),
            },
            UMPL2Expr::If(if_stmt) => {
                let parent = self.fn_value();
                let cond = return_none!(self.compile_expr(if_stmt.cond())?).into_int_value();
                let zero_const = self.context.bool_type().const_int(0, false);
                self.builder.build_int_compare(
                    inkwell::IntPredicate::EQ,
                    cond,
                    zero_const,
                    "ifcond",
                );
                let then_bb = self.context.append_basic_block(parent, "then");
                let else_bb = self.context.append_basic_block(parent, "else");
                let cont_bb = self.context.append_basic_block(parent, "ifcont");
                self.builder
                    .build_conditional_branch(cond, then_bb, else_bb);
                self.builder.position_at_end(then_bb);
                let then_val = self.compile_scope(if_stmt.cons())?;
                self.builder.build_unconditional_branch(cont_bb);

                let then_bb = self.builder.get_insert_block().unwrap();

                // build else block
                self.builder.position_at_end(else_bb);
                let else_val = self.compile_scope(if_stmt.alt())?;
                self.builder.build_unconditional_branch(cont_bb);

                let else_bb = self.builder.get_insert_block().unwrap();

                // emit merge block
                self.builder.position_at_end(cont_bb);

                let phi = self.builder.build_phi(self.context.bool_type(), "iftmp");
                match (then_val, else_val) {
                    (None, None) => phi.add_incoming(&[
                        (
                            &self.context.bool_type().const_zero().as_basic_value_enum(),
                            then_bb,
                        ),
                        (
                            &self.context.bool_type().const_zero().as_basic_value_enum(),
                            else_bb,
                        ),
                    ]),
                    (None, Some(_)) => phi.add_incoming(&[
                        (
                            &self.context.bool_type().const_zero().as_basic_value_enum(),
                            then_bb,
                        ),
                        (
                            &self.context.bool_type().const_zero().as_basic_value_enum(),
                            else_bb,
                        ),
                    ]),
                    (Some(_), None) => phi.add_incoming(&[
                        (
                            &self.context.bool_type().const_zero().as_basic_value_enum(),
                            then_bb,
                        ),
                        (
                            &self.context.bool_type().const_zero().as_basic_value_enum(),
                            else_bb,
                        ),
                    ]),
                    (Some(then_val), Some(else_val)) => {
                        phi.add_incoming(&[(&then_val, then_bb), (&else_val, else_bb)])
                    }
                }

                Ok(Some(phi.as_basic_value()))
            }
            UMPL2Expr::Unless(_) => todo!(),
            UMPL2Expr::Stop(s) => {
                // let parent = self.fn_value();
                // self.context.append_basic_block(parent, "exit");
                // let basic_block  = self.builder.get_insert_block().unwrap();
                // self.builder.position_at_end(basic_block);
                // self.builder
                //     .build_return(Some(&return_none!(self.compile_expr(&*s)?)));
                Ok(None)
            }
            UMPL2Expr::Skip => todo!(),
            UMPL2Expr::Until(_) => todo!(),
            UMPL2Expr::GoThrough(_) => todo!(),
            UMPL2Expr::ContiueDoing(_) => todo!(),
            UMPL2Expr::Fanction(_) => todo!(),
            UMPL2Expr::Application(a) => {
                let op = match &a.args()[0] {
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
                let f = return_none!(self.compile_expr(&a.args()[1])?).into_int_value();
                let l = return_none!(self.compile_expr(&a.args()[2])?).into_int_value();
                Ok(Some(
                    match op {
                        FnKeyword::Add => self.builder.build_int_add(f, l, "tmpadd"),
                        FnKeyword::Sub => self.builder.build_int_sub(f, l, "tmpsub"),
                        FnKeyword::Mul => self.builder.build_int_mul(f, l, "tmpmul"),
                        FnKeyword::Div => self.builder.build_int_signed_div(f, l, "tmpdiv"),
                        FnKeyword::Mod => todo!(),
                    }
                    .as_basic_value_enum(),
                ))
            }
            UMPL2Expr::Quoted(_) => todo!(),
            UMPL2Expr::Label(_) => todo!(),
            UMPL2Expr::FnParam(s) => match self.variables.get(&format!("{s}")) {
                Some(var) => Ok(Some(self.builder.build_load(*var, s.to_string().as_str()))),
                None => Err("Could not find a matching variable."),
            },
            UMPL2Expr::Hempty => todo!(),
            UMPL2Expr::Link(_, _) => todo!(),
            UMPL2Expr::Tree(_) => todo!(),
            UMPL2Expr::FnKW(_) => todo!(),
            UMPL2Expr::Let(i, v) => {
                let v = return_none!(self.compile_expr(v)?);
                let ty = self.context.bool_type();
                let ptr = self.builder.build_alloca(ty, i);
                self.builder.build_store(ptr, v);
                let alloca = self.create_entry_block_alloca(i.to_string().as_str());
                self.variables.insert(i.to_string(), alloca);
                return Ok(Some(
                    self.context.bool_type().const_zero().as_basic_value_enum(),
                ));
            }
        }
    }
}
