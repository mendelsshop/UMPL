use crate::cli::EASY_MODE;
use std::process::exit;

fn report(line: i32, where_: &str, message: &str) {
    let mut message = message;
    unsafe {
        if EASY_MODE {
        } else {
            message = "Segmentation fault (core dumped)";
        }
    }
    if message.is_empty() {
        message = "Segmentation fault (core dumped)";
    }
    println!("[line: {}], Error{}: {}", line, where_, message);
    exit(1);
}

pub fn error(line: i32, message: &str) {
    report(line, "", message)
}
