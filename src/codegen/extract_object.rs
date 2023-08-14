use super::{Compiler, TyprIndex};
use inkwell::values::{BasicValueEnum, StructValue};

macro_rules! make_extract {
    ($fn_name:ident, $type:ident) => {
        pub(super) fn $fn_name(
            &self,
            val: StructValue<'ctx>,
        ) -> Result<BasicValueEnum<'ctx>, String> {
            let current_fn = self.current_fn_value()?;
            let prefix = |end| format!("extract-{}:{end}", "$type");
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
            self.exit(&format!("not a {}\n", "$type"), 1);

            self.builder.position_at_end(ret_block);
            self.builder
                .build_extract_value(val, TyprIndex::$type as u32 + 1, &prefix("return"))
                .ok_or("could not extract value".to_string())
        }
    };
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    make_extract!(extract_number, number);
    make_extract!(extract_string, string);
    make_extract!(extract_symbol, symbol);
    make_extract!(extract_bool, boolean);
    make_extract!(extract_primitve, primitive);
    make_extract!(extract_labmda, lambda);
    make_extract!(extract_thunk, thunk);
    make_extract!(extract_cons_inner, cons);
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
}