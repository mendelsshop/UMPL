use inkwell::values::{AnyValue, BasicValue, BasicValueEnum, PointerValue};

use crate::ast::UMPL2Expr;

use super::{Compiler, EvalType, TyprIndex};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    fn extract_arguements(&mut self, root: PointerValue<'ctx>, exprs: &[UMPL2Expr]) {
        let current_node = self
            .builder
            .build_alloca(self.types.generic_pointer, "arg pointer");
        self.builder.build_store(current_node, root);
        // let arg_cound = self.context.i64_type().const_zero();
        let UMPL2Expr::Number(n) = &exprs[0] else {
            todo!("this function should return result so this can error")
        };

        for i in 0..(n.floor() as u32) {
            let arg_load =
                self.builder
                    .build_load(self.types.generic_pointer, current_node, "arg_load");
            let arg_object = self
                .builder
                .build_struct_gep(
                    self.types.args,
                    arg_load.into_pointer_value(),
                    0,
                    "arg data",
                )
                .unwrap();
            self.insert_variable(i.to_string().into(), arg_object);
            let next_arg = self
                .builder
                .build_struct_gep(
                    self.types.args,
                    arg_load.into_pointer_value(),
                    1,
                    "next arg",
                )
                .unwrap();
            let next_arg =
                self.builder
                    .build_load(self.types.generic_pointer, next_arg, "load next arg");
            self.builder.build_store(current_node, next_arg);
        }
    }

    pub(crate) fn special_form_lambda(
        &mut self,
        exprs: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        let body = if exprs.len() == 2 {
            &exprs[1]
        } else if exprs.len() == 3 {
            &exprs[2]
        } else {
            return Err("lambda expression needs at least 2 subexpressions".to_string());
        };
        // if its var arg dont make it var arg, just make it arg_count+1  number of parameters
        let env = self.get_scope();
        let old_fn = self.fn_value;
        let old_block = self.builder.get_insert_block();
        let UMPL2Expr::Scope(body) = body else {
            return Err("function without scope".to_string());
        };

        // call info should be inserted before the env pointer, b/c when function called first comes env pointer and then call_info
        let fn_value = self
            .module
            .add_function("lambda", self.types.lambda_ty, None);
        for (name, arg) in fn_value.get_param_iter().skip(2).enumerate() {
            arg.set_name(&name.to_string());
        }
        let entry = self.context.append_basic_block(fn_value, "entry");
        self.fn_value = Some(fn_value);
        self.builder.position_at_end(entry);
        let call_info = fn_value.get_nth_param(1).unwrap().into_struct_value();
        let jmp_block = self
            .builder
            .build_extract_value(call_info, 1, "basic block address")
            .unwrap()
            .into_pointer_value();
        let jump_bb = self.context.append_basic_block(fn_value, "not-jmp");
        let cont_bb = self
            .context
            .append_basic_block(fn_value, "normal evaluation");
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
        let _ac = self
            .builder
            .build_extract_value(call_info, 0, "get number of args")
            .unwrap();
        let env_iter = self.get_current_env_name().cloned().collect::<Vec<_>>();
        let envs = self
            .builder
            .build_load(
                env.0,
                fn_value.get_first_param().unwrap().into_pointer_value(),
                "load env",
            )
            .into_struct_value();
        self.new_env();
        for i in 0..env.0.count_fields() {
            let cn = env_iter[i as usize].clone();
            let alloca = self
                .create_entry_block_alloca(self.types.object, &cn)
                .unwrap();
            let arg = self
                .builder
                .build_extract_value(envs, i, "load captured")
                .unwrap();
            self.builder.build_store(alloca, arg);
            self.insert_variable(cn.clone(), alloca);
        }
        let args = fn_value.get_nth_param(2).unwrap().into_pointer_value();
        self.extract_arguements(args, exprs);
        self.builder
            .position_at_end(fn_value.get_last_basic_block().unwrap());
        self.state.push(EvalType::Function);
        let compile_scope = self.compile_scope(body);
        self.state.pop();
        if let Some(ret) = compile_scope? {
            self.builder.build_return(Some(&ret));
        }

        // reset to previous state (before function) needed for functions in functions
        if let Some(end) = old_block {
            self.builder.position_at_end(end);
        }
        self.fn_value = old_fn;
        self.pop_env();
        // return the whole thing after verification and optimization
        self.const_lambda(fn_value, env.1).map_or_else(
            |_| {
                println!();
                self.print_ir();
                unsafe {
                    fn_value.delete();
                }

                Err("Invalid generated function.".to_string())
            },
            |lambda| Ok(Some(lambda.as_basic_value_enum())),
        )
    }

    pub(crate) fn compile_application(
        &mut self,
        application: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        let op = if let UMPL2Expr::Ident(ident) = &application[0] {
            if let Some(var) = self.get_variable(ident) {
                match var {
                    super::env::VarType::Lisp(val) => {
                        self.builder.build_load(self.types.object, val, ident)
                    }
                    super::env::VarType::SpecialForm(sf) => {
                        return sf(self, &application[1..]);
                    }
                    super::env::VarType::Macro => todo!(),
                }
            } else {
                return Err(format!("variable '{ident}' not found",));
            }
        } else {
            return_none!(self.compile_expr(&application[0])?)
        };
        let arg_len = application.len();
        let call_info = self.types.call_info.const_named_struct(&[
            self.context
                .i64_type()
                .const_int(arg_len as u64, false)
                .into(),
            self.types.generic_pointer.const_null().into(),
        ]);
        let val = self.actual_value(op.into_struct_value());
        let primitve_bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "primitve-application");
        let lambda_bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "lambda-application");
        let cont_bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "cont-application");
        let null = self.types.generic_pointer.const_null();
        let args = return_none!(application.iter().skip(1).try_fold(null, |init, current| {
            let ptr = self.builder.build_alloca(self.types.args, "add arg");
            self.builder.build_store(ptr, self.const_thunk(current)?);
            let next = self
                .builder
                .build_struct_gep(self.types.args, ptr, 1, "next arg")
                .unwrap();
            self.builder.build_store(next, init);
            Some(ptr)
        }));
        let fn_ty = self.extract_type(val).unwrap();
        let is_primitive = self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            fn_ty.into_int_value(),
            self.types.ty.const_int(TyprIndex::primitive as u64, false),
            "application::fntype::cmp",
        );
        self.builder
            .build_conditional_branch(is_primitive, primitve_bb, lambda_bb);
        self.builder.position_at_end(primitve_bb);
        let op = self.extract_primitve(val).unwrap().into_pointer_value();
        let unwrap_left_prim = self
            .builder
            .build_indirect_call(
                self.types.primitive_ty,
                op,
                &[call_info.into(), args.into()],
                "application:call",
            )
            .try_as_basic_value()
            .unwrap_left();
        let primitve_bb = self.builder.get_insert_block().unwrap();
        self.builder.build_unconditional_branch(cont_bb);
        self.builder.position_at_end(lambda_bb);
        let op = self.extract_labmda(val).unwrap();
        let function_pointer = self
            .builder
            .build_extract_value(op.into_struct_value(), 0, "function load")
            .unwrap()
            .as_any_value_enum()
            .into_pointer_value();
        let any_value_enum = self
            .builder
            .build_extract_value(op.into_struct_value(), 1, "function env load")
            .unwrap()
            .as_any_value_enum();
        let env_pointer = any_value_enum.into_pointer_value();
        // should probavly figure out that actual param count of function cause supposedly tail calls dont work on varidiac aargument function
        let unwrap_left = self
            .builder
            .build_indirect_call(
                self.types.lambda_ty,
                function_pointer,
                &[env_pointer.into(), call_info.into(), args.into()],
                "application:call",
            )
            .try_as_basic_value()
            .unwrap_left();
        let lambda_bb = self.builder.get_insert_block().unwrap();
        self.builder.build_unconditional_branch(cont_bb);
        self.builder.position_at_end(cont_bb);
        let cont = self
            .builder
            .build_phi(self.types.object, "application::done");
        cont.add_incoming(&[(&unwrap_left, lambda_bb), (&unwrap_left_prim, primitve_bb)]);
        Ok(Some(cont.as_basic_value()))
    }
}
