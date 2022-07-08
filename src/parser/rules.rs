use crate::token::TokenType;
use std::fmt::{self, Debug, Display};

use super::{Displays, Thing, Tree};
// TODO: make proper constructors for each struct/enum
#[derive(PartialEq, Clone, Debug)]
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

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inside)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Stuff {
    Literal(Literal),
    Identifier(Box<Identifier>),
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
    pub fn new_string(string: String, line: i32) -> Literal {
        Literal {
            literal: LiteralType::String(string),
            line,
        }
    }

    pub fn new_number(number: f64, line: i32) -> Literal {
        Literal {
            literal: LiteralType::Number(number),
            line,
        }
    }

    pub fn new_boolean(boolean: bool, line: i32) -> Literal {
        Literal {
            literal: LiteralType::Boolean(boolean),
            line,
        }
    }

    pub fn new_null(line: i32) -> Literal {
        Literal {
            literal: LiteralType::Null,
            line,
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}
#[derive(PartialEq, Clone, Debug)]
pub enum LiteralType {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl LiteralType {}

impl Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralType::Number(num) => write!(f, "{}", num),
            LiteralType::String(string) => write!(f, "{}", string),
            LiteralType::Boolean(bool) => write!(f, "{}", bool),
            LiteralType::Null => write!(f, "null"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum IdentifierType {
    List(Box<List>),
    Vairable(Box<Vairable>),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Identifier {
    pub name: String,
    pub value: IdentifierType,
    pub line: i32,
}

impl Identifier {
    pub fn new(name: String, value: IdentifierType, line: i32) -> Identifier {
        Identifier { name, value, line }
    }
    pub fn new_empty(name: String, line: i32) -> Identifier {
        Identifier {
            name,
            value: IdentifierType::Vairable(Box::new(Vairable::new_empty(line))),
            line,
        }
    }
}

impl Display for Identifier {
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

#[derive(PartialEq, Clone, Debug)]
pub struct Call {
    pub keyword: TokenType,
    pub arguments: Vec<Stuff>,
    pub line: i32,
}

impl Call {
    pub fn new(arguments: Vec<Stuff>, line: i32, keyword: TokenType) -> Call {
        Call {
            arguments,
            line,
            keyword,
        }
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut c = String::from("");
        for arg in self.arguments.iter() {
            c.push_str(&format!("{} ", arg));
        }
        write!(f, "{:?}: [{}]", self.keyword, c)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum OtherStuff {
    Literal(Literal),
    Identifier(Identifier),
    Expression(Expression),
}

impl Display for OtherStuff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OtherStuff::Literal(literal) => write!(f, "{}", literal),
            OtherStuff::Identifier(identifier) => write!(f, "{}", identifier),
            OtherStuff::Expression(expression) => write!(f, "{}", expression),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Function {
    pub name: char,
    pub num_arguments: f64,
    pub body: Vec<Tree<Thing>>,
    pub line: i32,
}

impl Function {
    pub fn new(name: char, num_arguments: f64, body: Vec<Tree<Thing>>, line: i32) -> Self {
        Function {
            name,
            num_arguments,
            body,
            line,
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Function: {} with {} arguments and body: {}",
            self.name,
            self.num_arguments,
            self.body.to_strings()
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct List {
    pub line: i32,
    pub first: OtherStuff,
    pub second: OtherStuff,
}

impl Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "with {} {}", self.first, self.second)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Vairable {
    pub value: OtherStuff,
}

impl Vairable {
    pub fn new(value: OtherStuff) -> Self {
        Vairable { value }
    }

    pub fn new_empty(line: i32) -> Self {
        Vairable {
            value: OtherStuff::Literal(Literal::new_null(line)),
        }
    }
}

impl Display for Vairable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "with: {}", self.value)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct IfStatement {
    pub condition: OtherStuff,
    pub body_true: Vec<Tree<Thing>>,
    pub body_false: Vec<Tree<Thing>>,
    pub line: i32,
}

impl IfStatement {
    pub fn new(
        condition: OtherStuff,
        body_true: Vec<Tree<Thing>>,
        body_false: Vec<Tree<Thing>>,
        line: i32,
    ) -> Self {
        IfStatement {
            condition,
            body_true,
            body_false,
            line,
        }
    }
}

impl Display for IfStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "if statement: {} when true: {:?} and when false: {:?}",
            self.condition, self.body_true, self.body_false
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct LoopStatement {
    pub body: Vec<Tree<Thing>>,
    pub line: i32,
}

impl LoopStatement {
    pub fn new(body: Vec<Tree<Thing>>, line: i32) -> Self {
        LoopStatement { body, line }
    }
}

impl Display for LoopStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "loop statement: {:?}", self.body)
    }
}
