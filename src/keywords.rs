use std::collections::HashMap;

use crate::token::{BuiltinFunction, TokenType};
#[derive(PartialEq, Clone)]
pub struct Keyword {
    pub keywords: HashMap<String, TokenType>,
    pub builtin_functions: HashMap<String, BuiltinFunction>,
}

// TODO: make each keyword with whacky case semantics ie: evary 5th character has to be uppercase etc
impl Keyword {
    pub fn new() -> Self {
        let num = unsafe { crate::cli::TOGGLE_CASE };
        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        let mut builtin_functions: HashMap<String, BuiltinFunction> = HashMap::new();
        builtin_functions.insert("plus".to_string(), BuiltinFunction::Plus);
        builtin_functions.insert("minus".to_string(), BuiltinFunction::Minus);
        builtin_functions.insert("multiply".to_string(), BuiltinFunction::Multiply);
        builtin_functions.insert("divide".to_string(), BuiltinFunction::Divide);
        builtin_functions.insert("not".to_string(), BuiltinFunction::Not);
        builtin_functions.insert("or".to_string(), BuiltinFunction::Or);
        builtin_functions.insert("and".to_string(), BuiltinFunction::And);
        builtin_functions.insert("eq".to_string(), BuiltinFunction::Equal);
        builtin_functions.insert("ne".to_string(), BuiltinFunction::NotEqual);
        builtin_functions.insert("gt".to_string(), BuiltinFunction::GreaterThan);
        builtin_functions.insert("lt".to_string(), BuiltinFunction::LessThan);
        builtin_functions.insert("le".to_string(), BuiltinFunction::LessEqual);
        builtin_functions.insert("ge".to_string(), BuiltinFunction::GreaterEqual);
        keywords.insert("create".to_string(), TokenType::Create);
        builtin_functions.insert("addwith".to_string(), BuiltinFunction::AddWith);
        builtin_functions.insert("dividewith".to_string(), BuiltinFunction::DivideWith);
        builtin_functions.insert("subtractwith".to_string(), BuiltinFunction::SubtractWith);
        builtin_functions.insert("multiplywith".to_string(), BuiltinFunction::MultiplyWith);
        keywords.insert("list".to_string(), TokenType::List);
        keywords.insert("car".to_string(), TokenType::Car);
        keywords.insert("cdr".to_string(), TokenType::Cdr);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("break".to_string(), TokenType::Break);
        keywords.insert("continue".to_string(), TokenType::Continue);
        keywords.insert("loop".to_string(), TokenType::Loop);
        keywords.insert("potato".to_string(), TokenType::Potato);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("else".to_string(), TokenType::Else);
        builtin_functions.insert("input".to_string(), BuiltinFunction::Input);
        builtin_functions.insert("new".to_string(), BuiltinFunction::New);
        builtin_functions.insert("setwith".to_string(), BuiltinFunction::Set);
        builtin_functions.insert("exit".to_string(), BuiltinFunction::Exit);
        builtin_functions.insert("error".to_string(), BuiltinFunction::Error);
        keywords.insert("with".to_string(), TokenType::With);
        builtin_functions.insert("strtonum".to_string(), BuiltinFunction::StrToNum);
        builtin_functions.insert("strtobool".to_string(), BuiltinFunction::StrToBool);
        builtin_functions.insert("strtohempty".to_string(), BuiltinFunction::StrToHempty);
        builtin_functions.insert("runcommand".to_string(), BuiltinFunction::RunCommand);
        builtin_functions.insert("open".to_string(), BuiltinFunction::Open);
        builtin_functions.insert("close".to_string(), BuiltinFunction::Close);
        builtin_functions.insert("write".to_string(), BuiltinFunction::Write);
        builtin_functions.insert("read".to_string(), BuiltinFunction::Read);
        builtin_functions.insert("readline".to_string(), BuiltinFunction::ReadLine);
        builtin_functions.insert("delete".to_string(), BuiltinFunction::Delete);
        builtin_functions.insert("spliton".to_string(), BuiltinFunction::SplitOn);
        builtin_functions.insert("writeline".to_string(), BuiltinFunction::WriteLine);
        builtin_functions.insert("createfile".to_string(), BuiltinFunction::CreateFile);
        builtin_functions.insert("deletefile".to_string(), BuiltinFunction::DeleteFile);
        builtin_functions.insert("type".to_string(), BuiltinFunction::Type);
        builtin_functions.insert("module".to_string(), BuiltinFunction::Module);
        if num != 0 {
            for (key, value) in &keywords.clone() {
                keywords.remove(key);
                keywords.insert(toggle_case(key, num), value.clone());
            }
        }
        Self {
            keywords,
            builtin_functions,
        }
    }

    pub fn get(&self, name: &str) -> Option<TokenType> {
        self.keywords.get(name).cloned()
    }

    pub fn is_keyword(&self, token_type: &TokenType) -> bool {
        self.keywords.values().any(|val| val == token_type)
    }

    pub fn is_builtin_function(&self, token_type: &TokenType) -> bool {
        self.builtin_functions.values().any(|val| {
            if let TokenType::BuiltinFunction(builtin) = token_type {
                val == builtin
            } else {
                false
            }
        })
    }

    pub fn string_is_keyword(&self, string: &str) -> Option<TokenType> {
        self.keywords.get(string).cloned()
    }

    pub fn string_is_builtin_function(&self, string: &str) -> Option<BuiltinFunction> {
        self.builtin_functions.get(string).cloned()
    }
}
impl Default for Keyword {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::cast_possible_wrap)]
fn toggle_case(string: &str, num: i32) -> String {
    println!("{}", string.len() + num as usize % 10);
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
