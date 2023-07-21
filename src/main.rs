#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(
    clippy::use_self,
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_panics_doc
)]
#![allow(clippy::similar_names)]

use std::time::Duration;

use inkwell::{context::Context, passes::PassManager};
use unic_emoji_char::is_emoji_presentation;

use crate::{codegen::Compiler, lexer::umpl_parse};

pub mod ast;
// mod codegen_back;
// pub mod eval;
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
    let fn_type = umpl_parse(" (print 5)<").unwrap();
    println!("{fn_type:?}");
    let mut complier = Compiler::new(&context, &module, &builder, &fpm);
    complier.compile_program(&fn_type).map_or_else(
        || {
            complier.print_ir();
            let ret = complier.run();

            print!("\nret {ret}\n", );
        },
        |err| {
            println!("error: {err}");
        },
    );
}
