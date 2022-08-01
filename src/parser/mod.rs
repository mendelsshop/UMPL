pub(crate) mod rules;
use crate::{
    error::error,
    token::{Token, TokenType},
};
use log::{debug, info, warn};
use rules::{
    Call, Expression, Function, Identifier, IdentifierPointer, IfStatement, Literal, LoopStatement,
    OtherStuff, Stuff,
};
use std::fmt::{self, Display};

pub struct Parser {
    paren_count: usize,
    weird_bracket_count: usize,
    current_position: usize,
    tokens: Vec<Token>,
    token: Token,
    done: bool,
    in_function: bool,
    in_loop: bool,
    variables: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            paren_count: 0,
            current_position: 0,
            tokens,
            token: Token {
                token_type: TokenType::EOF,
                line: 0,
                lexeme: "".to_string(),
            },
            done: false,
            weird_bracket_count: 0,
            in_function: false,
            in_loop: false,
            variables: Vec::new(),
        }
    }

    pub fn advance(&mut self, fn_name: &str) {
        match self.tokens[self.current_position].token_type {
            TokenType::Return { .. } => {
                if self.in_function {
                    self.token = self.tokens[self.current_position].clone();
                } else {
                    error(
                        self.tokens[self.current_position].line,
                        "Return statement outside of function",
                    );
                }
            }
            TokenType::Break | TokenType::Continue => {
                if self.in_loop {
                    self.token = self.tokens[self.current_position].clone();
                } else {
                    error(
                        self.tokens[self.current_position].line,
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
                        self.tokens[self.current_position].line,
                        "unmatched right parenthesis",
                    );
                }
                self.paren_count -= 1;
                if self.paren_count == 0
                    && !(vec![TokenType::GreaterThanSymbol, TokenType::LessThanSymbol]
                        .contains(&self.tokens[self.current_position + 1].token_type))
                {
                    error(
                        self.tokens[self.current_position].line,
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
                        self.tokens[self.current_position].line,
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
                program.push(t.clone());
                debug!("{:?}", t);
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
            TokenType::LeftParen => match self.after_left_paren() {
                Callorexpression::Expression(e) => Some(Thing::Expression(e)),
                _ => error(self.token.line, "expected expression"),
            },
            TokenType::CodeBlockEnd => None,
            TokenType::Identifier { .. } => {
                error(self.token.line, "variable not allowed in this context");
            }
            keyword if crate::KEYWORDS.is_keyword(&keyword) => {
                info!("found keyword {}", self.token.token_type);
                match self.token.token_type.clone() {
                    TokenType::Potato => {
                        self.advance("parse_from_token after function looking for function name");
                        match self.token.token_type.clone() {
                            TokenType::FunctionIdentifier { name } => {
                                info!("function identifier found");
                                self.advance("parse_from_token after function name looking for function arguments");
                                // check if the next token is a number and save it in a vairable num_args
                                let num_args: f64 = match self.token.token_type {
                                    TokenType::Number { literal } => {
                                        if literal.trunc() == literal {
                                            self.advance("parse_from_token found number or args looking for function body");
                                            literal
                                        } else {
                                            error(
                                                self.token.line,
                                                format!("number expected in function declaration found floating point number literal with {}", literal),
                                            );
                                        }
                                    }
                                    TokenType::CodeBlockBegin => 0f64,
                                    _ => {
                                        error(
                                            self.token.line,
                                            format!("number expected after function identifier, found {}", self.token),
                                        );
                                    }
                                };
                                info!("int function declaration before code block");
                                if self.token.token_type == TokenType::CodeBlockBegin {
                                    let mut function: Vec<Thing> = Vec::new();
                                    self.in_function = true;
                                    while self.tokens[self.current_position].token_type
                                        != TokenType::CodeBlockEnd
                                    {
                                        self.in_function = true; // for funtions inside functions see loop below for more info
                                        match self.parse_from_token() {
                                            Some(t) => function.push(t),
                                            None => {}
                                        }
                                    }
                                    self.advance("parse_from_token after function body looking for function end");
                                    self.in_function = false;
                                    debug!("new function {:?}", function);
                                    Some(Thing::Function(Function::new(
                                        name,
                                        num_args,
                                        &function,
                                        self.token.line,
                                    )))
                                } else {
                                    error(
                                        self.token.line,
                                        format!("code block expected after function identifier, found {}", self.token.token_type),
                                    );
                                }
                            }
                            tokentype => {
                                error(
                                    self.token.line,
                                    format!("function identifier expected after \"potato\", found TokenType::{:?}", tokentype),
                                );
                            }
                        }
                    }
                    TokenType::List => {
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
                                            Some(Thing::Identifier(Identifier::new(
                                                name,
                                                &[thing, thing1],
                                                self.token.line,
                                            )))
                                        } else {
                                            error(
                                                self.token.line,
                                                format!(
                                                    "right bracket expected after list, found {}",
                                                    self.token.token_type
                                                )
                                                .as_str(),
                                            );
                                        }
                                    } else {
                                        error(
                                            self.token.line,
                                            format!(
                                                "left bracket expected after \"with\", found {}",
                                                self.token.token_type
                                            )
                                            .as_str(),
                                        );
                                    }
                                } else {
                                    error(
                                        self.token.line,
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
                                    self.tokens[1].line,
                                    format!(
                                        "identifier expected, after \"list\" found TokenType::{:?}",
                                        tokentype
                                    )
                                    .as_str(),
                                );
                            }
                        }
                    }
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
                                        self.token.line,
                                    )))
                                } else {
                                    error(
                                        self.token.line,
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
                                    self.token.line,
                                    format!(
                                        "identifier expected after \"create\", found TokenType::{:?}",
                                        tokentype
                                    )
                                        .as_str(),
                                );
                            }
                        }
                    }
                    TokenType::Loop => {
                        info!("loop found");
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
                                self.token.line,
                            )))
                        } else {
                            error(self.token.line, "code block expected after \"loop\"");
                        }
                    }
                    TokenType::If => {
                        self.advance("parse_from_token after if expecting left brace");
                        if self.token.token_type == TokenType::LeftBrace {
                            info!("if statement");
                            self.advance("parse_from_token finding condition");
                            let mut if_body: Vec<Thing>;
                            let mut else_body: Vec<Thing>;
                            let thing: OtherStuff = match self.token.clone().token_type {
                                TokenType::Boolean { value } => OtherStuff::Literal(
                                    Literal::new_boolean(value, self.token.line),
                                ),
                                TokenType::LeftParen => match self.after_left_paren() {
                                    Callorexpression::Expression(thing) => {
                                        OtherStuff::Expression(thing)
                                    }
                                    _ => {
                                        error(self.token.line, "call found expected expression");
                                    }
                                },
                                TokenType::Identifier { name } => {
                                    OtherStuff::Identifier(self.var(name))
                                }
                                tokentype => {
                                    error(
                                        self.token.line,
                                        format!(
                                            "boolean expected, in if statement condition found TokenType::{:?}",
                                            tokentype
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
                                    while self.tokens[self.current_position].token_type
                                        != TokenType::CodeBlockEnd
                                    {
                                        info!(
                                            "c {:?} n {}",
                                            self.token.token_type,
                                            self.tokens[self.current_position].token_type
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
                                            Some(Thing::IfStatement(IfStatement::new(
                                                thing,
                                                if_body,
                                                else_body,
                                                self.token.line,
                                            )))
                                        } else {
                                            error(
                                                self.token.line,
                                                "code block expected after \"else\"",
                                            );
                                        }
                                    } else {
                                        error(
                                            self.token.line,
                                            "else keyword expected after if statement",
                                        );
                                    }
                                } else {
                                    error(self.token.line, "code block expected after \"if\"");
                                }
                            } else {
                                error(self.token.line, "right brace expected after if condition");
                            }
                        } else {
                            error(
                                self.token.line,
                                format!(
                                    "{{ expected after \"if\" found TokenType::{:?}",
                                    self.token.token_type
                                )
                                .as_str(),
                            );
                        }
                    }
                    TokenType::Return { .. } => {
                        if self.tokens[self.current_position].token_type == TokenType::Colon {
                            self.advance("parse_from_token return expecting expression");
                            return Some(Thing::Return(None, self.token.line));
                        }
                        // TODO: capture the value returned if any

                        self.advance("parse_from_token return expecting expression");
                        let thing = self.parse_to_other_stuff();

                        Some(Thing::Return(Some(thing), self.token.line))
                    }
                    TokenType::Break => {
                        info!("break statement");
                        Some(Thing::Break(self.token.line))
                    }
                    TokenType::Continue => {
                        info!("continue statement");
                        Some(Thing::Continue(self.token.line))
                    }
                    _ => {
                        error(
                            self.token.line,
                            "keyword not allowed in expression before left parenthesis",
                        );
                    }
                }
            }
            _ => {
                error(
                    self.token.line,
                    format!("{:?} not allowed in this context", self.token.token_type),
                );
            }
        }
    }

    fn after_left_paren(&mut self) -> Callorexpression {
        if self.paren_count == 1 {
            info!("found expresssion");
            self.advance("after_left_paren expression");
            let stuff: Stuff = self.parse_to_stuff();
            info!("done parsing expression {}", stuff);
            self.advance("after left paren expr");
            if self.token.token_type == TokenType::RightParen {
                info!("right paren found");
            } else {
                error(self.token.line, "right parenthesis expected");
            }
            self.advance("after left paren expr");
            info!("found express");
            let prints: bool = match self.token.token_type {
                TokenType::GreaterThanSymbol => true,
                TokenType::LessThanSymbol => false,
                _ => {
                    error(
                        self.token.line,
                        "greater than symbol or less than symbol expected",
                    );
                }
            };
            let new_line = if prints {
                match self.tokens[self.current_position].token_type {
                    TokenType::GreaterThanSymbol => {
                        self.advance("after left paren expr");
                        false
                    }
                    _ => true,
                }
            } else {
                false
            };
            warn!("{:?}", prints);
            Callorexpression::Expression(Expression {
                inside: stuff,
                print: prints,
                line: self.token.line,
                new_line,
            })
        } else {
            self.advance("after left paren");
            if self.token.token_type == TokenType::New {
                self.advance("after left paren");
            }
            let keyword: TokenType = self.token.token_type.clone();
            let line: i32 = self.token.line;
            info!("found call {}", keyword);
            self.advance("after left paren");
            let mut args = Vec::new();
            while self.token.token_type != TokenType::RightParen {
                info!("looking for args");
                args.push(self.parse_to_stuff());
                self.advance("after left paren");
            }
            Callorexpression::Call(Call {
                keyword,
                arguments: args,
                line,
            })
        }
    }

    fn var(&mut self, name: String) -> IdentifierPointer {
        if name.starts_with('$') && self.in_function {
            if self.tokens[self.current_position].token_type == TokenType::With {
                error(
                    self.tokens[self.current_position].line,
                    "function arguments are immutable",
                );
            } else {
                IdentifierPointer::new(name, self.token.line)
            }
        } else if self.tokens[self.current_position].token_type == TokenType::Dot {
            self.advance("Var");
            self.advance("Var");
            if let TokenType::Car | TokenType::Cdr = self.token.token_type {
                // make a string with the name + . and
                info!("found dot {}", self.token.token_type);
                let name: String =
                    name + "." + &format!("{:?}", self.token.token_type).to_lowercase();
                IdentifierPointer::new(name, self.token.line)
            } else {
                error(self.token.line, "car or Cdr expected after dot");
            }
        } else {
            IdentifierPointer::new(name, self.token.line)
        }
    }
    fn get_value(&mut self) -> OtherStuff {
        match self.token.token_type.clone() {
            TokenType::Number { literal } => {
                OtherStuff::Literal(Literal::new_number(literal, self.token.line))
            }
            TokenType::String { literal } => {
                OtherStuff::Literal(Literal::new_string(literal, self.token.line))
            }
            TokenType::Hempty => OtherStuff::Literal(Literal::new_hempty(self.token.line)),
            TokenType::Boolean { value } => {
                OtherStuff::Literal(Literal::new_boolean(value, self.token.line))
            }
            TokenType::LeftParen => match self.after_left_paren() {
                Callorexpression::Expression(expression) => OtherStuff::Expression(expression),
                _ => error(
                    self.token.line,
                    "expression expected after left parenthesis, found call",
                ),
            },
            TokenType::Identifier { name } => OtherStuff::Identifier(self.var(name)),
            tokentype => {
                error(
                    self.token.line,
                    format!(
                        "identifier expected, after \"create\" found TokenType::{:?}",
                        tokentype
                    )
                    .as_str(),
                );
            }
        }
    }

    fn parse_to_stuff(&mut self) -> Stuff {
        info!("parsing stuff");
        match self.token.token_type.clone() {
            TokenType::LeftParen => {
                info!("found left paren");
                // self.advance("parse to stuff");
                match self.after_left_paren() {
                    Callorexpression::Call(call) => Stuff::Call(call),
                    Callorexpression::Expression(a) => error(
                        self.token.line,
                        format!("call expected after left parenthesis found {:?}", a),
                    ),
                }
            }
            TokenType::Number { literal } => {
                Stuff::Literal(Literal::new_number(literal, self.token.line))
            }
            TokenType::String { literal } => {
                Stuff::Literal(Literal::new_string(literal, self.token.line))
            }
            TokenType::Hempty => Stuff::Literal(Literal::new_hempty(self.token.line)),
            TokenType::Boolean { value } => {
                Stuff::Literal(Literal::new_boolean(value, self.token.line))
            }
            TokenType::Identifier { name } => Stuff::Identifier(self.var(name)),
            _ => {
                error(
                    self.token.line,
                    format!("{:?} not allowed in this context", self.token.token_type),
                );
            }
        }
    }

    fn parse_to_other_stuff(&mut self) -> OtherStuff {
        match self.token.token_type.clone() {
            TokenType::LeftParen => {
                // self.advance("parse to other stuff");
                match self.after_left_paren() {
                    Callorexpression::Expression(expression) => OtherStuff::Expression(expression),
                    _ => error(
                        self.token.line,
                        "expression expected after left parenthesis, found call",
                    ),
                }
            }
            TokenType::Number { literal } => {
                OtherStuff::Literal(Literal::new_number(literal, self.token.line))
            }
            TokenType::String { literal } => {
                OtherStuff::Literal(Literal::new_string(literal, self.token.line))
            }
            TokenType::Hempty => OtherStuff::Literal(Literal::new_hempty(self.token.line)),
            TokenType::Boolean { value } => {
                OtherStuff::Literal(Literal::new_boolean(value, self.token.line))
            }
            TokenType::Identifier { name } => OtherStuff::Identifier(self.var(name)),
            _ => {
                error(
                    self.token.line,
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

#[derive(PartialEq, Clone)]
pub enum Thing {
    // we have vairants for each type of token that has a value ie number or the name of an identifier
    Identifier(Identifier),
    Expression(Expression),
    Function(Function),
    IfStatement(IfStatement),
    LoopStatement(LoopStatement),
    Break(i32),
    Continue(i32),
    Return(Option<OtherStuff>, i32),
}

impl Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Thing::Expression(expression) => write!(f, "{}", expression),
            Thing::Identifier(s) => write!(f, "Identifier({})", s),
            Thing::Function(function) => write!(f, "{}", function),
            Thing::IfStatement(if_statement) => write!(f, "{}", if_statement),
            Thing::LoopStatement(loop_statement) => write!(f, "{}", loop_statement),
            Thing::Break(_) => write!(f, "Break"),
            Thing::Continue(_) => write!(f, "Continue"),
            Thing::Return(stuff, _) => write!(
                f,
                "Return{}",
                match stuff {
                    Some(stuff) => format!("({})", stuff),
                    None => String::from(""),
                }
            ),
        }
    }
}

impl fmt::Debug for Thing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Thing::Expression(expression) => write!(f, "{:?}", expression),
            Thing::Identifier(t) => write!(f, "[Identifier({}) at line: {}]", t, t.line),
            Thing::Function(function) => write!(f, "{:?}", function),
            Thing::IfStatement(if_statement) => write!(f, "{:?}", if_statement),
            Thing::LoopStatement(loop_statement) => write!(f, "{:?}", loop_statement),
            Thing::Break(line) => write!(f, "[Break at line: {}]", line),
            Thing::Continue(line) => write!(f, "[Continue at line: {}]", line),
            Thing::Return(stuff, line) => write!(
                f,
                "[Return{} at line: {}]",
                match stuff {
                    Some(stuff) => format!("({:?})", stuff),
                    None => String::from(""),
                },
                line
            ),
        }
    }
}
