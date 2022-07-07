mod rules;
use rules::{
    Expression, Function, Identifier, IfStatement, List, Literal, LiteralType, LoopStatement,
    Vairable, Call, IdentifierType, OtherStuff, Stuff
};

use crate::token::TokenType;
use crate::{error, keywords};
use crate::{lexer::Lexer, token::Token};

use std::fmt::{self, Debug};


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
                    "Thing is not a literal, identifier, or expression",
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

    pub fn convert_to_stuff(&mut self) -> Tree<Stuff> {
        match self {
            Tree::Leaf(thing) => {
                Tree::Leaf(match thing {
                    Thing::Literal(literal) => Stuff::Literal(literal.clone()),
                    Thing::Identifier(identifier) => {
                        Stuff::Identifier(Box::new(identifier.clone()))
                    }
                    // Thing::Call(call) => Stuff::Call(call),
                    _ => error::error(
                        thing.get_line(),
                        "Thing is not a literal, identifier, or Call",
                    ),
                })
            }
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
    } else {
        let keywords = keywords::Keyword::new();
        if keywords.is_keyword(&token.token_type) {
            match token.token_type.clone() {
                TokenType::Potato => match &tokens[0].token_type.clone() {
                    TokenType::FunctionIdentifier { name } => {
                        tokens.remove(0);
                        // check if the next token is a number and save it in a vairable num_args
                        let num_args = match tokens[0].token_type {
                            TokenType::Number { literal } => {
                                tokens.remove(0);
                                if literal.trunc() == literal {
                                    // tokens.remove(0);
                                    literal
                                } else {
                                    error::error(
                                        tokens[0].line,
                                        format!("number expected in function declaration found floating point number literal with {}", literal).as_str(),
                                    );
                                }
                            }
                            TokenType::CodeBlockBegin => 0f64,
                            _ => {
                                error::error(
                                    tokens[0].line,
                                    "number expected after function identifier",
                                );
                            }
                        };
                        if tokens[0].token_type == TokenType::CodeBlockBegin {
                            let mut function: Tree<Thing> =
                                Tree::new(Token::new(TokenType::Function, "", tokens[0].line));
                            // println!("{}", function);
                            tokens.remove(0);
                            while tokens[0].token_type != TokenType::CodeBlockEnd {
                                function.add_child(parse_from_token(tokens, paren_count));
                            }
                            tokens.remove(0);
                            return Tree::Leaf(Thing::Function(Function::new(
                                *name,
                                num_args,
                                Box::new(function),
                                tokens[0].line,
                            )));
                        } else {
                            error::error(
                                tokens[0].line,
                                "code block expected after function identifier",
                            );
                        }
                    }
                    tokentype => {
                        error::error(
                                tokens[1].line,
                                format!("function identifier expected after \"potato\", found TokenType::{:?}", tokentype).as_str(),
                            );
                    }
                },
                TokenType::List => match &tokens[0].token_type {
                    TokenType::Identifier { name } => {
                        tokens.remove(0);
                        if tokens[0].token_type == TokenType::With {
                            tokens.remove(0);
                        } else {
                            error::error(
                                tokens[0].line,
                                format!(
                                    "with keyword expected, found TokenType::{:?}",
                                    tokens[0].token_type
                                )
                                .as_str(),
                            );
                        }
                    }
                    tokentype => {
                        error::error(
                            tokens[1].line,
                            format!(
                                "identifier expected, after \"list\" found TokenType::{:?}",
                                tokentype
                            )
                            .as_str(),
                        );
                    }
                },
                TokenType::Create => match &tokens[0].token_type.clone() {
                    TokenType::Identifier { name } => {
                        tokens.remove(0);
                        if tokens[0].token_type == TokenType::With {
                            tokens.remove(0);
                            return Tree::Leaf(Thing::Identifier(
                                // TODO: get the actual value and don't just set it to null
                                Identifier::new(name.clone(),IdentifierType::Vairable(Box::new(Vairable::new_empty(tokens[0].line))), tokens[0].line),
                            ));
                        } else {
                            error::error(
                                tokens[0].line,
                                format!(
                                    "with keyword expected, found TokenType::{:?}",
                                    tokens[0].token_type
                                )
                                .as_str(),
                            );
                        }
                    }
                    tokentype => {
                        error::error(
                            tokens[1].line,
                            format!(
                                "identifier expected after \"create\", found TokenType::{:?}",
                                tokentype
                            )
                            .as_str(),
                        );
                    }
                },
                TokenType::Return => {
                    tokens.remove(0);
                    // return Tree::Leaf(Thing::Return(Return::new(tokens[0].line)));
                }
                keyword => {
                    let temp = parse_stuff_from_tokens(tokens, paren_count);
                    let stuff: Vec<Stuff> = temp.0;
                    paren_count = temp.1;
                    return Tree::Leaf(Thing::Call(Call::new(stuff, tokens[0].line, keyword,)));
                }
            }
        } else if token.token_type == TokenType::GreaterThanSymbol
            || token.token_type == TokenType::LessThanSymbol
        {
            error::error(token.line, "greater than symbol (>) or less than symbol (<) not allowed in middle of expression");
        }
        Tree::Leaf(atom(token))
    }
}

