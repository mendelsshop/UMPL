use crate::{error, token::TokenType};
use std::fmt::{self, Debug, Display, Write};

use super::Thing;
pub mod new_parser {
    use std::fmt::{self, Display};

    use crate::{
        error::error,
        token::{Info, Token, TokenType}, parser::rules::Expression,
    };

    #[derive(Debug, Clone, PartialEq)]
    pub struct Expr<'a> {
        pub info: Info<'a>,
        pub expr: ExprType<'a>,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum ExprType<'a> {
        Literal(Lit<'a>),
        List(Box<List<'a>>),
        Fn(Box<FnDef<'a>>),
        Call(Box<FnCall<'a>>),
        If(Box<If<'a>>),
        Loop(Box<Loop<'a>>),
        Var(Box<Var<'a>>),
    }

    impl<'a> Expr<'a> {
        pub fn new_literal(info: Info<'a>, value: Lit<'a>) -> Self {
            Self {
                info,
                expr: ExprType::Literal(value),
            }
        }

        pub fn new_list(info: Info<'a>, value: List<'a>) -> Self {
            Self {
                info,
                expr: ExprType::List(Box::new(value)),
            }
        }

        pub fn new_fn(info: Info<'a>, value: FnDef<'a>) -> Self {
            Self {
                info,
                expr: ExprType::Fn(Box::new(value)),
            }
        }

        pub fn new_call(info: Info<'a>, value: FnCall<'a>) -> Self {
            Self {
                info,
                expr: ExprType::Call(Box::new(value)),
            }
        }

        pub fn new_if(info: Info<'a>, value: If<'a>) -> Self {
            Self {
                info,
                expr: ExprType::If(Box::new(value)),
            }
        }

        pub fn new_loop(info: Info<'a>, value: Loop<'a>) -> Self {
            Self {
                info,
                expr: ExprType::Loop(Box::new(value)),
            }
        }
    }

    impl<'a> Display for Expr<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.expr {
                ExprType::Literal(lit) => write!(f, "literal [{}]", lit),
                ExprType::List(list) => write!(f, "list [{}]", list),
                ExprType::Fn(fn_def) => write!(f, "fn [{}]", fn_def),
                ExprType::Call(call) => write!(f, "call [{}]", call),
                ExprType::If(if_) => write!(f, "if [{}]", if_),
                ExprType::Loop(loop_def) => write!(f, "loop [{}]", loop_def),
                ExprType::Var(var) => write!(f, "var [{}]", var),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Lit<'a> {
        pub info: Info<'a>,
        pub value: LitType<'a>,
    }

    impl<'a> Lit<'a> {
        pub fn new_number(info: Info<'a>, value: f64) -> Self {
            Self {
                info,
                value: LitType::Number(value),
            }
        }

        pub fn new_string(info: Info<'a>, value: &'a str) -> Self {
            Self {
                info,
                value: LitType::String(value),
            }
        }

        pub fn new_boolean(info: Info<'a>, value: bool) -> Self {
            Self {
                info,
                value: LitType::Boolean(value),
            }
        }

        pub fn new_hemty(info: Info<'a>) -> Self {
            Self {
                info,
                value: LitType::Hemty,
            }
        }

        pub fn new_file(info: Info<'a>, value: &'a str) -> Self {
            Self {
                info,
                value: LitType::File(value),
            }
        }
    }

    impl<'a> Display for Lit<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} at [{}]", self.value, self.info)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LitType<'a> {
        String(&'a str),
        Number(f64),
        Boolean(bool),
        File(&'a str),
        Hemty,
    }

    impl fmt::Display for LitType<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                LitType::String(s) => write!(f, "string: {}", s),
                LitType::Number(n) => write!(f, "number: {}", n),
                LitType::Boolean(b) => write!(f, "boolean: {}", b),
                LitType::File(s) => write!(f, "file: {}", s),
                LitType::Hemty => write!(f, "hemty"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct List<'a> {
        pub info: Info<'a>,
        pub car: Expr<'a>,
        pub cdr: Expr<'a>,
    }

    impl<'a> List<'a> {
        pub fn new(info: Info<'a>, car: Expr<'a>, cdr: Expr<'a>) -> Self {
            Self { info, car, cdr }
        }

        pub fn car(&self) -> &Expr<'a> {
            &self.car
        }

        pub fn cdr(&self) -> &Expr<'a> {
            &self.cdr
        }
    }

    impl<'a> Display for List<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "list: [car: {} cdr: {} [{}]]",
                self.car, self.cdr, self.info
            )
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum FnExpr<'a> {
        Expr(Expr<'a>),
        Return(Expr<'a>),
    }

    impl<'a> Display for FnExpr<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                FnExpr::Expr(e) => write!(f, "{}", e),
                FnExpr::Return(e) => write!(f, "return: {}", e),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FnDef<'a> {
        pub info: Info<'a>,
        pub name: &'a str,
        pub param_count: usize,
        pub extra_params: bool,
        pub body: Vec<FnExpr<'a>>,
    }

    impl<'a> FnDef<'a> {
        pub fn new(
            info: Info<'a>,
            name: &'a str,
            param_count: usize,
            extra_params: bool,
            body: Vec<FnExpr<'a>>,
        ) -> Self {
            Self {
                info,
                name,
                param_count,
                extra_params,
                body,
            }
        }
    }

    impl<'a> Display for FnDef<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "fn: [{} with {} {} parameters {} [{}]]",
                self.name,
                if self.extra_params { "at least" } else { "" },
                self.param_count,
                self.body
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                self.info
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum PrintType {
        Newline,
        NoNewline,
        None,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FnCall<'a> {
        pub info: Info<'a>,
        pub args: Vec<Expr<'a>>,
        pub print_type: PrintType,
    }

    impl<'a> FnCall<'a> {
        pub fn new(info: Info<'a>, args: Vec<Expr<'a>>, print_type: PrintType) -> Self {
            Self {
                info,
                args,
                print_type,
            }
        }
    }

    impl<'a> Display for FnCall<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "fn: {}() with {} args [{}]",

                self.args.iter().map(|e| e.to_string()).collect::<String>(),
                self.args.len(),
                self.info
            )
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct If<'a> {
        pub info: Info<'a>,
        pub condition: Expr<'a>,
        pub then: Vec<Expr<'a>>,
        pub otherwise: Vec<Expr<'a>>,
    }

    impl<'a> If<'a> {
        pub fn new(
            info: Info<'a>,
            condition: Expr<'a>,
            then: Vec<Expr<'a>>,
            otherwise: Vec<Expr<'a>>,
        ) -> Self {
            Self {
                info,
                condition,
                then,
                otherwise,
            }
        }
    }

    impl<'a> Display for If<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "if {} then {} else {} [{}]",
                self.condition,
                self.then
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                self.otherwise
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                self.info
            )
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum LoopExpr<'a> {
        Break,
        Continue,
        Expr(Expr<'a>),
    }

    impl<'a> Display for LoopExpr<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                LoopExpr::Break => write!(f, "break"),
                LoopExpr::Continue => write!(f, "continue"),
                LoopExpr::Expr(e) => write!(f, "{}", e),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Loop<'a> {
        pub info: Info<'a>,
        pub body: Vec<LoopExpr<'a>>,
    }

    impl<'a> Loop<'a> {
        pub fn new(info: Info<'a>, body: Vec<LoopExpr<'a>>) -> Self {
            Self { info, body }
        }
    }

    impl<'a> Display for Loop<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "loop {} [{}]",
                self.body
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                self.info
            )
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Var<'a> {
        pub info: Info<'a>,
        pub name: &'a str,
        pub value: Expr<'a>,
    }

    impl<'a> Var<'a> {
        pub fn new(info: Info<'a>, name: &'a str, value: Expr<'a>) -> Self {
            Self { info, name, value }
        }
    }

    impl<'a> Display for Var<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "var {} = {} [{}]", self.name, self.value, self.info)
        }
    }

    pub struct Parser<'a> {
        tokens: Vec<Token<'a>>,
        current_position: usize,
        done: bool,
        token: Token<'a>,
        in_function: bool,
        in_loop: bool,
        paren_count: usize,
        weird_bracket_count: usize,
        file_path: &'a str,
    }
    static START_TOKEN: Token<'static> = Token {
        token_type: TokenType::EOF,
        info: Info::new("", 0, 0),
        lexeme: String::new(),
    };

    type Module<'a> = Vec<Expr<'a>>;
    impl<'a> Parser<'a> {
        pub fn new(tokens: Vec<Token<'a>>, file_path: &'a str) -> Self {
            Self {
                tokens,
                current_position: 0,
                done: false,
                token: START_TOKEN.clone(),
                in_function: false,
                in_loop: false,
                paren_count: 0,
                weird_bracket_count: 0,
                file_path,
            }
        }

        pub fn parse(&mut self) -> Module<'a> {
            let mut module = Vec::new();
            while !self.done {
                if let Some(token) = self.parse_from_token() {
                    module.push(token);
                }
            }
            module
        }

        fn parse_from_token(&mut self) -> Option<Expr<'a>> {
            // advance in token stream - important especially for the first time each time we parse something
            // becuase token is set to nothing so this initializes token
            self.advance("parse_from_token - start");
            // check if we ran out of tokens or are done parsing (if EOF is encountered)
            if self.tokens.is_empty() {
                error(0, "no self.tokens found");
            }
            if self.done {
                return None;
            }
            match self.token.token_type.clone() {
                // we have entered an expression
                TokenType::LeftParen => self.parse_parenthissized(),
                // we hit a keyword
                keyword if crate::KEYWORDS.is_keyword(&keyword) => {
                    todo!()
                }
                keyword => {
                    todo!("keyword: {:?}", keyword)
                }
            }

        }

        fn parse_parenthissized(&mut self) -> Option<Expr<'a>> {
            println!("parse_parenthissized");
            // save the line number because the next token can be on the next line
            let start_line = self.token.info.line;
            let mut args = Vec::new();
            // self.advance("parse_parenthissized - start looking for args");
            while self.token.token_type != TokenType::RightParen {
                if let Some(token) = self.parse_from_token() {
                    args.push(token);
                }
                self.advance("parse_parenthissized - looking for args");
            }
            // check printing indicator (<) no print (>) print (>>) print no newline
            self.advance("parse_parenthissized - looking for printing indicator");
            println!("printing indicator: {:?}", self.token);
            match self.token.token_type {
                TokenType::LessThanSymbol => {
                    Some(Expr::new_call(
                        Info::new(self.file_path, start_line, self.token.info.line),
                        FnCall::new(
                            Info::new(self.file_path, start_line, self.token.info.line),
                            args,
                            PrintType::None,
                        ),
                    ))


                } 
                TokenType::GreaterThanSymbol => {
                    None
                    // check if the next token is a greater than or not


                }
                _ => {
                    // should never happen as this is caught in the advance method
                    error(self.token.info.line, "Expected printing indicator");
                }
            }
        }

        fn advance(&mut self, caller: &str) {
            match self.tokens[self.current_position].token_type {
                TokenType::Return { .. } => {
                    if self.in_function {
                        self.token = self.tokens[self.current_position].clone();
                    } else {
                        error(
                            self.tokens[self.current_position].info.line,
                            "Return statement outside of function",
                        );
                    }
                }
                TokenType::Break | TokenType::Continue => {
                    if self.in_loop {
                        self.token = self.tokens[self.current_position].clone();
                    } else {
                        error(
                            self.tokens[self.current_position].info.line,
                            "Break or continue statement outside of loop",
                        );
                    }
                }
                TokenType::EOF => {
                    self.done = true;
                    self.token = self.tokens[self.current_position].clone();
                }
                TokenType::LeftParen => {
                    self.paren_count += 1;
                    self.token = self.tokens[self.current_position].clone();
                }
                TokenType::RightParen => {
                    if self.paren_count == 0 {
                        error(
                            self.tokens[self.current_position].info.line,
                            "unmatched right parenthesis",
                        );
                    }
                    self.paren_count -= 1;
                    if !(vec![TokenType::GreaterThanSymbol, TokenType::LessThanSymbol]
                            .contains(&self.tokens[self.current_position + 1].token_type))
                    {
                        error(
                            self.tokens[self.current_position].info.line,
                            format!(
                                "greater than symbol (>) or less than symbol (<) expected found {}",
                                self.tokens[self.current_position + 1].token_type
                            )
                            .as_str(),
                        )
                    }
                    self.token = self.tokens[self.current_position].clone();
                }
                TokenType::CodeBlockBegin => {
                    self.weird_bracket_count += 1;
                    self.token = self.tokens[self.current_position].clone();
                }
                TokenType::CodeBlockEnd => {
                    self.weird_bracket_count -= 1;
                    self.token = self.tokens[self.current_position].clone();
                }
                TokenType::GreaterThanSymbol | TokenType::LessThanSymbol => {
                        self.token = self.tokens[self.current_position].clone();
                }
                _ => {
                    self.token = self.tokens[self.current_position].clone();
                }
            };
            println!("token: {:?} caller: {}", self.token, caller);
            self.current_position += 1;
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Expression {
    pub inside: Stuff,
    pub print: bool,
    pub line: u32,
    pub new_line: bool,
    pub filename: String,
}

impl Expression {
    pub const fn new(
        inside: Stuff,
        print: bool,
        line: u32,
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct IdentifierPointer {
    pub name: String,
    pub line: u32,
    pub filename: String,
}

impl IdentifierPointer {
    pub const fn new(name: String, line: u32, filename: String) -> Self {
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
    Function(Function),
    If(Box<IfStatement>),
    List(Box<List>),
}

impl Display for Stuff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::Identifier(identifier) => write!(f, "{identifier}"),
            Self::Call(call) => write!(f, "{call}"),
            Self::Function(function) => write!(f, "{function}"),
            Self::If(if_statement) => write!(f, "{if_statement}"),
            Self::List(list) => write!(f, "{list}"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Literal {
    pub literal: LiteralType,
    pub line: u32,
    pub filename: String,
}

impl Literal {
    pub const fn new_string(string: String, line: u32, filename: String) -> Self {
        Self {
            literal: LiteralType::String(string),
            line,
            filename,
        }
    }

    pub const fn new_number(number: f64, line: u32, filename: String) -> Self {
        Self {
            literal: LiteralType::Number(number),
            line,
            filename,
        }
    }

    pub const fn new_boolean(boolean: bool, line: u32, filename: String) -> Self {
        Self {
            literal: LiteralType::Boolean(boolean),
            line,
            filename,
        }
    }

    pub const fn new_hempty(line: u32, filename: String) -> Self {
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
    pub fn from_other_stuff(thing: &OtherStuff, line: u32) -> Self {
        match thing {
            OtherStuff::Literal(literal) => literal.literal.clone(),
            _ => error::error(line, "not a literal"),
        }
    }
    pub fn from_stuff(thing: &Stuff, line: u32) -> Self {
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
    pub fn new(thing: &[OtherStuff], line: u32) -> Self {
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
    pub line: u32,
    pub filename: String,
}

impl Identifier {
    pub fn new(name: String, value: &[OtherStuff], line: u32, filename: String) -> Self {
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
    pub line: u32,
    pub end_line: u32,
    pub filename: String,
}

impl Call {
    pub fn new(
        arguments: &[Stuff],
        line: u32,
        filename: String,
        end_line: u32,
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
    pub name: String,
    pub num_arguments: f64,
    pub extra_arguments: bool,
    pub body: Vec<Thing>,
    pub line: u32,
    pub end_line: u32,
    pub filename: String,
}

impl Function {
    pub fn new(
        name: String,
        num_arguments: f64,
        body: &[Thing],
        line: u32,
        filename: String,
        end_line: u32,
        extra_arguments: bool,
    ) -> Self {
        Self {
            name,
            num_arguments,
            body: body.to_vec(),
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
    pub line: u32,
    pub end_line: u32,
    pub filename: String,
}

impl IfStatement {
    pub fn new(
        condition: OtherStuff,
        body_true: Vec<Thing>,
        body_false: Vec<Thing>,
        line: u32,
        filename: String,
        end_line: u32,
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
    pub line: u32,
    pub end_line: u32,
    pub filename: String,
}

impl LoopStatement {
    pub fn new(body: &[Thing], line: u32, filename: String, end_line: u32) -> Self {
        Self {
            body: body.to_vec(),
            line,
            end_line,
            filename,
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
