use crate::{
    error,
    parser::rules::{LiteralType, OtherStuff, Stuff},
};
use std::fmt::{self, Debug, Display};
#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    // single character tokens
    RightParen,
    LeftParen,
    GreaterThanSymbol,
    LessThanSymbol,
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
    Identifier { name: String },
    FunctionIdentifier { name: char },
    String { literal: String },
    Number { literal: f64 },
    Create,
    With,
    Set,
    AddWith,
    SubtractWith,
    DivideWith,
    MultiplyWith,
    List,
    First,
    Second,
    // other keywords
    Return { value: Option<Box<OtherStuff>> },
    Colon,
    Break,
    Continue,
    Loop,
    Potato,
    If,
    Else,
    Hempty,
    Boolean { value: bool },
    Input,
    New,
    Function,
    FunctionArgument { name: String },

    Exit,
    Error,
    EOF,
    Program,
}

impl TokenType {
    pub fn r#do(&self, args: Vec<Stuff>, line: i32) -> LiteralType {
        if crate::KEYWORDS.is_keyword(self) {
            match self {
                TokenType::Not => {
                    if args.len() != 1 {
                        error::error(line, "Expected 1 argument for not operator");
                    }
                    match &args[0] {
                        Stuff::Literal(literal) => match literal.literal {
                            LiteralType::Boolean(b) => LiteralType::Boolean(!b),
                            _ => error::error(line, "Expected boolean for not operator"),
                        },
                        _ => error::error(line, "Expected a literal for not operator"),
                    }
                }
                keyword if crate::KEYWORDS.is_keyword(keyword) => {
                    todo!()
                }
                _ => {
                    error::error(line, format!("Keyword not found {}", self));
                }
            }
        } else {
            error::error(line, "Unknown keyword");
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TokenType {self:?}",)
    }
}

#[derive(PartialEq, Clone)]
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

impl Display for Token {
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
