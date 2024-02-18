use inkwell::values::{
    BasicValueEnum, FloatValue, FunctionValue, IntValue, PointerValue, StructValue,
};

use crate::{
    ast::{Boolean, UMPL2Expr},
    interior_mut::RC,
};

use super::{Compiler, TyprIndex};

enum TableAccess {
    String,
    Symbol,
}

macro_rules! builder_object {
    ($value:ident, $type:ty) => {
        pub fn $value(&self, value: $type) -> StructValue<'ctx> {
            let ty = TyprIndex::$value;
            let obj = self.types.object.const_zero();
            let obj = self
                .builder
                .build_insert_value(obj, self.types.ty.const_int(ty as u64, false), 0, "type")
                .unwrap();
            let obj = self
                .builder
                .build_insert_value(obj, value, ty as u32 + 1, "value")
                .unwrap();
            obj.into_struct_value()
        }
    };
}

/// Seperate impl for the [Compiler] for making new objects
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    builder_object!(boolean, IntValue<'ctx>);
    builder_object!(number, FloatValue<'ctx>);
    builder_object!(string, PointerValue<'ctx>);
    builder_object!(primitive, PointerValue<'ctx>);
    builder_object!(cons, PointerValue<'ctx>);
    builder_object!(lambda, StructValue<'ctx>);
    builder_object!(thunk, StructValue<'ctx>);
    builder_object!(symbol, PointerValue<'ctx>);
    pub fn hempty(&self) -> StructValue<'ctx> {
        let ty = TyprIndex::hempty;
        let obj = self.types.object.const_zero();
        let obj = self
            .builder
            .build_insert_value(
                obj,
                self.types.ty.const_int(ty as u64, false),
                0,
                "hempty-type",
            )
            .unwrap();
        obj.into_struct_value()
    }
    pub(crate) fn const_number(&self, value: f64) -> StructValue<'ctx> {
        self.number(self.types.number.const_float(value))
    }
    pub(crate) fn const_boolean(&self, value: Boolean) -> StructValue<'ctx> {
        match value {
            Boolean::False => self.boolean(self.types.boolean.const_int(0, false)),
            Boolean::True => self.boolean(self.types.boolean.const_int(1, false)),
            Boolean::Maybee => {
                // gen psudo rand number
                let random = self
                    .builder
                    .build_call(self.functions.rand, &[], "random number")
                    .unwrap()
                    .try_as_basic_value()
                    .unwrap_left();
                // mod it to be 0 or 1
                let random_bool = self
                    .builder
                    .build_int_unsigned_rem(
                        random.into_int_value(),
                        self.context.i32_type().const_int(2, false),
                        "get random bool",
                    )
                    .unwrap();
                // trunc it to bool
                let random_bool = self
                    .builder
                    .build_int_truncate(random_bool, self.types.boolean, "trunc to bool")
                    .unwrap();
                self.boolean(random_bool)
            }
        }
    }

    pub(crate) fn const_string(&mut self, value: &RC<str>) -> StructValue<'ctx> {
        let str = self.make_string(Some(TableAccess::String), value);
        self.string(str)
    }

    pub(crate) fn const_symbol(&mut self, value: &RC<str>) -> StructValue<'ctx> {
        let str = self.make_string(Some(TableAccess::Symbol), value);
        self.symbol(str)
    }

    /// returns true if false otherwise false
    pub(crate) fn is_false(&self, object: BasicValueEnum<'ctx>) -> IntValue<'ctx> {
        let object_type = self
            .extract_type(object.into_struct_value())
            .unwrap()
            .into_int_value();
        let is_bool = self
            .builder
            .build_int_compare(
                inkwell::IntPredicate::EQ,
                object_type,
                self.types.ty.const_int(TyprIndex::boolean as u64, false),
                "boolean?",
            )
            .unwrap();
        let bool_val = self
            .builder
            .build_extract_value(
                object.into_struct_value(),
                TyprIndex::boolean as u32 + 1,
                "get boolean value",
            )
            .unwrap()
            .into_int_value();
        let is_false = self
            .builder
            .build_int_compare(
                inkwell::IntPredicate::EQ,
                bool_val,
                self.types.boolean.const_zero(),
                "false?",
            )
            .unwrap();
        self.builder
            .build_and(is_bool, is_false, "is boolean and false?")
            .unwrap()
    }

    fn make_string(
        &mut self,
        kind: Option<TableAccess>,
        value: &std::rc::Rc<str>,
    ) -> PointerValue<'ctx> {
        #[allow(clippy::map_unwrap_or)]
        // allowing this lint b/c we insert in self.string in None case and rust doesn't like that after trying to get from self.string
        kind.map_or_else(
            || {
                self.builder
                    .build_global_string_ptr(value, value)
                    .unwrap()
                    .as_pointer_value()
            },
            |acces| {
                let string_map = match acces {
                    TableAccess::String => &mut self.string,
                    TableAccess::Symbol => &mut self.ident,
                };

                if let Some(str) = string_map.get(value) {
                    str.as_pointer_value()
                } else {
                    let str = self.builder.build_global_string_ptr(value, value).unwrap();
                    string_map.insert(value.clone(), str);
                    str.as_pointer_value()
                }
            },
        )
    }

    pub(super) fn const_thunk(&mut self, object: &UMPL2Expr) -> Option<StructValue<'ctx>> {
        let env = self.get_scope();
        let old_fn = self.fn_value;
        let old_block = self.builder.get_insert_block();
        let thunk = self.module.add_function("thunk", self.types.thunk, None);
        self.fn_value = Some(thunk);
        let entry = self.context.append_basic_block(thunk, "entry");
        self.builder.position_at_end(entry);
        let env_iter = self.get_current_env_name();
        // right now even though we take the first parameter to be the "envoirnment" we don't actully use it, maybee remove that parameter
        let envs = self
            .builder
            .build_load(
                env.0,
                thunk.get_first_param().unwrap().into_pointer_value(),
                "load env",
            )
            .unwrap()
            .into_struct_value();
        self.new_env();
        for i in 0..env.0.count_fields() {
            let cn = env_iter[i as usize].clone();
            // self.module.add_global(type_, address_space, name)
            let alloca = self
                .create_entry_block_alloca(self.types.object, &cn)
                .unwrap();
            let arg = self
                .builder
                .build_extract_value(envs, i, "load captured")
                .unwrap();
            self.builder.build_store(alloca, arg).unwrap();
            self.insert_new_variable(cn.clone(), alloca).unwrap(); // allowed to unwrap b/c we create new scope and copy variables from old scope to here so if old scope correct -> new scope correct
        }
        let ret = self.compile_expr(object);
        match ret {
            Ok(v) => {
                let v = self.actual_value(v?.into_struct_value());
                self.builder.build_return(Some(&v)).unwrap();
            }
            Err(e) => self.exit(&e, 2),
        };
        self.fn_value = old_fn;
        if let Some(bb) = old_block {
            self.builder.position_at_end(bb);
        }
        if !thunk.verify(true) {
            self.print_ir();
            panic!("invalid function")
        }
        self.pop_env();
        self.fpm.run_on(&thunk);
        let value = self.types.thunk_ty.const_zero();
        let value = self
            .builder
            .build_insert_value(
                value,
                thunk.as_global_value().as_pointer_value(),
                0,
                "save-thunk-fn",
            )
            .unwrap();
        let value = self
            .builder
            .build_insert_value(value, env.1, 1, "save-thunk-env")
            .unwrap();
        Some(self.thunk(value.into_struct_value()))
    }

    pub(crate) fn const_cons(
        &self,
        left_tree: StructValue<'ctx>,
        this: StructValue<'ctx>,
        right_tree: StructValue<'ctx>,
    ) -> StructValue<'ctx> {
        // TODO: try to not use globals
        // let left_ptr = create_entry_block_alloca(types.object, "cdr").unwrap();
        // builder.build_store(left_ptr, left_tree).unwrap();
        // let this_ptr = create_entry_block_alloca(types.object, "car").unwrap();
        // builder.build_store(this_ptr, this).unwrap();
        // let right_ptr = create_entry_block_alloca(types.object, "cgr").unwrap();
        // builder.build_store(right_ptr, right_tree).unwrap();
        let tree_type = self.types.cons.const_zero();
        let tree_type = self
            .builder
            .build_insert_value(tree_type, left_tree, 0, "car-set")
            .unwrap();
        let tree_type = self
            .builder
            .build_insert_value(tree_type, this, 1, "cdr-set")
            .unwrap();
        let tree_type = self
            .builder
            .build_insert_value(tree_type, right_tree, 2, "cgr-set")
            .unwrap();
        // let tree_ptr = self.module.add_global(tree_type.get_type(), None, "cons");
        // tree_ptr.set_initializer(&self.types.cons.const_zero());
        let tree_ptr = self
            .create_entry_block_alloca(tree_type.into_struct_value().get_type(), "cons")
            .unwrap();
        self.builder.build_store(tree_ptr, tree_type).unwrap();
        self.cons(tree_ptr)
    }

    pub(crate) fn const_cons_with_ptr(
        &self,
        pv: PointerValue<'ctx>,
        left_tree: StructValue<'ctx>,
        this: StructValue<'ctx>,
        right_tree: StructValue<'ctx>,
    ) -> StructValue<'ctx> {
        // TODO: try to not use globals
        // let left_ptr = create_entry_block_alloca(types.object, "cdr").unwrap();
        // builder.build_store(left_ptr, left_tree).unwrap();
        // let this_ptr = create_entry_block_alloca(types.object, "car").unwrap();
        // builder.build_store(this_ptr, this).unwrap();
        // let right_ptr = create_entry_block_alloca(types.object, "cgr").unwrap();
        // builder.build_store(right_ptr, right_tree).unwrap();
        let tree_type = self.types.cons.const_zero();
        let tree_type = self
            .builder
            .build_insert_value(tree_type, left_tree, 0, "car-set")
            .unwrap();
        let tree_type = self
            .builder
            .build_insert_value(tree_type, this, 1, "cdr-set")
            .unwrap();
        let tree_type = self
            .builder
            .build_insert_value(tree_type, right_tree, 2, "cgr-set")
            .unwrap();
        // let tree_ptr = self.module.add_global(tree_type.get_type(), None, "cons");
        // tree_ptr.set_initializer(&self.types.cons.const_zero());
        self.builder.build_store(pv, tree_type).unwrap();
        self.cons(pv)
    }

    pub(super) fn const_lambda(
        &self,
        function: FunctionValue<'ctx>,
        env: PointerValue<'ctx>,
    ) -> Result<StructValue<'ctx>, &str> {
        if function.verify(true) {
            self.fpm.run_on(&function);
            let value = self.types.lambda.const_zero();
            let value = self
                .builder
                .build_insert_value(
                    value,
                    function.as_global_value().as_pointer_value(),
                    0,
                    "save-fn",
                )
                .unwrap();
            let value = self
                .builder
                .build_insert_value(value, env, 1, "save-fn")
                .unwrap();
            // .const_named_struct(&[
            // function.as_global_value().as_pointer_value().into(),
            // env.into(),
            // ]);
            Ok(self.lambda(value.into_struct_value()))
        } else {
            function.print_to_stderr();
            unsafe { function.delete() }
            Err("function defined incorrectly")
        }
    }
}
