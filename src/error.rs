use crate::cli::EASY_MODE;
use std::{fmt::Display, process::exit};

fn report<T: Display>(line: i32, where_: &str, message: T) -> ! {
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
    eprintln!("[line: {}], Error{}: {}", line, where_, message);
    exit(1);
}

pub fn error<T: Display>(line: i32, message: T) -> ! {
    report(line, "", message)
}
