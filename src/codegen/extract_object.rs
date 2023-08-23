use super::{Compiler, TyprIndex};
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue, StructValue};

macro_rules! make_extract {
    ($fn_name:ident, $type:ident, $name:literal) => {
        pub(super) fn $fn_name(
            &self,
            val: StructValue<'ctx>,
        ) -> Result<BasicValueEnum<'ctx>, String> {
            let current_fn = self.current_fn_value()?;
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
                self.types.ty.const_int(TyprIndex::$type as u64, false),
                &prefix("cmp-type"),
            );
            self.builder
                .build_conditional_branch(condition, ret_block, error_block);
            self.builder.position_at_end(error_block);
            // self.print_type(val, current_fn);
            self.exit(&format!(" does not work as {}\n", $name), 1);

            self.builder.position_at_end(ret_block);
            self.builder
                .build_extract_value(val, TyprIndex::$type as u32 + 1, &prefix("return"))
                .ok_or("could not extract value".to_string())
        }
    };
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    make_extract!(extract_number, number, "number");
    make_extract!(extract_string, string, "string");
    make_extract!(extract_symbol, symbol, "symbol");
    make_extract!(extract_bool, boolean, "boolean");
    make_extract!(extract_primitve, primitive, "primitive");
    make_extract!(extract_labmda, lambda, "lambda");
    make_extract!(extract_thunk, thunk, "thunk");
    make_extract!(extract_cons_inner, cons, "cons");
    pub(super) fn extract_cons(
        &self,
        val: StructValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        self.extract_cons_inner(val).map(|pointer| {
            self.builder
                .build_load(self.types.cons, pointer.into_pointer_value(), "loadcons")
        })
    }
    pub(super) fn extract_type(
        &self,
        cond_struct: StructValue<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> {
        self.builder.build_extract_value(cond_struct, 0, "get_type")
    }

    fn print_type(&self, object: StructValue<'ctx>, fn_value: FunctionValue<'ctx>) {
        let type_index = self.extract_type(object).unwrap().into_int_value();
        let print_fn = self.functions.printf;
        let bool_block = self.context.append_basic_block(fn_value, "bool");
        let number_block = self.context.append_basic_block(fn_value, "number");
        let string_block = self.context.append_basic_block(fn_value, "string");
        let cons_block = self.context.append_basic_block(fn_value, "cons");
        let lambda_block = self.context.append_basic_block(fn_value, "lambda");
        let symbol_block = self.context.append_basic_block(fn_value, "hempty");
        let hempty_block = self.context.append_basic_block(fn_value, "symbol");
        let thunk_block = self.context.append_basic_block(fn_value, "thunk");
        let error_block = self.context.append_basic_block(fn_value, "error");
        let ret_block = self.context.append_basic_block(fn_value, "return");
        let print_type = |block, name| {
            self.builder.position_at_end(block);

            self.builder.build_call(
                print_fn,
                &[self
                    .builder
                    .build_global_string_ptr(name, name)
                    .as_basic_value_enum()
                    .into()],
                &format!("print {name}"),
            );
            self.builder.build_unconditional_branch(ret_block);
        };
        self.builder.build_switch(
            type_index,
            error_block,
            &[
                (
                    self.types.ty.const_int(TyprIndex::boolean as u64, false),
                    bool_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::number as u64, false),
                    number_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::string as u64, false),
                    string_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::cons as u64, false),
                    cons_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::hempty as u64, false),
                    hempty_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::symbol as u64, false),
                    symbol_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::lambda as u64, false),
                    lambda_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::thunk as u64, false),
                    thunk_block,
                ),
            ],
        );
        print_type(bool_block, "bool");
        print_type(number_block, "number");
        print_type(string_block, "string");
        print_type(symbol_block, "symbol");
        print_type(cons_block, "cons");
        print_type(lambda_block, "lambda");
        print_type(thunk_block, "thunk");
        print_type(hempty_block, "hempty");
        print_type(error_block, "unknown error");
        self.builder.position_at_end(ret_block);
    }
}
