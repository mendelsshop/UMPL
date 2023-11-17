use super::{Compiler, TyprIndex};
use inkwell::values::{BasicValue, BasicValueEnum, StructValue};

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
            self.print_type(val);
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

    pub(crate) fn print_type(&self, object: StructValue<'ctx>) {
        let type_index = self.extract_type(object).unwrap().into_int_value();
        let print_fn = self.functions.printf;

        let type_as_str = |index, name| {
            (
                self.builder.build_int_compare(
                    inkwell::IntPredicate::EQ,
                    type_index,
                    self.types.ty.const_int(index as u64, false),
                    "",
                ),
                self.builder
                    .build_global_string_ptr(name, name)
                    .as_basic_value_enum(),
            )
        };
        let type_name = self.build_n_select(
            self.builder
                .build_global_string_ptr("type not recognized", "")
                .as_basic_value_enum(),
            &[
                type_as_str(TyprIndex::boolean, "boolean"),
                type_as_str(TyprIndex::number, "number"),
                type_as_str(TyprIndex::string, "string"),
                type_as_str(TyprIndex::cons, "cons"),
                type_as_str(TyprIndex::hempty, "hempty"),
                type_as_str(TyprIndex::symbol, "symbol"),
                type_as_str(TyprIndex::lambda, "lambda"),
                type_as_str(TyprIndex::thunk, "thunk"),
            ],
        );
        self.builder
            .build_call(print_fn, &[type_name.into()], "print type");
    }
}
