use std::iter::Peekable;

use crate::ast::{Expr, ExprKind, State};

// parser/lexer combination - both should happen at the same time for each expr
// for basic scheme
// so like (+ 1 2) -> ExprKind::List(Expr{ExprKind::Symbol("+"), State::Evaluated}, [Expr{ExprKind::Number(1), State::Evaluated}, Expr{ExprKind::Number(2), State::Evaluated}])
// also support for lambda and define and begin aswell as variable assignment and strings and booleans

pub fn parse(chars: &mut Peekable<impl Iterator<Item = char>>) -> Expr {
    let peek = chars.peek().copied();
    peek.map_or_else(
        || panic!("No input"),
        |c| match c {
            '0'..='9' => parse_number(chars),

            '"' => parse_string(chars),
            '(' => parse_list(chars),
            ' ' | '\n' | '\t' => {
                chars.next();
                parse(chars)
            }
            _ => parse_symbol(chars),
        },
    )
}

fn parse_list(chars: &mut Peekable<impl Iterator<Item = char>>) -> Expr {
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
    // check for special forms
    // ie (define (f x) (+ x 1)) def
    // ie (lambda (x) (+ x 1)) lambda
    // ie (begin (define x 1) (define y 2) (+ x y)) begin
    // ie (define x 1) var
    // otherwise apply
    // ie (+ 1 2) apply

    if exprs.is_empty() {
        Expr {
            expr: ExprKind::List(vec![]),
            state: State::Evaluated,
            file: String::new(),
        }
    } else {
        let expr = exprs.remove(0);
        match expr.expr {
            ExprKind::Symbol(sym) if "define" == sym.as_str() => parse_define(exprs),
            ExprKind::Symbol(sym) if "lambda" == sym.as_str() => parse_lambda(exprs),
            ExprKind::Symbol(sym) if "begin" == sym.as_str() => parse_begin(exprs),
            ExprKind::Symbol(sym) if "set!" == sym.as_str() => parse_set(exprs),
            // (if pred consq alt)
            ExprKind::Symbol(sym) if "if" == sym.as_str() => parse_if(exprs),
            _ => Expr {
                expr: {
                    let mut exprs = exprs;
                    exprs.insert(0, expr);
                    ExprKind::List(exprs)
                },
                state: State::Evaluated,
                file: String::new(),
            },
        }
    }
}

fn parse_if(mut exprs: Vec<Expr>) -> Expr {
    let predicate = exprs.remove(0);
    let consequent = exprs.remove(0);
    let alternative = exprs.remove(0);
    // TODO: check that no more exprs in list or exprs
    Expr {
        expr: ExprKind::If(
            Box::new(predicate),
            Box::new(consequent),
            Box::new(alternative),
        ),
        state: State::Evaluated,
        file: String::new(),
    }
}

const fn parse_begin(exprs: Vec<Expr>) -> Expr {
    Expr {
        expr: ExprKind::Begin(exprs),
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
    while let Some(c) = chars.peek() {
        match c {
            '0'..='9' => {
                num.push(*c);
                chars.next();
            }
            _ => break,
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
    while let Some(c) = chars.peek() {
        match c {
            ' ' | ')' => break,
            _ => {
                sym.push(*c);
                chars.next();
            }
        }
    }
    Expr {
        expr: ExprKind::Symbol(sym),
        state: State::Evaluated,
        file: String::new(),
    }
}

pub fn parse_string(chars: &mut Peekable<impl Iterator<Item = char>>) -> Expr {
    chars.next();
    let mut string = String::new();
    // we don't need to use a loop here because we know that the string will end with a "
    // and that whatever calls this function should consume the closing " in this case
    for c in &mut *chars {
        if c == '"' {
            break;
        }
        string.push(c);
    }
    Expr {
        expr: ExprKind::Word(string),
        state: State::Evaluated,
        file: String::new(),
    }
}

pub fn parse_define(mut exprs: Vec<Expr>) -> Expr {
    // if the first expr is a symbol then it is a variable assignment
    // ie (define x 1)
    // were given x and 1 from parse_list
    // if the first expr is a list then it is a function definition
    // ie (define (f x) (+ x 1))
    // were given (f x) and (+ x 1) from parse_list
    let expr = exprs.remove(0);
    match expr.expr {
        // x
        ExprKind::Symbol(s) => {
            Expr {
                expr: ExprKind::Var(
                    s,
                    // 1
                    Box::new(exprs.remove(0)),
                ),
                state: State::Evaluated,
                file: String::new(),
            }
        }
        // (f x y z)
        ExprKind::List(mut args) => {
            let sym = args.remove(0);
            // f
            let ExprKind::Symbol(name) = sym.expr else { panic!("Invalid function name") };
            // x y z
            exprs.insert(
                0,
                Expr {
                    expr: ExprKind::List(args),
                    state: State::Evaluated,
                    file: String::new(),
                },
            );

            Expr {
                expr: ExprKind::Def(name, Box::new(parse_lambda(exprs))),
                state: State::Evaluated,
                file: String::new(),
            }
        }
        _ => panic!("Invalid define"),
    }
}

pub fn parse_lambda(mut exprs: Vec<Expr>) -> Expr {
    // lambda is a special form defined as
    // UserLambda(
    //     Box<Expr>, // body of the lambda which is an Expr { expr: ExprKind::Begin(..) }
    //     Vec<String>,
    // ),
    // ie (lambda (x) (+ x 1) (/ 1 2 3))
    // this function is given (x) (+ x 1) (/ 1 2 3) from parse_list or parse_begin
    // it can also be given () (+ 1 2) or (x y) (+ x y)
    // would be |exprs| exprs[0] + 1, ["x"]
    // x
    let args = match exprs.remove(0).expr {
        ExprKind::List(args) => args
            .into_iter()
            .map(|arg| match arg.expr {
                ExprKind::Symbol(s) => s,
                other => panic!("Invalid lambda {other:?}"),
            })
            .collect::<Vec<String>>(),
        bad => panic!("Invalid lambda {bad:?}"),
    };
    // (+ x 1) (/ 1 2 3)
    let body = parse_begin(exprs);
    Expr {
        expr: ExprKind::UserLambda(Box::new(body), args, None),
        state: State::Evaluated,
        file: String::new(),
    }
}

// (set! x 1)
// or (set! x (+ x 1))
pub fn parse_set(mut exprs: Vec<Expr>) -> Expr {
    // x
    let ExprKind::Symbol(var) = exprs.remove(0).expr else { panic!("Invalid set!") };
    // 1
    let val = exprs.remove(0);
    Expr {
        expr: ExprKind::Set(var, Box::new(val)),
        state: State::Evaluated,
        file: String::new(),
    }
}
