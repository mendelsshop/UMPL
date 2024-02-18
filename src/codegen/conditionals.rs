use inkwell::values::BasicValueEnum;

use crate::ast::UMPL2Expr;

use super::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    // special form if
    pub fn special_form_if(
        &mut self,
        exprs: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        if exprs.len() < 2 || exprs.len() > 3 {
            return Err("bad form for if expression".to_string());
        }
        let parent = self.current_fn_value()?;
        let thunked = return_none!(self.compile_expr(&exprs[0])?).into_struct_value();
        let cond_struct = self.actual_value(thunked);
        let cond = self.is_false(cond_struct.into());
        let then_bb = self.context.append_basic_block(parent, "then");
        let else_bb = self.context.append_basic_block(parent, "else");
        let cont_bb = self.context.append_basic_block(parent, "ifcont");
        self.builder
            .build_conditional_branch(cond, else_bb, then_bb)
            .unwrap();

        // else block
        self.builder.position_at_end(else_bb);
        let else_val = if let Some(alt) = exprs.get(2) {
            self.compile_expr(alt)?
        } else {
            Some(self.const_boolean(crate::ast::Boolean::False).into())
        };
        if else_val.is_some() {
            self.builder.build_unconditional_branch(cont_bb).unwrap();
        }
        let else_bb = self.builder.get_insert_block().unwrap();

        // build else block
        self.builder.position_at_end(then_bb);
        let then_val = self.compile_expr(&exprs[1])?;
        if then_val.is_some() {
            self.builder.build_unconditional_branch(cont_bb).unwrap();
        }
        let then_bb = self.builder.get_insert_block().unwrap();

        // emit merge block
        self.builder.position_at_end(cont_bb);

        let phi = self
            .builder
            .build_phi(self.types.object, "if:phi-cont")
            .unwrap();
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
