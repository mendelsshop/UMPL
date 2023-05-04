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
    // check for special forms
    // ie (define (f x) (+ x 1)) def
    // ie (lambda (x) (+ x 1)) lambda
    // ie (begin (define x 1) (define y 2) (+ x y)) begin
    // ie (define x 1) var
    // otherwise apply
    // ie (+ 1 2) apply

    if let Some(expr) = exprs.first() {
        let expr = exprs.remove(0);
        match expr.expr {
            ExprKind::Symbol(sym) if "define" == sym.as_str() => {
                parse_define(exprs)
            }
            ExprKind::Symbol(sym) if "lambda" == sym.as_str() => {
                parse_lambda( exprs)
            }
            ExprKind::Symbol(sym) if "begin" == sym.as_str() => {
                parse_begin(exprs)
            }
            _ => {
                Expr {
                    expr: ExprKind::Apply(Box::new(expr), exprs),
                    state: State::Evaluated,
                    file: String::new(),
                }
            }
        }
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

pub fn parse_define(mut exprs: Vec<Expr>) -> Expr {
    // if the first expr is a symbol then it is a variable assignment
    // ie (define x 1)
    // if the first expr is a list then it is a function definition
    // ie (define (f x) (+ x 1))

    let expr = exprs.remove(0);
    match expr.expr {
        ExprKind::Symbol(s) => {
            let value = exprs.remove(0);
            Expr {
                expr: ExprKind::Var(s, Box::new(value)),
                state: State::Evaluated,
                file: String::new(),
            }
        }
        ExprKind::Apply(sym, args) => {
            let mut args = args;
            let sym = *sym;
            let name = match sym.expr {
                ExprKind::Symbol(s) => s,
                _ => panic!("Invalid function name"),
            };
    
            let body = exprs.remove(0);
            if !matches!(body.expr, ExprKind::Lambda(..)) {
                panic!("Invalid function body");
            }
            Expr {
                expr: ExprKind::Def(name, Box::new(body)),
                state: State::Evaluated,
                file: String::new(),
            }
            
        }
        _ => panic!("Invalid define"),
    }    
}

pub fn parse_lambda(mut exprs: Vec<Expr>) -> Expr {
    // lambda is a special form defined as
    // Lambda(
    //     fn(Vec<Expr>, Env) -> Expr,
    //     Vec<String>,
    // ),
    // ie (lambda (x) (+ x 1))
    // would be |exprs| exprs[0] + 1, ["x"]

    let args = match exprs.remove(0).expr {
        ExprKind::Apply(arg0 , args) => {
            let mut args= args.iter().map(|arg| match arg.expr {
                ExprKind::Symbol(s) => s,
                _ => panic!("Invalid lambda"),
            }).collect::<Vec<String>>();
    
            match arg0.expr {
                ExprKind::Symbol(s) => {
                    args.insert(0, s);
                }
                _ => panic!("Invalid lambda"),
            }
            args
        }
        _ => panic!("Invalid lambda"),
    };
    let body = exprs.remove(0);
    Expr {
        expr: ExprKind::Lambda(move |_, env| eval_expr(body, env, 0), args),
        state: State::Evaluated,
        file: String::new(),
    }
}
