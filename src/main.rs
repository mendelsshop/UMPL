#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(
    clippy::use_self,
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_panics_doc
)]
#![allow(clippy::similar_names)]

use std::{
    fs,
    io::{self, Write},
    process::exit,
};

use inkwell::{context::Context, passes::PassManager};

use crate::{codegen::Compiler, lexer::umpl_parse};
use clap::{Parser, Subcommand};
pub mod analyzer;
pub mod ast;
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
#[derive(Parser, Debug)]
#[clap(author = "mendelsshop", version, about, long_about = None, name = "UMPL")]
pub struct Args {
    #[clap(subcommand)]
    arg: ArgType,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ArgType {
    /// Start a `UMPL` repl
    Repl,
    /// Compile some code
    Compile {
        filename: String,
        /// Output file name excluding file extension
        output: String,
    },
    /// Run some code
    Run { filename: String },
}

fn main() {
    let args = Args::parse();
    match args.arg {
        ArgType::Repl => repl(),
        ArgType::Compile { filename, output } => compile(&filename, &output),
        ArgType::Run { filename } => run(&filename),
    }
}

fn init_function_optimizer<'ctx>(
    module: &inkwell::module::Module<'ctx>,
) -> PassManager<inkwell::values::FunctionValue<'ctx>> {
    let fpm = PassManager::create(module);
    fpm.add_new_gvn_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_aggressive_inst_combiner_pass();
    // // doesn't work with current goto implementation
    // // fpm.add_cfg_simplification_pass();
    fpm.add_aggressive_dce_pass();
    fpm.add_instruction_simplify_pass();

    fpm.add_verifier_pass();
    fpm.add_bit_tracking_dce_pass();
    fpm.add_merged_load_store_motion_pass();
    fpm.add_ind_var_simplify_pass();
    // // doesn't work with current goto implementation
    // // fpm.add_jump_threading_pass();

    fpm.add_scalarizer_pass();
    fpm.add_tail_call_elimination_pass();

    fpm.initialize();
    fpm
}

fn repl() {
    let context = Context::create();
    let module = context.create_module("repl");
    let builder = context.create_builder();

    // Create FPM
    let fpm = init_function_optimizer(&module);

    let mut input = String::new();
    let mut input_new = String::new();
    // we use repl as opposed to jit see https://github.com/TheDan64/inkwell/issues/397
    let mut complier =
        { Compiler::new(&context, &module, &builder, &fpm, codegen::EngineType::Jit) };
    while let Ok(_) = {
        print!(">>> ");
        io::stdout().flush().expect("couldn't flush output");
        io::stdin().read_line(&mut input_new)
    } {
        if input_new.trim() == "run" {
            // really eneffecient to create a new compiler every time (and not whats expected either)
            // but currently there is no way to add onto main function after first compile

            let parsed = umpl_parse(&input).unwrap();
            let program = analyzer::Analyzer::analyze(&parsed);
            complier.compile_program(&program.1, program.0).map_or_else(
                || {
                    complier.export_ir("bin/main");
                    complier.run().expect("no execution engine found");
                    println!();
                },
                |err| {
                    println!("error: {err}");
                    exit(1);
                },
            );
            input.clear();
        } else if input_new.trim() == "quit" {
            exit(0)
        } else {
            input += &input_new;
        }
        input_new.clear();
    }
}

fn compile(file: &str, out: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let parsed = umpl_parse(&contents).unwrap();
    let program = analyzer::Analyzer::analyze(&parsed);
    let context = Context::create();
    let module = context.create_module(file);
    let builder = context.create_builder();

    // Create FPM
    let fpm = init_function_optimizer(&module);
    let mut complier =
        { Compiler::new(&context, &module, &builder, &fpm, codegen::EngineType::None) };
    complier.compile_program(&program.1, program.0).map_or_else(
        || {
            // TODO: actually compile the program not just generate llvm ir
            complier.export_ir(out);
        },
        |err| {
            println!("error: {err}");
            exit(1);
        },
    );
}

fn run(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let parsed = umpl_parse(&contents).unwrap();
    let program = analyzer::Analyzer::analyze(&parsed);
    let context = Context::create();
    let module = context.create_module(file);
    let builder = context.create_builder();
    // Create FPM
    let fpm = init_function_optimizer(&module);
    let mut complier =
        { Compiler::new(&context, &module, &builder, &fpm, codegen::EngineType::Jit) };
    complier.compile_program(&program.1, program.0).map_or_else(
        || {
            let ret = complier.run().expect("no execution engine found");
            exit(ret);
        },
        |err| {
            println!("error: {err}");
            exit(1);
        },
    );
}
