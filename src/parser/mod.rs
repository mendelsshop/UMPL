mod rules;
use rules::{
    Call, Expression, Function, Identifier, IdentifierType, IfStatement, Literal, LiteralType,
    LoopStatement, OtherStuff, Stuff, Vairable,
};

use crate::token::Token;
use crate::token::TokenType;
use crate::{error, keywords};

use std::fmt::{self, Debug, Display};

pub trait Displays {
    fn to_strings(&self) -> String;
}
impl Displays for Vec<Tree<Thing>> {
    fn to_strings(&self) -> String {
        let mut s = String::new();
        for tree in self {
            match tree {
                Tree::Branch(thing) => s.push_str(format!("\n{}", thing.to_strings()).as_str()),
                Tree::Leaf(thing) => s.push_str(format!("{} ", thing).as_str()),
            }
        }
        s
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Tree<T> {
    Leaf(T),
    Branch(Vec<Tree<T>>),
}

impl Tree<Thing> {
    pub fn convert_to_stuff(&mut self) -> Tree<Stuff> {
        match self {
            Tree::Leaf(thing) => Tree::Leaf(match thing {
                Thing::Literal(literal) => Stuff::Literal(literal.clone()),
                Thing::Identifier(identifier) => Stuff::Identifier(Box::new(identifier.clone())),
                Thing::Call(call) => Stuff::Call(call.clone()),
                Thing::Other(tt, l) => match tt {
                    TokenType::Identifier { name } => {
                        Stuff::Identifier(Box::new(Identifier::new_empty(name.to_string(), *l)))
                    }
                    TokenType::Number { literal } => {
                        Stuff::Literal(Literal::new_number(*literal, *l))
                    }
                    TokenType::String { literal } => {
                        Stuff::Literal(Literal::new_string(literal.to_string(), *l))
                    }
                    TokenType::Boolean { value } => {
                        Stuff::Literal(Literal::new_boolean(*value, *l))
                    }
                    TokenType::Null => Stuff::Literal(Literal::new_null(*l)),
                    _ => error::error(
                        thing.get_line(),
                        format!(
                            "Thing {} is not a literal, identifier, or expression",
                            thing
                        )
                        .as_str(),
                    ),
                },
                _ => error::error(
                    thing.get_line(),
                    format!(
                        "Thing is not a literal, identifier, or Call: found {}",
                        thing
                    )
                    .as_str(),
                ),
            }),
            Tree::Branch(children) => {
                let mut new_children = Vec::new();
                for child in children {
                    new_children.push(child.convert_to_stuff());
                }
                Tree::Branch(new_children)
            }
        }
    }
}

pub struct Parser {
    paren_count: usize,
    weird_bracket_count: usize,
    current_position: usize,
    tokens: Vec<Token>,
    token: Token,
    done: bool,
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
        }
    }

