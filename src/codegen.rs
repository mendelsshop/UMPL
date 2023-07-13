use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicMetadataValueEnum, FloatValue, FunctionValue, PointerValue};
use inkwell::FloatPredicate;

use crate::ast::{Fanction, FnKeyword, UMPL2Expr};

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

        builder.build_alloca(self.context.f64_type(), name)
    }
    fn compile_fn(&mut self) -> Result<FunctionValue<'ctx>, &'static str> {
        let ret_type = self.context.f64_type();
        let name = self.function.name().to_string();
        let args_types = std::iter::repeat(ret_type)
            .take(self.function.param_count())
            .map(|f| f.into())
            .collect::<Vec<BasicMetadataTypeEnum>>();
        let args_types = args_types.as_slice();
        let fn_type = self
            .context
            .f64_type()
            .fn_type(args_types, self.function.optinal_params().is_some());
        let fn_val = self.module.add_function(&name, fn_type, None);
        for (i, arg) in fn_val.get_param_iter().enumerate() {
            arg.into_float_value().set_name(&format!("|{i}|"));
        }
        let entry = self.context.append_basic_block(fn_val, "entry");
        self.builder.position_at_end(entry);
        self.fn_value_opt = Some(fn_val);
        self.variables.reserve(self.function.param_count());
        for (i, arg) in fn_val.get_param_iter().enumerate() {
            let arg_name = format!("|{i}|");
            let alloca = self.create_entry_block_alloca(&arg_name);

            self.builder.build_store(alloca, arg);

            self.variables.insert(arg_name.clone(), alloca);
        }
        let scope = &self.function.scope()[0];
        let body = self.compile_expr(scope)?;
        self.builder.build_return(Some(&body));

        // return the whole thing after verification and optimization
        if fn_val.verify(true) {
            self.fpm.run_on(&fn_val);

            Ok(fn_val)
        } else {
            unsafe {
                fn_val.delete();
            }

            Err("Invalid generated function.")
        }
    }

    fn compile_expr(&mut self, expr: &UMPL2Expr) -> Result<FloatValue<'ctx>, &'static str> {
        match expr {
            UMPL2Expr::Bool(_) => todo!(),
            UMPL2Expr::Number(n) => Ok(self.context.f64_type().const_float(*n)),
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
                };
                let f = self.compile_expr(&a.args()[1])?;
                let l = self.compile_expr(&a.args()[2])?;
                match op {
                    FnKeyword::Add => Ok(self.builder.build_float_add(f, l, "tmpadd")),
                    FnKeyword::Sub => Ok(self.builder.build_float_sub(f, l, "tmpsub")),
                    FnKeyword::Mul => Ok(self.builder.build_float_mul(f, l, "tmpmul")),
                    FnKeyword::Div => Ok(self.builder.build_float_div(f, l, "tmpdiv")),
                    FnKeyword::Mod => todo!(),
                }
            }
            UMPL2Expr::Quoted(_) => todo!(),
            UMPL2Expr::Label(_) => todo!(),
            UMPL2Expr::FnParam(_) => todo!(),
            UMPL2Expr::Hempty => todo!(),
            UMPL2Expr::Link(_, _) => todo!(),
            UMPL2Expr::Tree(_) => todo!(),
            UMPL2Expr::FnKW(_) => todo!(),
        }
    }
}
