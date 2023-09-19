use inkwell::{
    basic_block::BasicBlock,
    values::{BasicValueEnum, IntValue},
};

use crate::ast::{Application, UMPL2Expr};

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
            .build_conditional_branch(cond, else_bb, then_bb);

        // else block
        self.builder.position_at_end(else_bb);
        let else_val = if let Some(alt) = exprs.get(2) {
            self.compile_expr(alt)?
        } else {
            Some(self.const_boolean(crate::ast::Boolean::False).into())
        };
        if else_val.is_some() {
            self.builder.build_unconditional_branch(cont_bb);
        }
        let else_bb = self.builder.get_insert_block().unwrap();

        // build else block
        self.builder.position_at_end(then_bb);
        let then_val = self.compile_expr(&exprs[1])?;
        if then_val.is_some() {
            self.builder.build_unconditional_branch(cont_bb);
        }
        let then_bb = self.builder.get_insert_block().unwrap();

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

    // TODO: need to work on macros so we can do cond->if
    // fn special_form_cond(
    //     &mut self,
    //     exprs: &[UMPL2Expr],
    // ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
    //     // 1. get condition
    //     // 2. see if condition is else -> make sure no more cases
    //     // 3. eval -> actual value condtion
    //     let exprs: Vec<Application> = exprs
    //         .into_iter()
    //         .map(|case| {
    //             if let UMPL2Expr::Application(app) = case {
    //                 Ok(app.clone())
    //             } else {
    //                 Err("cond case is not list")
    //             }
    //         })
    //         .collect::<Result<Vec<Application>, &str>>()?;
    //     let has_else = exprs
    //         .last()
    //         .and_then(|last| {
    //             if last.args()[0] == UMPL2Expr::Ident("else".into()) {
    //                 Some(())
    //             } else {
    //                 None
    //             }
    //         })
    //         .is_some();
    //     let (cases, elses): (Vec<Application>, Vec<Application>) = exprs
    //         .into_iter()
    //         .partition(|case| case.args()[0] == UMPL2Expr::Ident("else".into()));
    //     if elses.len() > 1 {
    //         Err("cond statement with multiple elses".to_string())
    //     } else if !has_else && elses.len() == 1 {
    //         Err("cond statement has else in middle".to_string())
    //     } else {
    //         let current_fn = self.current_fn_value()?;
    //         let start_bb = self.builder.get_insert_block().unwrap();
    //         let else_bb = self.context.append_basic_block(current_fn, "cond:else");
    //         let done_bb = self.context.append_basic_block(current_fn, "cond:done");
    //         self.builder.position_at_end(else_bb);
    //         let else_part = if let Some(r#else) = elses.get(0) {
    //             return_none!(self.compile_expr(&r#else.args()[1])?)
    //         } else {
    //             self.hempty().into()
    //         };
    //         self.builder.build_unconditional_branch(done_bb);
    //         let else_bblock = self.builder.get_insert_block().unwrap();
            
    //         let conds = return_none!(cases
    //             .iter()
    //             .enumerate()
    //             .map(|(i, case)| {
    //                 let bb = self
    //                     .context
    //                     .append_basic_block(current_fn, &format!("cond:case{i}"));
    //                 self.builder.position_at_end(start_bb);
    //                 let cond = self.compile_expr(&case.args()[0])?;
    //                 let cond = self.builder.build_not(
    //                     self.is_false(
    //                         self.actual_value(return_none!(cond).into_struct_value())
    //                             .into(),
    //                     ),
    //                     "cond:invert-cond",
    //                 );
    //                 self.builder.position_at_end(bb);
    //                 let expr = return_none!(self.compile_expr(&case.args()[1])?);
    //                 self.builder.build_unconditional_branch(done_bb);
    //                 let bb = self.builder.get_insert_block().unwrap();

    //                 Ok(Some((bb, cond, expr)))
    //             })
    //             .collect::<Result<
    //                 Option<Vec<(BasicBlock<'ctx>, IntValue<'ctx>, BasicValueEnum<'ctx>)>>,
    //                 String,
    //             >>()?);
    //             self.builder.position_at_end(start_bb);
    //             // self.builder.build_switch(, else_block, cases)
            
    //         Err("cond statement has else in middle".to_string())
    //     }
    // }
}
