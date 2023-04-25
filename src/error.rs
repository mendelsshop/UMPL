use crate::{cli::EASY_MODE, token::Info};
use std::{fmt::Display, process::exit};

pub fn error<T: Display>(info: Info<'_>, message: T) -> ! {
    let where_ = "";
    let message = message.to_string();
    unsafe {
        if EASY_MODE && !message.is_empty() {
        } else {
            eprint!("[{info}], Error{where_}");
            stackoverflow();
        }
    }
    eprintln!("[{info}], Error{where_}: {message}");
    exit(1);
}

#[allow(unconditional_recursion)]
pub(crate) fn stackoverflow() {
    stackoverflow();
    stackoverflow();
    stackoverflow();
}

#[allow(clippy::module_name_repetitions)]
pub fn arg_error<T: Display>(
    num_args: u64,   // Number of arguments required
    given_args: u64, // Number of arguments provided
    function: T,     // Name of the function
    at_least: bool,  // Whether the function requires at least the number of arguments
    info: Info<'_>,
) {
    if at_least {
        if num_args > given_args {
            error(
                info,
                format!("{function} requires at least {num_args} arguments"),
            );
        }
    } else if num_args != given_args {
        error(info, format!("{function} requires {num_args} arguments"));
    }
}
