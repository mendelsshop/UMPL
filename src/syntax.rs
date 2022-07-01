use std::fmt::{self, Debug};

use crate::error;
use crate::token::TokenType;
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
    // for i in Lexer::new(src.clone()).scan_tokens().to_vec() {
    //     println!("{}", i.token_type)
    // }
    parse_from_token(&mut Lexer::new(src).scan_tokens().to_vec())
}
#[derive(Debug, Clone)]
pub enum Tree<T> {
    Leaf(T),
    Branch(Vec<Tree<T>>),
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

fn parse_from_token(tokens: &mut Vec<Token>) -> Tree<Thing> {
    if tokens.is_empty() {
        error::error(0, "")
    }
    let token = tokens.remove(0);

    if token.token_type == TokenType::LeftParen {
        let mut stuff = Vec::new();
        while tokens[0].token_type != TokenType::RightParen {
            stuff.push(parse_from_token(tokens));
        }
        tokens.remove(0);

        Tree::Branch(stuff)
    } else if token.token_type == TokenType::RightParen {
        error::error(0, "");
        Tree::Leaf(Thing::Other(TokenType::Null))
    } else {
        // println!("{:?}", token.token_type);
        // let keywords = keywords::Keyword::new();
        // let keywords = keywords.keywords;
        // println!("{:?}", keywords);
        // if keywords.is_keyword(&token.token_type) {
        //     println!("{:?}", token);
        // }
        Tree::Leaf(atom(token))
    }
}
#[derive(Debug, Clone)]
pub enum Thing {
    Number(f64),
    String(String),
    Other(TokenType),
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
