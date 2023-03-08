use crate::parser::rules::Expr;
// use hexponent::FloatLiteral;
use std::fmt::{self, Debug, Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
pub struct Info<'a> {
    pub file_name: &'a str,
    pub line: u32,
    pub end_line: u32,
}

impl<'a> Info<'a> {
    pub const fn new(file_name: &'a str, line: u32, end_line: u32) -> Self {
        Self {
            file_name,
            line,
            end_line,
        }
    }
}

impl Display for Info<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}..{}", self.file_name, self.line, self.end_line)
    }
}

#[derive(PartialEq, Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub enum TokenType<'a> {
    RightParen,
    LeftParen,
    GreaterThanSymbol,
    LessThanSymbol,
    Dot,
    Bang,
    Star,
    RightBracket,
    LeftBracket,
    RightBrace,
    LeftBrace,
    CodeBlockBegin,
    CodeBlockEnd,
    Identifier { name: String },
    FunctionIdentifier { name: String },
    String { literal: String },
    Number { literal: f64 },
    Create,
    With,
    List,
    Car,
    Cdr,
    Return { value: Option<Box<Expr<'a>>> },
    Colon,
    Break,
    Continue,
    Loop,
    Potato,
    If,
    Else,
    Hempty,
    Boolean { literal: bool },
    Function,
    FunctionArgument { name: String },
    EOF,
    Program,
    BuiltinFunction(BuiltinFunction),
}

#[derive(PartialEq, Eq, Debug, Clone)]

pub enum BuiltinFunction {
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
    Delete,
    SplitOn,
    WriteLine,
    CreateFile,
    DeleteFile,
    Type,
    Module,
    Input,
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
    New,
    Set,
    AddWith,
    SubtractWith,
    DivideWith,
    MultiplyWith,
}

