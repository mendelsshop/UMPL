#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(clippy::use_self, rust_2018_idioms)]
#![allow(clippy::similar_names, clippy::missing_errors_doc)]

use std::io::Write;

use crate::{env::Env, eval::actual_value};

mod ast;
mod env;
mod eval;
mod parser;

fn main() {
    let env = Env::new();
    let mut input = String::new();
    //clear the screen
    print!("\x1B[2J\x1B[1;1H");
    loop {
        input.clear();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        // remove ending \n
        let mut chars = input.trim().chars().peekable();
        while chars.peek().is_some() {
            let expr = parser::parse(&mut chars);
            // we evaluate the expression but don't print the result
            // because we want to any calls to display to print before the output prompt (=>)
            // force lazy things
            let result = actual_value(expr, env.clone());
            println!("\n=>");
            println!("{result}");
        }
    }
}
