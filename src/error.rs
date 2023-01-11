use crate::cli::EASY_MODE;
use std::{fmt::Display, process::exit};

pub fn error<T: Display>(line: i32, message: T) -> ! {
    let where_ = "";
    let message = message.to_string();
    let mut message = message.as_str();
    unsafe {
        if EASY_MODE {
        } else {
            message = "Segmentation fault (core dumped)";
        }
    }
    if message.is_empty() {
        message = "Segmentation fault (core dumped)";
    }
    eprintln!("[line: {line}], Error{where_}: {message}");
    exit(1);
}

#[allow(clippy::module_name_repetitions)]
pub fn arg_error<T: Display>(
    num_args: u32,
    given_args: u32,
    function: T,
    at_least: bool,
    line: i32,
) {
    if at_least {
        if num_args < given_args {
            error(
                line,
                format!("{function} requires at least {num_args} arguments"),
            );
        }
    } else if num_args != given_args {
        error(
            line,
            format!("{function} requires {num_args} arguments"),
        );
    }
}