impl TokenType<'_> {
    // #[allow(clippy::too_many_lines)]
    // pub fn r#do(&self, args: &[LiteralType], line: u32, scope: &mut Eval) -> LiteralType {
    //     if crate::KEYWORDS.is_keyword(self) {
    //         match self {
    //             Self::Not => {
    //                 if args.len() != 1 {
    //                     error::error(line, "Expected 1 argument for not operator");
    //                 }
    //                 match &args[0] {
    //                     LiteralType::Boolean(b) => LiteralType::Boolean(!b),
    //                     _ => error::error(line, "Expected boolean for not operator"),
    //                 }
    //             }
    //             Self::Plus | Self::Minus | Self::Divide | Self::Multiply => {
    //                 match &args[0] {
    //                     LiteralType::Number(number) => {
    //                         // check if minus and only one argument
    //                         let mut total: f64 = if self == &Self::Minus && args.len() == 1 {
    //                             -number
    //                         } else {
    //                             *number
    //                         };
    //                         for thing in args.iter().skip(1) {
    //                             if let LiteralType::Number(number) = thing {
    //                                 {
    //                                     // convert the self to an operator
    //                                     match self {
    //                                         Self::Plus => {
    //                                             total += number;
    //                                         }
    //                                         Self::Minus => {
    //                                             total -= number;
    //                                         }
    //                                         Self::Divide => {
    //                                             total /= number;
    //                                         }
    //                                         Self::Multiply => {
    //                                             total *= number;
    //                                         }
    //                                         _ => {}
    //                                     };
    //                                 }
    //                             }
    //                         }
    //                         LiteralType::Number(total)
    //                     }
    //                     LiteralType::String(string) => {
    //                         let mut new_string = string.to_string();
    //                         for (index, thing) in args.iter().skip(1).enumerate() {
    //                             match self {
    //                                 Self::Plus => {
    //                                     match thing {
    //                                         LiteralType::String(ref string) => {
    //                                             new_string.push_str(string);
    //                                         }
    //                                         LiteralType::Number(number) => {
    //                                             new_string.push_str(&number.to_string());
    //                                         }
    //                                         LiteralType::Boolean(boolean) => {
    //                                             new_string.push_str(&boolean.to_string());
    //                                         }
    //                                         LiteralType::Hempty => {
    //                                             new_string.push_str("HEMPTY");
    //                                         }
    //                                     };
    //                                 }
    //                                 Self::Multiply => {
    //                                     if index > 0 {
    //                                         error::error(
    //                                             line,
    //                                             "Multiply can only be used with the car argument",
    //                                         );
    //                                     }
    //                                     match thing {
    //                                         LiteralType::Number(number) => {
    //                                             let mut new_new_string = String::new();
    //                                             for _ in 0..*number as i32 - 1 {
    //                                                 new_new_string.push_str(&new_string);
    //                                             }
    //                                             new_string = new_new_string;
    //                                         }
    //                                         _ => {
    //                                             error::error(
    //                                                 line,
    //                                                 "strings can only be multiplied by numbers",
    //                                             );
    //                                         }
    //                                     }
    //                                 }
    //                                 Self::Divide | Self::Minus => {
    //                                     error::error(
    //                                         line,
    //                                         "Only numbers can be divided or subtracted found",
    //                                     );
    //                                 }
    //                                 _ => {}
    //                             };
    //                         }
    //                         LiteralType::String(new_string)
    //                     }
    //                     _ => error::error(line, "Invalid literal arguments"),
    //                 }
    //             }
    //             Self::Error
    //             | Self::Input
    //             | Self::StrToBool
    //             | Self::StrToHempty
    //             | Self::StrToNum
    //             | Self::RunCommand => {
    //                 arg_error(1, args.len() as u32, self, false, line);
    //                 match &args[0] {
    //                     LiteralType::String(ref string) => match self {
    //                         Self::Error => exit(1),
    //                         Self::Input => {
    //                             let mut input = String::new();
    //                             print!("{string}");
    //                             // flush stdout
    //                             io::stdout().flush().unwrap_or_else(|_| {
    //                                 error::error(line, "Error flushing stdout");
    //                             });
    //                             io::stdin().read_line(&mut input).unwrap_or_else(|_| {
    //                                 error::error(line, "Failed to read input");
    //                             });
    //                             LiteralType::String(input.trim().to_string())
    //                         }
    //                         Self::StrToBool => {
    //                             if string == "true" {
    //                                 LiteralType::Boolean(true)
    //                             } else if string == "false" {
    //                                 LiteralType::Boolean(false)
    //                             } else {
    //                                 error::error(line, "Expected true or false");
    //                             }
    //                         }
    //                         Self::StrToHempty => {
    //                             if string == "HEMPTY" {
    //                                 LiteralType::Hempty
    //                             } else {
    //                                 error::error(line, "Expected HEMPTY");
    //                             }
    //                         }
    //                         Self::StrToNum => {
    //                             let string = match string {
    //                                 strings if string.starts_with("0x") => {
    //                                     strings.clone().trim().to_owned()
    //                                 }
    //                                 strings => format!("0x{}", strings.trim()),
    //                             };
    //                             let number: FloatLiteral = string.parse().map_or_else(
    //                                 |_| {
    //                                     error::error(
    //                                         line,
    //                                         format!(
    //                                             "Error parsing string {} to number",
    //                                             string.trim()
    //                                         ),
    //                                     )
    //                                 },
    //                                 |value: FloatLiteral| value,
    //                             );
    //                             LiteralType::Number(number.convert::<f64>().inner())
    //                         }
    //                         Self::RunCommand => {
    //                             let cmd = if OS == "windows" {
    //                                 let mut cmd = Command::new("powershell");
    //                                 cmd.args(["-c", string.trim()]).output()
    //                             } else {
    //                                 let mut cmd = Command::new("sh");
    //                                 cmd.args(["-c", string.trim()]).output()
    //                             };
    //                             let cmd: String = match cmd {
    //                                 Ok(value) => {
    //                                     if value.status.success() {
    //                                         String::from_utf8_lossy(&value.stdout).into()
    //                                     } else {
    //                                         String::from_utf8_lossy(&value.stderr).into()
    //                                     }
    //                                 }
    //                                 Err(_) => error::error(
    //                                     line,
    //                                     format!("Error running command {}", string.trim()),
    //                                 ),
    //                             };
    //                             LiteralType::String(cmd)
    //                         }
    //                         _ => {
    //                             error::error(line, "command not found");
    //                         }
    //                     },
    //                     _ => error::error(line, "Expected string for input operator"),
    //                 }
    //             }
    //             Self::NotEqual | Self::Equal => {
    //                 if args.len() != 2 {
    //                     error::error(line, format!("Expected 2 arguments for {self:?} operator"));
    //                 }
    //                 let type_ = &args[0];
    //                 let type_1 = &args[1];
    //                 if type_.type_eq(type_1) {
    //                 } else {
    //                     error::error(
    //                         line,
    //                         format!(
    //                             "{type_} and {type_1} are not the same type which is required for {self} operator"
    //                         ),
    //                     );
    //                 }
    //                 if self == &Self::Equal {
    //                     LiteralType::Boolean(type_ == type_1)
    //                 } else {
    //                     LiteralType::Boolean(!(type_ == type_1))
    //                 }
    //             }
    //             Self::Or | Self::And => {
    //                 if args.len() != 2 {
    //                     error::error(line, format!("Expected 2 arguments for {self:?} operator"));
    //                 }
    //                 let bool_1 = match &args[0] {
    //                     LiteralType::Boolean(boolean) => boolean,
    //                     _ => error::error(line, format!("Expected boolean for {self:?} operator")),
    //                 };
    //                 let bool_2 = match &args[1] {
    //                     LiteralType::Boolean(boolean) => boolean,
    //                     _ => error::error(line, format!("Expected boolean for {self:?} operator")),
    //                 };
    //                 if bool_1 == bool_2 {
    //                     if bool_1 == &true {
    //                         LiteralType::Boolean(true)
    //                     } else {
    //                         LiteralType::Boolean(false)
    //                     }
    //                 } else {
    //                     LiteralType::Boolean(false)
    //                 }
    //             }
    //             Self::GreaterThan | Self::LessThan | Self::GreaterEqual | Self::LessEqual => {
    //                 if args.len() != 2 {
    //                     error::error(line, format!("Expected 2 arguments for {self:?} operator"));
    //                 }
    //                 let type_ = match &args[0] {
    //                     LiteralType::Number(number) => number,
    //                     _ => error::error(line, format!("Expected number for {self:?} operator")),
    //                 };
    //                 let type_1 = match &args[1] {
    //                     LiteralType::Number(number) => number,
    //                     _ => error::error(line, format!("Expected number for {self:?} operator")),
    //                 };
    //                 if self == &Self::GreaterThan {
    //                     LiteralType::Boolean(type_ > type_1)
    //                 } else if self == &Self::LessThan {
    //                     LiteralType::Boolean(type_ < type_1)
    //                 } else if self == &Self::GreaterEqual {
    //                     LiteralType::Boolean(type_ >= type_1)
    //                 } else {
    //                     LiteralType::Boolean(type_ <= type_1)
    //                 }
    //             }
    //             Self::Exit => {
    //                 if args.len() == 1 {
    //                     match &args[0] {
    //                         LiteralType::Number(number) => exit(*number as i32),
    //                         _ => {
    //                             error::error(line, format!("Expected number for {self:?} operator"))
    //                         }
    //                     }
    //                 } else {
    //                     error::error(line, format!("Expected 1 argument for {self:?} operator"));
    //                 }
    //             }
    //             Self::SplitOn => {
    //                 if args.len() < 2 {
    //                     error::error(
    //                         line,
    //                         format!("Expected al least 2 arguments for {self:?} operator"),
    //                     );
    //                 }
    //                 let og_string = match &args[0] {
    //                     LiteralType::String(string) => string,
    //                     _ => error::error(line, format!("Expected string for {self:?} operator")),
    //                 };
    //                 let split_on = match &args[1] {
    //                     LiteralType::String(string) => string,
    //                     _ => error::error(line, format!("Expected string for {self:?} operator")),
    //                 };
    //                 // check if there is a third argument (number)
    //                 args.get(2).map_or_else(
    //                     || {
    //                         og_string.split_once(split_on).map_or_else(
    //                             || LiteralType::String(og_string.to_string()),
    //                             |v| LiteralType::String(v.0.to_string()),
    //                         )
    //                     },
    //                     |number| -> LiteralType {
    //                         if let LiteralType::Number(number) = number {
    //                             let number = *number as usize;
    //                             // return the string until the nth time split_on is found
    //                             let string: Vec<&str> =
    //                                 og_string.split_inclusive(split_on).collect::<Vec<&str>>();
    //                             if number > string.len() {
    //                                 error::error(
    //                                     line,
    //                                     format!("{number} is greater than the number of splits"),
    //                                 );
    //                             }
    //                             // loop through the splits and add them to the string if they are less than the number
    //                             let mut ret_string = String::new();
    //                             string.iter().take(number).for_each(|i: &&str| {
    //                                 ret_string.push_str(i);
    //                             });
    //                             let ret_string = ret_string
    //                                 .rsplit_once(split_on)
    //                                 .map_or(og_string.to_string(), |string| string.0.to_string());
    //                             LiteralType::String(ret_string)
    //                         } else {
    //                             error::error(line, format!("Expected number for {self:?} operator"))
    //                         }
    //                     },
    //                 )
    //             }
    //             Self::Module => {
    //                 if args.len() != 2 {
    //                     error::error(line, format!("Expected 2 arguments for {self:?} operator"));
    //                 }
    //                 let module_name = match &args[0] {
    //                     LiteralType::String(string) => string,
    //                     _ => error::error(line, format!("Expected string for {self:?} operator")),
    //                 };
    //                 match &args[1] {
    //                     LiteralType::String(filename) => {
    //                         let prev_module_name = scope.module_name.clone();
    //                         if scope.module_name.is_empty() {
    //                             scope.module_name = module_name.clone();
    //                         } else {
    //                             scope.module_name = scope.module_name.clone() + "$" + module_name;
    //                         }
    //                         let module_name = scope.module_name.clone();
    //                         let file = File::open(filename);
    //                         if let Ok(mut file) = file {
    //                             let mut buf = String::new();
    //                             if let Err(err) = file.read_to_string(&mut buf) {
    //                                 error::error(line, format!("Failed to read file: {err}"));
    //                             }
    //                             let mut lexer = Lexer::new(&buf, filename);
    //                             lexer.set_module(module_name);
    //                             let lexed = lexer.scan_tokens();
    //                             let mut parsed = Parser::new(lexed, filename.clone());
    //                             let body = parsed.parse();
    //                             scope.find_functions(body);
    //                             scope.module_name = prev_module_name;
    //                         } else {
    //                             error::error(line, format!("Could not open file {filename:?}"));
    //                         };
    //                     }
    //                     _ => error::error(line, format!("Expected string for {self:?} operator")),
    //                 };

    //                 // see if there is the second argument is a string
    //                 LiteralType::String(module_name.to_string())
    //             }
    //             keyword if crate::KEYWORDS.is_keyword(keyword) => {
    //                 error::error(line, format!("Keyword not found {self}"));
    //             }
    //             _ => {
    //                 error::error(line, format!("Keyword not found {self}"));
    //             }
    //         }
    //     } else {
    //         error::error(line, format!("Unknown keyword, {self}"));
    //     }
    // }
}

impl Display for TokenType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::BuiltinFunction(builtinfn) => write!(f, "{builtinfn}"),
            _ => write!(f, "{self:?}"),
        }
    }
}

impl Display for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BuiltinFunction {self:?}",)
    }
}

#[derive(PartialEq, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub lexeme: String,
    pub info: Info<'a>,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType<'a>, lexeme: &str, info: Info<'a>) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_string(),
            info,
        }
    }

    pub const fn is_literal(&self) -> bool {
        matches!(
            self.token_type,
            TokenType::String { .. }
                | TokenType::Number { .. }
                | TokenType::Boolean { .. }
                | TokenType::Hempty
        )
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.token_type {
            TokenType::String { literal } => {
                write!(f, "String: [lexeme {:?}, value {literal:?}]", self.lexeme)
            }
            TokenType::Number { literal } => {
                write!(f, "Number: [lexeme {:?}, value {literal:?}]", self.lexeme)
            }
            _ => write!(f, "{:?}: [lexeme {:?}]", self.token_type, self.lexeme),
        }
    }
}

impl fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self} at {}", self.info)
    }
}
