// use hexponent::FloatLiteral;
use std::fmt::{self, Debug, Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd, Ord, Default)]
pub struct Info<'a> {
    pub file_name: &'a str,
    pub begin: Position,
    pub end: Position,
}

// struct to store the line and column of a token
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd, Ord, Default)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

impl Position {
    pub const fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl<'a> Info<'a> {
    pub const fn new(file_name: &'a str, begin: Position, end: Position) -> Self {
        Self {
            file_name,
            begin,
            end,
        }
    }
}

impl Display for Info<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "File [{}:{}]",
            self.file_name,
            if self.begin == self.end {
                format!("{}", self.begin)
            } else {
                // TODO figure out how to get ides to select the whole range not just the first char
                format!("{}.{}", self.begin, self.end)
            }
        )
    }
}

#[derive(PartialEq, Debug, Clone, Copy, PartialOrd)]
#[allow(clippy::module_name_repetitions)]
pub enum TokenType<'a> {
    CallEnd,
    CallBegin,
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
    Identifier(&'a str),
    FunctionIdentifier(char),
    ModuleIdentifier(char),
    String(&'a str),
    Number(f64),
    Create,
    With,
    List,
    Car,
    Cdr,
    Cgr,
    Return,
    Break,
    Continue,
    Loop,
    Potato,
    If,
    Else,
    Hempty,
    Boolean(bool),
    Function,
    FunctionArgument(u64),
    EOF,
    Program,
    BuiltinFunction(BuiltinFunction),
    PlusSymbol,
    Module,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord, Hash)]

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

impl TokenType<'_> {}

impl Display for TokenType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BuiltinFunction(builtinfn) => write!(f, "builtin {builtinfn}"),
            Self::Boolean(bool) => write!(f, "bool {bool}"),
            Self::Number(number) => write!(f, "number {number}"),
            Self::String(string) => write!(f, "string {string}"),
            Self::Hempty => write!(f, "hempty"),
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
pub struct Token<'a, 'b> {
    pub token_type: TokenType<'a>,
    pub lexeme: String,
    pub info: Info<'b>,
}

impl<'a, 'b> Token<'a, 'b> {
    pub fn new(token_type: TokenType<'a>, lexeme: &str, info: Info<'b>) -> Self {
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

impl Display for Token<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.token_type {
            TokenType::String(literal) => {
                write!(f, "String: [lexeme {:?}, value {literal:?}]", self.lexeme)
            }
            TokenType::Number(literal) => {
                write!(f, "Number: [lexeme {:?}, value {literal:?}]", self.lexeme)
            }
            TokenType::Boolean(literal) => {
                write!(f, "Boolean: [lexeme {:?}, value {literal:?}]", self.lexeme)
            }
            _ => write!(f, "{:?}: [lexeme {:?}]", self.token_type, self.lexeme),
        }
    }
}

impl fmt::Debug for Token<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self} at {}", self.info)
    }
}