fn parse_stuff_from_tokens(tokens: &mut Vec<Token>, paren_count: usize) -> (Vec<Stuff>, usize) {
    let mut stuff: Vec<Stuff> = Vec::new();
    while tokens[0].token_type != TokenType::RightParen {
        let token = tokens.remove(0);
        match token.token_type {
            TokenType::String { ref literal } => {
                stuff.push(Stuff::Literal(Literal::new(
                    LiteralType::new_string(literal.to_string()),
                    token.line,
                )));
            }
            TokenType::Number { literal } => {
                stuff.push(Stuff::Literal(Literal::new(
                    LiteralType::new_number(literal),
                    token.line,
                )));
            }
            TokenType::Boolean { value } => {
                stuff.push(Stuff::Literal(Literal::new(
                    LiteralType::new_boolean(value),
                    token.line,
                )));
            }
            TokenType::Null => {
                stuff.push(Stuff::Literal(Literal::new(
                    LiteralType::new_null(),
                    token.line,
                )));
            }
            TokenType::New => {
                tokens.remove(0);
                return parse_stuff_from_tokens(tokens, paren_count);
            }
            TokenType::Identifier { name } => {
                stuff.push(Stuff::Identifier(Box::new(Identifier::new(
                    name,
                    // TODO: get the actual value and don't just set it to null
                    IdentifierType::Vairable(Box::new(Vairable::new_empty(token.line))),
                    token.line,
                ))));
            }
            TokenType::FunctionArgument { ref name } => {
                stuff.push(Stuff::Identifier(Box::new(Identifier::new(
                    name.to_string(),
                    // TODO: get the actual value and don't just set it to null
                    IdentifierType::Vairable(Box::new(Vairable::new_empty(token.line))),
                    token.line,
                ))));
            }
            TokenType::FunctionIdentifier { name }  => {
                stuff.push(Stuff::Identifier(Box::new(Identifier::new(
                    name.to_string(),
                    // TODO: get the actual value and don't just set it to null
                    IdentifierType::Vairable(Box::new(Vairable::new_empty(token.line))),
                    token.line,
                ))));
            }
            a => {
                error::error(token.line, format!("literal expected {}", a).as_str());
            }
        }
    }
    (stuff, paren_count)
}

#[derive(Clone)]
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

    // for the rest of the tokens we just have the token type and the line number
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
}

impl fmt::Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Thing::Expression(expression) => write!(f, "{:?}", expression),
            Thing::Literal(literal) => write!(f, "{}", literal),
            Thing::Other(t, _) => write!(f, "{:?}", t),
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
        _ => Thing::Other(token.token_type, token.line),
    }
}
