use std::collections::HashMap;

use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};

use crate::{ast::UMPL2Expr, interior_mut::RC};

use super::Compiler;

/// envoirnment/variable handling functions
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    // for adding builtin functions
    pub(super) fn insert_function(&mut self, name: RC<str>, function: FunctionValue<'ctx>) {
        if function.verify(true) {
            self.fpm.run_on(&function);
            let p = self.primitive(function.as_global_value().as_pointer_value());
            let gloabl_lambda = self.module.add_global(p.get_type(), None, &name);
            gloabl_lambda.set_initializer(&p);
            self.insert_new_variable(name, gloabl_lambda.as_pointer_value())
                .unwrap(); // allowed to unwrap here b.c this is only used for inseting builtin functions
        } else {
            println!("Failed to verify function {name}");
            self.print_ir();
            unsafe { function.delete() }
        }
    }

    pub fn special_form_define(
        &mut self,
        exprs: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        if exprs.len() != 2 {
            return Err("define must have 2 expressions".to_string());
        }
        match &exprs[0] {
            UMPL2Expr::Ident(i) => {
                let v = return_none!(self.compile_expr(&exprs[1])?);
                self.insert_variable_new_ptr(i, v)?;
                Ok(Some(self.hempty().into()))
            }
            UMPL2Expr::Application(app) => {
                if app.len() < 2 || app.len() > 3 {
                    return Err("defining procedures with define must specify name, arg count and possibly varidicity".to_string());
                }
                let UMPL2Expr::Ident(name) = &app[0] else {
                    return Err("first expression in define procedure not a symbol".to_string());
                };
                let argc = &app[1];
                let varidicity = app.get(2).cloned();
                let scope = &exprs[1];
                let lambda = return_none!(if let Some(vard) = varidicity {
                    self.special_form_lambda(&[argc.clone(), vard, scope.clone()])
                } else {
                    self.special_form_lambda(&[argc.clone(), scope.clone()])
                }?);
                self.insert_variable_new_ptr(name, lambda)?;
                Ok(Some(self.hempty().into()))
            }
            _ => {
                Err("first expression must be either an identifier or a function head".to_string())
            }
        }
    }

    // pub(super) fn insert_lambda(&mut self, name: &RC<str>, lambda: StructValue<'ctx>) {
    //     let name = self
    //         .module_list
    //         .iter()
    //         .map(|m| m.to_string() + "#")
    //         .collect::<String>()
    //         + name;
    //     self.insert_variable_new_ptr(name.into(), lambda.into());
    // }

    pub(super) fn new_env(&mut self) {
        self.variables.push(HashMap::new());
    }

    pub(super) fn pop_env(&mut self) {
        self.variables.pop();
    }

    pub(super) fn insert_new_variable(
        &mut self,
        name: RC<str>,
        value: PointerValue<'ctx>,
    ) -> Result<(), String> {
        self.variables.last_mut().map_or_else(
            || Err(format!("cannot create variable `{name}`")),
            |scope| {
                if scope.insert(name.clone(), VarType::Lisp(value)).is_some() {
                    Err(format!("cannot reassign {name}, use set! instead",))
                } else {
                    Ok(())
                }
            },
        )
    }

    pub fn set_variable(
        &mut self,
        name: RC<str>,
        value: BasicValueEnum<'ctx>,
    ) -> Result<(), String> {
        let ptr = self
            .get_variable(&name)
            .ok_or(format!("could not use set!: {name} not found"))?;
        match ptr {
            VarType::Lisp(l) => {
                self.builder.build_store(l, value);
            }
            VarType::SpecialForm(_) => {
                let ty = self.types.object;
                let ptr = self.create_entry_block_alloca(ty, &name).unwrap();
                self.builder.build_store(ptr, value);
                {
                    self.variables.last_mut().map_or_else(
                        || Err(format!("cannot create variable `{name}`")),
                        |scope| {
                            scope.insert(name.clone(), VarType::Lisp(ptr));
                            Ok(())
                        },
                    )
                }?;
            }
        }
        Ok(())
    }

    pub fn set_or_new(&mut self, name: RC<str>, ptr: PointerValue<'ctx>) -> Result<(), String> {
        let scope = self
            .variables
            .last_mut()
            .ok_or_else(|| format!("cannot create variable `{name}`"))?;
        scope.insert(name, VarType::Lisp(ptr));
        Ok(())
    }
    pub fn special_form_set(
        &mut self,
        exprs: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        if exprs.len() != 2 {
            return Err("set must have 2 expressions".to_string());
        }
        if let UMPL2Expr::Ident(i) = &exprs[0] {
            let v = return_none!(self.compile_expr(&exprs[1])?);
            self.set_variable(i.clone(), v)?;
            Ok(Some(self.hempty().into()))
        } else {
            Err("set must provide identifier".to_string())
        }
    }

    pub fn get_scope(&self) -> (inkwell::types::StructType<'ctx>, PointerValue<'ctx>) {
        let prev = self.get_current_env_name();

        let value: Vec<_> = prev;
        let env_struct_type = self.context.struct_type(
            &std::iter::repeat(self.types.object)
                .take(value.len())
                .map(std::convert::Into::into)
                .collect::<Vec<_>>(),
            false,
        );
        let env_pointer = self
            .create_entry_block_alloca(env_struct_type, "env")
            .unwrap();

        for (i, v) in value.iter().enumerate() {
            let value = self.get_var(v).unwrap();
            let gep = self
                .builder
                .build_struct_gep(env_struct_type, env_pointer, i as u32, "save env")
                .unwrap();
            self.builder.build_store(gep, value);
        }
        (env_struct_type, env_pointer)
    }

    pub fn get_current_env_name(&self) -> Vec<RC<str>> {
        self.variables
            .last()
            .unwrap()
            .iter()
            .map(|v| v.0.clone())
            .collect()
    }

    // returns a procedure or special form, while get var returns only a lisp expression (so could be a proc)
    pub fn get_variable(&self, name: &RC<str>) -> Option<VarType<'a, 'ctx>> {
        self.variables
            .iter()
            .rev()
            .flatten()
            .find(|v| v.0 == name)
            .map(|v| v.1)
            .cloned()
    }

    pub(super) fn get_var(&self, s: &std::rc::Rc<str>) -> Result<BasicValueEnum<'ctx>, String> {
        let ptr = self.get_variable(s).ok_or(format!("{s} not found"))?;
        let VarType::Lisp(ptr) = ptr else {
            return Err("attempted to lookup variable but whas not a variable: ".to_string() + s);
        };
        Ok(self.builder.build_load(self.types.object, ptr, s))
    }

    pub fn insert_special_form(
        &mut self,
        name: RC<str>,
        func: fn(
            &mut Compiler<'a, 'ctx>,
            &[UMPL2Expr],
        ) -> Result<Option<BasicValueEnum<'ctx>>, String>,
    ) {
        if let Some(scope) = self.variables.last_mut() {
            scope.insert(name, VarType::SpecialForm(func));
        }
    }
}

#[derive(Clone, Debug)]
pub enum VarType<'a, 'ctx> {
    Lisp(PointerValue<'ctx>),
    SpecialForm(
        fn(&mut Compiler<'a, 'ctx>, &[UMPL2Expr]) -> Result<Option<BasicValueEnum<'ctx>>, String>,
    ),
}
