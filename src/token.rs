use std::fmt;

#[derive(Debug, Clone)]
pub enum TokenType {
    // single character tokens
    RightParen,
    LeftParen,
    GreaterThanSymbol,
    LessThanSymbol,
    Colon,
    Dot,
    Bang,
    RightBracket,
    LeftBracket,
    RightBrace,
    LeftBrace,
    // math operators
    Plus,
    Minus,
    Divide,
    Multiply,
    // Comparison stuffs
    Equal,
    NotEqual,
    GreaterEqual,
    LessEqual,
    GreaterThan,
    LessThan,
    And,
    Or,
    Not,
    // variable stuff
    Identifier,
    String { literal: String },
    Number { literal: f64 },
    Create,
    With,
    AddWith,
    SubtractWith,
    DivideWith,
    List,
    First,
    Second,
    // other keywords
    Return,
    Break,
    Continue,
    Loop,
    Potato,
    If,
    Else,
    Null,
    True,
    False,

    EOF,
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: i32) -> Token {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.token_type {
            TokenType::String { literal } => {
                write!(f, "String {:?} {:?}", self.lexeme, literal)
            }
            TokenType::Number { literal } => {
                write!(f, "Number {:?} {:?}", self.lexeme, literal)
            }
            _ => write!(f, "Token {:?} {:?}", self.token_type, self.lexeme),
        }
    }
}
