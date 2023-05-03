use std::iter::Peekable;

use crate::{ast::{Expr, ExprKind, State}, eval_expr};

// parser/lexer combination - both should happen at the same time for each expr
// for basic scheme 
// so like (+ 1 2) -> ExprKind::Apply(Expr{ExprKind::Symbol("+"), State::Evaluated}, [Expr{ExprKind::Number(1), State::Evaluated}, Expr{ExprKind::Number(2), State::Evaluated}])
// also support for lambda and define and begin aswell as variable assignment and strings and booleans

pub fn parse(chars: &mut Peekable<impl Iterator<Item = char>>) -> Expr {
    let peek = chars.peek();
    match peek {
        Some(c) => match c {
            '0'..='9' => parse_number(chars),
            
            '"' => parse_string(chars),
            '(' => 
            {
                pare_list(chars)
            }
            _ => parse_symbol(chars),
        },
        None => panic!("No input"),
        
    }

}

fn pare_list(chars: &mut Peekable<impl Iterator<Item = char>>) -> Expr {
    chars.next();
    let mut exprs = vec![];
    loop {
        match chars.peek() {
            Some(c) => match c {
                ')' => {
                    chars.next();
                    break;
                }
                ' ' => {
                    chars.next();
                    continue;
                }
                _ => exprs.push(parse(chars)),
            },
            None => panic!("No closing bracket"),
        }
    }
    // TODO:
    // check for special forms
    // ie (define (f x) (+ x 1)) def
    // ie (lambda (x) (+ x 1)) lambda
    // ie (begin (define x 1) (define y 2) (+ x y)) begin
    // ie (define x 1) var
    // otherwise apply
    // ie (+ 1 2) apply

    Expr {
        expr: ExprKind::Apply(Box::new(exprs[0].clone()), exprs[1..].to_vec()),
        state: State::Evaluated,
        file: String::new(),
    }
}

pub fn parse_number(chars: &mut Peekable<impl Iterator<Item = char>>) -> Expr {
    let mut num = String::new();
    // the reason why this is a loop instead of for
    // is because if we reach a non-number character
    // for will consume it
    // ie (+ 1 2)
    // with for loop the 2 will be consumed and then it will go again
    // and consume the closing bracket so when we return the expr back parse_list
    // it the iterator will be empty and we will panic because we expect a closing bracket
    loop {
        match chars.peek() {
            Some(c) => match c {
                '0'..='9' => {
                    num.push(*c);
                    chars.next();
                }
                _ => break,
            },
            None => break,
        }
    }
    Expr {
        expr: ExprKind::Number(num.parse::<i32>().unwrap()),
        state: State::Evaluated,
        file: String::new(),
    }
}

pub fn parse_symbol(chars: &mut Peekable<impl Iterator<Item = char>>) -> Expr {
    let mut sym = String::new();
    // see comment in parse_number
    loop {
        match chars.peek() {
            Some(c) => match c {
                ' ' => {
                    chars.next();
                    break;
                }
                ')' => {
                    break;
                }
                _ => {
                    sym.push(*c);
                    chars.next();
                }
            },
            None => break,
        }
    }
    Expr {
        expr: ExprKind::Symbol(sym),
        state: State::Evaluated,
        file: String::new(),
    }
}

pub fn parse_string(chars: &mut Peekable<impl Iterator<Item = char>>) -> Expr {
    let mut string = String::new();
    // we don't need to use a loop here because we know that the string will end with a "
    // and that whatever calls this function should consume the closing " in this case
    for c in chars {
        if c == '"' {
            break;
        } else {
            string.push(c);
        }
    }
    Expr {
        expr: ExprKind::Word(string),
        state: State::Evaluated,
        file: String::new(),
    }
}