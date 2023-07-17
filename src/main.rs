use inkwell::{context::Context, passes::PassManager, values::AnyValue};

use crate::{codegen::Compiler, lexer::parse_umpl};

pub mod ast;
mod codegen;
// pub mod eval;
mod cg;
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
    let fn_type = match parse_umpl("fanction 🚗  1 ᚜   ᚛").unwrap() {
        ast::UMPL2Expr::Bool(_) => todo!(),
        ast::UMPL2Expr::Number(_) => todo!(),
        ast::UMPL2Expr::String(_) => todo!(),
        ast::UMPL2Expr::Scope(_) => todo!(),
        ast::UMPL2Expr::Ident(_) => todo!(),
        ast::UMPL2Expr::If(_) => todo!(),
        ast::UMPL2Expr::Unless(_) => todo!(),
        ast::UMPL2Expr::Stop(_) => todo!(),
        ast::UMPL2Expr::Skip => todo!(),
        ast::UMPL2Expr::Until(_) => todo!(),
        ast::UMPL2Expr::GoThrough(_) => todo!(),
        ast::UMPL2Expr::ContiueDoing(_) => todo!(),
        ast::UMPL2Expr::Fanction(f) => f,
        ast::UMPL2Expr::Application(_) => todo!(),
        ast::UMPL2Expr::Quoted(_) => todo!(),
        ast::UMPL2Expr::Label(_) => todo!(),
        ast::UMPL2Expr::FnParam(_) => todo!(),
        ast::UMPL2Expr::Hempty => todo!(),
        ast::UMPL2Expr::Link(_, _) => todo!(),
        ast::UMPL2Expr::Tree(_) => todo!(),
        ast::UMPL2Expr::FnKW(_) => todo!(),
        ast::UMPL2Expr::Let(_, _) => todo!(),
    };
    println!("{fn_type:?}");
    match Compiler::compile(&context, &builder, &fpm, &module, &fn_type) {
        Ok(o) => {
            println!("{o}");
            o.print_to_stderr();
        }
        Err(e) => println!("{e}"),
    }

    println!("Hello, world!");
}
