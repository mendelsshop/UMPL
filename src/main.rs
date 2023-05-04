#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(clippy::use_self, rust_2018_idioms)]
#![allow(clippy::similar_names, clippy::missing_errors_doc)]

use std::io::Write;

use crate::{env::Env, eval::eval_expr};

mod ast;
mod env;
mod eval;
mod parser;

fn main() {
    let env = Env::new();
    let mut input = String::new();
    loop {
        input.clear();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        // remove ending \n
        let mut chars = input.trim().chars().peekable();
        while chars.peek().is_some() {
            let expr = parser::parse(&mut chars);
            println!("=>");
            println!("{}", eval_expr(expr, env.clone()));
        }
    }
}
