use inkwell::values::BasicValueEnum;

use super::{Compiler, TyprIndex};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn compile_if(
        &mut self,
        if_stmt: &crate::ast::If,
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        let parent = self.current_fn_value()?;
        let thunked = return_none!(self.compile_expr(if_stmt.cond())?).into_struct_value();
        let cond_struct = self.actual_value(thunked);
        // TODO: dont assume a bool
        let bool_val = self.extract_bool(cond_struct).unwrap().into_int_value();
        let object_type = self.extract_type(cond_struct).unwrap().into_int_value();
        // if its not a bool type
        let cond = self.builder.build_int_compare(
            inkwell::IntPredicate::NE,
            object_type,
            self.types.ty.const_int(TyprIndex::boolean as u64, false),
            "if:cond:boolean?",
        );

        // conditinal: either not bool or true
        let cond = self.builder.build_or(bool_val, cond, "if:cond:false?");
        let then_bb = self.context.append_basic_block(parent, "then");
        let else_bb = self.context.append_basic_block(parent, "else");
        let cont_bb = self.context.append_basic_block(parent, "ifcont");
        self.builder
            .build_conditional_branch(cond, then_bb, else_bb);
        self.builder.position_at_end(then_bb);
        let then_val = self.compile_scope(if_stmt.alt())?;
        if then_val.is_some() {
            self.builder.build_unconditional_branch(cont_bb);
        }
        let then_bb = self.builder.get_insert_block().unwrap();

        // build else block
        self.builder.position_at_end(else_bb);
        let else_val = self.compile_scope(if_stmt.cons())?;
        if else_val.is_some() {
            self.builder.build_unconditional_branch(cont_bb);
        }
        let else_bb = self.builder.get_insert_block().unwrap();

        // emit merge block
        self.builder.position_at_end(cont_bb);

        let phi = self.builder.build_phi(self.types.object, "if:phi-cont");
        match (then_val, else_val) {
            (None, None) => phi.add_incoming(&[]),
            (None, Some(else_val)) => phi.add_incoming(&[(&else_val, else_bb)]),
            (Some(then_val), None) => phi.add_incoming(&[(&then_val, then_bb)]),
            (Some(then_val), Some(else_val)) => {
                phi.add_incoming(&[(&then_val, then_bb), (&else_val, else_bb)]);
            }
        }
        Ok(Some(phi.as_basic_value()))
    }
}
