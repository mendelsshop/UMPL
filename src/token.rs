use crate::{
    error,
    parser::rules::{LiteralType, OtherStuff},
};
use hexponent::FloatLiteral;
use std::{
    env::consts::OS,
    fmt::{self, Debug, Display},
    io::{self, Write},
    process::{exit, Command},
};
#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
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
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    GreaterEqual,
    LessEqual,
    GreaterThan,
    LessThan,
    And,
    Or,
    Not,
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
    StrToNum,
    StrToBool,
    StrToHempty,
    Eval,
    RunCommand,
    Open,
    Close,
    Write,
    Read,
    ReadLine,
    Exit,
    Error,
    EOF,
    Program,
    Delete,
}

impl TokenType {
    pub fn r#do(&self, args: Vec<LiteralType>, line: i32) -> LiteralType {
        if crate::KEYWORDS.is_keyword(self) {
            match self {
                TokenType::Not => {
                    if args.len() != 1 {
                        error::error(line, "Expected 1 argument for not operator");
                    }
                    match &args[0] {
                        LiteralType::Boolean(b) => LiteralType::Boolean(!b),
                        _ => error::error(line, "Expected boolean for not operator"),
                    }
                }
                TokenType::Plus | TokenType::Minus | TokenType::Divide | TokenType::Multiply => {
                    match args[0] {
                        LiteralType::Number(number) => {
                            // check if minus and only one argument
                            let mut total;
                            if self == &TokenType::Minus && args.len() == 1 {
                                total = -number
                            } else {
                                total = number
                            }
                            for thing in args.iter().skip(1) {
                                if let LiteralType::Number(number) = thing {
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
                            LiteralType::Number(total)
                        }
                        LiteralType::String(ref string) => {
                            let mut new_string = string.clone();
                            for (index, thing) in args.iter().skip(1).enumerate() {
                                match self {
                                    TokenType::Plus => {
                                        match thing {
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
                                    TokenType::Multiply => {
                                        if index > 0 {
                                            error::error(
                                                line,
                                                "Multiply can only be used with the first argument",
                                            );
                                        }

                                        match thing {
                                            LiteralType::Number(number) => {
                                                let mut new_new_string = String::new();
                                                for _ in 0..*number as i32 - 1 {
                                                    println!("{}", new_string);
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
                    }
                }
                TokenType::Error
                | TokenType::Input
                | TokenType::StrToBool
                | TokenType::StrToHempty
                | TokenType::StrToNum
                | TokenType::RunCommand
                | TokenType::Eval
                | TokenType::Open
                | TokenType::Write => {
                    if args.len() != 1 {
                        error::error(
                            line,
                            format!("Expected 1 arguments for {:?} operator", self),
                        );
                    }
                    match &args[0] {
                        LiteralType::String(ref string) => match self {
                            TokenType::Error => {
                                println!("{}", string);
                                exit(1)
                            }
                            TokenType::Input => {
                                let mut input = String::new();
                                print!("{}", string);
                                // flush stdout
                                io::stdout().flush().unwrap();
                                io::stdin().read_line(&mut input).unwrap();
                                LiteralType::String(input.trim().to_string())
                            }
                            TokenType::StrToBool => {
                                if string == "true" {
                                    LiteralType::Boolean(true)
                                } else if string == "false" {
                                    LiteralType::Boolean(false)
                                } else {
                                    error::error(line, "Expected true or false");
                                }
                            }
                            TokenType::StrToHempty => {
                                if string == "HEMPTY" {
                                    LiteralType::Hempty
                                } else {
                                    error::error(line, "Expected HEMPTY");
                                }
                            }
                            TokenType::StrToNum => {
                                // TODO: check if 0x is already in the string
                                let number: FloatLiteral = match format!("0x{}", string.trim())
                                    .parse()
                                {
                                    Ok(value) => value,
                                    Err(_) => error::error(
                                        line,
                                        format!("Error parsing string {} to number", string.trim()),
                                    ),
                                };

                                LiteralType::Number(number.convert::<f64>().inner())
                            }
                            TokenType::RunCommand => {
                                let cmd = match OS {
                                    "windows" => {
                                        let mut cmd = Command::new("powershell");
                                        cmd.args(&["-c", string.trim()]).output()
                                    }
                                    _ => {
                                        let mut cmd = Command::new("sh");
                                        cmd.args(&["-c", string.trim()]).output()
                                    }
                                };
                                let cmd: String = match cmd {
                                    Ok(value) => {
                                        if value.status.success() {
                                            String::from_utf8_lossy(&value.stdout).into()
                                        } else {
                                            String::from_utf8_lossy(&value.stderr).into()
                                        }
                                    }
                                    Err(_) => error::error(
                                        line,
                                        format!("Error running command {}", string.trim()),
                                    ),
                                };
                                LiteralType::String(cmd)
                            }

                            _ => todo!(),
                        },
                        _ => error::error(line, "Expected string for input operator"),
                    }
                }
                TokenType::NotEqual | TokenType::Equal => {
                    if args.len() != 2 {
                        error::error(
                            line,
                            format!("Expected 2 arguments for {:?} operator", self),
                        );
                    }
                    let type_ = &args[0];

                    let type_1 = &args[1];
                    if type_.type_eq(type_1) {
                    } else {
                        error::error(
                            line,
                            format!(
                                "{} and {} are not the same type which is required for {} operator",
                                type_, type_1, self
                            ),
                        );
                    }

                    if self == &TokenType::Equal {
                        LiteralType::Boolean(type_ == type_1)
                    } else {
                        LiteralType::Boolean(!(type_ == type_1))
                    }
                }
                TokenType::Or | TokenType::And => {
                    if args.len() != 2 {
                        error::error(
                            line,
                            format!("Expected 2 arguments for {:?} operator", self),
                        );
                    }
                    let bool_1 = match &args[0] {
                        LiteralType::Boolean(boolean) => boolean,
                        _ => {
                            error::error(line, format!("Expected boolean for {:?} operator", self))
                        }
                    };
                    let bool_2 = match &args[1] {
                        LiteralType::Boolean(boolean) => boolean,
                        _ => {
                            error::error(line, format!("Expected boolean for {:?} operator", self))
                        }
                    };
                    if bool_1 == bool_2 {
                        if bool_1 == &true {
                            LiteralType::Boolean(true)
                        } else {
                            LiteralType::Boolean(false)
                        }
                    } else {
                        LiteralType::Boolean(false)
                    }
                }
                TokenType::GreaterThan
                | TokenType::LessThan
                | TokenType::GreaterEqual
                | TokenType::LessEqual => {
                    if args.len() != 2 {
                        error::error(
                            line,
                            format!("Expected 2 arguments for {:?} operator", self),
                        );
                    }
                    let type_ = match &args[0] {
                        LiteralType::Number(number) => number,
                        _ => error::error(line, format!("Expected number for {:?} operator", self)),
                    };
                    let type_1 = match &args[1] {
                        LiteralType::Number(number) => number,
                        _ => error::error(line, format!("Expected number for {:?} operator", self)),
                    };
                    if self == &TokenType::GreaterThan {
                        LiteralType::Boolean(type_ > type_1)
                    } else if self == &TokenType::LessThan {
                        LiteralType::Boolean(type_ < type_1)
                    } else if self == &TokenType::GreaterEqual {
                        LiteralType::Boolean(type_ >= type_1)
                    } else {
                        LiteralType::Boolean(type_ <= type_1)
                    }
                }
                TokenType::Exit => {
                    if args.len() == 1 {
                        match &args[0] {
                            LiteralType::Number(number) => exit(*number as i32),
                            _ => error::error(
                                line,
                                format!("Expected number for {:?} operator", self),
                            ),
                        }
                    } else {
                        error::error(line, format!("Expected 1 argument for {:?} operator", self));
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
