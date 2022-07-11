mod rules;
use rules::{
    Call, Expression, Function, Identifier, IdentifierType, IfStatement, List, Literal,
    LiteralType, LoopStatement, OtherStuff, Stuff, Vairable,
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
    pub fn new(token: Token) -> Tree<Thing> {
        Tree::Leaf(Thing::new(token))
    }

    pub fn remove_leaf(self, tree: Thing) -> Option<Tree<Thing>> {
        match self.clone() {
            Tree::Branch(mut branches) => {
                for (num, branch) in branches.clone().iter().enumerate() {
                    let clon_branch = branch.clone().remove_leaf(tree.clone());
                    match clon_branch {
                        Some(new_branch) => {
                            branches[num] = new_branch;
                        }
                        None => {
                            branches.remove(num);
                        }
                    }
                }

                Some(Tree::Branch(branches))
            }
            Tree::Leaf(leaf) => match leaf {
                Thing::Other(tt, _) => {
                    if tt == tree.get_tt() {
                        println!("removing leaf");
                        drop(self);
                        None
                    } else {
                        Some(self)
                    }
                }
                _ => Some(self),
            },
        }
    }

    pub fn add_child(&mut self, child: Tree<Thing>) {
        match self {
            Tree::Leaf(thing) => {
                *self = Tree::Branch(vec![Tree::Leaf(thing.clone()), child]);
            }
            Tree::Branch(children) => {
                children.push(child);
            }
        }
    }

    pub fn convert_to_other_stuff(&self) -> Tree<OtherStuff> {
        match self {
            Tree::Leaf(thing) => match thing {
                Thing::Literal(literal) => Tree::Leaf(OtherStuff::Literal(literal.clone())),
                Thing::Identifier(identifier) => {
                    Tree::Leaf(OtherStuff::Identifier(identifier.clone()))
                }
                Thing::Expression(expression) => {
                    Tree::Leaf(OtherStuff::Expression(expression.clone()))
                }
                thing => error::error(
                    thing.get_line(),
                    format!(
                        "Thing {} is not a literal, identifier, or expression",
                        thing
                    )
                    .as_str(),
                ),
            },
            Tree::Branch(children) => {
                let mut new_children: Vec<Tree<OtherStuff>> = Vec::new();
                for child in children {
                    new_children.push(child.convert_to_other_stuff());
                }
                Tree::Branch(new_children)
            }
        }
    }

    pub fn check_gt_lt(self) -> (Option<bool>, Tree<Thing>) {
        let self_clone = self.clone();
        let leaf_gt = Thing::Other(TokenType::GreaterThanSymbol, 0);
        let leaf_lt = Thing::Other(TokenType::LessThanSymbol, 0);
        let self_clone = self_clone.remove_leaf(leaf_gt).unwrap();
        if self_clone == self {
            (Some(false), self.remove_leaf(leaf_lt).unwrap())
        } else {
            (Some(true), self.remove_leaf(leaf_lt).unwrap())
        }
    }

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
impl Display for Tree<Thing> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let mut level = 0;
        match self {
            Tree::Leaf(t) => write!(f, "{} ", t),
            Tree::Branch(t) => {
                write!(f, "{{ ",)?;
                for i in t {
                    write!(f, "{}", i)?;
                }
                write!(f, "}} ",)
            }
        }
    }
}

pub fn print(tree: Tree<Thing>, space: usize, levels: usize) -> usize {
    // TODO: make multiple branches in a branch print on the same line
    let mut level = space;
    let mut acm = levels;
    match tree {
        Tree::Leaf(t) => {
            print!("{} ", t);
            acm += format!("{} ", t).len();
            return acm;
        }
        Tree::Branch(v) => {
            println!("⫟");
            level += acm;
            print!("{}", space_for_level(acm));
            for i in v {
                acm = print(i, level, acm);
            }
        }
    }
    acm
}

