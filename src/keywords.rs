use std::collections::HashMap;

use crate::token::{BuiltinFunction, TokenType};
#[derive(PartialEq, Clone)]
pub struct Keyword<'a> {
    pub keywords: HashMap<String, TokenType<'a>>,
    pub builtin_functions: HashMap<String, BuiltinFunction>,
}

// TODO: make each keyword with whacky case semantics ie: evary 5th character has to be uppercase etc
impl Keyword<'_> {
    pub fn new() -> Self {
        let mut keywords: HashMap<String, TokenType<'_>> = HashMap::new();
        let mut builtin_functions: HashMap<String, BuiltinFunction> = HashMap::new();
        builtin_functions.insert(toggle_case("plus"), BuiltinFunction::Plus);
        builtin_functions.insert(toggle_case("minus"), BuiltinFunction::Minus);
        builtin_functions.insert(toggle_case("multiply"), BuiltinFunction::Multiply);
        builtin_functions.insert(toggle_case("divide"), BuiltinFunction::Divide);
        builtin_functions.insert(toggle_case("not"), BuiltinFunction::Not);
        builtin_functions.insert(toggle_case("or"), BuiltinFunction::Or);
        builtin_functions.insert(toggle_case("and"), BuiltinFunction::And);
        builtin_functions.insert(toggle_case("eq"), BuiltinFunction::Equal);
        builtin_functions.insert(toggle_case("ne"), BuiltinFunction::NotEqual);
        builtin_functions.insert(toggle_case("gt"), BuiltinFunction::GreaterThan);
        builtin_functions.insert(toggle_case("lt"), BuiltinFunction::LessThan);
        builtin_functions.insert(toggle_case("le"), BuiltinFunction::LessEqual);
        builtin_functions.insert(toggle_case("ge"), BuiltinFunction::GreaterEqual);
        keywords.insert(toggle_case("create"), TokenType::Create);
        builtin_functions.insert(toggle_case("addwith"), BuiltinFunction::AddWith);
        builtin_functions.insert(toggle_case("dividewith"), BuiltinFunction::DivideWith);
        builtin_functions.insert(toggle_case("subtractwith"), BuiltinFunction::SubtractWith);
        builtin_functions.insert(toggle_case("multiplywith"), BuiltinFunction::MultiplyWith);
        keywords.insert(toggle_case("list"), TokenType::List);
        keywords.insert(toggle_case("car"), TokenType::Car);
        keywords.insert(toggle_case("cdr"), TokenType::Cdr);
        keywords.insert(toggle_case("return"), TokenType::Return);
        keywords.insert(toggle_case("break"), TokenType::Break);
        keywords.insert(toggle_case("continue"), TokenType::Continue);
        keywords.insert(toggle_case("loop"), TokenType::Loop);
        keywords.insert(toggle_case("potato"), TokenType::Potato);
        keywords.insert(toggle_case("if"), TokenType::If);
        keywords.insert(toggle_case("else"), TokenType::Else);
        keywords.insert(toggle_case("module"), TokenType::Module);
        builtin_functions.insert(toggle_case("input"), BuiltinFunction::Input);
        builtin_functions.insert(toggle_case("new"), BuiltinFunction::New);
        builtin_functions.insert(toggle_case("setwith"), BuiltinFunction::Set);
        builtin_functions.insert(toggle_case("exit"), BuiltinFunction::Exit);
        builtin_functions.insert(toggle_case("error"), BuiltinFunction::Error);
        keywords.insert(toggle_case("with"), TokenType::With);
        builtin_functions.insert(toggle_case("strtonum"), BuiltinFunction::StrToNum);
        builtin_functions.insert(toggle_case("strtobool"), BuiltinFunction::StrToBool);
        builtin_functions.insert(toggle_case("strtohempty"), BuiltinFunction::StrToHempty);
        builtin_functions.insert(toggle_case("runcommand"), BuiltinFunction::RunCommand);
        builtin_functions.insert(toggle_case("open"), BuiltinFunction::Open);
        builtin_functions.insert(toggle_case("close"), BuiltinFunction::Close);
        builtin_functions.insert(toggle_case("write"), BuiltinFunction::Write);
        builtin_functions.insert(toggle_case("read"), BuiltinFunction::Read);
        builtin_functions.insert(toggle_case("readline"), BuiltinFunction::ReadLine);
        builtin_functions.insert(toggle_case("delete"), BuiltinFunction::Delete);
        builtin_functions.insert(toggle_case("spliton"), BuiltinFunction::SplitOn);
        builtin_functions.insert(toggle_case("writeline"), BuiltinFunction::WriteLine);
        builtin_functions.insert(toggle_case("createfile"), BuiltinFunction::CreateFile);
        builtin_functions.insert(toggle_case("deletefile"), BuiltinFunction::DeleteFile);
        builtin_functions.insert(toggle_case("type"), BuiltinFunction::Type);
        Self {
            keywords,
            builtin_functions,
        }
    }

    pub fn get(&self, name: &str) -> Option<TokenType<'_>> {
        self.keywords.get(name).copied()
    }

    pub fn is_keyword(&self, token_type: &TokenType<'_>) -> bool {
        self.keywords.values().any(|val| val == token_type)
    }

    pub const fn is_builtin_function(&self, token_type: &TokenType<'_>) -> bool {
        matches!(token_type, TokenType::BuiltinFunction(_))
    }

    pub fn string_is_keyword(&self, string: &str) -> Option<TokenType<'_>> {
        self.keywords.get(string).copied()
    }

    pub fn string_is_builtin_function(&self, string: &str) -> Option<BuiltinFunction> {
        self.builtin_functions.get(string).copied()
    }
}
impl Default for Keyword<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::cast_possible_wrap)]
fn toggle_case(string: &str) -> String {
    let num = unsafe { crate::cli::TOGGLE_CASE };
    if num == 0 {
        string.to_string()
    } else {
        let num: usize = match string.len() as i32 - num {
            nums if nums <= 0 => num as usize % 50,
            nums => nums as usize % 50,
        };
        let mut new_string = String::new();
        for (count, c) in string.chars().enumerate() {
            if count % num == 0 {
                new_string.push(c.to_uppercase().next().unwrap());
            } else {
                new_string.push(c);
            }
        }
        new_string
    }
}
