use std::fmt::{self, Debug};

use crate::token::TokenType;
use crate::{error, keywords};
use crate::{lexer::Lexer, token::Token};

pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}
pub enum Expr {
    Literal {
        l_paren: Token,
        value: Literal,
        r_paren: Token,
        end: Token,
    },
}

pub fn parse(src: String) -> Tree<Thing> {
    let mut tokens = Lexer::new(src).scan_tokens().to_vec();
    let mut program: Tree<Thing> = Tree::new(Token::new(TokenType::Program, "", 0));
    // loop until we have no more tokens
    // in the loop, we use parse_from_tokens to parse the next expression
    // and add it to the program tree
    while !tokens.is_empty() {
        let expr = parse_from_token(&mut tokens, 0);
        program.add_child(expr);
    }
    program
}
#[derive(Debug, Clone)]
pub enum Tree<T> {
    Leaf(T),
    Branch(Vec<Tree<T>>),
}

impl Tree<Thing> {
    pub fn new(token: Token) -> Tree<Thing> {
        Tree::Leaf(Thing::new(token))
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
}
impl fmt::Display for Tree<Thing> {
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

fn parse_from_token(tokens: &mut Vec<Token>, mut paren_count: usize) -> Tree<Thing> {
    if tokens.is_empty() {
        error::error(55, "no tokens found");
    }
    let token = tokens.remove(0);
    if token.token_type == TokenType::LeftParen {
        paren_count += 1;
        let mut stuff = Vec::new();
        while tokens[0].token_type != TokenType::RightParen {
            stuff.push(parse_from_token(tokens, paren_count));
        }
        if tokens[0].token_type == TokenType::RightParen {
            paren_count -= 1;
            tokens.remove(0);
        };

        match tokens[0].token_type {
            TokenType::GreaterThanSymbol => {
                if paren_count == 0 {
                    stuff.push(Tree::Leaf(Thing::Other(
                        TokenType::GreaterThanSymbol,
                        tokens[0].line,
                    )));
                    tokens.remove(0);
                } else {
                    error::error(
                        tokens[0].line,
                        "greater than symbol (>) no allowed in middle of expression",
                    );
                }
            }
            TokenType::LessThanSymbol => {
                if paren_count == 0 {
                    stuff.push(Tree::Leaf(Thing::Other(
                        TokenType::LessThanSymbol,
                        tokens[0].line,
                    )));
                    tokens.remove(0);
                } else {
                    error::error(
                        tokens[0].line,
                        "less than symbol (<) no allowed in middle of expression",
                    );
                }
            }
            _ => {
                if paren_count == 0 {
                    error::error(
                        tokens[0].line,
                        "greater than symbol (>) or less than symbol (<) expected",
                    );
                };
            }
        }
        Tree::Branch(stuff)
    } else if token.token_type == TokenType::RightParen {
        error::error(token.line, "unmatched right parenthesis");
        Tree::Leaf(Thing::Other(TokenType::Null, token.line))
    } else {
        let keywords = keywords::Keyword::new();
        if keywords.is_keyword(&token.token_type) {
        } else if token.token_type == TokenType::GreaterThanSymbol
            || token.token_type == TokenType::LessThanSymbol
        {
            error::error(token.line, "greater than symbol (>) or less than symbol (<) not allowed in middle of expression");
        }
        Tree::Leaf(atom(token))
    }
}
#[derive(Clone)]
pub enum Thing {
    // we have vairants for each type of token that has a value ie number or the name of an identifier
    Number(f64, i32),
    String(String, i32),
    Identifier(String, i32),
    FunctionIdentifier(char, i32),
    // for the rest of the tokens we just have the token type and the line number
    Other(TokenType, i32),
}

impl Thing {
    pub fn new(token: Token) -> Thing {
        match token.token_type {
            TokenType::Number { literal } => Thing::Number(literal, token.line),
            TokenType::String { literal } => Thing::String(literal, token.line),
            TokenType::Identifier { name } => Thing::Identifier(name, token.line),
            TokenType::FunctionIdentifier { name } => Thing::FunctionIdentifier(name, token.line),
            _ => Thing::Other(token.token_type, token.line),
        }
    }
}

impl fmt::Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Thing::Number(n, _) => write!(f, "{}", n),
            Thing::String(s, _) => write!(f, "{}", s),
            Thing::Other(t, _) => write!(f, "{:?}", t),
            Thing::Identifier(s, _) => write!(f, "Identifier({})", s),
            Thing::FunctionIdentifier(s, _) => write!(f, "FunctionIdentifier({})", s),
        }
    }
}

impl fmt::Debug for Thing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Thing::Number(n, l) => write!(f, "Number({}) at line: {}", n, l),
            Thing::String(s, l) => write!(f, "String({}) at line: {}", s, l),
            Thing::Other(t, l) => write!(f, "TokenType::{:?} at line: {}", t, l),
            Thing::Identifier(t, l) => write!(f, "Identifier({}) at line: {}", t, l),
            Thing::FunctionIdentifier(t, l) => {
                write!(f, "FunctionIdentifier({}) at line: {}", t, l)
            }
        }
    }
}
fn atom(token: Token) -> Thing {
    match token.token_type {
        TokenType::Number { literal } => Thing::Number(literal, token.line),
        TokenType::String { literal } => Thing::String(literal, token.line),
        TokenType::Identifier { name } => Thing::Identifier(name, token.line),
        TokenType::FunctionIdentifier { name } => Thing::FunctionIdentifier(name, token.line),
        _ => Thing::Other(token.token_type, token.line),
    }
}
