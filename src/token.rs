use crate::{
    error,
    parser::rules::{LiteralType, OtherStuff, Stuff},
};
use std::{
    fmt::{self, Debug, Display},
    io::{self, Write},
};
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
                TokenType::Plus | TokenType::Minus | TokenType::Divide | TokenType::Multiply => {
                    match &args[0] {
                        Stuff::Literal(value) => match value.literal {
                            LiteralType::Number(number) => {
                                // check if minus and only one argument
                                let mut total;
                                if self == &TokenType::Minus && args.len() == 1 {
                                    total = -number
                                } else {
                                    total = number
                                }
                                for thing in args.iter().skip(1) {
                                    match thing {
                                        Stuff::Literal(literal) => {
                                            if let LiteralType::Number(number) = literal.literal {
                                                {
                                                    // convert the self to an operator
                                                    match self {
                                                        TokenType::Plus => {
                                                            total += number;
                                                        }
                                                        TokenType::Minus => {
                                                            total -= number;
                                                        }
                                                        TokenType::Divide => {
                                                            total /= number;
                                                        }
                                                        TokenType::Multiply => {
                                                            total *= number;
                                                        }
                                                        _ => {}
                                                    };
                                                }
                                            }
                                        }
                                        _ => {
                                            error::error(
                                                line,
                                                format!(
                                                    "Only numbers can be added found {}",
                                                    thing
                                                ),
                                            );
                                        }
                                    }
                                }
                                LiteralType::Number(total)
                            }
                            LiteralType::String(ref string) => {
                                let mut new_string = string.clone();
                                for (index, thing) in args.iter().skip(1).enumerate() {
                                    match self {
                                        TokenType::Plus => {
                                            if let Stuff::Literal(literal) = thing {
                                                match literal.literal {
                                                    LiteralType::String(ref string) => {
                                                        new_string.push_str(string);
                                                    }
                                                    LiteralType::Number(number) => {
                                                        new_string.push_str(&number.to_string());
                                                    }
                                                    LiteralType::Boolean(boolean) => {
                                                        new_string.push_str(&boolean.to_string());
                                                    }
                                                    LiteralType::Hempty => {
                                                        new_string.push_str("HEMPTY");
                                                    }
                                                };
                                            }
                                        }
                                        TokenType::Multiply => {
                                            if index > 0 {
                                                error::error(line, "Multiply can only be used with the first argument");
                                            }
                                            if let Stuff::Literal(literal) = thing {
                                                match literal.literal {
                                                    LiteralType::Number(number) => {
                                                        let mut new_new_string = String::new();
                                                        for _ in 0..number as i32 {
                                                            new_new_string.push_str(&new_string);
                                                        }
                                                        new_string = new_new_string;
                                                    }
                                                    _ => {
                                                        error::error(
                                                                line,
                                                                "strings can only be multiplied by numbers",
                                                            );
                                                    }
                                                }
                                            }
                                        }
                                        TokenType::Divide | TokenType::Minus => {
                                            error::error(
                                                line,
                                                "Only numbers can be divided or subtracted found",
                                            );
                                        }
                                        _ => {}
                                    };
                                }
                                LiteralType::String(new_string)
                            }
                            _ => error::error(0, "Invalid literal arguments"),
                        },
                        _ => {
                            error::error(line, "Invalid type for operation");
                        }
                    }
                }
                TokenType::Error | TokenType::Input => {
                    if args.len() != 1 {
                        error::error(line, format!("Expected 1 arguments for {} operator", self));
                    }
                    match &args[0] {
                        Stuff::Literal(literal) => match literal.literal {
                            LiteralType::String(ref string) => {
                                if self == &TokenType::Error {
                                    println!("{}", string);
                                    std::process::exit(1);
                                } else {
                                    print!("{}", string);
                                    io::stdout().flush().unwrap();
                                    let mut input = String::new();
                                    io::stdin().read_line(&mut input).unwrap();
                                    LiteralType::String(input)
                                }
                            }
                            _ => error::error(line, "Expected string for input operator"),
                        },
                        _ => error::error(line, "Expected string for input operator"),
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
