#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(
    clippy::use_self,
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_panics_doc
)]
#![allow(clippy::similar_names, dead_code, unused)]

use std::{
    fs,
    io::{self, Write},
    process::exit,
};

use codegen::{
    register_to_llvm::CodeGen,
    sicp::{Linkage, Register},
};
use inkwell::{context::Context, passes::PassManager};

use crate::{
    ast::{immutable_add_to_vec, pass1},
    codegen::Compiler,
    macros::parse_and_expand,
};
use clap::{Parser, Subcommand};

pub mod ast;
mod codegen;

pub mod lexer;
mod macros;
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
    Run {
        filename: String,
    },
    Expand {
        filename: String,
    },
    Sicp {
        filename: String,
    },
}

fn main() {
    let args = Args::parse();
    match args.arg {
        ArgType::Repl => repl(),
        ArgType::Compile { filename, output } => compile(&filename, &output),
        ArgType::Run { filename } => run(&filename),
        ArgType::Expand { filename } => expand(&filename),
        ArgType::Sicp { filename } => sicp(&filename),
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
    // // doesn't work with current goto implementation (non sicp)
    fpm.add_cfg_simplification_pass();
    fpm.add_aggressive_dce_pass();
    fpm.add_instruction_simplify_pass();

    fpm.add_verifier_pass();
    fpm.add_bit_tracking_dce_pass();
    fpm.add_merged_load_store_motion_pass();
    fpm.add_ind_var_simplify_pass();
    // // doesn't work with current goto implementation (non sicp)
    fpm.add_jump_threading_pass();

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
        { Compiler::new(&context, &module, &builder, &fpm, &codegen::EngineType::Jit) };
    while {
        print!(">>> ");
        io::stdout().flush().expect("couldn't flush output");
        io::stdin().read_line(&mut input_new)
    }
    .is_ok()
    {
        if input_new.trim() == "run" {
            // really eneffecient to create a new compiler every time (and not whats expected either)
            // but currently there is no way to add onto main function after first compile

            let program = parse_and_expand(&input).unwrap();
            complier.compile_program(&program.0, program.1).map_or_else(
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
    let program = parse_and_expand(&contents).unwrap();
    let context = Context::create();
    let module = context.create_module(file);
    let builder = context.create_builder();
    // Create FPM
    let fpm = init_function_optimizer(&module);
    let mut complier = {
        Compiler::new(
            &context,
            &module,
            &builder,
            &fpm,
            &codegen::EngineType::None,
        )
    };
    complier.compile_program(&program.0, program.1).map_or_else(
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
    let program = parse_and_expand(&contents).unwrap();
    let context = Context::create();
    let module = context.create_module(file);
    let builder = context.create_builder();
    // Create FPM
    // println!("{program:?}");
    let fpm = init_function_optimizer(&module);
    let mut complier =
        { Compiler::new(&context, &module, &builder, &fpm, &codegen::EngineType::Jit) };
    complier.compile_program(&program.0, program.1).map_or_else(
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

fn expand(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let program = parse_and_expand(&contents).unwrap();
    println!("{program:?}");
}

fn sicp(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let program = parse_and_expand(&contents).unwrap();
    let typed_program = program
        .0
        .into_iter()
        .try_fold((vec![], vec![]), |(exps, env), exp| {
            pass1((exp, env)).map(|(exp, env)| (immutable_add_to_vec(exps, exp), env))
        })
        .unwrap();
    // eprintln!(
    //     "{}\n",
    //     typed_program.0.iter().map(ToString::to_string).join("\n")
    // );
    let ir: Vec<_> = typed_program
        .0
        .into_iter()
        .flat_map(|expr| {
            codegen::sicp::compile(expr, Register::Val, Linkage::Next)
                .instructions()
                .to_vec()
        })
        .collect();
    // eprintln!("{}", ir.iter().map(ToString::to_string).join("\n"));
    let context = Context::create();
    let module = context.create_module(file);
    let builder = context.create_builder();
    let fpm = init_function_optimizer(&module);
    let mut codegen = CodeGen::new(&context, &builder, &module, &fpm);
    codegen.compile(ir);
    println!("\n{}", codegen.export_ir());
}