fn space_for_level(level: usize) -> String {
    let mut s = String::new();
    for _ in 0..level {
        s.push(' ');
    }
    s
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
            }
            TokenType::RightParen => {
                self.paren_count -= 1;
            }
            TokenType::CodeBlockBegin => {
                self.weird_bracket_count += 1;
            }
            TokenType::CodeBlockEnd => {
                self.weird_bracket_count -= 1;
            }
            _ => {}
        };
        println!("{}", self.paren_count); //
        println!("new token: {}", self.token);
        self.token = self.tokens[self.current_position].clone();
        self.current_position += 1;
    }

    pub fn parse(&mut self) -> Vec<Tree<Thing>> {
        let mut program: Vec<Tree<Thing>> = Vec::new();
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
    fn parse_from_token(&mut self) -> Option<Tree<Thing>> {
        if self.tokens.is_empty() {
            error::error(0, "no self.tokens found");
        }
        if self.done {
            return Some(Tree::Leaf(Thing::Other(TokenType::EOF, self.token.line)));
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
            let mut stuff = self.parse_from_token().unwrap();
            self.advance();
            if self.token.token_type == TokenType::RightParen {
                println!("right paren found");
            };
            self.advance();
            let mut prints = false;
            match self.token.token_type {
                TokenType::GreaterThanSymbol => {
                    println!("went down second if {}", self.paren_count);
                    if self.paren_count == 0 {
                        prints = true;
                    } else {
                        error::error(
                            self.token.line,
                            "greater than symbol (>) no allowed in middle of expression",
                        );
                    }
                }
                TokenType::LessThanSymbol => {
                    if self.paren_count == 0 {
                    } else {
                        error::error(
                            self.token.line,
                            "less than symbol (<) no allowed in middle of expression",
                        );
                    }
                }
                _ => {
                    if self.paren_count == 0 {
                        error::error(
                            self.token.line,
                            format!(
                                "greater than symbol (>) or less than symbol (<) expected found {}",
                                self.token
                            )
                            .as_str(),
                        );
                    };
                }
            }
            Some(Tree::Leaf(Thing::Expression(Expression {
                inside: stuff.convert_to_stuff(),
                print: prints,
                line: self.token.line,
            })))
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
                                    let mut function: Vec<Tree<Thing>> = Vec::new();
                                    self.advance();
                                    while self.token.token_type != TokenType::CodeBlockEnd {
                                        match self.parse_from_token() {
                                            Some(t) => function.push(t),
                                            None => {}
                                        }
                                    }
                                    self.advance();
                                    return Some(Tree::Leaf(Thing::Function(Function::new(
                                        name,
                                        num_args,
                                        function,
                                        self.token.line,
                                    ))));
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
                                            let mut z = self.parse_from_token().unwrap();
                                            println!("problem: {:?}", z);
                                            // check whether z contains greater than symbol or less than symbol
                                            let result = z.check_gt_lt();
                                            z = result.1;
                                            // let prints: bool;
                                            let prints: bool = match result.0 {
                                                Some(x) => x,
                                                None => {
                                                    error::error(
                                                self.token.line,
                                                "greater than symbol (>) or less than symbol (<) expected",
                                            );
                                                }
                                            };
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

                                    return Some(Tree::Leaf(Thing::Identifier(
                                        // TODO: get the actual value and don't just set it to null
                                        Identifier::new(
                                            name,
                                            IdentifierType::Vairable(Box::new(Vairable::new(
                                                thing,
                                            ))),
                                            self.token.line,
                                        ),
                                    )));
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
                        let mut loop_body: Vec<Tree<Thing>> = Vec::new();
                        if self.token.token_type == TokenType::CodeBlockBegin {
                            while self.token.token_type != TokenType::CodeBlockEnd {
                                println!("parsing loop body");
                                match self.parse_from_token() {
                                    Some(t) => loop_body.push(t),
                                    None => {}
                                }
                            }
                            println!("Done parsing loop body");
                            return Some(Tree::Leaf(Thing::LoopStatement(LoopStatement::new(
                                loop_body,
                                self.token.line,
                            ))));
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
                            let mut if_body: Vec<Tree<Thing>>;
                            let mut else_body: Vec<Tree<Thing>>;
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
                                        
                                        "boolean expected, in if statement condition found null"
                                    
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
                                    let mut z = self.parse_from_token().unwrap(); // might have to make a match statement instead of unwrap
                                                                                  // check whether z contains greater than symbol or less than symbol
                                    let result = z.check_gt_lt();
                                    z = result.1;
                                    // let prints: bool;
                                    let prints: bool = match result.0 {
                                        Some(x) => x,
                                        None => {
                                            error::error(
                                            self.token.line,
                                            "greater than symbol (>) or less than symbol (<) expected",
                                        );
                                        }
                                    };
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
                                            return Some(Tree::Leaf(Thing::IfStatement(
                                                IfStatement::new(
                                                    thing,
                                                    if_body,
                                                    else_body,
                                                    self.token.line,
                                                ),
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
                        return Some(Tree::Leaf(Thing::Other(
                            self.token.token_type.clone(),
                            self.token.line,
                        )));
                    }
                    TokenType::Continue => {
                        println!("continue statement");
                        return Some(Tree::Leaf(Thing::Other(
                            self.token.token_type.clone(),
                            self.token.line,
                        )));
                    }
                    keyword => {
                        println!("keyword");
                        let stuff: Vec<Stuff> = self.parse_stuff_from_tokens();
                        println!("after ");
                        return Some(Tree::Leaf(Thing::Call(Call::new(
                            stuff,
                            self.token.line,
                            keyword,
                        ))));
                    }
                }
            } else if (self.token.token_type == TokenType::GreaterThanSymbol
                || self.token.token_type == TokenType::LessThanSymbol)
                && self.paren_count != 0
            {
                error::error(self.token.line, "greater than symbol (>) or less than symbol (<) not allowed in middle of expression");
            }
            println!("found terminal token {}", self.token.token_type);
            Some(Tree::Leaf(atom(self.token.clone())))
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
    List(List),
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
            Thing::List(list) => list.line,
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
}

impl Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Thing::Expression(expression) => write!(f, "{}", expression),
            Thing::Literal(literal) => write!(f, "{}", literal),
            Thing::Other(t, _) => write!(f, "{}", t),
            Thing::Identifier(s) => write!(f, "Identifier({})", s),
            Thing::Function(function) => write!(f, "{{{}}}", function),
            Thing::List(list) => write!(f, "{}", list),
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
            Thing::List(list) => write!(f, "{:?}", list),
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
