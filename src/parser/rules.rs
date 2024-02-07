use crate::{error, token::TokenType};
use std::fmt::{self, Debug, Display, Write};

use super::Thing;
#[derive(PartialEq, Clone, Debug)]
pub struct Expression {
    pub inside: Stuff,
    pub print: bool,
    pub line: i32,
    pub new_line: bool,
    pub filename: String,
}

impl Expression {
    pub const fn new(
        inside: Stuff,
        print: bool,
        line: i32,
        filename: String,
        new_line: bool,
    ) -> Self {
        Self {
            inside,
            print,
            line,
            new_line,
            filename,
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "expr({})", self.inside)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Block {
    pub body: Vec<Thing>,
    pub line: i32,
    pub end_line: i32,
    pub filename: String,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.body
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Block {
    pub fn new(body: Vec<Thing>, line: i32, end_line: i32, filename: String) -> Self {
        Self {
            body,
            line,
            end_line,
            filename,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct IdentifierPointer {
    pub name: String,
    pub line: i32,
    pub filename: String,
}

impl IdentifierPointer {
    pub const fn new(name: String, line: i32, filename: String) -> Self {
        Self {
            name,
            line,
            filename,
        }
    }
}

impl Display for IdentifierPointer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Stuff {
    Literal(Literal),
    Identifier(IdentifierPointer),
    Call(Call),
    Block(Block),
}

impl Display for Stuff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::Identifier(identifier) => write!(f, "{identifier}"),
            Self::Call(call) => write!(f, "{call}"),
            Self::Block(block) => write!(f, "{block}"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Literal {
    pub literal: LiteralType,
    pub line: i32,
    pub filename: String,
}

impl Literal {
    pub const fn new_string(string: String, line: i32, filename: String) -> Self {
        Self {
            literal: LiteralType::String(string),
            line,
            filename,
        }
    }

    pub const fn new_number(number: f64, line: i32, filename: String) -> Self {
        Self {
            literal: LiteralType::Number(number),
            line,
            filename,
        }
    }

    pub const fn new_boolean(boolean: bool, line: i32, filename: String) -> Self {
        Self {
            literal: LiteralType::Boolean(boolean),
            line,
            filename,
        }
    }

    pub const fn new_hempty(line: i32, filename: String) -> Self {
        Self {
            literal: LiteralType::Hempty,
            line,
            filename,
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}
#[derive(PartialEq, Clone, Debug)]
pub enum LiteralType {
    Number(f64),
    String(String),
    Boolean(bool),
    Hempty,
}

impl LiteralType {
    pub fn from_other_stuff(thing: &OtherStuff, line: i32) -> Self {
        match thing {
            OtherStuff::Literal(literal) => literal.literal.clone(),
            _ => error::error(line, "not a literal"),
        }
    }
    pub fn from_stuff(thing: &Stuff, line: i32) -> Self {
        match thing {
            Stuff::Literal(literal) => literal.literal.clone(),
            _ => error::error(line, "not a literal"),
        }
    }
    pub const fn new_string(string: String) -> Self {
        Self::String(string)
    }

    pub const fn new_number(number: f64) -> Self {
        Self::Number(number)
    }

    pub const fn new_boolean(boolean: bool) -> Self {
        Self::Boolean(boolean)
    }

    pub const fn new_hempty() -> Self {
        Self::Hempty
    }
    pub const fn type_eq(&self, other: &Self) -> bool {
        match self {
            Self::Number(_) => matches!(other, Self::Number(_)),
            Self::String(_) => matches!(other, Self::String(_)),
            Self::Boolean(_) => matches!(other, Self::Boolean(_)),
            Self::Hempty => matches!(other, Self::Hempty),
        }
    }

    pub fn get_type(&self) -> String {
        match self {
            Self::Number(_) => "number".to_string(),
            Self::String(_) => "string".to_string(),
            Self::Boolean(_) => "boolean".to_string(),
            Self::Hempty => "hempty".to_string(),
        }
    }
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{num}"),
            Self::String(string) => write!(f, "{string}"),
            Self::Boolean(bool) => write!(f, "{bool}"),
            Self::Hempty => write!(f, "hempty"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum IdentifierType {
    List(Box<List>),
    Vairable(Box<Vairable>),
}

impl IdentifierType {
    pub fn new(thing: &[OtherStuff], line: i32) -> Self {
        match thing.len() {
            0 => error::error(line, "expected Identifier, got empty list"),
            1 => Self::Vairable(Box::new(Vairable::new(thing[0].clone()))),
            2 => Self::List(Box::new(List::new(thing))),
            _ => error::error(
                line,
                "expected Identifier, got list with more than 2 elements",
            ),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Identifier {
    pub name: String,
    pub value: IdentifierType,
    pub line: i32,
    pub filename: String,
}

impl Identifier {
    pub fn new(name: String, value: &[OtherStuff], line: i32, filename: String) -> Self {
        Self {
            name,
            value: IdentifierType::new(value, line),
            line,
            filename,
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.name,
            match &self.value {
                IdentifierType::List(list) => format!("list: {list}"),
                IdentifierType::Vairable(vairable) => format!("variable: {vairable}"),
            }
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Call {
    pub keyword: TokenType,
    pub arguments: Vec<Stuff>,
    pub line: i32,
    pub end_line: i32,
    pub filename: String,
}

impl Call {
    pub fn new(
        arguments: &[Stuff],
        line: i32,
        filename: String,
        end_line: i32,
        keyword: TokenType,
    ) -> Self {
        Self {
            arguments: arguments.to_vec(),
            line,
            keyword,
            end_line,
            filename,
        }
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut c = String::new();
        for arg in self.arguments.iter().enumerate() {
            write!(c, "{}{}", arg.1, {
                if arg.0 < self.arguments.len() - 1 {
                    ", "
                } else {
                    ""
                }
            })?;
        }
        write!(f, "{:?}: [{c}]", self.keyword)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum OtherStuff {
    Literal(Literal),
    Identifier(IdentifierPointer),
    Expression(Expression),
}

impl Display for OtherStuff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::Identifier(identifier) => write!(f, "{identifier}"),
            Self::Expression(expression) => write!(f, "{expression}"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    pub name: char,
    pub num_arguments: f64,
    pub extra_arguments: bool,
    pub body: Block,
    pub line: i32,
    pub end_line: i32,
    pub filename: String,
}

impl Function {
    pub fn new(
        name: char,
        num_arguments: f64,
        body: Block,
        line: i32,
        filename: String,
        end_line: i32,
        extra_arguments: bool,
    ) -> Self {
        Self {
            name,
            num_arguments,
            body,
            line,
            end_line,
            filename,
            extra_arguments,
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Function: {} with {} arguments and body: [\n\t{}\n]",
            self.name, self.num_arguments, self.body
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct List {
    pub car: OtherStuff,
    pub cdr: OtherStuff,
}

impl List {
    pub fn new(thing: &[OtherStuff]) -> Self {
        Self {
            car: thing[0].clone(),
            cdr: thing[1].clone(),
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "with: [{}, {}]", self.car, self.cdr)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Vairable {
    pub value: OtherStuff,
}

impl Vairable {
    const fn new(value: OtherStuff) -> Self {
        Self { value }
    }
}

impl Display for Vairable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "with: {}", self.value)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct IfStatement {
    pub condition: OtherStuff,
    pub body_true: Block,
    pub body_false: Block,
    pub line: i32,
    pub end_line: i32,
    pub filename: String,
}

impl IfStatement {
    pub fn new(
        condition: OtherStuff,
        body_true: Block,
        body_false: Block,
        line: i32,
        end_line: i32,
        filename: String,
    ) -> Self {
        Self {
            condition,
            body_true,
            body_false,
            line,
            end_line,
            filename,
        }
    }
}

impl Display for IfStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "if statement: with condition: [{}] when true: [\n{}\n] and when false: [\n{}\n]",
            self.condition, self.body_true, self.body_false,
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct LoopStatement {
    pub body: Block,
    pub line: i32,
    pub end_line: i32,
    pub filename: String,
}

impl LoopStatement {
    pub fn new(body: Block, line: i32, filename: String, end_line: i32) -> Self {
        Self {
            body,
            line,
            end_line,
            filename,
        }
    }
}

impl Display for LoopStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "loop statement: [\n{}\n]", self.body)
    }
}
