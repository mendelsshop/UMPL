use inkwell::{
    types::FunctionType,
    values::{BasicValue, BasicValueEnum, StructValue},
};

use super::{Compiler, TyprIndex};

/// provides a standard library and adds the functions to the root envoirment
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(super) fn make_print(&mut self) {
        // maybe make print should turn into make string

        let print_fn_ty: FunctionType<'_> = self.types.object.fn_type(
            &[self.types.call_info.into(), self.types.object.into()],
            false,
        );
        let print_fn = self.module.add_function("print", print_fn_ty, None);
        self.fn_value = Some(print_fn);
        let entry_block = self.context.append_basic_block(print_fn, "entry");
        let bool_block = self.context.append_basic_block(print_fn, "bool");
        let number_block = self.context.append_basic_block(print_fn, "number");
        let string_block = self.context.append_basic_block(print_fn, "string");
        let cons_block = self.context.append_basic_block(print_fn, "cons");
        // let lambda_block = self.context.append_basic_block(print_fn, "lambda");
        let symbol_block = self.context.append_basic_block(print_fn, "hempty");
        let hempty_block = self.context.append_basic_block(print_fn, "symbol");
        let ret_block = self.context.append_basic_block(print_fn, "return");
        let error_block = self.context.append_basic_block(print_fn, "error");
        self.builder.position_at_end(entry_block);
        let args = print_fn.get_nth_param(1).unwrap().into_struct_value();
        let args = self.actual_value(args);
        let ty = self.extract_type(args).unwrap().into_int_value();
        self.builder.build_switch(
            ty,
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
            ],
        );
        let print = self.functions.printf;
        let print_type = |block,
                          extracter: fn(
            &Compiler<'a, 'ctx>,
            StructValue<'ctx>,
        ) -> Result<BasicValueEnum<'ctx>, String>,
                          fmt_spec,
                          name| {
            self.builder.position_at_end(block);
            let val = extracter(self, args).unwrap();

            self.builder.build_call(
                print,
                &[
                    self.builder
                        .build_global_string_ptr(fmt_spec, &format!("{name} fmt specifier"))
                        .as_basic_value_enum()
                        .into(),
                    val.into(),
                ],
                &format!("print {name}"),
            );
            self.builder.build_unconditional_branch(ret_block);
            self.builder.get_insert_block().unwrap()
        };
        let bool_block = print_type(bool_block, Self::extract_bool, "%i", "boolean");
        let number_block = print_type(number_block, Self::extract_number, "%f", "number");
        let string_block = print_type(string_block, Self::extract_string, "%s", "string");
        let symbol_block = print_type(symbol_block, Self::extract_symbol, "%s", "symbol");
        self.builder.position_at_end(cons_block);
        let call_info = self.types.call_info.const_named_struct(&[
            self.context.i64_type().const_int(1, false).into(),
            self.types.generic_pointer.const_null().into(),
        ]);
        // let val = self.extract_cons( args).unwrap();
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr("(", "open paren")
                .as_basic_value_enum()
                .into()],
            "print open",
        );
        let val = self
            .builder
            .build_call(
                self.module.get_function("car").unwrap(),
                &[call_info.into(), args.into()],
                "getcar",
            )
            .try_as_basic_value()
            .unwrap_left();
        self.builder
            .build_call(print_fn, &[call_info.into(), val.into()], "printcar");
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr(" ", "space")
                .as_basic_value_enum()
                .into()],
            "print space",
        );
        let val = self
            .builder
            .build_call(
                self.module.get_function("cdr").unwrap(),
                &[call_info.into(), args.into()],
                "getcar",
            )
            .try_as_basic_value()
            .unwrap_left();
        self.builder
            .build_call(print_fn, &[call_info.into(), val.into()], "printcar");
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr(" ", "space")
                .as_basic_value_enum()
                .into()],
            "print space",
        );
        let val = self
            .builder
            .build_call(
                self.module.get_function("cgr").unwrap(),
                &[call_info.into(), args.into()],
                "getcar",
            )
            .try_as_basic_value()
            .unwrap_left();
        self.builder
            .build_call(print_fn, &[call_info.into(), val.into()], "printcar");
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr(")", "open paren")
                .as_basic_value_enum()
                .into()],
            "print open",
        );
        self.builder.build_unconditional_branch(ret_block);
        let cons_block = self.builder.get_insert_block().unwrap();
        self.builder.position_at_end(hempty_block);
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr("hempty", "hempty")
                .as_pointer_value()
                .into()],
            "printcar",
        );
        self.builder.build_unconditional_branch(ret_block);
        self.builder.position_at_end(error_block);
        self.builder.build_call(
            print,
            &[
                self.builder
                    .build_global_string_ptr("not valid type %d\n", "idk")
                    .as_pointer_value()
                    .into(),
                ty.into(),
            ],
            "printcar",
        );

        self.exit("", 1);

        self.builder.position_at_end(ret_block);
        let phi = self.builder.build_phi(self.types.object, "print return");
        phi.add_incoming(&[
            (&args, bool_block),
            (&args, number_block),
            (&args, string_block),
            (&args, cons_block),
            (&args, hempty_block),
            (&args, symbol_block),
        ]);
        self.builder.build_return(Some(&phi.as_basic_value()));
        self.insert_function("print".into(), print_fn)
    }

    pub(super) fn make_add(&mut self) {
        let fn_ty = self.types.primitive_ty;
        let func = self.module.add_function("add", fn_ty, None);
        self.fn_value = Some(func);
        let entry = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(entry);
        let va_list = self
            .create_entry_block_alloca(self.types.generic_pointer, "va_list")
            .unwrap();
        self.builder
            .build_call(self.functions.va_start, &[va_list.into()], "init args");
        self.builder
            .build_va_arg(va_list, self.types.generic_pointer, "va first");
        self.builder
            .build_call(self.functions.va_end, &[va_list.into()], "va end");
        self.builder.build_return(Some(&self.hempty()));
        self.insert_function("add".into(), func);
    }

    pub(super) fn make_accesors(&mut self) {
        let fn_ty = self.types.object.fn_type(
            &[self.types.call_info.into(), self.types.object.into()],
            false,
        );
        let mut accesor = |idx, name| {
            let func = self.module.add_function(name, fn_ty, None);
            let entry = self.context.append_basic_block(func, name);
            self.builder.position_at_end(entry);
            let args = func.get_nth_param(1).unwrap().into_struct_value();
            self.fn_value = Some(func);
            let args = self.actual_value(args);
            let cons_object = self.extract_cons(args).unwrap().into_struct_value();
            let car = self
                .builder
                .build_extract_value(cons_object, idx, &format!("get-{name}"))
                .unwrap();
            self.builder.build_return(Some(&car));
            self.insert_function(name.into(), func);
        };
        accesor(0, "car");
        accesor(1, "cdr");
        accesor(2, "cgr");
    }

    pub(super) fn init_stdlib(&mut self) {
        self.make_accesors();
        self.make_add();
        self.make_print();
    }
}
