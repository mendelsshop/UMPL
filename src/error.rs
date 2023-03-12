use crate::cli::EASY_MODE;
use std::{fmt::Display, process::exit};

pub fn error<T: Display>(line: u32, message: T) -> ! {
    let where_ = "";
    let message = message.to_string();
    unsafe {
        if EASY_MODE && !message.is_empty() {
        } else {
            eprint!("[line: {line}], Error{where_}");
            stackoverflow();
        }
    }
    eprintln!("[line: {line}], Error{where_}: {message}");
    exit(1);
}

#[allow(unconditional_recursion)]
fn stackoverflow() {
    stackoverflow();
}

#[allow(clippy::module_name_repetitions)]
pub fn arg_error<T: Display>(
    num_args: u32,
    given_args: u32,
    function: T,
    at_least: bool,
    line: u32,
) {
    if at_least {
        if num_args > given_args {
            error(
                line,
                format!("{function} requires at least {num_args} arguments"),
            );
        }
    } else if num_args != given_args {
        error(line, format!("{function} requires {num_args} arguments"));
    }
}
