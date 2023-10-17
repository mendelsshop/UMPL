use super::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    // try to retrieve the function and block address from the goto hashmap
    // if not there save whatevers needed and once all codegen completed retry to get information function/address for label from goto hashmap
    // and information to build at the right positon and do it

    // should add unreachable after this?
    // what should this return?

    // if let Some(link) = self.links.get(s) {
    //     let call_info = self.types.call_info.const_named_struct(&[
    //         self.context.i64_type().const_zero().into(),
    //         link.0.into(),
    //     ]);

    //     self.builder.build_call(
    //         link.1,
    //         &[
    //             self.types.generic_pointer.const_null().into(),
    //             call_info.into(),
    //             self.types.generic_pointer.const_null().into(),
    //         ],
    //         "jump",
    //     );
    // // maybe should be signal that we jumped somewhere
    // } else {
    //     let basic_block = self.builder.get_insert_block().unwrap();
    //     // will be overriden later if we have a link for the basic block
    //     self.builder.build_alloca(self.types.ty, "placeholder");
    //     let last_inst = basic_block.get_last_instruction();
    //     self.non_found_links
    //         .push((s.clone(), basic_block, last_inst));
    // }
    // Ok(Some(self.hempty().into()))

    // UMPL2Expr::ComeTo(n) => {
    //     let block = self.context.append_basic_block(self.fn_value.unwrap(), n);
    //     self.links.insert(
    //         n.clone(),
    //         (
    //             unsafe { block.get_address().unwrap() },
    //             self.fn_value.unwrap(),
    //         ),
    //     );
    //     self.builder.build_unconditional_branch(block);
    //     self.builder.position_at_end(block);
    //     Ok(Some(self.hempty().into()))
    // }
}
