use std::{
    fmt::{self, Debug},
};
use crate::token::TokenType;

use super::{Thing, Tree};
// TODO: make proper constructors for each struct/enum
#[derive(Clone, Debug)]
pub struct Expression {
    pub inside: Tree<Stuff>,
    pub print: bool,
    pub line: i32,
}

impl Expression {
    pub fn new(inside: Tree<Stuff>, print: bool, line: i32) -> Expression {
        Expression {
            inside,
            print,
            line,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inside)
    }
}

#[derive(Clone, Debug)]
pub enum Stuff {
    Literal(Literal),
    Identifier(Box<Identifier>),
    Call(Call),
}

impl fmt::Display for Stuff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stuff::Literal(literal) => write!(f, "{}", literal),
            Stuff::Identifier(identifier) => write!(f, "{}", identifier),
            Stuff::Call(call) => write!(f, "{}", call),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Literal {
    pub literal: LiteralType,
    pub line: i32,
}

impl Literal {
    pub fn new(literal: LiteralType, line: i32) -> Literal {
        Literal { literal, line }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}
#[derive(Clone, Debug)]
pub enum LiteralType {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl LiteralType {
    pub fn new_string(string: String) -> LiteralType {
        LiteralType::String(string)
    }

    pub fn new_number(number: f64) -> LiteralType {
        LiteralType::Number(number)
    }

    pub fn new_boolean(boolean: bool) -> LiteralType {
        LiteralType::Boolean(boolean)
    }

    pub fn new_null() -> LiteralType {
        LiteralType::Null
    }
}

impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralType::Number(num) => write!(f, "{}", num),
            LiteralType::String(string) => write!(f, "{}", string),
            LiteralType::Boolean(bool) => write!(f, "{}", bool),
            LiteralType::Null => write!(f, "null"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum IdentifierType {
    List(Box<List>),
    Vairable(Box<Vairable>),
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub name: String,
    pub value: IdentifierType,
    pub line: i32,
}

impl Identifier {
    pub fn new(name: String, value: IdentifierType, line: i32) -> Identifier {
        Identifier { name, value, line }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(Clone, Debug)]
pub struct Call {
    pub keyword: TokenType,
    pub arguments: Vec<Stuff>,
    pub line: i32,
}

impl Call {
    pub fn new(arguments: Vec<Stuff>, line: i32, keyword: TokenType) -> Call {
        Call { arguments, line, keyword }
    }
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut c = String::from("( ");
        for arg in self.arguments.iter() {
            c.push_str(&format!("{} ", arg));
        };
        c.push(')');
        write!(f, "{:?}: [{}]",self.keyword, c)
    }
}

#[derive(Clone, Debug)]
pub enum OtherStuff {
    Literal(Literal),
    Identifier(Identifier),
    Expression(Expression),
}

impl fmt::Display for OtherStuff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OtherStuff::Literal(literal) => write!(f, "{}", literal),
            OtherStuff::Identifier(identifier) => write!(f, "{}", identifier),
            OtherStuff::Expression(expression) => write!(f, "{}", expression),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: char,
    pub num_arguments: f64,
    pub body: Box<Tree<Thing>>,
    pub line: i32,
}

impl Function {
    pub fn new(name: char, num_arguments: f64, body: Box<Tree<Thing>>, line: i32) -> Self {
        Function {
            name,
            num_arguments,
            body,
            line,
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Function: {} with {} arguments and body: {}",
            self.name, self.num_arguments, self.body
        )
    }
}

#[derive(Clone, Debug)]
pub struct List {
    pub line: i32,
    pub first: OtherStuff,
    pub second: OtherStuff,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "with {} {}", self.first, self.second)
    }
}

#[derive(Clone, Debug)]
pub struct Vairable {
    pub value: OtherStuff,
}

impl Vairable {
    pub fn new(value: OtherStuff, ) -> Self {
        Vairable { value, }
    }

    pub fn new_empty(line: i32) -> Self {
        Vairable {
            value: OtherStuff::Literal(Literal::new(LiteralType::Null, line)),
        }
    }
}

impl fmt::Display for Vairable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "with: {}", self.value)
    }
}

#[derive(Clone, Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub body_true: Box<Tree<Thing>>,
    pub body_false: Box<Tree<Thing>>,
    pub line: i32,
}

impl fmt::Display for IfStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "if statement: {} when true: {:?} and when false: {:?}",
            self.condition, self.body_true, self.body_false
        )
    }
}

#[derive(Clone, Debug)]
pub struct LoopStatement {
    pub body: Box<Tree<Thing>>,
    pub line: i32,
}

impl LoopStatement {
    pub fn new(body: Box<Tree<Thing>>, line: i32) -> Self {
        LoopStatement { body, line }
    }
}

impl fmt::Display for LoopStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "loop statement: {:?}", self.body)
    }
}
