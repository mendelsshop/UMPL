use std::collections::HashMap;

use crate::token::TokenType;
#[derive(Clone)]
pub struct Keyword {
    pub keywords: HashMap<String, TokenType>,
}

// TODO: make each keyword with whacky case semantics ie: evary 5th character has to be uppercase etc
impl Keyword {
    pub fn new() -> Keyword {
        let mut keywords = HashMap::new();
        // math
        keywords.insert("plus".to_string(), TokenType::Plus);
        keywords.insert("minus".to_string(), TokenType::Minus);
        keywords.insert("multiply".to_string(), TokenType::Multiply);
        keywords.insert("divide".to_string(), TokenType::Divide);
        // comparison
        keywords.insert("not".to_string(), TokenType::Not);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("eq".to_string(), TokenType::Equal);
        keywords.insert("ne".to_string(), TokenType::NotEqual);
        keywords.insert("gt".to_string(), TokenType::GreaterThan);
        keywords.insert("lt".to_string(), TokenType::LessThan);
        keywords.insert("le".to_string(), TokenType::LessEqual);
        keywords.insert("ge".to_string(), TokenType::GreaterEqual);
        // variable stuff
        keywords.insert("create".to_string(), TokenType::Create);
        keywords.insert("with".to_string(), TokenType::With);
        keywords.insert("addwith".to_string(), TokenType::AddWith);
        keywords.insert("dividewith".to_string(), TokenType::DivideWith);
        keywords.insert("subtractwith".to_string(), TokenType::SubtractWith);
        keywords.insert("multiplywith".to_string(), TokenType::MultiplyWith);
        keywords.insert("list".to_string(), TokenType::List);
        keywords.insert("first".to_string(), TokenType::First);
        keywords.insert("second".to_string(), TokenType::Second);
        // misc keywords
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("break".to_string(), TokenType::Break);
        keywords.insert("continue".to_string(), TokenType::Continue);
        keywords.insert("loop".to_string(), TokenType::Loop);
        keywords.insert("potato".to_string(), TokenType::Potato);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("null".to_string(), TokenType::Null);
        keywords.insert("input".to_string(), TokenType::Input);
        keywords.insert("negative".to_string(), TokenType::Negative);
        keywords.insert("new".to_string(), TokenType::New);
        keywords.insert("input".to_string(), TokenType::Input);
        Keyword { keywords }
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
