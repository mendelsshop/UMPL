fn report(line: i32, where_: &str, message: &str) {
    let mut message = message;
    if message.is_empty() {
        message = "Segmentation fault (core dumped)";
    }
    println!("[line: {}], Error{}: {}", line, where_, message);
}

pub fn error(line: i32, message: &str) {
    report(line, "", message)
}
