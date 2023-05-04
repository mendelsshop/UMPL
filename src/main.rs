#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(clippy::use_self, rust_2018_idioms)]
#![allow(clippy::similar_names, clippy::missing_errors_doc)]
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::ast::{Expr, ExprKind, State};

mod ast;
mod parser;

#[derive(Debug, Clone, Default)]
pub struct Env {
    scope: Rc<RefCell<HashMap<String, Expr>>>,
    pub parent: Option<Box<Env>>,
}

impl Env {
    #[must_use]
    pub fn new() -> Self {
        Self {
            scope: Rc::new(RefCell::new(HashMap::new())),
            parent: None,
        }
    }

    pub fn insert(&self, key: String, val: Expr) {
        self.scope.borrow_mut().insert(key, val);
    }

    #[must_use]
    pub fn get(&self, key: &String) -> Option<Expr> {
        self.scope.borrow().get(key).cloned().or_else(|| {
            println!("in parent");
            self.parent.as_ref().and_then(|p| p.get(key))
        })
    }

    #[must_use]
    pub fn new_child(&self) -> Self {
        Self {
            scope: Rc::new(RefCell::new(HashMap::new())),
            parent: Some(Box::new(self.clone())),
        }
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nScope:\n")?;
        for (k, v) in self.scope.borrow().iter() {
            writeln!(f, "{k:?}: {v:?}")?;
        }
        if let Some(p) = &self.parent {
            write!(f, "Parent:{p}")?;
        }
        write!(f, "\n--end--\n")
    }
}

fn eval_expr(epr: Expr, vars: Env) -> Expr {
    match epr.expr {
        // case: self-evaluating
        ExprKind::Nil | ExprKind::Word(_) | ExprKind::Number(_) | ExprKind::Bool(_) => {
            println!("self-eval: {epr:?}");
            epr
        }
        // case: lookup
        ExprKind::Symbol(s) => {
            println!("lookup: {s}");
            vars.get(&s).unwrap()
        }
        // case: define variable
        ExprKind::Var(s, i) => {
            println!("define: {s}");
            let v = eval_expr(*i, vars.clone());
            vars.insert(s, v);
            Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "var.rs".to_string(),
            }
        }
        ExprKind::Begin(exprs) => {
            println!("begin");
            let mut final_val = Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "begin.rs".to_string(),
            };
            for expr in exprs {
                final_val = eval_expr(expr, vars.clone());
            }
            final_val
        }
        ExprKind::Lambda(proc, params) => Expr {
            expr: ExprKind::Lambda(proc, params),
            state: State::Evaluated,
            file: "lambda.rs".to_string(),
        },
        ExprKind::Def(name, lambda) => {
            println!("def: {name}");
            match lambda.expr {
                ExprKind::Lambda(proc, params) => {
                    vars.insert(
                        name,
                        Expr {
                            expr: ExprKind::Lambda(proc, params),
                            state: State::Evaluated,
                            file: "def-assign.rs".to_string(),
                        },
                    );
                }
                ExprKind::UserLambda(proc, params, closure) => {
                    let closure = closure.unwrap_or_else(|| vars.new_child());
                    vars.insert(
                        name,
                        Expr {
                            expr: ExprKind::UserLambda(proc, params, Some(closure)),
                            state: State::Evaluated,
                            file: "def-assign.rs".to_string(),
                        },
                    );
                }
                _ => panic!("Not a lambda: {lambda:?}"),
            }
            Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "def.rs".to_string(),
            }
        }
        ExprKind::UserLambda(proc, params, closure) => Expr {
            expr: ExprKind::UserLambda(
                proc,
                params,
                Some(closure.unwrap_or_else(|| vars.new_child())),
            ),
            state: State::Evaluated,
            file: "user-lambda.rs".to_string(),
        },

        ExprKind::Apply(func, args) => {
            let func = eval_expr(*func, vars.clone());
            match func.expr {
                ExprKind::Lambda(p, _) => {
                    let args = args
                        .into_iter()
                        .map(|epr| eval_expr(epr, vars.clone()))
                        .collect();
                    p(args, vars)
                }
                ExprKind::UserLambda(p, params, closure) => {
                    let env = closure.unwrap_or_else(|| vars.new_child());
                    args.into_iter()
                        .map(|epr| eval_expr(epr, vars.clone()))
                        .zip(params.into_iter())
                        .for_each(|epr| {
                            println!("adding to env: {epr:?}");
                            let (e, p) = epr;
                            env.insert(p, e);
                        });
                    eval_expr(*p, env)
                }
                // any literal or symbol should be evaluat
                e => panic!("Not a lambda: {e:?}"),
            }
        }
    }
}

macro_rules! add_math_fn {
    ($symbol:literal, $op:tt, $env:expr) => {
        {
            let v = vec![String::from("x"), String::from("y")];
            let e = Expr {
                expr: ExprKind::Lambda(|args, _env|
                    if args.len() != 2 {
                        panic!()
                    } else {

                        args[0].clone() $op args[1].clone()
                    }
             ,v),
                state: State::Evaluated,
                file: format!("{}.rs", $symbol),
            };
            $env.insert($symbol.to_string(),  e);
        }

    };
}
fn main() {
    let env = Env::new();

    add_math_fn!("+", +, env);
    add_math_fn!("-", -, env);
    add_math_fn!("*", *, env);
    add_math_fn!("/", /, env);
    let display = Expr {
        expr: ExprKind::Lambda(
            |args, _env| {
                println!("displaying {}", args[0]);
                Expr {
                    expr: ExprKind::Nil,
                    state: State::Evaluated,
                    file: "display.rs".to_string(),
                }
            },
            vec![String::from("x")],
        ),
        state: State::Evaluated,
        file: "display.rs".to_string(),
    };
    env.insert("display".to_string(), display);
    let cons_str = "(define (cons x y) (lambda (m) (m x y)))";
    let cons_expr = parser::parse(&mut cons_str.chars().peekable());
    eval_expr(cons_expr, env.clone());
    let cons_str = "(define (car z) (z (lambda (p q) p)))";
    let cons_expr = parser::parse(&mut cons_str.chars().peekable());
    eval_expr(cons_expr, env.clone());
    let ast = vec![
        parser::parse(&mut "(define x 5)".chars().peekable()),
        parser::parse(&mut "(+ x 5)".chars().peekable()),
        parser::parse(&mut "(define z (cons 1 2))".chars().peekable()),
        parser::parse(&mut "(car z)".chars().peekable()),
    ];
    let mut expr;

    for a in ast {
        expr = eval_expr(a, env.clone());

        println!(
            "expr: done: {}",
            eval_expr(eval_expr(expr, env.clone()), env.clone())
        );
    }
}
