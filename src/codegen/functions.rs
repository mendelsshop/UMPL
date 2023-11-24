use std::collections::HashMap;

use inkwell::values::{AnyValue, BasicValue, BasicValueEnum, IntValue, PointerValue};

use crate::{ast::UMPL2Expr, interior_mut::RC};

use super::{Compiler, EvalType, TyprIndex};

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum PsudoVariable {
    UserDefined,
    SpecialForm(RC<str>),
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    fn make_used_variables(&self) -> HashMap<RC<str>, PsudoVariable> {
        // start at root env go up until we hit our current env
        self.variables.iter().fold(HashMap::new(), |mut acc, env| {
            for (name, var) in env {
                match var {
                    super::env::VarType::Lisp(_) => {
                        acc.insert(name.clone(), PsudoVariable::UserDefined);
                    }
                    super::env::VarType::SpecialForm(_) => {}
                }
            }
            acc
        })
    }

    #[allow(unused)]
    fn special_form_psudo_lambda(
        &mut self,
        mut variables: HashMap<RC<str>, PsudoVariable>,
        exprs: &[UMPL2Expr],
    ) -> Result<Vec<RC<str>>, String> {
        if let Some(UMPL2Expr::Application(a)) = exprs.first() {
            let n = match a.as_slice() {
                [UMPL2Expr::Number(n), UMPL2Expr::String(s)]
                    if (&["*", "+"]).contains(&&s.to_string().as_str()) =>
                {
                    *n + 1.0
                }

                [UMPL2Expr::Number(n)] => *n,
                _ => {
                    return Err(
                        "lambda signature must be (lambda (argc [\"+\"|\"*\"|\"\"]) exprs)"
                            .to_string(),
                    )
                }
            };
            variables.extend(
                (0..n.trunc() as u64).map(|i| (i.to_string().into(), PsudoVariable::UserDefined)),
            );
            Ok(a[1..]
                .into_iter()
                .map(|e| self.find_free_variables_expr(e, variables.clone()))
                .flatten()
                .collect())
        } else {
            Err("lambda signature must be (lambda (argc [\"+\"|\"*\"|\"\"]) exprs)".to_string())
        }
    }

    #[allow(unused)]
    fn find_free_variables_expr(
        &mut self,
        expr: &UMPL2Expr,
        variables: HashMap<RC<str>, PsudoVariable>,
    ) -> Vec<RC<str>> {
        match expr {
            UMPL2Expr::Ident(ident) => {
                if !variables.contains_key(ident) {
                    vec![ident.clone()]
                } else {
                    vec![]
                }
            }
            UMPL2Expr::Application(a) => self.find_free_variables(a, variables),
            _ => vec![],
        }
    }

    #[allow(dead_code)]
    fn find_free_variables(
        &mut self,
        exprs: &[UMPL2Expr],
        mut variables: HashMap<RC<str>, PsudoVariable>,
    ) -> Vec<RC<str>> {
        let mut free = vec![];
        for expr in exprs {
            match expr {
                UMPL2Expr::Ident(ident) => {
                    if !variables.contains_key(ident) {
                        free.push(ident.clone());
                    }
                }
                UMPL2Expr::Application(a) => {
                    if let UMPL2Expr::Ident(ident) = &a[0] {
                        if let Some(PsudoVariable::SpecialForm(sf)) = variables.get(ident) {
                            match sf.to_string().as_str() {
                                "lambda" => {
                                    let mut variables_clone = variables.clone();
                                    // add parameters to the variables $0, $1, $2, ...

                                    variables.extend(
                                        self.find_free_variables(&a[2..], variables_clone)
                                            .into_iter()
                                            .map(|v| (v, PsudoVariable::UserDefined)),
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                    let variables_clone = variables.clone();
                    variables.extend(
                        a.iter()
                            .filter_map(|e| match e {
                                UMPL2Expr::Ident(ident) => {
                                    if !variables_clone.contains_key(ident) {
                                        Some(vec![(ident.clone(), PsudoVariable::UserDefined)])
                                    } else {
                                        None
                                    }
                                }
                                UMPL2Expr::Application(a) => Some(
                                    self.find_free_variables(a, variables_clone.clone())
                                        .into_iter()
                                        .map(|v| (v, PsudoVariable::UserDefined))
                                        .collect(),
                                ),
                                _ => None,
                            })
                            .flatten(),
                    );
                }
                _ => {}
            }
        }
        free
    }
    fn extract_arguements(
        &mut self,
        root: PointerValue<'ctx>,
        argc: IntValue<'ctx>,
        exprs: &[UMPL2Expr],
    ) {
        let current_node = self
            .builder
            .build_alloca(self.types.generic_pointer, "arg pointer");
        self.builder.build_store(current_node, root);
        // let arg_cound = self.context.i64_type().const_zero();

        let (n, var) = match &exprs[0] {
            // UMPL2Expr::Number(n) => (n, "".into()),
            UMPL2Expr::Application(a) => match a.as_slice() {
                [UMPL2Expr::Number(n), UMPL2Expr::Ident(s)]
                    if ["+".into(), "*".into()].contains(s) =>
                {
                    (n, (s.clone()))
                }

                [UMPL2Expr::Number(n)] => (n, "".into()),
                _ => todo!("self function should return result so self can error"),
            },
            _ => todo!("self function should return result so self can error"),
        };
        if n.fract() != 0f64 {
            todo!("self function should return result so self can error")
        };
        let arg_err = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "arrity mismatch");
        let normal = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "normal");
        let arrity_mismatch = match var.to_string().as_str() {
            "+" => self.builder.build_int_compare(
                inkwell::IntPredicate::UGE,
                argc,
                self.context
                    .i64_type()
                    .const_int(n.trunc() as u64 + 1, false),
                "",
            ),

            "*" => self.builder.build_int_compare(
                inkwell::IntPredicate::UGE,
                argc,
                self.context.i64_type().const_int(n.trunc() as u64, false),
                "",
            ),

            _ => self.builder.build_int_compare(
                inkwell::IntPredicate::EQ,
                argc,
                self.context.i64_type().const_int(n.trunc() as u64, false),
                "",
            ),
        };
        self.builder
            .build_conditional_branch(arrity_mismatch, normal, arg_err);
        self.builder.position_at_end(arg_err);
        self.exit("arrity mismatch", 2);
        self.builder.position_at_end(normal);
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
            self.insert_new_variable(i.to_string().into(), arg_object)
                .unwrap(); // allowed to unwarp b/c error only if theres is no environment -> compiler borked
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
            // TODO: get var args
        }
        // means there is a variable amount of arguments
        if !var.is_empty() {
            let argleft = self.builder.build_int_sub(
                argc,
                self.context.i64_type().const_int(n.trunc() as u64, false),
                "args left",
            );

            let cur_load =
                self.builder
                    .build_load(self.types.generic_pointer, current_node, "load");
            self.insert_variable_new_ptr(
                &n.trunc().to_string().into(),
                self.builder
                    .build_call(
                        self.functions.va_procces,
                        &[cur_load.into(), argleft.into()],
                        "variadic arg procces",
                    )
                    .try_as_basic_value()
                    .unwrap_left(),
            )
            .unwrap();
        }
    }

    // lambda defined as ("lambda" (argc "+"|"*"|"") exprs)
    // to fix foward refernces we scan out for variables that are not defined yet and bind them to a special undefined value which will error at runtime if not bound at function call time
    // might have to similiar thing for thunks
    // or if we could have a way to signal that a lookup is happening in a lambda and if the variable is not bound we create new binding with unassigned value
    // the only problem is
    // (define (f)
    /// (define (g) ; create unassigned u in g's environment
    ///   (lambda () u)
    /// ))
    /// (define x ((f)))
    /// (define u 1)
    /// (x) ; access u in top level environment (which is reacheable from g's environment, but we created different u in g's environment)
    pub(crate) fn special_form_lambda(
        &mut self,
        exprs: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        if exprs.is_empty() {
            return Err("lambda expression needs at least 2 subexpressions".to_string());
        };
        // if its var arg dont make it var arg, just make it arg_count+1  number of parameters
        let env = self.get_scope();
        let old_fn = self.fn_value;
        let old_block = self.builder.get_insert_block();
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
        let ac = self
            .builder
            .build_extract_value(call_info, 0, "get number of args")
            .unwrap();
        let env_iter = self.get_current_env_name();
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
            self.insert_new_variable(cn.clone(), alloca).unwrap();
        }
        let args = fn_value.get_nth_param(2).unwrap().into_pointer_value();
        self.extract_arguements(args, ac.into_int_value(), exprs);
        self.builder
            .position_at_end(fn_value.get_last_basic_block().unwrap());
        self.state.push(EvalType::Function);
        let compile_scope = self.compile_scope(&exprs[1..]);
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
                }
            } else {
                return Err(format!("variable '{ident}' not found",));
            }
        } else {
            return_none!(self.compile_expr(&application[0])?)
        };
        let arg_len = application.len() - 1; // ignore the first thing (the function itself)
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
        let args =
            return_none!(application
                .iter()
                .skip(1)
                .rev()
                .try_fold(null, |init, current| {
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

    pub(crate) fn make_va_process(&mut self) {
        let old_f = self.fn_value;
        let function = self.functions.va_procces;
        self.fn_value = Some(function);

        // params
        let size = function.get_nth_param(1).unwrap().into_int_value();
        let var_args = function.get_first_param().unwrap().into_pointer_value();

        // basic blocks
        let entry = self.context.append_basic_block(function, "entry");
        let early_exit = self.context.append_basic_block(function, "early exit");
        let normal = self.context.append_basic_block(function, "normal");

        // entry
        self.builder.position_at_end(entry);
        self.builder.build_call(
            self.functions.printf,
            &[
                self.builder
                    .build_global_string_ptr("\ncall:  (sub) tree size: %d\n", "p-format")
                    .as_pointer_value()
                    .into(),
                size.into(),
            ],
            "print pointer",
        );
        let done_subtree = self.builder.build_int_compare(
            inkwell::IntPredicate::SLT,
            size,
            self.context.i64_type().const_int(1, false),
            "early exit?",
        );
        self.builder
            .build_conditional_branch(done_subtree, early_exit, normal);

        // early exit
        self.builder.position_at_end(early_exit);
        let return_struct = self.types.va_arg.const_zero();
        let return_struct = self
            .builder
            .build_insert_value(return_struct, self.hempty(), 0, "return struct - data")
            .unwrap();
        let return_struct = self
            .builder
            .build_insert_value(return_struct, var_args, 1, "return struct - next")
            .unwrap();
        self.builder.build_return(Some(&return_struct));

        // normal
        self.builder.position_at_end(normal);
        let mid = self.builder.build_int_signed_div(
            size,
            self.context.i64_type().const_int(2, false),
            "mid",
        );

        let left_struct = self
            .builder
            .build_call(function, &[var_args.into(), mid.into()], "handle left")
            .try_as_basic_value()
            .unwrap_left()
            .into_struct_value();
        let left_tree = self
            .builder
            .build_extract_value(left_struct, 0, "left tree")
            .unwrap()
            .into_struct_value();
        let next = self
            .builder
            .build_extract_value(left_struct, 1, "left next")
            .unwrap()
            .into_pointer_value();
        let data = self
            .builder
            .build_load(self.types.object, next, "this data")
            .into_struct_value();
        let next = self
            .builder
            .build_struct_gep(self.types.args, next, 1, "next arg")
            .unwrap();

        let next = self
            .builder
            .build_load(self.types.generic_pointer, next, "next")
            .into_pointer_value();
        let right_struct = self
            .builder
            .build_call(
                function,
                &[
                    next.into(),
                    self.builder
                        .build_int_sub(
                            self.builder.build_int_sub(size, mid, "right size"),
                            self.context.i64_type().const_int(1, false),
                            "right size",
                        )
                        .into(),
                ],
                "handle right",
            )
            .try_as_basic_value()
            .unwrap_left()
            .into_struct_value();
        let right_tree = self
            .builder
            .build_extract_value(right_struct, 0, "right tree")
            .unwrap()
            .into_struct_value();
        let next = self
            .builder
            .build_extract_value(right_struct, 1, "right next")
            .unwrap()
            .into_pointer_value();

        // we get around problems with cons and aloca by using malloc
        let ptr = self.builder.build_malloc(self.types.cons, "tree").unwrap();
        let tree = self.const_cons_with_ptr(ptr, left_tree, data, right_tree);

        let return_struct = self.types.va_arg.const_zero();
        let return_struct = self
            .builder
            .build_insert_value(return_struct, tree, 0, "return struct - data")
            .unwrap();
        let return_struct = self
            .builder
            .build_insert_value(return_struct, next, 1, "return struct - next")
            .unwrap();
        self.builder.build_return(Some(&return_struct));

        function.verify(true);
        self.fpm.run_on(&function);
        self.fn_value = old_f;
    }
}
