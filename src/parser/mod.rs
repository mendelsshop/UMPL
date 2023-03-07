pub mod rules;
use crate::{
    error::error,
    parser::rules::{IdentifierType, LiteralType},
    token::{Info, Token, TokenType},
};
use log::{debug, info, warn};
use rules::{
    Call, Expression, Function, Identifier, IdentifierPointer, IfStatement, Literal, LoopStatement,
    OtherStuff, Stuff,
};
use std::fmt::{self, format, Display};

use self::rules::List;

pub struct Parser<'a> {
    paren_count: usize,
    weird_bracket_count: usize,
    current_position: usize,
    tokens: Vec<Token<'a>>,
    token: Token<'a>,
    done: bool,
    in_function: bool,
    in_loop: bool,
    variables: Vec<String>,
    filename: String,
}

static START_TOKEN: Token<'static> = Token {
    token_type: TokenType::EOF,
    info: Info::new("", 0, 0),
    lexeme: String::new(),
};

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>, name: String) -> Self {
        Self {
            paren_count: 0,
            current_position: 0,
            tokens,
            token: START_TOKEN.clone(),
            done: false,
            weird_bracket_count: 0,
            in_function: false,
            in_loop: false,
            variables: Vec::new(),
            filename: name,
        }
    }

    pub fn advance(&mut self, fn_name: &str) {
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
                if self.paren_count == 0
                    && !(vec![TokenType::GreaterThanSymbol, TokenType::LessThanSymbol]
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
                if self.paren_count == 0 {
                    self.token = self.tokens[self.current_position].clone();
                } else {
                    error(
                        self.tokens[self.current_position].info.line,
                        "greater than symbol (>) or less than symbol (<) not allowed in middle of expression",
                    );
                }
            }
            _ => {
                self.token = self.tokens[self.current_position].clone();
            }
        };
        info!("{}", self.paren_count); //
        info!("new token: {} in function {}", self.token, fn_name);
        self.current_position += 1;
    }

    pub fn parse(&mut self) -> Vec<Thing> {
        let mut program: Vec<Thing> = Vec::new();
        // loop until we have no more self.tokens
        // in the loop, we use parse_from_tokens to parse the next expression
        // and add it to the program tree
        info!("{:?}", self.tokens);
        while !self.done {
            let expr: Option<Thing> = self.parse_from_token();
            if let Some(t) = expr {
                debug!("{:?}", t);
                program.push(t);
            }
        }
        info!("Done parsing");
        program
    }
    #[allow(clippy::too_many_lines)]
    fn parse_from_token(&mut self) -> Option<Thing> {
        self.advance("parse_from_token");
        info!("new iteration");
        if self.tokens.is_empty() {
            error(0, "no self.tokens found");
        }
        if self.done {
            return None;
        }
        info!("PARSEfromTOKEN {}", self.token);
        match self.token.token_type.clone() {
            TokenType::LeftParen => Some(Thing::Expression(self.after_left_paren())),
            TokenType::CodeBlockEnd => None,
            TokenType::Identifier { .. } => {
                error(self.token.info.line, "variable not allowed in this context");
            }
            keyword if crate::KEYWORDS.is_keyword(&keyword) => {
                info!("found keyword {}", self.token.token_type);
                match self.token.token_type.clone() {
                    TokenType::Potato => Some(Thing::Function(self.get_function())),
                    TokenType::List => Some(Thing::Identifier(self.get_list())),
                    TokenType::Create => {
                        self.advance("parse_from_token");
                        match self.token.token_type.clone() {
                            TokenType::Identifier { name } => {
                                info!("create identifier found");
                                self.advance("parse_from_token");
                                if self.token.token_type == TokenType::With {
                                    self.advance("parse_from_token");
                                    info!("create identifier with {}", self.token.token_type);
                                    let thing = self.get_value();
                                    self.variables.push(name.clone());
                                    Some(Thing::Identifier(Identifier::new(
                                        name,
                                        &[thing],
                                        self.token.info.line,
                                        self.filename.clone(),
                                    )))
                                } else {
                                    error(
                                        self.token.info.line,
                                        format!(
                                            "with keyword expected, found TokenType::{:?}",
                                            self.token.token_type
                                        )
                                        .as_str(),
                                    );
                                }
                            }
                            tokentype => {
                                error(
                                    self.token.info.line,
                                    format!(
                                        "identifier expected after \"create\", found TokenType::{tokentype:?}"
                                    )
                                        .as_str(),
                                );
                            }
                        }
                    }
                    TokenType::Loop => {
                        info!("loop found");
                        let start_line = self.token.info.line;
                        self.advance("parse_from_token looking for loop body");
                        let mut loop_body: Vec<Thing> = Vec::new();
                        if self.token.token_type == TokenType::CodeBlockBegin {
                            self.in_loop = true;
                            while self.tokens[self.current_position].token_type
                                != TokenType::CodeBlockEnd
                            {
                                info!("parsing loop body");
                                self.in_loop = true; // just in case we encounter a loop inside a loop and when the inner loop ends it will set this to false which will make the outer loop panic if it encountweers a break|contuinue statement
                                if let Some(t) = self.parse_from_token() {
                                    loop_body.push(t);
                                }
                            }
                            info!("Done parsing loop body");
                            self.advance("parse_from_token after loop body looking for loop end");
                            self.in_loop = false;
                            Some(Thing::LoopStatement(LoopStatement::new(
                                &loop_body,
                                start_line,
                                self.filename.clone(),
                                self.token.info.line,
                            )))
                        } else {
                            error(self.token.info.line, "code block expected after \"loop\"");
                        }
                    }
                    TokenType::If => Some(Thing::IfStatement(self.get_if())),
                    TokenType::Return { .. } => {
                        if self.tokens[self.current_position].token_type == TokenType::Colon {
                            self.advance("parse_from_token return expecting expression");
                            return Some(Thing::Return(
                                None,
                                self.token.info.line,
                                self.filename.clone(),
                            ));
                        }
                        self.advance("parse_from_token return expecting expression");
                        let thing = self.parse_to_other_stuff();
                        Some(Thing::Return(
                            Some(thing),
                            self.token.info.line,
                            self.filename.clone(),
                        ))
                    }
                    TokenType::Break => {
                        info!("break statement");
                        Some(Thing::Break(self.token.info.line, self.filename.clone()))
                    }
                    TokenType::Continue => {
                        info!("continue statement");
                        Some(Thing::Continue(self.token.info.line, self.filename.clone()))
                    }
                    _ => {
                        error(
                            self.token.info.line,
                            "keyword not allowed in expression before left parenthesis",
                        );
                    }
                }
            }
            _ => {
                error(
                    self.token.info.line,
                    format!("{:?} not allowed in this context", self.token.token_type),
                );
            }
        }
    }

    fn get_if(&mut self) -> IfStatement {
        let start_line = self.token.info.line;
        self.advance("parse_from_token after if expecting left brace");
        if self.token.token_type == TokenType::LeftBrace {
            info!("if statement");
            self.advance("parse_from_token finding condition");
            let mut if_body: Vec<Thing>;
            let mut else_body: Vec<Thing>;
            let thing: OtherStuff = match self.token.clone().token_type {
                TokenType::Boolean { value } => OtherStuff::Literal(Literal::new_boolean(
                    value,
                    self.token.info.line,
                    self.filename.clone(),
                )),
                TokenType::LeftParen => OtherStuff::Expression(self.after_left_paren()),
                TokenType::Identifier { name } => OtherStuff::Identifier(self.var(name)),
                tokentype => {
                    error(
                        self.token.info.line,
                        format!(
                            "boolean expected, in if statement condition found TokenType::{tokentype:?}"
                        )
                            .as_str(),
                    );
                }
            };
            info!("after conditon if statement");
            self.advance("parse_from_token if expecting left brace");
            if self.token.token_type == TokenType::RightBrace {
                self.advance("parse_from_token looking for if body");
                if self.token.token_type == TokenType::CodeBlockBegin {
                    if_body = Vec::new();
                    while self.tokens[self.current_position].token_type != TokenType::CodeBlockEnd {
                        info!(
                            "c {:?} n {}",
                            self.token.token_type, self.tokens[self.current_position].token_type
                        );
                        if let Some(token) = self.parse_from_token() {
                            if_body.push(token);
                        }
                    }
                    self.advance("parse_from_token looking for else body");
                    self.advance("parse_from_token before else");
                    if self.token.token_type == TokenType::Else {
                        self.advance("parse_from_token looking for else body");
                        if self.token.token_type == TokenType::CodeBlockBegin {
                            info!("else found");
                            else_body = Vec::new();
                            while self.tokens[self.current_position].token_type
                                != TokenType::CodeBlockEnd
                            {
                                if let Some(x) = self.parse_from_token() {
                                    else_body.push(x);
                                }
                            }
                            info!("in else_body");
                            self.advance("parse_from_token after else");
                            IfStatement::new(
                                thing,
                                if_body,
                                else_body,
                                start_line,
                                self.filename.clone(),
                                self.token.info.line,
                            )
                        } else {
                            error(self.token.info.line, "code block expected after \"else\"");
                        }
                    } else {
                        error(
                            self.token.info.line,
                            "else keyword expected after if statement",
                        );
                    }
                } else {
                    error(self.token.info.line, "code block expected after \"if\"");
                }
            } else {
                error(
                    self.token.info.line,
                    "right brace expected after if condition",
                );
            }
        } else {
            error(
                self.token.info.line,
                format!(
                    "{{ expected after \"if\" found TokenType::{:?}",
                    self.token.token_type
                )
                .as_str(),
            );
        }
    }

    fn get_list(&mut self) -> Identifier {
        self.advance("parse_from_token");
        match self.token.token_type.clone() {
            TokenType::Identifier { name } => {
                info!("list identifier found");
                self.advance("parse_from_token");
                if self.token.token_type == TokenType::With {
                    self.advance("parse_from_token");
                    if self.token.token_type == TokenType::LeftBracket {
                        self.advance("parse_from_token");
                        info!("list with");
                        let thing: OtherStuff = self.parse_to_other_stuff();
                        self.advance("parse_from_token");
                        let thing1: OtherStuff = self.parse_to_other_stuff();
                        self.advance("  ");
                        if self.token.token_type == TokenType::RightBracket {
                            self.variables.push(name.clone());
                            Identifier::new(
                                name,
                                &[thing, thing1],
                                self.token.info.line,
                                self.filename.clone(),
                            )
                        } else {
                            error(
                                self.token.info.line,
                                format!(
                                    "right bracket expected after list, found {}",
                                    self.token.token_type
                                )
                                .as_str(),
                            );
                        }
                    } else {
                        error(
                            self.token.info.line,
                            format!(
                                "left bracket expected after \"with\", found {}",
                                self.token.token_type
                            )
                            .as_str(),
                        );
                    }
                } else {
                    error(
                        self.token.info.line,
                        format!(
                            "with keyword expected, found TokenType::{:?}",
                            self.token.token_type
                        )
                        .as_str(),
                    );
                }
            }
            tokentype => {
                error(
                    self.tokens[1].info.line,
                    format!("identifier expected, after \"list\" found TokenType::{tokentype:?}")
                        .as_str(),
                );
            }
        }
    }

    fn get_function(&mut self) -> Function {
        let start_line = self.token.info.line;
        self.advance("parse_from_token after function looking for function name");
        match self.token.token_type.clone() {
            TokenType::FunctionIdentifier { name } => {
                info!("function identifier found");
                self.advance("parse_from_token after function name looking for function arguments");
                // check if the next token is a number and save it in a vairable num_args
                let num_of_args_and_extra: (f64, bool) = match self.token.token_type {
                    TokenType::Number { literal } => {
                        if literal.trunc() == literal {
                            self.advance(
                                "parse_from_token found number or args looking for function body",
                            );
                            if self.token.token_type == TokenType::Star {
                                self.advance(
                                    "parse_from_token found star looking for function body",
                                );
                                (literal, true)
                            } else {
                                (literal, false)
                            }
                        } else {
                            error(
                                self.token.info.line,
                                format!("number expected in function declaration found floating point number literal with {literal}"),
                            );
                        }
                    }
                    TokenType::Star => {
                        self.advance("parse_from_token found star looking for function body");
                        (0.0, true)
                    }
                    TokenType::CodeBlockBegin => (0.0, false),
                    _ => {
                        error(
                            self.token.info.line,
                            format!(
                                "number expected after function identifier, found {}",
                                self.token
                            ),
                        );
                    }
                };

                info!("int function declaration before code block");
                if self.token.token_type == TokenType::CodeBlockBegin {
                    let mut function: Vec<Thing> = Vec::new();
                    self.in_function = true;
                    while self.tokens[self.current_position].token_type != TokenType::CodeBlockEnd {
                        self.in_function = true; // for funtions inside functions see loop below for more info
                        if let Some(t) = self.parse_from_token() {
                            function.push(t);
                        }
                    }
                    self.advance("parse_from_token after function body looking for function end");
                    self.in_function = false;
                    debug!("new function {:?}", function);
                    Function::new(
                        name,
                        num_of_args_and_extra.0,
                        &function,
                        start_line,
                        self.filename.clone(),
                        self.token.info.line,
                        num_of_args_and_extra.1,
                    )
                } else {
                    error(
                        self.token.info.line,
                        format!(
                            "code block expected after function identifier, found {}",
                            self.token.token_type
                        ),
                    );
                }
            }
            tokentype => {
                error(
                    self.token.info.line,
                    format!("function identifier expected after \"potato\", found TokenType::{tokentype:?}"),
                );
            }
        }
    }

    fn after_left_paren(&mut self) -> Expression {
        let start_line = self.token.info.line;

        self.advance("after left paren");
        if self.token.token_type == TokenType::New {
            self.advance("after left paren");
            match self.token.token_type {
                TokenType::FunctionIdentifier { .. } => {}
                _ => {
                    error(
                        self.token.info.line,
                        "function identifier expected after new",
                    );
                }
            }
        }
        let keyword: TokenType = self.token.token_type.clone();
        let line: u32 = self.token.info.line;
        info!("found call {}", keyword);
        // check if keyword is a literal if it is then it is an expression
        let value = match &keyword {
            TokenType::Number { literal } => Some(LiteralType::Number(*literal)),
            TokenType::String { literal } => Some(LiteralType::String(literal.to_string())),
            TokenType::Boolean { value } => Some(LiteralType::Boolean(*value)),
            TokenType::Hempty => Some(LiteralType::Hempty),
            _ => None,
        };
        if let Some(t) = value {
            self.advance("after left paren");
            if self.token.token_type != TokenType::RightParen {
                error(self.token.info.line, "right paren expected after literal");
            }
            return Expression::new(
                Stuff::Literal(Literal {
                    literal: t,
                    line: start_line,
                    filename: self.filename.clone(),
                }),
                false,
                start_line,
                self.filename.clone(),
                false,
            );
        }
        self.advance("after left paren");
        let mut args = Vec::new();

        while self.token.token_type != TokenType::RightParen {
            info!("looking for args");
            println!("looking for args");
            args.push(self.parse_to_stuff());
            self.advance("after left paren");
        }

        self.check_end(LitorExpr::Call(Call {
            keyword,
            arguments: args,
            line: start_line,
            end_line: line,
            filename: self.filename.clone(),
        }))
    }

    fn check_end(&mut self, value: LitorExpr) -> Expression {
        // check if the paren count
        todo!()
    }

    fn var(&mut self, name: String) -> IdentifierPointer {
        if name.starts_with('$') && self.in_function {
            if self.tokens[self.current_position].token_type == TokenType::With {
                error(
                    self.tokens[self.current_position].info.line,
                    "function arguments are immutable",
                );
            } else {
                IdentifierPointer::new(name, self.token.info.line, self.filename.clone())
            }
        } else if self.tokens[self.current_position].token_type == TokenType::Dot {
            self.advance("Var");
            self.advance("Var");
            if let TokenType::Car | TokenType::Cdr = self.token.token_type {
                // make a string with the name + . and
                info!("found dot {}", self.token.token_type);
                let name: String =
                    name + "." + &format!("{:?}", self.token.token_type).to_lowercase();
                IdentifierPointer::new(name, self.token.info.line, self.filename.clone())
            } else {
                error(self.token.info.line, "car or Cdr expected after dot");
            }
        } else {
            IdentifierPointer::new(name, self.token.info.line, self.filename.clone())
        }
    }
    fn get_value(&mut self) -> OtherStuff {
        match self.token.token_type.clone() {
            TokenType::Number { literal } => OtherStuff::Literal(Literal::new_number(
                literal,
                self.token.info.line,
                self.filename.clone(),
            )),
            TokenType::String { literal } => OtherStuff::Literal(Literal::new_string(
                literal,
                self.token.info.line,
                self.filename.clone(),
            )),
            TokenType::Hempty => OtherStuff::Literal(Literal::new_hempty(
                self.token.info.line,
                self.filename.clone(),
            )),
            TokenType::Boolean { value } => OtherStuff::Literal(Literal::new_boolean(
                value,
                self.token.info.line,
                self.filename.clone(),
            )),
            TokenType::LeftParen => OtherStuff::Expression(self.after_left_paren()),
            TokenType::Identifier { name } => OtherStuff::Identifier(self.var(name)),
            tokentype => {
                error(
                    self.token.info.line,
                    format!("identifier expected, after \"create\" found TokenType::{tokentype:?}")
                        .as_str(),
                );
            }
        }
    }

    fn parse_to_stuff(&mut self) -> Stuff {
        info!("parsing stuff");
        println!("parsing stuff");
        println!("token type {:?}", self.token.token_type);
        match self.token.token_type.clone() {
            TokenType::LeftParen => {
                info!("found left paren");
                self.advance("parse to stuff");
                // match self.after_left_paren() {
                //     Callorexpression::Call(call) => Stuff::Call(call),
                //     Callorexpression::Expression(a) => error(
                //         self.token.info.line,
                //         format!("call expected after left parenthesis found {a:?}"),
                //     ),
                // }
                self.after_left_paren();
                todo!()
            }
            TokenType::Number { literal } => Stuff::Literal(Literal::new_number(
                literal,
                self.token.info.line,
                self.filename.clone(),
            )),
            TokenType::String { literal } => Stuff::Literal(Literal::new_string(
                literal,
                self.token.info.line,
                self.filename.clone(),
            )),
            TokenType::Hempty => Stuff::Literal(Literal::new_hempty(
                self.token.info.line,
                self.filename.clone(),
            )),
            TokenType::Boolean { value } => Stuff::Literal(Literal::new_boolean(
                value,
                self.token.info.line,
                self.filename.clone(),
            )),
            TokenType::Identifier { name } => Stuff::Identifier(self.var(name)),
            TokenType::Create => {
                todo!("variables created in an expression");
            }
            TokenType::If => {
                let if_statement = self.get_if();
                Stuff::If(Box::new(if_statement))
            }
            TokenType::Loop => {
                unimplemented!("loops in an expression");
            }
            TokenType::Potato => {
                let fn_definition = self.get_function();
                Stuff::Function(fn_definition)
            }
            TokenType::List => {
                let list = self.get_list();
                match list.value {
                    IdentifierType::List(list) => Stuff::List(list),
                    _ => todo!(),
                }
            }
            _ => {
                error(
                    self.token.info.line,
                    format!("-{:?} not allowed in this context", self.token.token_type),
                );
            }
        }
    }

    fn parse_to_other_stuff(&mut self) -> OtherStuff {
        match self.token.token_type.clone() {
            TokenType::LeftParen => {
                // self.advance("parse to other stuff");
                OtherStuff::Expression(self.after_left_paren())
            }
            TokenType::Number { literal } => OtherStuff::Literal(Literal::new_number(
                literal,
                self.token.info.line,
                self.filename.clone(),
            )),
            TokenType::String { literal } => OtherStuff::Literal(Literal::new_string(
                literal,
                self.token.info.line,
                self.filename.clone(),
            )),
            TokenType::Hempty => OtherStuff::Literal(Literal::new_hempty(
                self.token.info.line,
                self.filename.clone(),
            )),
            TokenType::Boolean { value } => OtherStuff::Literal(Literal::new_boolean(
                value,
                self.token.info.line,
                self.filename.clone(),
            )),
            TokenType::Identifier { name } => OtherStuff::Identifier(self.var(name)),
            _ => {
                error(
                    self.token.info.line,
                    format!("{:?} not allowed in this context", self.token.token_type),
                );
            }
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Callorexpression {
    Call(Call),
    Expression(Expression),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Thing {
    // we have vairants for each type of token that has a value ie number or the name of an identifier
    Identifier(Identifier),
    Expression(Expression),
    Function(Function),
    IfStatement(IfStatement),
    LoopStatement(LoopStatement),
    Break(u32, String),
    Continue(u32, String),
    Return(Option<OtherStuff>, u32, String),
}

impl Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expression(expression) => write!(f, "{expression}"),
            Self::Identifier(s) => write!(f, "Identifier({s})"),
            Self::Function(function) => write!(f, "{function}"),
            Self::IfStatement(if_statement) => write!(f, "{if_statement}"),
            Self::LoopStatement(loop_statement) => write!(f, "{loop_statement}"),
            Self::Break(..) => write!(f, "Break"),
            Self::Continue(..) => write!(f, "Continue"),
            Self::Return(stuff, _, _) => write!(
                f,
                "Return{}",
                stuff
                    .as_ref()
                    .map_or_else(String::new, |stuff| format!("({stuff})")),
            ),
        }
    }
}

enum LitorExpr {
    Literal(Literal),
    Call(Call),
}
