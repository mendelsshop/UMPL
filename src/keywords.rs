use std::collections::HashMap;

use crate::token::TokenType;
#[derive(PartialEq, Clone)]
pub struct Keyword {
    pub keywords: HashMap<String, TokenType>,
}

// TODO: make each keyword with whacky case semantics ie: evary 5th character has to be uppercase etc
impl Keyword {
    pub fn new() -> Self {
        let num = unsafe {
            crate::cli::TOGGLE_CASE
        };
        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        keywords.insert("plus".to_string(), TokenType::Plus);
        keywords.insert("minus".to_string(), TokenType::Minus);
        keywords.insert("multiply".to_string(), TokenType::Multiply);
        keywords.insert("divide".to_string(), TokenType::Divide);
        keywords.insert("not".to_string(), TokenType::Not);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("eq".to_string(), TokenType::Equal);
        keywords.insert("ne".to_string(), TokenType::NotEqual);
        keywords.insert("gt".to_string(), TokenType::GreaterThan);
        keywords.insert("lt".to_string(), TokenType::LessThan);
        keywords.insert("le".to_string(), TokenType::LessEqual);
        keywords.insert("ge".to_string(), TokenType::GreaterEqual);
        keywords.insert("create".to_string(), TokenType::Create);
        keywords.insert("addwith".to_string(), TokenType::AddWith);
        keywords.insert("dividewith".to_string(), TokenType::DivideWith);
        keywords.insert("subtractwith".to_string(), TokenType::SubtractWith);
        keywords.insert("multiplywith".to_string(), TokenType::MultiplyWith);
        keywords.insert("list".to_string(), TokenType::List);
        keywords.insert("car".to_string(), TokenType::Car);
        keywords.insert("cdr".to_string(), TokenType::Cdr);
        keywords.insert("return".to_string(), TokenType::Return { value: None });
        keywords.insert("break".to_string(), TokenType::Break);
        keywords.insert("continue".to_string(), TokenType::Continue);
        keywords.insert("loop".to_string(), TokenType::Loop);
        keywords.insert("potato".to_string(), TokenType::Potato);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("input".to_string(), TokenType::Input);
        keywords.insert("new".to_string(), TokenType::New);
        keywords.insert("input".to_string(), TokenType::Input);
        keywords.insert("setwith".to_string(), TokenType::Set);
        keywords.insert("exit".to_string(), TokenType::Exit);
        keywords.insert("error".to_string(), TokenType::Error);
        keywords.insert("with".to_string(), TokenType::With);
        keywords.insert("strtonum".to_string(), TokenType::StrToNum);
        keywords.insert("strtobool".to_string(), TokenType::StrToBool);
        keywords.insert("strtohempty".to_string(), TokenType::StrToHempty);
        keywords.insert("runcommand".to_string(), TokenType::RunCommand);
        keywords.insert("open".to_string(), TokenType::Open);
        keywords.insert("close".to_string(), TokenType::Close);
        keywords.insert("write".to_string(), TokenType::Write);
        keywords.insert("read".to_string(), TokenType::Read);
        keywords.insert("readline".to_string(), TokenType::ReadLine);
        keywords.insert("delete".to_string(), TokenType::Delete);
        keywords.insert("spliton".to_string(), TokenType::SplitOn);
        keywords.insert("writeline".to_string(), TokenType::WriteLine);
        keywords.insert("createfile".to_string(), TokenType::CreateFile);
        keywords.insert("deletefile".to_string(), TokenType::DeleteFile);
        if num != 0 {
        for (key, value) in keywords.clone().iter() {
            keywords.remove(key);
            keywords.insert(toggle_case(key.to_string(), num), value.clone());
        }}
        Self { keywords }
    }

    pub fn get(&self, name: &str) -> Option<TokenType> {
        self.keywords.get(name).cloned()
    }

    pub fn is_keyword(&self, token_type: &TokenType) -> bool {
        self.keywords.values().any(|val| val == token_type)
    }
}
impl Default for Keyword {
    fn default() -> Self {
        Self::new()
    }
}

fn toggle_case(string: String, num: i32) -> String {
    // uppercase every x characters based on num instead of uppercaseing every character
    string.to_uppercase()
}