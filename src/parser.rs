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
        error::error(0, "")
    }
    let token = tokens.remove(0);
    println!("f {} at level {}", token, paren_count);
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

        println!("a {} at level {}", tokens[0], paren_count);
        match tokens[0].token_type {
            // TODO: figure outr how to handle function which have multpiple expressions and we have extra parentheses level
            TokenType::GreaterThanSymbol => {
                if paren_count == 0 {
                    stuff.push(Tree::Leaf(Thing::Other(TokenType::GreaterThanSymbol)));
                    tokens.remove(0);
                } else {
                    error::error(12, "")
                }
            }
            TokenType::LessThanSymbol => {
                if paren_count == 0 {
                    stuff.push(Tree::Leaf(Thing::Other(TokenType::LessThanSymbol)));
                    tokens.remove(0);
                } else {
                    error::error(12, "")
                }
            }
            _ => {
                if paren_count == 0 {
                    error::error(12, "")
                };
            }
        }

        Tree::Branch(stuff)
    } else if token.token_type == TokenType::RightParen {
        error::error(0, "");
        Tree::Leaf(Thing::Other(TokenType::Null))
    } else {
        let keywords = keywords::Keyword::new();
        if keywords.is_keyword(&token.token_type) {
            // println!("{:?}", token);
        } else if token.token_type == TokenType::GreaterThanSymbol
            || token.token_type == TokenType::LessThanSymbol
        {
            error::error(1, "")
        }
        println!("{}", token);
        Tree::Leaf(atom(token))
    }
}
#[derive(Debug, Clone)]
pub enum Thing {
    Number(f64),
    String(String),
    Other(TokenType),
}

impl Thing {
    pub fn new(token: Token) -> Thing {
        match token.token_type {
            TokenType::Number { literal } => Thing::Number(literal),
            TokenType::String { literal } => Thing::String(literal),
            _ => Thing::Other(token.token_type),
        }
    }
}

impl fmt::Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Thing::Number(n) => write!(f, "{}", n),
            Thing::String(s) => write!(f, "{}", s),
            Thing::Other(t) => write!(f, "{}", t),
        }
    }
}
fn atom(token: Token) -> Thing {
    match token.token_type {
        TokenType::Number { literal } => Thing::Number(literal),
        TokenType::String { literal } => Thing::String(literal),
        _ => Thing::Other(token.token_type),
    }
}
