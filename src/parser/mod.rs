pub(crate) mod rules;
use crate::{token::{Token,TokenType}, error, keywords};
use rules::{
    Call, Expression, Function, Identifier, IfStatement, Literal, LiteralType, LoopStatement,
    OtherStuff, Stuff, IdentifierPointer,
};

use std::{collections::HashMap,fmt::{self, Display}};



pub struct Parser {
    paren_count: usize,
    weird_bracket_count: usize,
    current_position: usize,
    tokens: Vec<Token>,
    token: Token,
    done: bool,
    in_function: bool,
    in_loop: bool,
    keywords: keywords::Keyword,
    variables: HashMap<String, Identifier>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
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
            keywords: keywords::Keyword::new(),
            variables: HashMap::new(),
        }
    }

    pub fn advance(&mut self) {
        match self.tokens[self.current_position].token_type {
            TokenType::Return { .. } => {
                if self.in_function {
                    self.token = self.tokens[self.current_position].clone();
                } else {
                    error::error(
                        self.tokens[self.current_position].line,
                        "Return statement outside of function",
                    );
                }
            }
            TokenType::Break | TokenType::Continue => {
                if self.in_loop {
                    self.token = self.tokens[self.current_position].clone();
                } else {
                    error::error(
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
                    error::error(
                        self.tokens[self.current_position].line,
                        "unmatched right parenthesis",
                    );
                }
                self.paren_count -= 1;
                if self.paren_count == 0
                    && !(vec![TokenType::GreaterThanSymbol, TokenType::LessThanSymbol]
                        .contains(&self.tokens[self.current_position + 1].token_type))
                {
                    error::error(
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
                    error::error(
                        self.tokens[self.current_position].line,
                        "greater than symbol (>) or less than symbol (<) not allowed in middle of expression",
                    );
                }
            }
            _ => {
                self.token = self.tokens[self.current_position].clone();
            }
        };
        println!("{}", self.paren_count); //
        println!("new token: {}", self.token);

        self.current_position += 1;
    }

    pub fn parse(&mut self) -> Vec<Thing> {
        let mut program: Vec<Thing> = Vec::new();
        // loop until we have no more self.tokens
        // in the loop, we use parse_from_tokens to parse the next expression
        // and add it to the program tree
        println!("{:?}", self.tokens);
        while !self.done {
            let expr = self.parse_from_token();
            match expr {
                Some(t) => {
                    program.push(t.clone());
                    println!("{:?}", t);
                }
                None => {}
            }
        }
        println!("Done parsing");
        program
    }
    fn parse_from_token(&mut self) -> Option<Thing> {
        println!("new iteration");
        if self.tokens.is_empty() {
            error::error(0, "no self.tokens found");
        }
        if self.done {
            return None;
        }
        self.advance();
        println!("PARSEfromTOKEN {}", self.token);
        match self.token.token_type.clone() {
            TokenType::LeftParen => self.after_left_paren(),
            TokenType::CodeBlockEnd => None,
            TokenType::Identifier { name } => {
                let temp = self.var(name.clone());
                match temp {
                    Some(value) => Some(Thing::IdentifierPointer(value)),
                    None => Some(Thing::Identifier(
                        self.variables.get(&name).unwrap().clone(),
                    )),
                }
            }
            keyword if self.keywords.is_keyword(&keyword) => {
                println!("found keyword {}", self.token.token_type);
                match self.token.token_type.clone() {
                    TokenType::Potato => {
                        self.advance();
                        match self.token.token_type.clone() {
                            TokenType::FunctionIdentifier { name } => {
                                println!("function identifier found");
                                self.advance();
                                // check if the next token is a number and save it in a vairable num_args
                                let num_args = match self.token.token_type {
                                    TokenType::Number { literal } => {
                                        if literal.trunc() == literal {
                                            self.advance();
                                            literal
                                        } else {
                                            error::error(
                                        self.token.line,
                                        format!("number expected in function declaration found floating point number literal with {}", literal).as_str(),
                                    );
                                        }
                                    }
                                    TokenType::CodeBlockBegin => 0f64,
                                    _ => {
                                        error::error(
                                    self.token.line,
                                    format!("number expected after function identifier, found {}", self.token).as_str(),
                                );
                                    }
                                };
                                println!("int function declaration before code block");
                                if self.token.token_type == TokenType::CodeBlockBegin {
                                    let mut function: Vec<Thing> = Vec::new();
                                    self.in_function = true;
                                    while self.token.token_type != TokenType::CodeBlockEnd {
                                        match self.parse_from_token() {
                                            Some(t) => function.push(t),
                                            None => {}
                                        }
                                    }
                                    self.in_function = false;
                                    Some(Thing::Function(Function::new(
                                        name,
                                        num_args,
                                        function,
                                        self.token.line,
                                    )))
                                } else {
                                    error::error(
                                self.token.line,
                                format!("code block expected after function identifier, found {}", self.token.token_type).as_str(),
                            );
                                }
                            }
                            tokentype => {
                                error::error(
                                self.token.line,
                                format!("function identifier expected after \"potato\", found TokenType::{:?}", tokentype).as_str(),
                            );
                            }
                        }
                    }
                    TokenType::List => {
                        self.advance();
                        match self.token.token_type.clone() {
                            TokenType::Identifier { name } => {
                                println!("list identifier found");
                                self.advance();
                                if self.token.token_type == TokenType::With {
                                    self.advance();
                                    if self.token.token_type == TokenType::LeftBracket {
                                        let thing = OtherStuff::from_thing(
                                            self.parse_from_token().unwrap(),
                                        );
                                        let thing1 = OtherStuff::from_thing(
                                            self.parse_from_token().unwrap(),
                                        );
                                        self.advance();
                                        if self.token.token_type == TokenType::RightBracket {
                                            self.variables.insert(
                                                name.clone(),
                                                Identifier::new(
                                                    name.clone(),
                                                    vec![thing.clone(), thing1.clone()],
                                                    self.token.line,
                                                ),
                                            );
                                            Some(Thing::Identifier(Identifier::new(
                                                name,
                                                vec![thing, thing1],
                                                self.token.line,
                                            )))
                                        } else {
                                            error::error(
                                                self.token.line,
                                                format!(
                                                    "right bracket expected after list, found {}",
                                                    self.token.token_type
                                                )
                                                .as_str(),
                                            );
                                        }
                                    } else {
                                        error::error(
                                            self.token.line,
                                            format!(
                                                "left bracket expected after \"with\", found {}",
                                                self.token.token_type
                                            )
                                            .as_str(),
                                        );
                                    }
                                } else {
                                    error::error(
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
                                error::error(
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
                        self.advance();
                        match self.token.token_type.clone() {
                            TokenType::Identifier { name } => {
                                println!("create identifier found");
                                self.advance();
                                if self.token.token_type == TokenType::With {
                                    self.advance();

                                    println!("create identifier with {}", self.token.token_type);
                                    let thing: OtherStuff = match self.token.token_type.clone() {
                                        TokenType::Number { literal } => OtherStuff::Literal(
                                            Literal::new_number(literal, self.token.line),
                                        ),
                                        TokenType::String { literal } => OtherStuff::Literal(
                                            Literal::new_string(literal, self.token.line),
                                        ),
                                        TokenType::Null => {
                                            OtherStuff::Literal(Literal::new_null(self.token.line))
                                        }
                                        TokenType::Boolean { value } => OtherStuff::Literal(
                                            Literal::new_boolean(value, self.token.line),
                                        ),
                                        TokenType::LeftParen => {
                                            OtherStuff::from_thing(self.after_left_paren().unwrap())
                                        }
                                        TokenType::Identifier { name } => {
                                            OtherStuff::Identifier(self.var(name).unwrap())
                                        }
                                        tokentype => {
                                            error::error(
                                        self.token.line,
                                        format!(
                                            "identifier expected, after \"create\" found TokenType::{:?}",
                                            tokentype
                                        )
                                        .as_str(),
                                    );
                                        }
                                    };
                                    self.variables.insert(
                                        name.clone(),
                                        Identifier::new(
                                            name.clone(),
                                            vec![thing.clone()],
                                            self.token.line,
                                        ),
                                    );
                                    Some(Thing::Identifier(Identifier::new(
                                        name,
                                        vec![thing],
                                        self.token.line,
                                    )))
                                } else {
                                    error::error(
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
                                error::error(
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
                        println!("loop found");
                        self.advance();
                        let mut loop_body: Vec<Thing> = Vec::new();
                        if self.token.token_type == TokenType::CodeBlockBegin {
                            self.in_loop = true;
                            while self.token.token_type != TokenType::CodeBlockEnd {
                                println!("parsing loop body");
                                match self.parse_from_token() {
                                    Some(t) => loop_body.push(t),
                                    None => {}
                                }
                            }
                            println!("Done parsing loop body");
                            self.in_loop = false;
                            Some(Thing::LoopStatement(LoopStatement::new(
                                loop_body,
                                self.token.line,
                            )))
                        } else {
                            error::error(self.token.line, "code block expected after \"loop\"");
                        }
                    }
                    TokenType::If => {
                        self.advance();
                        if self.token.token_type == TokenType::LeftBrace {
                            println!("if statement");
                            self.advance();
                            let mut if_body: Vec<Thing>;
                            let mut else_body: Vec<Thing>;
                            let thing: OtherStuff = match self.token.clone().token_type {
                                TokenType::Boolean { value } => OtherStuff::Literal(
                                    Literal::new_boolean(value, self.token.line),
                                ),
                                TokenType::LeftParen => {
                                    OtherStuff::from_thing(self.after_left_paren().unwrap())
                                }
                                TokenType::Identifier { name } => {
                                    OtherStuff::Identifier(self.var(name).unwrap())
                                }
                                tokentype => {
                                    error::error(
                                    self.token.line,
                                    format!(
                                        "boolean expected, in if statement condition found TokenType::{:?}",
                                        tokentype
                                    )
                                    .as_str(),
                                );
                                }
                            };
                            println!("after conditon if statement");
                            self.advance();
                            if self.token.token_type == TokenType::RightBrace {
                                self.advance();
                                if self.token.token_type == TokenType::CodeBlockBegin {
                                    if_body = Vec::new();
                                    while self.token.token_type != TokenType::CodeBlockEnd {
                                        match self.parse_from_token() {
                                            Some(token) => {
                                                if_body.push(token);
                                            }
                                            None => {}
                                        }
                                    }
                                    self.advance();
                                    if self.token.token_type == TokenType::Else {
                                        self.advance();
                                        if self.token.token_type == TokenType::CodeBlockBegin {
                                            println!("else found");
                                            else_body = Vec::new();
                                            while self.token.token_type != TokenType::CodeBlockEnd {
                                                match self.parse_from_token() {
                                                    Some(x) => else_body.push(x),
                                                    None => {}
                                                }
                                            }
                                            println!("in else_body");

                                            Some(Thing::IfStatement(IfStatement::new(
                                                thing,
                                                if_body,
                                                else_body,
                                                self.token.line,
                                            )))
                                        } else {
                                            error::error(
                                                self.token.line,
                                                "code block expected after \"else\"",
                                            );
                                        }
                                    } else {
                                        error::error(
                                            self.token.line,
                                            "else keyword expected after if statement",
                                        );
                                    }
                                } else {
                                    error::error(
                                        self.token.line,
                                        "code block expected after \"if\"",
                                    );
                                }
                            } else {
                                error::error(
                                    self.token.line,
                                    "right brace expected after if condition",
                                );
                            }
                        } else {
                            error::error(
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
                            self.advance();
                            return Some(Thing::Other(
                                TokenType::Return {
                                    value: None
                                },
                                self.token.line,
                            ));
                        }
                        // TODO: capture the value returned if any
                        let thing = OtherStuff::from_thing(self.parse_from_token().unwrap());
                        Some(Thing::Other(
                            TokenType::Return {
                                value: Some(Box::new(thing)),
                            },
                            self.token.line,
                        ))
                    }
                    TokenType::Break => {
                        println!("break statement");
                        Some(Thing::Other(self.token.token_type.clone(), self.token.line))
                    }
                    TokenType::Continue => {
                        println!("continue statement");
                        Some(Thing::Other(self.token.token_type.clone(), self.token.line))
                    }
                    _ => {
                        error::error(
                            self.token.line,
                            "keyword not allowed in expression before left parenthesis",
                        );
                    }
                }
            }
            _ => {
                println!("found terminal token {}", self.token.token_type);
                Some(atom(self.token.clone()))
            }
        }
    }

    fn after_left_paren(&mut self) -> Option<Thing> {
        if self.paren_count == 1 {
            println!("found expresssion");
            let stuff = self.parse_from_token().unwrap();
            self.advance();
            if self.token.token_type == TokenType::RightParen {
                println!("right paren found");
            } else {
                error::error(self.token.line, "right parenthesis expected");
            }
            self.advance();
            println!("found express");
            let mut prints = false;
            match self.token.token_type {
                TokenType::GreaterThanSymbol => {
                    prints = true;
                }
                TokenType::LessThanSymbol => {
                    prints = false;
                }
                _ => {}
            }
            Some(Thing::Expression(Expression {
                inside: stuff.convert_to_stuff(),
                print: prints,
                line: self.token.line,
            }))
        } else {
            self.advance();
            if self.token.token_type == TokenType::New {
                self.advance();
            }
            let keyword = self.token.token_type.clone();
            let line = self.token.line;
            println!("found call");
            let mut args = Vec::new();
            while self.tokens[self.current_position].token_type != TokenType::RightParen {
                args.push(Stuff::from_thing(self.parse_from_token().unwrap()));
            }
            self.advance();
            Some(Thing::Call(Call {
                keyword,
                arguments: args,
                line,
            }))
        }
    }

    fn var(&mut self, name: String) -> Option<IdentifierPointer> {
        if name.starts_with('$') && self.in_function {
            if self.tokens[self.current_position].token_type == TokenType::With {
                error::error(
                    self.tokens[self.current_position].line,
                    "function arguments are immutable",
                );
            } else {
                Some(IdentifierPointer::new(name, self.token.line))
            }
        } else {
            match self.tokens[self.current_position].token_type {
                TokenType::Dot => {
                    self.advance();
                    self.advance();
                    match self.token.token_type {
                        TokenType::First | TokenType::Second => {
                            self.advance();
                            match self.token.token_type {
                                TokenType::With => {
                                    // do the samething as descried in other With block
                                    if self.paren_count > 0 {
                                        error::error(self.tokens[self.current_position].line, "variables cannot be mutated with keyword: \"with\" in an expression");
                                    } else {
                                        // check if the identifier is in variables
                                        // get new value
                                        // set the identifier with the name from string in variables to the new value
                                        // return none
                                        if !self.variables.contains_key(&name) {
                                            error::error(self.token.line, format!("variables {} not found, therefore it cannot be mutated", name).as_str());
                                        }
                                        println!("in with");
                                        self.advance();
                                        let mut thing_list: Option<OtherStuff> = None;
                                        if self.token.token_type == TokenType::LeftBracket {
                                            println!("left bracket");
                                            thing_list = match self.token.token_type.clone() {
                                                TokenType::Number { literal } => {
                                                    Some(OtherStuff::Literal(Literal::new_number(
                                                        literal,
                                                        self.token.line,
                                                    )))
                                                }
                                                TokenType::String { literal } => {
                                                    Some(OtherStuff::Literal(Literal::new_string(
                                                        literal,
                                                        self.token.line,
                                                    )))
                                                }
                                                TokenType::Null => Some(OtherStuff::Literal(
                                                    Literal::new_null(self.token.line),
                                                )),
                                                TokenType::Boolean { value } => {
                                                    Some(OtherStuff::Literal(Literal::new_boolean(
                                                        value,
                                                        self.token.line,
                                                    )))
                                                }
                                                TokenType::LeftParen => {
                                                    Some(OtherStuff::from_thing(
                                                        self.after_left_paren().unwrap(),
                                                    ))
                                                }
                                                TokenType::Identifier { name } => Some(
                                                    OtherStuff::Identifier(self.var(name).unwrap()),
                                                ),
                                                tokentype => {
                                                    error::error(
                                                self.token.line,
                                                format!(
                                                    "identifier expected, after \"create\" found TokenType::{:?}",
                                                    tokentype
                                                )
                                                .as_str(),
                                            );
                                                }
                                            };
                                            self.advance();
                                        }

                                        println!("{} get token", self.token);
                                        let thing: OtherStuff = match self.token.token_type.clone()
                                        {
                                            TokenType::Number { literal } => OtherStuff::Literal(
                                                Literal::new_number(literal, self.token.line),
                                            ),
                                            TokenType::String { literal } => OtherStuff::Literal(
                                                Literal::new_string(literal, self.token.line),
                                            ),
                                            TokenType::Null => OtherStuff::Literal(
                                                Literal::new_null(self.token.line),
                                            ),
                                            TokenType::Boolean { value } => OtherStuff::Literal(
                                                Literal::new_boolean(value, self.token.line),
                                            ),
                                            TokenType::LeftParen => OtherStuff::from_thing(
                                                self.after_left_paren().unwrap(),
                                            ),
                                            TokenType::Identifier { name } => {
                                                OtherStuff::Identifier(self.var(name).unwrap())
                                            }
                                            tokentype => {
                                                error::error(
                                            self.token.line,
                                            format!(
                                                "identifier expected, after \"create\" found TokenType::{:?}",
                                                tokentype
                                            )
                                            .as_str(),
                                        );
                                            }
                                        };
                                        match thing_list {
                                            Some(list) => {
                                                self.variables.insert(
                                                    name.clone(),
                                                    Identifier::new(
                                                        name,
                                                        vec![list, thing],
                                                        self.token.line,
                                                    ),
                                                );
                                            }
                                            None => {
                                                self.variables.insert(
                                                    name.clone(),
                                                    Identifier::new(
                                                        name,
                                                        vec![thing],
                                                        self.token.line,
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                    None
                                }
                                _ => {
                                    // make a string with the name + . and
                                    let name =
                                        name + "." + self.token.token_type.to_string().as_str();
                                    Some(IdentifierPointer::new(name, self.token.line))
                                }
                            }
                        }
                        _ => {
                            error::error(self.token.line, "first or second expected after dot");
                        }
                    }
                }
                TokenType::With => {
                    if self.paren_count > 0 {
                        error::error(
                            self.tokens[self.current_position].line,
                            "variables cannot be mutated with keyword: \"with\" in an expression",
                        );
                    } else {
                        // check if the identifier is in variables
                        // get new value
                        // set the identifier with the name from string in variables to the new value
                        // return none
                        if !self.variables.contains_key(&name) {
                            error::error(
                                self.token.line,
                                format!(
                                    "variables {} not found, therefore it cannot be mutated",
                                    name
                                )
                                .as_str(),
                            );
                        }
                        self.advance();
                        let mut thing_list: Option<OtherStuff> = None;
                        if self.token.token_type == TokenType::LeftBracket {
                            thing_list = match self.token.token_type.clone() {
                                TokenType::Number { literal } => Some(OtherStuff::Literal(
                                    Literal::new_number(literal, self.token.line),
                                )),
                                TokenType::String { literal } => Some(OtherStuff::Literal(
                                    Literal::new_string(literal, self.token.line),
                                )),
                                TokenType::Null => {
                                    Some(OtherStuff::Literal(Literal::new_null(self.token.line)))
                                }
                                TokenType::Boolean { value } => Some(OtherStuff::Literal(
                                    Literal::new_boolean(value, self.token.line),
                                )),
                                TokenType::LeftParen => {
                                    Some(OtherStuff::from_thing(self.after_left_paren().unwrap()))
                                }
                                TokenType::Identifier { name } => {
                                    Some(OtherStuff::Identifier(self.var(name).unwrap()))
                                }
                                tokentype => {
                                    error::error(
                                        self.token.line,
                                        format!(
                                    "identifier expected, after \"create\" found TokenType::{:?}",
                                    tokentype
                                )
                                        .as_str(),
                                    );
                                }
                            };
                        }
                        self.advance();
                        let thing: OtherStuff = match self.token.token_type.clone() {
                            TokenType::Number { literal } => {
                                OtherStuff::Literal(Literal::new_number(literal, self.token.line))
                            }
                            TokenType::String { literal } => {
                                OtherStuff::Literal(Literal::new_string(literal, self.token.line))
                            }
                            TokenType::Null => {
                                OtherStuff::Literal(Literal::new_null(self.token.line))
                            }
                            TokenType::Boolean { value } => {
                                OtherStuff::Literal(Literal::new_boolean(value, self.token.line))
                            }
                            TokenType::LeftParen => {
                                OtherStuff::from_thing(self.after_left_paren().unwrap())
                            }
                            TokenType::Identifier { name } => {
                                OtherStuff::Identifier(self.var(name).unwrap())
                            }
                            tokentype => {
                                error::error(
                                    self.token.line,
                                    format!(
                                "identifier expected, after \"create\" found TokenType::{:?}",
                                tokentype
                            )
                                    .as_str(),
                                );
                            }
                        };
                        match thing_list {
                            Some(list) => {
                                self.variables.insert(
                                    name.clone(),
                                    Identifier::new(name, vec![list, thing], self.token.line),
                                );
                            }
                            None => {
                                self.variables.insert(
                                    name.clone(),
                                    Identifier::new(name, vec![thing], self.token.line),
                                );
                            }
                        }
                        None
                    }
                }
                _ => Some(IdentifierPointer::new(name, self.token.line)),
            }
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Thing {
    // we have vairants for each type of token that has a value ie number or the name of an identifier
    Literal(Literal),
    Identifier(Identifier),
    Expression(Expression),
    Function(Function),
    IdentifierPointer(IdentifierPointer),
    IfStatement(IfStatement),
    LoopStatement(LoopStatement),
    Call(Call),
    // make this into a custom struct

    // for the rest of the self.tokens we just have the token type and the line number
    Other(TokenType, i32),
}

impl Thing {
    pub fn new(token: Token) -> Thing {
        match token.token_type {
            TokenType::Number { literal } => Thing::Literal(Literal {
                literal: LiteralType::Number(literal),
                line: token.line,
            }),
            TokenType::String { literal } => Thing::Literal(Literal {
                literal: LiteralType::String(literal),
                line: token.line,
            }),
            TokenType::Boolean { value } => Thing::Literal(Literal {
                literal: LiteralType::Boolean(value),
                line: token.line,
            }),
            TokenType::Null => Thing::Literal(Literal {
                literal: LiteralType::Null,
                line: token.line,
            }),
            _ => Thing::Other(token.token_type, token.line),
        }
    }

    pub fn get_line(&self) -> i32 {
        match self {
            Thing::Literal(literal) => literal.line,
            Thing::Identifier(identifier) => identifier.line,
            Thing::Expression(expression) => expression.line,
            Thing::Function(function) => function.line,
            Thing::IfStatement(if_statement) => if_statement.line,
            Thing::LoopStatement(loop_statement) => loop_statement.line,
            Thing::Call(call) => call.line,
            Thing::Other(_, line) => *line,
            Thing::IdentifierPointer(identifier_pointer) => identifier_pointer.line,
        }
    }
    #[allow(dead_code)]
    fn get_tt(&self) -> TokenType {
        match self {
            Thing::Other(tt, _) => tt.clone(),
            _ => panic!("get_tt called on non-other thing"),
        }
    }

    fn convert_to_stuff(&self) -> Stuff {
        match self {
            Thing::Literal(literal) => Stuff::Literal(literal.clone()),
            Thing::IdentifierPointer(identifier) => Stuff::Identifier(identifier.clone()),
            Thing::Call(call) => Stuff::Call(call.clone()),
            other => error::error(
                other.get_line(),
                format!("convert_to_stuff called on non-literal: {:?}", other).as_str(),
            ),
        }
    }
}

impl Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Thing::Expression(expression) => write!(f, "{}", expression),
            Thing::Literal(literal) => write!(f, "{}", literal),
            Thing::Other(t, _) => write!(f, "{}", t),
            Thing::Identifier(s) => write!(f, "Identifier({})", s),
            Thing::Function(function) => write!(f, "{{{}}}", function),
            Thing::IfStatement(if_statement) => write!(f, "{}", if_statement),
            Thing::LoopStatement(loop_statement) => write!(f, "{}", loop_statement),
            Thing::Call(call) => write!(f, "{}", call),
            Thing::IdentifierPointer(identifier_pointer) => write!(f, "{}", identifier_pointer),
        }
    }
}

impl fmt::Debug for Thing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Thing::Expression(expression) => write!(f, "{:?}", expression),
            Thing::Literal(literal) => {
                write!(f, "[{:?} at line: {}]", literal.literal, literal.line)
            }
            Thing::Other(t, l) => write!(f, "[TokenType::{:?} at line: {}]", t, l),
            Thing::Identifier(t) => write!(f, "[Identifier({}) at line: {}]", t, t.line),
            Thing::Function(function) => write!(f, "{:?}", function),
            Thing::IfStatement(if_statement) => write!(f, "{:?}", if_statement),
            Thing::LoopStatement(loop_statement) => write!(f, "{:?}", loop_statement),
            Thing::Call(call) => write!(f, "{:?}", call),
            Thing::IdentifierPointer(identifier_pointer) => {
                write!(f, "{:?}", identifier_pointer)
            }
        }
    }
}
fn atom(token: Token) -> Thing {
    match token.token_type {
        TokenType::Number { literal } => Thing::Literal(Literal {
            literal: LiteralType::Number(literal),
            line: token.line,
        }),
        TokenType::String { literal } => Thing::Literal(Literal {
            literal: LiteralType::String(literal),
            line: token.line,
        }),
        TokenType::Boolean { value } => Thing::Literal(Literal {
            literal: LiteralType::Boolean(value),
            line: token.line,
        }),
        TokenType::Null => Thing::Literal(Literal {
            literal: LiteralType::Null,
            line: token.line,
        }),
        _ => Thing::Other(token.token_type, token.line),
    }
}
