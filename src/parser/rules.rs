use crate::{error, token::TokenType};
use std::fmt::{self, Debug, Display, Write};

use super::Thing;
#[derive(PartialEq, Clone, Debug)]
pub struct Expression {
    pub inside: Stuff,
    pub print: bool,
    pub line: i32,
    pub new_line: bool,
}

impl Expression {
    pub const fn new(inside: Stuff, print: bool, line: i32, new_line: bool) -> Self {
        Self {
            inside,
            print,
            line,
            new_line,
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "expr({})", self.inside)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct IdentifierPointer {
    pub name: String,
    pub line: i32,
}

impl IdentifierPointer {
    pub const fn new(name: String, line: i32) -> Self {
        Self { name, line }
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
}

impl Display for Stuff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stuff::Literal(literal) => write!(f, "{}", literal),
            Stuff::Identifier(identifier) => write!(f, "{}", identifier),
            Stuff::Call(call) => write!(f, "{}", call),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Literal {
    pub literal: LiteralType,
    pub line: i32,
}

impl Literal {
    pub const fn new_string(string: String, line: i32) -> Self {
        Self {
            literal: LiteralType::String(string),
            line,
        }
    }

    pub const fn new_number(number: f64, line: i32) -> Self {
        Self {
            literal: LiteralType::Number(number),
            line,
        }
    }

    pub const fn new_boolean(boolean: bool, line: i32) -> Self {
        Self {
            literal: LiteralType::Boolean(boolean),
            line,
        }
    }

    pub const fn new_hempty(line: i32) -> Self {
        Self {
            literal: LiteralType::Hempty,
            line,
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
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::String(string) => write!(f, "{}", string),
            Self::Boolean(bool) => write!(f, "{}", bool),
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
}

impl Identifier {
    pub fn new(name: String, value: &[OtherStuff], line: i32) -> Self {
        Self {
            name,
            value: IdentifierType::new(value, line),
            line,
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
                IdentifierType::List(list) => format!("list: {}", list),
                IdentifierType::Vairable(vairable) => format!("variable: {}", vairable),
            }
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Call {
    pub keyword: TokenType,
    pub arguments: Vec<Stuff>,
    pub line: i32,
}

impl Call {
    pub fn new(arguments: &[Stuff], line: i32, keyword: TokenType) -> Self {
        Self {
            arguments: arguments.to_vec(),
            line,
            keyword,
        }
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut c = String::from("");
        for arg in self.arguments.iter().enumerate() {
            write!(c, "{}{}", arg.1, {
                if arg.0 < self.arguments.len() - 1 {
                    ", "
                } else {
                    ""
                }
            })
            .unwrap();
        }
        write!(f, "{:?}: [{}]", self.keyword, c)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum OtherStuff {
    Literal(Literal),
    Identifier(IdentifierPointer),
    Expression(Expression),
}

impl OtherStuff {
    pub fn from_stuff(stuff: &Stuff, line: i32) -> Self {
        match stuff {
            Stuff::Literal(literal) => Self::Literal(literal.clone()),
            Stuff::Identifier(identifier) => Self::Identifier(identifier.clone()),
            _ => error::error(line, "expected literal or identifier"),
        }
    }
}

impl Display for OtherStuff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::Identifier(identifier) => write!(f, "{}", identifier),
            Self::Expression(expression) => write!(f, "{}", expression),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    pub name: char,
    pub num_arguments: f64,
    pub body: Vec<Thing>,
    pub line: i32,
}

impl Function {
    pub fn new(name: char, num_arguments: f64, body: &[Thing], line: i32) -> Self {
        Self {
            name,
            num_arguments,
            body: body.to_vec(),
            line,
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Function: {} with {} arguments and body: [\n\t{}\n]",
            self.name,
            self.num_arguments,
            self.body
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join("\n")
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
    pub body_true: Vec<Thing>,
    pub body_false: Vec<Thing>,
    pub line: i32,
}

impl IfStatement {
    pub fn new(
        condition: OtherStuff,
        body_true: Vec<Thing>,
        body_false: Vec<Thing>,
        line: i32,
    ) -> Self {
        Self {
            condition,
            body_true,
            body_false,
            line,
        }
    }
}

impl Display for IfStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "if statement: with condition: [{}] when true: [\n{}\n] and when false: [\n{}\n]",
            self.condition,
            self.body_true
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join("\n"),
            self.body_false
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct LoopStatement {
    pub body: Vec<Thing>,
    pub line: i32,
}

impl LoopStatement {
    pub fn new(body: &[Thing], line: i32) -> Self {
        Self {
            body: body.to_vec(),
            line,
        }
    }
}

impl Display for LoopStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "loop statement: [\n{}\n]",
            self.body
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
