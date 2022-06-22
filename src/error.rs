fn report(line: usize, where_: &str, message: &str) {
    println!("[line: {}], Error{}: {}", line, where_, message);
}

pub fn error_line(line: usize, message: &str) {
    report(line, "", message)
}