    pub fn advance(&mut self) {
        match self.tokens[self.current_position].token_type {
            TokenType::EOF => {
                self.done = true;
            }
            TokenType::LeftParen => {
                self.paren_count += 1;
                self.token = self.tokens[self.current_position].clone();
            }
            TokenType::RightParen => {
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
                Some(t) => program.push(t),
                None => {}
            }
        }
        println!("Done parsing");
        program
    }
    fn parse_from_token(&mut self) -> Option<Thing> {
        if self.tokens.is_empty() {
            error::error(0, "no self.tokens found");
        }
        if self.done {
            return None;
        }
        self.advance();
        println!("PARSEfromTOKEN {}", self.token);
        if self.token.token_type == TokenType::LeftParen {
            println!("went down first if");
            // let mut stuff = Vec::new();
            // while self.token.token_type != TokenType::RightParen {
            //     let expr = self.parse_from_token();
            //     match expr {
            //         Some(t) => stuff.push(t),
            //         None => {}
            //     }
            // }
            let stuff = self.parse_from_token().unwrap();
            self.advance();
            if self.token.token_type == TokenType::RightParen {
                println!("right paren found");
            };
            self.advance();
            let mut prints = false;
            match self.token.token_type {
                TokenType::GreaterThanSymbol => {
                    println!("went down second if {}", self.paren_count);
                    prints = true;
                }
                TokenType::LessThanSymbol => {}
                _ => {}
            }
            Some(Thing::Expression(Expression {
                inside: stuff.convert_to_stuff(),
                print: prints,
                line: self.token.line,
            }))
        } else if self.token.token_type == TokenType::CodeBlockEnd {
            None
        } else {
            let keywords = keywords::Keyword::new();
            if keywords.is_keyword(&self.token.token_type) {
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
                                    self.advance();
                                    while self.token.token_type != TokenType::CodeBlockEnd {
                                        match self.parse_from_token() {
                                            Some(t) => function.push(t),
                                            None => {}
                                        }
                                    }
                                    self.advance();
                                    return Some(Thing::Function(Function::new(
                                        name,
                                        num_args,
                                        function,
                                        self.token.line,
                                    )));
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
                                    let thing: OtherStuff;
                                    println!("create identifier with {}", self.token.token_type);
                                    match self.token.token_type.clone() {
                                        TokenType::Number { literal } => {
                                            thing = OtherStuff::Literal(Literal::new_number(
                                                literal,
                                                self.token.line,
                                            ));
                                            self.advance();
                                        }
                                        TokenType::String { literal } => {
                                            thing = OtherStuff::Literal(Literal::new_string(
                                                literal,
                                                self.token.line,
                                            ));
                                            self.advance()
                                        }
                                        TokenType::Null => {
                                            thing = OtherStuff::Literal(Literal::new_null(
                                                self.token.line,
                                            ));
                                            self.advance();
                                        }
                                        TokenType::Boolean { value } => {
                                            thing = OtherStuff::Literal(Literal::new_boolean(
                                                value,
                                                self.token.line,
                                            ));
                                            self.advance();
                                        }
                                        TokenType::LeftParen => {
                                            println!("left paren found variable declaration");
                                            let z = self.parse_from_token().unwrap();
                                            println!("problem: {:?}", z);
                                            // check whether z contains greater than symbol or less than symbol
                                            // let prints: bool;
                                            let prints: bool = true;
                                            thing = OtherStuff::Expression(Expression::new(
                                                z.convert_to_stuff(),
                                                prints,
                                                self.token.line,
                                            ));
                                            println!("done");
                                        }
                                        TokenType::Identifier { name } => {
                                            thing = OtherStuff::Identifier(Identifier::new(
                                                name,
                                                IdentifierType::Vairable(Box::new(
                                                    Vairable::new_empty(self.token.line),
                                                )),
                                                self.token.line,
                                            ));
                                            self.advance();
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
                                    }

                                    return Some(Thing::Identifier(
                                        // TODO: get the actual value and don't just set it to null
                                        Identifier::new(
                                            name,
                                            IdentifierType::Vairable(Box::new(Vairable::new(
                                                thing,
                                            ))),
                                            self.token.line,
                                        ),
                                    ));
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
                            while self.token.token_type != TokenType::CodeBlockEnd {
                                println!("parsing loop body");
                                match self.parse_from_token() {
                                    Some(t) => loop_body.push(t),
                                    None => {}
                                }
                            }
                            println!("Done parsing loop body");
                            return Some(Thing::LoopStatement(LoopStatement::new(
                                loop_body,
                                self.token.line,
                            )));
                        } else {
                            error::error(self.token.line, "code block expected after \"loop\"");
                        }
                    }
                    TokenType::If => {
                        self.advance();
                        if self.token.token_type == TokenType::LeftBrace {
                            println!("if statement");
                            let thing: OtherStuff;
                            self.advance();
                            let mut if_body: Vec<Thing>;
                            let mut else_body: Vec<Thing>;
                            match &self.token.token_type {
                                TokenType::Number { literal } => {
                                    error::error(
                                    self.token.line,
                                    format!(
                                        "boolean expected, in if statement condition found number wit value {}",
                                        literal
                                    )
                                    .as_str(),
                                );
                                }
                                TokenType::String { literal } => {
                                    error::error(
                                    self.token.line,
                                    format!(
                                        "boolean expected, in if statement condition found string wit value {}",
                                        literal
                                    )
                                    .as_str(),
                                );
                                }
                                TokenType::Null => {
                                    error::error(
                                        self.token.line,
                                        "boolean expected, in if statement condition found null",
                                    );
                                }
                                TokenType::Boolean { value } => {
                                    thing = OtherStuff::Literal(Literal::new_boolean(
                                        *value,
                                        self.token.line,
                                    ));
                                    self.advance();
                                }
                                TokenType::LeftParen => {
                                    // let mut print;
                                    println!("left paren in if statement condition");
                                    let z = self.parse_from_token().unwrap();
                                    let prints: bool = true;
                                    println!("left paren found in if condition",);
                                    thing = OtherStuff::Expression(Expression::new(
                                        z.convert_to_stuff(),
                                        prints,
                                        self.token.line,
                                    ));
                                }
                                TokenType::Identifier { name } => {
                                    thing = OtherStuff::Identifier(Identifier::new(
                                        name.to_string(),
                                        IdentifierType::Vairable(Box::new(Vairable::new_empty(
                                            self.token.line,
                                        ))),
                                        self.tokens[0].line,
                                    ));
                                    self.advance();
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
                            }
                            println!("after conditon if statement");
                            if self.token.token_type == TokenType::RightBrace {
                                self.advance();
                                if self.token.token_type == TokenType::CodeBlockBegin {
                                    self.advance();
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
                                            self.advance();
                                            else_body = Vec::new();
                                            while self.token.token_type != TokenType::CodeBlockEnd {
                                                match self.parse_from_token() {
                                                    Some(x) => else_body.push(x),
                                                    None => {}
                                                }
                                            }
                                            println!("in else_body");
                                            self.advance();
                                            return Some(Thing::IfStatement(IfStatement::new(
                                                thing,
                                                if_body,
                                                else_body,
                                                self.token.line,
                                            )));
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
                    TokenType::Return => {
                        self.advance();
                        // return Tree::Leaf(Thing::Return(Return::new(self.tokens[0].line)));
                    }
                    TokenType::Break => {
                        println!("break statement");
                        return Some(Thing::Other(self.token.token_type.clone(), self.token.line));
                    }
                    TokenType::Continue => {
                        println!("continue statement");
                        return Some(Thing::Other(self.token.token_type.clone(), self.token.line));
                    }
                    keyword => {
                        println!("keyword");
                        let stuff: Vec<Stuff> = self.parse_stuff_from_tokens();
                        println!("after ");
                        return Some(Thing::Call(Call::new(stuff, self.token.line, keyword)));
                    }
                }
            }
            println!("found terminal token {}", self.token.token_type);
            Some(atom(self.token.clone()))
        }
    }

    fn parse_stuff_from_tokens(&mut self) -> Vec<Stuff> {
        let mut stuff: Vec<Stuff> = Vec::new();
        println!("{:?} convert self.tokens", self.token.token_type);
        while self.token.token_type != TokenType::RightParen {
            println!("{:?} conet self.tokens in looop", self.token.token_type);
            self.advance();
            match self.token.token_type.clone() {
                TokenType::String { ref literal } => {
                    stuff.push(Stuff::Literal(Literal::new_string(
                        literal.to_string(),
                        self.token.line,
                    )));
                }
                TokenType::Number { literal } => {
                    stuff.push(Stuff::Literal(Literal::new_number(
                        literal,
                        self.token.line,
                    )));
                }
                TokenType::Boolean { value } => {
                    stuff.push(Stuff::Literal(Literal::new_boolean(value, self.token.line)));
                }
                TokenType::Null => {
                    stuff.push(Stuff::Literal(Literal::new_null(self.token.line)));
                }
                TokenType::New => {
                    self.advance();
                    return self.parse_stuff_from_tokens();
                }
                TokenType::Identifier { name } => {
                    stuff.push(Stuff::Identifier(Box::new(Identifier::new(
                        name,
                        // TODO: get the actual value and don't just set it to null
                        IdentifierType::Vairable(Box::new(Vairable::new_empty(self.token.line))),
                        self.token.line,
                    ))));
                }
                TokenType::FunctionArgument { ref name } => {
                    stuff.push(Stuff::Identifier(Box::new(Identifier::new(
                        name.to_string(),
                        // TODO: get the actual value and don't just set it to null
                        IdentifierType::Vairable(Box::new(Vairable::new_empty(self.token.line))),
                        self.token.line,
                    ))));
                }
                TokenType::FunctionIdentifier { name } => {
                    stuff.push(Stuff::Identifier(Box::new(Identifier::new(
                        name.to_string(),
                        // TODO: get the actual value and don't just set it to null
                        IdentifierType::Vairable(Box::new(Vairable::new_empty(self.token.line))),
                        self.token.line,
                    ))));
                }
                a => {
                    error::error(
                        self.token.line,
                        format!("literal expected found: {}", a).as_str(),
                    );
                }
            }
        }
        stuff
    }
}

#[derive(PartialEq, Clone)]
pub enum Thing {
    // we have vairants for each type of token that has a value ie number or the name of an identifier
    Literal(Literal),
    Identifier(Identifier),
    Expression(Expression),
    Function(Function),

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
        }
    }
    fn get_tt(&self) -> TokenType {
        match self {
            Thing::Other(tt, _) => tt.clone(),
            _ => panic!("get_tt called on non-other thing"),
        }
    }

    fn convert_to_stuff(&self) -> Stuff {
        match self {
            Thing::Literal(literal) => Stuff::Literal(literal.clone()),
            Thing::Identifier(identifier) => Stuff::Identifier(Box::new(identifier.clone())),
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
        TokenType::Identifier { name } => {
            Thing::Identifier(Identifier::new_empty(name, token.line))
        }
        _ => Thing::Other(token.token_type, token.line),
    }
}
