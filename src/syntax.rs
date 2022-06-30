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
    parse_from_token(&mut Lexer::new(src).scan_tokens().to_vec())
}
#[derive(Debug)]
pub enum Tree<T> {
    Leaf(T),
    Branch(Vec<Tree<T>>),
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
        return Tree::Branch(stuff);
    } else if token.token_type == TokenType::RightParen{
        error::error(0, ""); Tree::Leaf(Thing::Other(TokenType::Null))
    }
    else {
        return Tree::Leaf(atom(token));
    }

}
#[derive(Debug)]
pub enum Thing {
    Number(f64),
    String(String),
    Other(TokenType),
    // Ve(Vec<Tree<Thing>>)
}
fn atom(token: Token) -> Thing {
    match token.token_type { TokenType::Number { literal } => Thing::Number(literal),
    TokenType::String { literal } => Thing::String(literal),
    _ => Thing::Other(token.token_type)
}
}