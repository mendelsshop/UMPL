#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(
    clippy::use_self,
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_panics_doc
)]
#![allow(clippy::similar_names)]

use inkwell::{context::Context, passes::PassManager};

use crate::{codegen::Compiler, lexer::umpl_parse};

pub mod ast;
// mod codegen_back;
pub mod analyzer;
mod codegen;

pub mod lexer;
pub mod pc;
#[cfg(feature = "multi-threaded")]
pub mod interior_mut {
    use std::sync::{Arc, Mutex};

    pub type RC<T> = Arc<T>;
    pub type MUTEX<T> = Mutex<T>;
}

#[cfg(not(feature = "multi-threaded"))]
pub mod interior_mut {
    use std::{cell::RefCell, rc::Rc};

    pub type RC<T> = Rc<T>;
    pub type MUTEX<T> = RefCell<T>;
}

fn main() {
    let context = Context::create();
    let module = context.create_module("repl");
    let builder = context.create_builder();

    // Create FPM
    let fpm = PassManager::create(&module);

    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();

    fpm.initialize();
    // fanction  1* ᚜ (print '0')< ᚛
    let fn_type =
        umpl_parse("let cons 
                                fanction  2 ᚜ 
                                        let x '0' 
                                        let y '1' 
                                        fanction  1 ᚜ 
                                            if '0' 
                                                do ᚜x
                                            ᚛ 
                                                otherwise ᚜y
                                            ᚛
                                        ᚛
                                ᚛
                      
                     let k (cons 5 6)< 
                        
                        (print (k &)<)<
                        ").unwrap();
        // umpl_parse("let i 9 (print i)<").unwrap();
    println!("{fn_type:?}");
    let program = analyzer::Analyzer::analyze(&fn_type);
    let mut complier = Compiler::new(&context, &module, &builder, &fpm);
    complier.compile_program(&program.1, program.0).map_or_else(
        || {
            complier.print_ir();
            complier.export_bc("bin/main");
            complier.export_ir("bin/main");
            complier.export_object_and_asm("bin/main");
            let ret = complier.run();
            print!("\nret {ret}\n",);
        },
        |err| {
            println!("error: {err}");
        },
    );
}

#[test]
fn insert() {
    use inkwell::context::Context;

    let context = Context::create();
    let module = context.create_module("av");
    let void_type = context.void_type();
    let f32_type = context.f32_type();
    let i32_type = context.i32_type();
    let struct_type = context.struct_type(&[i32_type.into()], false);
    let array_type = struct_type.array_type(3);
    let fn_type = void_type.fn_type(&[], false);
    let fn_value = module.add_function("av_fn", fn_type, None);
    let builder = context.create_builder();
    let entry = context.append_basic_block(fn_value, "entry");

    builder.position_at_end(entry);

    let array_alloca = builder.build_alloca(array_type, "array_alloca");

    // #[cfg(any(
    //     feature = "llvm4-0",
    //     feature = "llvm5-0",
    //     feature = "llvm6-0",
    //     feature = "llvm7-0",
    //     feature = "llvm8-0",
    //     feature = "llvm9-0",
    //     feature = "llvm10-0",
    //     feature = "llvm11-0",
    //     feature = "llvm12-0",
    //     feature = "llvm13-0",
    //     feature = "llvm14-0"
    // ))]
    // let array = builder.build_load(array_alloca, "array_load").into_array_value();
    // #[cfg(any(feature = "llvm15-0", feature = "llvm16-0"))]
    let array = builder
        .build_load(array_type, array_alloca, "array_load")
        .into_array_value();

    // let z = context.opaque_struct_type("hi");



    let const_int1 = struct_type.const_named_struct(&[i32_type.const_int(2, false).into()]);
    let const_int2 = struct_type.const_named_struct(&[i32_type.const_int(5, false).into()]);
    let const_int3 = struct_type.const_named_struct(&[i32_type.const_int(6, false).into()]);

    assert!(builder
        .build_insert_value(array, const_int1, 0, "insert")
        .is_some());
    assert!(builder
        .build_insert_value(array, const_int2, 1, "insert")
        .is_some());
    assert!(builder
        .build_insert_value(array, const_int3, 2, "insert")
        .is_some());
    assert!(builder
        .build_insert_value(array, const_int3, 3, "insert")
        .is_none());
    builder.build_return(None);
    if fn_value.verify(true) {
        module.print_to_stderr()
    }
}
