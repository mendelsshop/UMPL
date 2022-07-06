use std::fmt;


use super::{Thing, Tree};

#[derive(Clone, Debug)]
pub struct Expression {
    pub inside: Tree<Stuff>,
    pub line: i32,
    pub print: bool,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inside)
    }
}

#[derive(Clone, Debug)]
pub enum Stuff {
    Literal(Literal),
    Identifier(Identifier),
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
pub struct Identifier {
    pub name: String,
    pub value: String,
    pub line: i32,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.name,
            if self.value.is_empty() {
                "".to_string()
            } else {
                format!(" with value: {}", self.value)
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct Call {
    pub arguments: Vec<OtherStuff>,
    pub line: i32,
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Call: [{:?}]", self.arguments)
    }
}

#[derive(Clone, Debug)]
pub enum OtherStuff {
    Literal(Literal),
    Identifier(Identifier),
    Call(Call),
    Expression(Expression),
}

impl fmt::Display for OtherStuff {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OtherStuff::Literal(literal) => write!(f, "{}", literal),
            OtherStuff::Identifier(identifier) => write!(f, "{}", identifier),
            OtherStuff::Call(call) => write!(f, "{}", call),
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
    pub name: String,
    pub line: i32,
    pub first: OtherStuff,
    pub second: OtherStuff,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List: {} with {} {}", self.name, self.first, self.second)
    }
}

#[derive(Clone, Debug)]
pub struct Vairable {
    pub name: String,
    pub value: Expression,
    pub line: i32,
}

impl fmt::Display for Vairable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Variable: {} with value: {}", self.name, self.value)
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
