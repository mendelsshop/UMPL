use inkwell::values::{BasicValueEnum, PhiValue, StructValue};

use crate::{ast::UMPL2Expr, interior_mut::RC};

use super::{Compiler, EvalType, TyprIndex};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn special_form_loop(
        &mut self,
        exprs: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        let loop_bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "loop");
        let done_bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "done-loop");
        self.builder.build_unconditional_branch(loop_bb);
        self.builder.position_at_end(done_bb);
        let phi_return = self.builder.build_phi(self.types.object, "loop ret");
        self.state.push(EvalType::Loop {
            loop_bb,
            done_loop_bb: done_bb,
            connection: phi_return,
        });
        self.builder.position_at_end(loop_bb);
        for expr in exprs {
            self.compile_expr(expr)?;
        }
        self.builder.build_unconditional_branch(loop_bb);

        self.builder.position_at_end(done_bb);
        self.state.pop();
        Ok(Some(phi_return.as_basic_value()))
    }

    pub(crate) fn special_form_for_loop(
        &mut self,
        exprs: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        if exprs.len() < 2 {
            return Err("Expected 3 expression for for loop".to_string());
        }
        // iterates with `in order`
        let UMPL2Expr::Ident(name) = &exprs[0] else {
            return Err("no identifier to usef for iteration".to_string());
        };
        let iter = &exprs[1];
        let iter = return_none!(self.compile_expr(iter)?).into_struct_value();
        let phi = self.make_iter(iter, name.clone(), &exprs[2..])?;
        Ok(Some(phi.as_basic_value()))
    }

    fn make_iter(
        &mut self,
        expr: StructValue<'ctx>,
        name: RC<str>,
        iter_scope: &[UMPL2Expr],
    ) -> Result<PhiValue<'ctx>, String> {
        let helper_struct = self.context.struct_type(
            &[self.types.object.into(), self.types.generic_pointer.into()],
            false,
        );

        // TODO: wherever there is null checks to stop/continue iteratoration we also need to check for hempty
        // keep current tree and a new helper tree (initally empty)
        // 1. check if tree is empty if jump to 4
        // 2. if left tree is empty then obtain current from tree and do code for iteration
        // and set main tree to right goto 1
        // 3. otherwise save/append (current and right) into helper by creating new tree with left null and current and right from tree and put onto root of helper
        // put lefttree into main tree and goto 1
        // 4. if helper empty goto 5
        // otherwise pop first from helper into main tree goto 1
        // 5. exit/return hempty
        // (needs to be slightly adjusted to build up new tree as opposed to doing code per iteration)

        // base logic done but it doesnt account for thunks and the actualt structure of the tree being objects

        // init blocks required

        // block where we check if tree null if so jump to loop swap or if not to loop process
        let loop_entry_bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "loop-entry");
        // block where we car down (jump to loop save) untill null car and then jump loop_bb
        let loop_process_bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "loop-process");
        // execute loop
        let loop_bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "loop");
        // save (null cdr cgr) to helper tree set tree to car jump to loop entry
        let loop_save_bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "loop_save");
        // pop of root from helper if both trees null exit otherwise jump to loop entry ()
        let loop_swap_bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "loop-swap");
        let loop_swap_inner_bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "loop-swap-inner");
        let loop_done_bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "done-loop");

        // allocate trees
        let tree = self
            .create_entry_block_alloca(self.types.object, "iter-tree")
            .unwrap();
        let helper = self
            .create_entry_block_alloca(self.types.generic_pointer, "iter-helper")
            .unwrap();
        let val = self.actual_value(expr);
        // initialize trees
        self.builder.build_store(tree, val);

        self.builder
            .build_store(helper, self.types.generic_pointer.const_null());

        self.builder.build_unconditional_branch(loop_entry_bb);

        // loop_entry
        self.builder.position_at_end(loop_entry_bb);

        let tree_load = self
            .builder
            .build_load(self.types.object, tree, "load tree")
            .into_struct_value();
        let is_tree_hempty = self.is_hempty(tree_load);

        self.builder
            .build_conditional_branch(is_tree_hempty, loop_swap_bb, loop_process_bb);

        // loop_process
        self.builder.position_at_end(loop_process_bb);
        // this logic is wrong b/c were already know thst the tree is non null -> the branch will also be non null
        // what we really need to check for is if the branch (car) is hempty (maybe also same problem for loop_entry)
        let tree_load = self
            .builder
            .build_load(self.types.object, tree, "load tree")
            .into_struct_value();
        let tree_cons = self.extract_cons(tree_load)?;
        let car = self
            .builder
            .build_extract_value(tree_cons.into_struct_value(), 0, "get car")
            .unwrap();
        let is_car_hempty = self.is_hempty(car.into_struct_value());

        self.builder
            .build_conditional_branch(is_car_hempty, loop_bb, loop_save_bb);

        // loop_done
        self.builder.position_at_end(loop_done_bb);
        let phi = self.builder.build_phi(self.types.object, "loop value");

        // loop
        self.state.push(EvalType::Loop {
            loop_bb: loop_entry_bb,
            done_loop_bb: loop_done_bb,
            connection: phi,
        });
        self.builder.position_at_end(loop_bb);
        let tree_load: StructValue<'_> = self
            .builder
            .build_load(self.types.object, tree, "load tree")
            .into_struct_value();
        let tree_cons = self.extract_cons(tree_load)?;
        let val = self
            .builder
            .build_extract_value(tree_cons.into_struct_value(), 1, "get current")
            .unwrap();
        let this = self.builder.build_alloca(self.types.object, "save this");
        self.builder.build_store(this, val);

        self.insert_variable(name, this);
        // code goes here
        for expr in iter_scope {
            self.compile_expr(expr)?;
        }
        // delete the variable
        // put cgr of tree as tree

        let cgr = self
            .builder
            .build_extract_value(tree_cons.into_struct_value(), 2, "get next")
            .unwrap();
        // let cgr = self.actual_value(cgr);
        self.builder.build_store(tree, cgr);
        self.builder.build_unconditional_branch(loop_entry_bb);

        // loop_save
        self.builder.position_at_end(loop_save_bb);

        let tree_load: StructValue<'_> = self
            .builder
            .build_load(self.types.object, tree, "load tree")
            .into_struct_value();
        let tree_cons = self
            .builder
            .build_extract_value(tree_load, TyprIndex::cons as u32 + 1, "extract cons")
            .unwrap();
        let this = self
            .builder
            .build_struct_gep(self.types.cons, tree_cons.into_pointer_value(), 1, "cdr")
            .unwrap();
        let this = self.builder.build_load(self.types.object, this, "load cdr");
        let cgr = self
            .builder
            .build_struct_gep(self.types.cons, tree_cons.into_pointer_value(), 2, "cgr")
            .unwrap();
        let cgr = self.builder.build_load(self.types.object, cgr, "load cgr");
        let new_cons = self
            .builder
            .build_alloca(self.types.cons, "new cons in loop");
        let save = self.const_cons_with_ptr(
            new_cons,
            self.hempty(),
            this.into_struct_value(),
            cgr.into_struct_value(),
        );
        let helper_load =
            self.builder
                .build_load(self.types.generic_pointer, helper, "load helper");
        let new_helper = self.builder.build_alloca(helper_struct, "new helper");
        let new_helper_value = helper_struct.const_zero();
        let new_helper_value = self
            .builder
            .build_insert_value(new_helper_value, save, 0, "insert current value")
            .unwrap();
        let new_helper_value = self
            .builder
            .build_insert_value(new_helper_value, helper_load, 1, "insert current prev")
            .unwrap();
        // let new_helper_obj = self
        //     .builder
        //     .build_struct_gep(helper_struct, new_helper, 0, "gep new helper current node")
        //     .unwrap();
        // self.builder.build_store(new_helper_obj, save);
        // let new_helper_prev = self
        //     .builder
        //     .build_struct_gep(
        //         helper_struct,
        //         new_helper,
        //         1,
        //         "gep new helper previous helper node",
        //     )
        //     .unwrap();
        // self.builder.build_store(new_helper_prev, helper_load);
        self.builder.build_store(new_helper, new_helper_value);
        self.builder.build_store(helper, new_helper);

        let car = self
            .builder
            .build_struct_gep(self.types.cons, tree_cons.into_pointer_value(), 0, "cgr")
            .unwrap();
        let car = self.builder.build_load(self.types.object, car, "load cgr");
        self.builder.build_store(tree, car);
        self.builder.build_unconditional_branch(loop_entry_bb);

        // loop_swap
        self.builder.position_at_end(loop_swap_bb);
        let helper_load = self
            .builder
            .build_load(self.types.generic_pointer, helper, "load helper")
            .into_pointer_value();
        phi.add_incoming(&[(&self.hempty(), self.builder.get_insert_block().unwrap())]);
        self.builder.build_conditional_branch(
            self.is_null(helper_load),
            loop_done_bb,
            loop_swap_inner_bb,
        );
        self.builder.position_at_end(loop_swap_inner_bb);
        let helper_load_load = self
            .builder
            .build_load(helper_struct, helper_load, "load load helper")
            .into_struct_value();
        let current = self
            .builder
            .build_extract_value(helper_load_load, 0, "current helper_node")
            .unwrap();
        let rest = self
            .builder
            .build_extract_value(helper_load_load, 1, "rest")
            .unwrap();
        self.builder.build_store(tree, current);
        self.builder.build_store(helper, rest);

        let tree_load = self
            .builder
            .build_load(self.types.object, tree, "load tree")
            .into_struct_value();
        let is_tree_null = self.is_hempty(tree_load);
        let helper_load = self
            .builder
            .build_load(self.types.generic_pointer, helper, "load helper")
            .into_pointer_value();

        let is_helper_null = self.is_null(helper_load);
        let are_both_null = self
            .builder
            .build_and(is_helper_null, is_tree_null, "both is null");
        self.builder
            .build_conditional_branch(are_both_null, loop_done_bb, loop_entry_bb);
        phi.add_incoming(&[(&self.hempty(), self.builder.get_insert_block().unwrap())]);
        self.state.pop();
        self.builder.position_at_end(loop_done_bb);
        Ok(phi)
    }

    pub(crate) fn special_form_while_loop(
        &mut self,
        exprs: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        if exprs.is_empty() {
            return Err("expected 2 expression for while loop".to_string());
        }
        let loop_bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "loop");
        let loop_start_bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "loop-start");
        let done_bb = self
            .context
            .append_basic_block(self.fn_value.unwrap(), "done-loop");
        self.builder.build_unconditional_branch(loop_start_bb);
        self.builder.position_at_end(done_bb);
        let phi_return = self.builder.build_phi(self.types.object, "loop ret");
        self.state.push(EvalType::Loop {
            done_loop_bb: done_bb,
            connection: phi_return,
            loop_bb: loop_start_bb,
        });

        self.builder.position_at_end(loop_start_bb);

        let expr = return_none!(self.compile_expr(&exprs[0])?);
        let expr = self.actual_value(expr.into_struct_value());
        let cond = self.is_false(expr.into());
        self.builder
            .build_conditional_branch(cond, done_bb, loop_bb);
        // if we break b/c condition not met the loop return hempty
        phi_return.add_incoming(&[(&self.hempty(), self.builder.get_insert_block().unwrap())]);
        self.builder.position_at_end(loop_bb);
        for expr in &exprs[1..] {
            self.compile_expr(expr)?;
        }
        self.builder.build_unconditional_branch(loop_start_bb);
        self.builder.position_at_end(done_bb);
        self.state.pop();
        Ok(Some(phi_return.as_basic_value()))
    }

    pub(crate) fn special_form_skip(
        &mut self,
        exprs: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        if !exprs.is_empty() {
            return Err("skip just skips no need for a value".to_string());
        }
        self.builder.build_unconditional_branch(
            *self
                .state
                .iter()
                .rev()
                .find_map(|state| match state {
                    EvalType::Function => None,
                    EvalType::Loop { loop_bb, .. } => Some(loop_bb),
                })
                .ok_or("skip found outside loop")?,
        );
        Ok(None)
    }
}
