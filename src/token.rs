use crate::{
    error::{self, arg_error},
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
#[allow(clippy::module_name_repetitions)]
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
    Car,
    Cdr,
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
    SplitOn,
    WriteLine,
    CreateFile,
    DeleteFile,
    Type
}

impl TokenType {
    #[allow(clippy::too_many_lines)]
    pub fn r#do(&self, args: &[LiteralType], line: i32) -> LiteralType {
        if crate::KEYWORDS.is_keyword(self) {
            match self {
                Self::Not => {
                    if args.len() != 1 {
                        error::error(line, "Expected 1 argument for not operator");
                    }
                    match &args[0] {
                        LiteralType::Boolean(b) => LiteralType::Boolean(!b),
                        _ => error::error(line, "Expected boolean for not operator"),
                    }
                }
                Self::Plus | Self::Minus | Self::Divide | Self::Multiply => {
                    match &args[0] {
                        LiteralType::Number(number) => {
                            // check if minus and only one argument
                            let mut total: f64 = if self == &Self::Minus && args.len() == 1 {
                                -number
                            } else {
                                *number
                            };
                            for thing in args.iter().skip(1) {
                                if let LiteralType::Number(number) = thing {
                                    {
                                        // convert the self to an operator
                                        match self {
                                            Self::Plus => {
                                                total += number;
                                            }
                                            Self::Minus => {
                                                total -= number;
                                            }
                                            Self::Divide => {
                                                total /= number;
                                            }
                                            Self::Multiply => {
                                                total *= number;
                                            }
                                            _ => {}
                                        };
                                    }
                                }
                            }
                            LiteralType::Number(total)
                        }
                        LiteralType::String(string) => {
                            let mut new_string = string.to_string();
                            for (index, thing) in args.iter().skip(1).enumerate() {
                                match self {
                                    Self::Plus => {
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
                                    Self::Multiply => {
                                        if index > 0 {
                                            error::error(
                                                line,
                                                "Multiply can only be used with the car argument",
                                            );
                                        }
                                        match thing {
                                            LiteralType::Number(number) => {
                                                let mut new_new_string = String::new();
                                                for _ in 0..*number as i32 - 1 {
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
                                    Self::Divide | Self::Minus => {
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
                        _ => error::error(line, "Invalid literal arguments"),
                    }
                }
                Self::Error
                | Self::Input
                | Self::StrToBool
                | Self::StrToHempty
                | Self::StrToNum
                | Self::RunCommand => {
                    arg_error(
                        1,
                        args.len() as u32,
                        self,
                        false,
                        line
                    );
                    match &args[0] {
                        LiteralType::String(ref string) => match self {
                            Self::Error => exit(1),
                            Self::Input => {
                                let mut input = String::new();
                                print!("{}", string);
                                // flush stdout
                                io::stdout().flush().unwrap_or_else(|_| {
                                    error::error(line, "Error flushing stdout");
                                });
                                io::stdin().read_line(&mut input).unwrap_or_else(|_| {
                                    error::error(line, "Failed to read input");
                                });
                                LiteralType::String(input.trim().to_string())
                            }
                            Self::StrToBool => {
                                if string == "true" {
                                    LiteralType::Boolean(true)
                                } else if string == "false" {
                                    LiteralType::Boolean(false)
                                } else {
                                    error::error(line, "Expected true or false");
                                }
                            }
                            Self::StrToHempty => {
                                if string == "HEMPTY" {
                                    LiteralType::Hempty
                                } else {
                                    error::error(line, "Expected HEMPTY");
                                }
                            }
                            Self::StrToNum => {
                                let string = match string {
                                    strings if string.starts_with("0x") => {
                                        strings.clone().trim().to_owned()
                                    }
                                    strings => format!("0x{}", strings.trim()),
                                };
                                let number: FloatLiteral = match string.parse() {
                                    Ok(value) => value,
                                    Err(_) => error::error(
                                        line,
                                        format!("Error parsing string {} to number", string.trim()),
                                    ),
                                };
                                LiteralType::Number(number.convert::<f64>().inner())
                            }
                            Self::RunCommand => {
                                let cmd = if OS == "windows" {
                                    let mut cmd = Command::new("powershell");
                                    cmd.args(&["-c", string.trim()]).output()
                                } else {
                                    let mut cmd = Command::new("sh");
                                    cmd.args(&["-c", string.trim()]).output()
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
                            _ => {
                                error::error(line, "command not found");
                            }
                        },
                        _ => error::error(line, "Expected string for input operator"),
                    }
                }
                Self::NotEqual | Self::Equal => {
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
                    if self == &Self::Equal {
                        LiteralType::Boolean(type_ == type_1)
                    } else {
                        LiteralType::Boolean(!(type_ == type_1))
                    }
                }
                Self::Or | Self::And => {
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
                Self::GreaterThan | Self::LessThan | Self::GreaterEqual | Self::LessEqual => {
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
                    if self == &Self::GreaterThan {
                        LiteralType::Boolean(type_ > type_1)
                    } else if self == &Self::LessThan {
                        LiteralType::Boolean(type_ < type_1)
                    } else if self == &Self::GreaterEqual {
                        LiteralType::Boolean(type_ >= type_1)
                    } else {
                        LiteralType::Boolean(type_ <= type_1)
                    }
                }
                Self::Exit => {
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
                Self::SplitOn => {
                    if args.len() < 2 {
                        error::error(
                            line,
                            format!("Expected al least 2 arguments for {:?} operator", self),
                        );
                    }
                    let og_string = match &args[0] {
                        LiteralType::String(string) => string,
                        _ => error::error(line, format!("Expected string for {:?} operator", self)),
                    };
                    let split_on = match &args[1] {
                        LiteralType::String(string) => string,
                        _ => error::error(line, format!("Expected string for {:?} operator", self)),
                    };
                    // check if there is a third argument (number)
                    args.get(2).map_or_else(
                        || match og_string.split_once(split_on) {
                            Some(v) => LiteralType::String(v.0.to_string()),
                            None => LiteralType::String(og_string.to_string()),
                        },
                        |number| -> LiteralType {
                            if let LiteralType::Number(number) = number {
                                let number = *number as usize;
                                // return the string until the nth time split_on is found
                                let string: Vec<&str> =
                                    og_string.split_inclusive(split_on).collect::<Vec<&str>>();
                                if number > string.len() {
                                    error::error(
                                        line,
                                        format!("{} is greater than the number of splits", number),
                                    );
                                }
                                // loop through the splits and add them to the string if they are less than the number
                                let mut ret_string = String::new();
                                string.iter().take(number).for_each(|i: &&str| {
                                    ret_string.push_str(i);
                                });
                                let ret_string = match ret_string.rsplit_once(split_on) {
                                    Some(string) => string.0.to_string(),
                                    None => ret_string,
                                };
                                LiteralType::String(ret_string)
                            } else {
                                error::error(
                                    line,
                                    format!("Expected number for {:?} operator", self),
                                )
                            }
                        },
                    )
                }
                keyword if crate::KEYWORDS.is_keyword(keyword) => {
                    error::error(line, format!("Keyword not found {}", self));
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
    pub fn new(token_type: TokenType, lexeme: &str, line: i32) -> Self {
        Self {
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
