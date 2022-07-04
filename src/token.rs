use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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
    CodeBlockBegin,
    CodeBlockEnd,
    // math operators
    Plus,
    Minus,
    Divide,
    Multiply,
    Negative,
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
    MultiplyWith,
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
    Input,
    New,
    FunctionArgument,

    EOF,
    Program,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TokenType {self:?}",)
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: i32,
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
                write!(f, "String: [lexeme {:?}, value {:?}]", self.lexeme, literal)
            }
            TokenType::Number { literal } => {
                write!(f, "Number: [lexeme {:?}, value {:?}]", self.lexeme, literal)
            }
            _ => write!(f, "{:?}: [lexeme {:?}]", self.token_type, self.lexeme),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}", self, self.line)
    }
}
