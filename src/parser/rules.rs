use crate::{error, token::TokenType};
use std::fmt::{self, Debug, Display, Write};

#[derive(PartialEq, Clone, Debug)]
pub enum Ast {
    Identifier(Identifier),
    Function(Function),
    If(If),
    Loop(Loop),
    Break(Break),
    Continue(Continue),
    Return(Return),
    Literal(Literal),
    Call(Call),
    Declaration(Declaration),
    Block(Block),
}
impl Ast {
    pub(crate) fn set_print(&mut self, prints: PrintType) {
        match self {
            Ast::Identifier(node) => node.print = prints,
            Ast::Function(node) => node.print = prints,
            Ast::If(node) => node.print = prints,
            Ast::Loop(node) => node.print = prints,
            Ast::Break(node) => node.print = prints,
            Ast::Continue(node) => node.print = prints,
            Ast::Return(node) => node.print = prints,
            Ast::Literal(node) => node.print = prints,
            Ast::Call(node) => node.print = prints,
            Ast::Declaration(node) => node.print = prints,
            Ast::Block(node) => node.print = prints,
        }
    }
}

impl Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ast::Identifier(node) => write!(f, "{node}"),
            Ast::Function(node) => write!(f, "{node}"),
            Ast::If(node) => write!(f, "{node}"),
            Ast::Loop(node) => write!(f, "{node}"),
            Ast::Break(node) => write!(f, "{node}"),
            Ast::Continue(node) => write!(f, "{node}"),
            Ast::Return(node) => write!(f, "{node}"),
            Ast::Literal(node) => write!(f, "{node}"),
            Ast::Call(node) => write!(f, "{node}"),
            Ast::Declaration(node) => write!(f, "{node}"),
            Ast::Block(node) => write!(f, "{node}"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct BreakNode();
impl Display for BreakNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "break")
    }
}
impl BreakNode {
    pub fn new() -> Self {
        Self()
    }
}
pub type Break = Located<BreakNode>;
#[derive(PartialEq, Clone, Debug)]
pub struct ContinueNode();

impl Display for ContinueNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "continue")
    }
}
impl ContinueNode {
    pub fn new() -> Self {
        Self()
    }
}
pub type Continue = Located<ContinueNode>;
#[derive(PartialEq, Clone, Debug)]
pub struct ReturnNode(pub Option<Box<Ast>>);

impl Display for ReturnNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(ref val) => write!(f, "return [{val}]"),
            None => write!(f, "return"),
        }
    }
}

impl ReturnNode {
    pub fn new(value: Ast) -> Self {
        Self(Some(Box::new(value)))
    }

    pub fn new_empty() -> Self {
        Self(None)
    }
}
pub type Return = Located<ReturnNode>;

#[derive(PartialEq, Clone, Debug)]
pub enum PrintType {
    Print,
    PrintLn,
    None,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Located<T: Clone + Debug + Display> {
    pub node: T,
    pub start_line: i32,
    pub end_line: Option<i32>,
    pub filename: String,
    pub print: PrintType,
}

impl<T: Clone + Debug + Display> Display for Located<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.node)
    }
}

impl<T: Clone + Debug + Display> Located<T> {
    pub fn new(node: T, start_line: i32, end_line: i32, filename: String) -> Self {
        Self {
            node,
            start_line,
            end_line: Some(end_line),
            filename,
            print: PrintType::None,
        }
    }
    pub fn new_single_line(node: T, start_line: i32, filename: String) -> Self {
        Self {
            node,
            start_line,
            end_line: None,
            filename,
            print: PrintType::None,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct BlockNode(pub Vec<Ast>);
impl BlockNode {
    pub fn new(block: Vec<Ast>) -> Self {
        Self(block)
    }
}
pub type Block = Located<BlockNode>;

impl Display for BlockNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct IdentifierNode(pub String);
pub type Identifier = Located<IdentifierNode>;

impl IdentifierNode {
    pub const fn new(name: String) -> Self {
        Self(name)
    }
}
impl Display for IdentifierNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum LiteralNode {
    Number(f64),
    String(String),
    Boolean(bool),
    Hempty,
}

pub type Literal = Located<LiteralNode>;
impl LiteralNode {
    pub fn get_from_ast(ast: &Ast, line: i32) -> Self {
        match ast {
            Ast::Literal(l) => l.node.clone(),
            _ => error::error(line, "not a literal"),
        }
    }
    // pub fn from_other_stuff(thing: &OtherStuff, line: i32) -> Self {
    //     match thing {
    //         OtherStuff::Literal(literal) => literal.literal.clone(),
    //         _ => error::error(line, "not a literal"),
    //     }
    // }
    // pub fn from_stuff(thing: &Stuff, line: i32) -> Self {
    //     match thing {
    //         Stuff::Literal(literal) => literal.literal.clone(),
    //         _ => error::error(line, "not a literal"),
    //     }
    // }
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

impl Display for LiteralNode {
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
pub enum DeclarationType {
    Cons(Box<List>),
    Variable(Box<Ast>),
}

impl DeclarationType {
    pub fn new(thing: &[Ast], line: i32) -> Self {
        match thing.len() {
            0 => error::error(line, "expected Identifier, got empty list"),
            1 => Self::Variable(Box::new(thing[0].clone())),
            2 => Self::Cons(Box::new(List::new(thing))),
            _ => error::error(
                line,
                "expected Identifier, got list with more than 2 elements",
            ),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct DeclarationNode {
    pub name: String,
    pub value: DeclarationType,
}

pub type Declaration = Located<DeclarationNode>;
impl DeclarationNode {
    pub fn new(name: String, value: &[Ast], line: i32) -> Self {
        Self {
            value: DeclarationType::new(value, line),
            name,
        }
    }
}
impl Display for DeclarationNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.name,
            match &self.value {
                DeclarationType::Cons(list) => format!("cons: {list}"),
                DeclarationType::Variable(vairable) => format!("variable: {vairable}"),
            }
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct CallNode {
    pub keyword: TokenType,
    pub arguments: Vec<Ast>,
}

impl CallNode {
    pub fn new(keyword: TokenType, arguments: Vec<Ast>) -> Self {
        Self { keyword, arguments }
    }
}

pub type Call = Located<CallNode>;
impl Display for CallNode {
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
pub struct FunctionNode {
    pub name: char,
    pub num_arguments: f64,
    pub extra_arguments: bool,
    pub body: Block,
}

impl FunctionNode {
    pub fn new(name: char, num_arguments: f64, extra_arguments: bool, body: Block) -> Self {
        Self {
            name,
            num_arguments,
            extra_arguments,
            body,
        }
    }
}

pub type Function = Located<FunctionNode>;

impl Display for FunctionNode {
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
    pub car: Ast,
    pub cdr: Ast,
}

impl List {
    pub fn new(thing: &[Ast]) -> Self {
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
pub struct IfNode {
    pub condition: Box<Ast>,
    pub body_true: Block,
    pub body_false: Block,
}

impl IfNode {
    pub fn new(condition: Box<Ast>, body_true: Block, body_false: Block) -> Self {
        Self {
            condition,
            body_true,
            body_false,
        }
    }
}

pub type If = Located<IfNode>;

impl Display for IfNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "if statement: with condition: [{}] when true: [\n{}\n] and when false: [\n{}\n]",
            self.condition, self.body_true, self.body_false,
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct LoopNode(pub Block);

impl LoopNode {
    pub fn new(block: Block) -> Self {
        Self(block)
    }
}

impl Display for LoopNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type Loop = Located<LoopNode>;
