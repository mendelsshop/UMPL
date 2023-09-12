use std::collections::HashMap;

use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue, StructValue};

use crate::interior_mut::RC;

use super::Compiler;

/// envoirnment/variable handling functions
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(super) fn insert_function(&mut self, name: RC<str>, function: FunctionValue<'ctx>) {
        if function.verify(true) {
            self.fpm.run_on(&function);
            let p = self.primitive(function.as_global_value().as_pointer_value());
            let gloabl_lambda = self.module.add_global(p.get_type(), None, &name);
            gloabl_lambda.set_initializer(&p);
            self.insert_variable(name, gloabl_lambda.as_pointer_value());
        } else {
            println!("Failed to verify function {name}");
            self.print_ir();
            unsafe { function.delete() }
        }
    }

    pub(super) fn insert_lambda(&mut self, name: &RC<str>, lambda: StructValue<'ctx>) {
        let name = self
            .module_list
            .iter()
            .map(|m| m.to_string() + "#")
            .collect::<String>()
            + name;
        let ty = self.types.object;
        let lambda_ptr = self.create_entry_block_alloca(ty, &name).unwrap();
        self.builder.build_store(lambda_ptr, lambda);
        self.insert_variable(name.into(), lambda_ptr);
    }

    pub(super) fn new_env(&mut self) {
        self.variables.push((HashMap::new(), vec![]));
    }

    pub(super) fn pop_env(&mut self) {
        self.variables.pop();
    }

    pub(super) fn insert_variable(&mut self, name: RC<str>, value: PointerValue<'ctx>) {
        if let Some(scope) = self.variables.last_mut() {
            scope.0.insert(name.clone(), value);
            scope.1.push(name);
        }
    }

    pub fn get_scope(&self) -> (inkwell::types::StructType<'ctx>, PointerValue<'ctx>) {
        let prev = self.get_current_env_name();

        let value: Vec<_> = prev.collect();
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

    pub fn get_current_env_name(&self) -> impl Iterator<Item = &RC<str>> {
        self.variables.last().unwrap().1.iter()
    }

    fn get_variable(&self, name: &RC<str>) -> Option<PointerValue<'ctx>> {
        self.variables
            .iter()
            .rev()
            .cloned()
            .flat_map(|v| v.0)
            .find(|v| v.0 == name.clone())
            .map(|v| v.1)
    }

    pub(super) fn get_var(&self, s: &std::rc::Rc<str>) -> Result<BasicValueEnum<'ctx>, String> {
        let ptr = self.get_variable(s).ok_or(format!("{s} not found"))?;
        // ptr.print_to_stderr();
        Ok(self.builder.build_load(self.types.object, ptr, s))
    }
}
