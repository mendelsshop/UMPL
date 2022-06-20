use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Eq,
    Bang,
    LParen,
    RParen,
    Number,
    Identifier,
    Boolean,
    String,
}
impl TokenType {
    // get a token type from a string
    pub fn token_type_from_string(s: &str) -> TokenType {
        match s {
            "plus" => TokenType::Plus,
            "minus" => TokenType::Minus,
            "multiply" => TokenType::Multiply,
            "divide" => TokenType::Divide,
            "power" => TokenType::Power,
            "eq" => TokenType::Eq,
            "bang" => TokenType::Bang,
            "lparen" => TokenType::LParen,
            "rparen" => TokenType::RParen,
            "number" => TokenType::Number,
            "identifier" => TokenType::Identifier,
            "boolean" => TokenType::Boolean,
            "string" => TokenType::String,
            _ => panic!("Unknown token type: {}", s),
        }
    }
}

// for display a tokentype
impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenType::Plus => "plus",
                TokenType::Minus => "minus",
                TokenType::Multiply => "multiply",
                TokenType::Divide => "divide",
                TokenType::Power => "power",
                TokenType::Eq => "eq",
                TokenType::Bang => "bang",
                TokenType::LParen => "lparen",
                TokenType::RParen => "rparen",
                TokenType::Number => "number",
                TokenType::Identifier => "identifier",
                TokenType::Boolean => "boolean",
                TokenType::String => "string",
            }
        )
    }
}

// file info about a token
pub struct TokenInfo {
    pub file: String,
    pub line: u32,
}

impl Display for TokenInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:line {}", self.file, self.line)
    }
}

// a token
// consists of a token type optionaly a value and token info
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub info: TokenInfo,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, file: String, line: u32) -> Token {
        Token {
            token_type,
            value,
            info: TokenInfo { file, line },
        }
    }
}

// when display a token, just display the token type
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.token_type)
    }
}

// for debugging we want to display file info token type and optionally the value
impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "`{}`{} at {}",
            self.token_type,
            if !self.value.is_empty() {
                format!(" with value `{}`", self.value)
            } else {
                String::from("")
            },
            self.info
        )
    }
}
