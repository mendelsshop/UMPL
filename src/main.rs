use std::{
    cell::RefCell,
    collections::HashMap,
    fmt,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use crate::ast::{ExprKind, State, Expr};

mod parser;
mod ast;

#[derive(Debug, Clone)]
pub struct Env {
    scope: Rc<RefCell<HashMap<String, Expr>>>,
    pub parent: Option<Box<Env>>,
}

impl  Env {
    pub fn new() -> Env {
        Env {
            scope: Rc::new(RefCell::new(HashMap::new())),
            parent: None,
        }
    }

    pub fn insert(&self, key: String, val: Expr) {
        self.scope.borrow_mut().insert(key, val);
    }

    pub fn get(&self, key: &String) -> Option<Expr> {
        self.scope.borrow().get(key).map(|e| e.clone())
    }

    pub fn borrow(&self) -> std::cell::Ref<'_, std::collections::HashMap<std::string::String, Expr>> {
        self.scope.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<'_, std::collections::HashMap<std::string::String, Expr>> {
        self.scope.borrow_mut()
    }

    pub fn new_child(&self) -> Env {
        Env {
            scope: Rc::new(RefCell::new(HashMap::new())),
            parent: Some(Box::new(self.clone())),
        }
    }
    
}



fn eval_expr(
    epr: Expr,
    vars: Env,
    ident: usize,
) -> Expr {
    match epr.expr {
        // case: self-evaluating
        ExprKind::Nil | ExprKind::Word(_) | ExprKind::Number(_) | ExprKind::Bool(_) => {
            println!("self-eval: {:?}", epr);
            epr},
        // case: lookup
        ExprKind::Symbol(s) => {
            println!("lookup: {}", s);
            let borrow = &vars.borrow();

            // println!("{indent}(symbol {s}", s = s);
            //    println!("{indent}(symbol {s} {:?})", borrow);
            let expr = borrow.get(&s).unwrap();
            // println!("lookup:-done {:?}", expr);
            // println!("{indent}(symbol {s} {expr:?})", s = s, expr = expr);
            expr.clone()
        }
        // case: define variable
        ExprKind::Var(s, i) => {
            println!("define: {}", s);
            let v = eval_expr(*i, vars.clone(), ident);
            vars.borrow_mut().insert(s, v);
            Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "var.rs".to_string(),
            }
        }
        ExprKind::Begin(exprs) => {
            let mut final_val = Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "begin.rs".to_string(),
            };
            for expr in exprs {
                final_val = eval_expr(expr, vars.clone(), ident);
            }
            final_val
        }
        ExprKind::Lambda(proc, params) => Expr {
            expr: ExprKind::Lambda(proc, params),
            state: State::Evaluated,
            file: "lambda.rs".to_string(),
        },
        ExprKind::Def(name, lambda) => {
            println!("def: {}", name);
            if let ExprKind::Lambda(proc, params) = lambda.expr {
                vars.borrow_mut().insert(
                    name,
                    Expr {
                        expr: ExprKind::Lambda(proc, params),
                        state: State::Evaluated,
                        file: "def-assign.rs".to_string(),
                    },
                );
            } else {
                panic!("Not a lambda");
            }
            Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "def.rs".to_string(),
            }
        }

        ExprKind::Apply(func, args) => {
            // println!("{indent}(thunk apply {func:?}");

            let func = eval_expr(*func, vars.clone(), ident);
            // let (body, params) =
            println!("apply: {:?}", func);
            let s = match func.expr {
                
                ExprKind::Lambda(p, params) => {
                    
                    let args = args
                        .into_iter()
                        .map(|epr| eval_expr(epr, vars.clone(), ident))
                        .collect();
                    // println!("apply: {:?}", args);
                    p(args, vars.clone())
                }
                e => panic!("Not a lambda: {:?}", e),
            };
            println!("apply:-done {:?}", s);
            // if let ExprKind::Lambda(_, _) = s.expr {
            //     s
            // } else {
            //     eval_expr(s, vars.clone(), ident)
            // }
            s
        }
    }
}


// impl  fmt::Debug for ExprKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ExprKind::Number(n) => write!(f, "{}", n),
//             ExprKind::Word(s) => write!(f, "{}", s),
//             ExprKind::Bool(b) => write!(f, "{}", b),
//             ExprKind::Nil => write!(f, "nil"),
//             ExprKind::Lambda(proc, params) => {
//                 write!(f, "(lambda) {:?} of {:?}",proc, params)
//             }
//             ExprKind::Def(name, lambda) => write!(f, "(def {} {:#?})", name, lambda),
//             ExprKind::Begin(exprs) => {
//                 write!(f, "(begin {:#?})", exprs)
//             }
//             ExprKind::Apply(func, args) => {
//                 write!(f, "(apply {:#?} to {:#?})", func, args)
//             }
//             ExprKind::Symbol(s) => write!(f, "(symbol {})", s),
//             ExprKind::Var(s, i) => write!(f, "(var {} {})", s, i),
//         }
//     }
// }


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
            $env.borrow_mut().insert($symbol.to_string(),  e);
        }

    };
}
fn main() {
    // let ast = vec![
    //     Expr {
    //         expr: ExprKind::Var("x".to_string(), Box::new(Expr {
    //             expr: ExprKind::Number(1),
    //             state: State::Evaluated,
    //             file: "main.rs".to_string(),
    //         }),),
    //         state: State::Evaluated,
    //         file: "main.rs".to_string(),
    //     },
    //     Expr {
    //     expr: ExprKind::Apply(
    //         Box::new(Expr {
    //             expr: ExprKind::Symbol("+".to_string()),
    //             state: State::Evaluated,
    //             file: "main.rs".to_string(),
    //         }),
    //         vec![
    //             Expr {
    //                 expr: ExprKind::Apply(
    //                     Box::new(Expr {
    //                         expr: ExprKind::Symbol("/".to_string()),
    //                         state: State::Evaluated,
    //                         file: "main.rs".to_string(),
    //                     }),
    //                     vec![
    //                         Expr {
    //                             expr: ExprKind::Symbol("x".to_string()),
    //                             state: State::Evaluated,
    //                             file: "main.rs".to_string(),
    //                         },
    //                         Expr {
    //                             expr: ExprKind::Number(1),
    //                             state: State::Evaluated,
    //                             file: "main.rs".to_string(),
    //                         },
    //                     ],
    //                 ),
    //                 state: State::Evaluated,
    //                 file: "main.rs".to_string(),
    //             },
    //             Expr {
    //                 expr: ExprKind::Number(2),
    //                 state: State::Evaluated,
    //                 file: "main.rs".to_string(),
    //             },
    //         ],
    //     ),
    //     state: State::Evaluated,
    //     file: "main.rs".to_string(),
    // }];
    let env = Env::new();

    add_math_fn!("+", +, env);
    add_math_fn!("-", -, env);
    add_math_fn!("*", *, env);
    add_math_fn!("/", /, env);
    // add cons should be defined via a lambda ie (cons x y) = (lambda (m) (m x y))
    //((lambda (m) (m 1 2))
    // (define (cons x y)    (lambda (m) (m x y)))
    // (define cons (lambda (x y) (lambda (m) (m x y)))
    // cons is defined as a lambda that takes two arguments x and y
    // (define cons (lambda (x y)
    // and returns a lambda that takes one argument m
    // (lambda (m)
    // and returns the result of applying m to x and y
    // (m x y)))
    let v = vec![String::from("x"), String::from("y"), String::from("cons")];
    // tip you might want to use Env::from to create a new env from an existing one
    // so that we can emulate lexical scoping and closures
    // without using actual closures from rust ie FnOnce FnMut Fn
    let e = Expr {
        expr: ExprKind::Lambda(
            |args, env| {
                if args.len() != 2 {
                    panic!()
                } else {
                    let mut new_env = Env::from(env);
                    new_env.insert("x".to_string(), args[0].clone());
                    new_env.insert("y".to_string(), args[1].clone());
                    Expr {
                        expr: ExprKind::Lambda(
                            |args, env| {
                                if args.len() != 1 {
                                    panic!()
                                } else {
                                    let mut new_env = Env::from(env);
                                    new_env.insert("m".to_string(), args[0].clone());
                                    eval_expr(
                                    Expr {
                                        expr: ExprKind::Apply(
                                            Box::new(new_env.get(&"m".to_string()).unwrap().clone()),
                                            vec![
                                                new_env.get(&"x".to_string()).unwrap().clone(),
                                                new_env.get(&"y".to_string()).unwrap().clone(),
                                            ],
                                        ),
                                        state: State::Evaluated,
                                        file: "cons-inner.rs".to_string(),
                                    } , new_env, 0)

                                }
                            },
                            vec!["m".to_string(), "cons-inner.rs".to_string()],
                        ),
                        state: State::Evaluated,
                        file: "cons.rs".to_string(),
                    }
                }
            },
            v,
        ),
        state: State::Evaluated,
        file: "cons.rs".to_string(),
    };
    env.borrow_mut().insert("cons".to_string(), e);

    // car
    // (define (car x) (x (lambda (p q) p)))
    // we define car
    // (define car
    // as a lambda that takes one argument x
    // (lambda (x)
    // and applies x to a lambda that takes two arguments
    // (x (lambda (p q)
    // and returns the first argument
    // p)))

    let v = vec![String::from("x"), String::from("car")];
    let e = Expr {
        expr: ExprKind::Lambda(
            |args, env| {
                if args.len() != 1 {
                    panic!("improper number of args - car");
                } else {
                    let new_env = Env::from(env);
                    // new_env.insert("x".to_string(), args[0].clone());
                    println!("car");
                    let s = eval_expr(
                    Expr {
                        expr: ExprKind::Apply({
                            println!("car inner");
                            println!("args");
                            Box::new(args[0].clone())},
                            vec![
                                Expr {
                                    expr: ExprKind::Lambda(
                                        |args, _| {
                                            println!("car inner");
                                            println!("args: {:?}", args);
                                            if args.len() != 2 {
                                                panic!("improper number of args - car inner");
                                            } else {
                                               args[0].clone() 
                                            }
                                        },
                                        vec!["p".to_string(), "q".to_string(), "car-inner.rs".to_string()],
                                    ),
                                    state: State::Evaluated,
                                    file: "car-inner.rs".to_string(),
                                },
                            ],
                        ),
                        state: State::Evaluated,
                        file: "car.rs".to_string(),
                    } , new_env, 0);
                    println!("car s: {:?}", s);
                    s
                }
            },
            v,
        ),
        state: State::Evaluated,
        file: "car.rs".to_string(),
    };
    env.borrow_mut().insert("car".to_string(), e);

    // cdr
    // (define (cdr x) (x (lambda (p q) q)))
    let v = vec![String::from("x"), String::from("cdr")];
    let e = Expr {
        expr: ExprKind::Lambda(
            |args, env| {
                if args.len() != 1 {
                    panic!("improper number of args - cdr")
                } else {
                    let new_env = Env::from(env);
                    println!("cdr");
                    // new_env.insert("x".to_string(), args[0].clone());
                    eval_expr(
                    Expr {
                        expr: ExprKind::Apply(
                            Box::new(args[0].clone()),
                            vec![
                                Expr {
                                    expr: ExprKind::Lambda(
                                        |args, _env| {
                                            if args.len() != 2 {
                                                panic!("improper number of args - cdr inner")
                                            } else {
                                                
                                                  args[1].clone() 
                                            }
                                        },
                                        vec!["p".to_string(), "q".to_string(), "cdr-inner.rs".to_string()],
                                    ),
                                    state: State::Evaluated,
                                    file: "cdr-inner.rs".to_string(),
                                },
                            ],
                        ),
                        state: State::Evaluated,
                        file: "cdr.rs".to_string(),
                    }, new_env, 0)
                }
            },
            v,
        ),
        state: State::Evaluated,
        file: "cdr.rs".to_string(),
    };
    env.borrow_mut().insert("cdr".to_string(), e);

   
    let ast = vec![
        // make var (list) that has the value of (cons 1 2)
        Expr {
            expr: ExprKind::Var(
                "list".to_string(),
                Box::new(Expr {
                    expr: ExprKind::Apply(
                        Box::new(Expr {
                            expr: ExprKind::Symbol("cons".to_string()),
                            state: State::Evaluated,
                            file: "main.rs".to_string(),
                        }),
                        vec![
                            Expr {
                                expr: ExprKind::Number(1),
                                state: State::Evaluated,
                                file: "main.rs".to_string(),
                            },
                            Expr {
                                expr: ExprKind::Number(2),
                                state: State::Evaluated,
                                file: "main.rs".to_string(),
                            },
                        ],
                    ),
                    state: State::Evaluated,
                    file: "main.rs".to_string(),
                }),
            ),
            state: State::Evaluated,
            file: "main.rs".to_string(),
        },
        // // acces car of list
        // Expr {
        //     expr: ExprKind::Apply(
        //         Box::new(Expr {
        //             expr: ExprKind::Symbol("car".to_string()),
        //             state: State::Evaluated,
        //             file: "main.rs".to_string(),
        //         }),
        //         vec![Expr {
        //             expr: ExprKind::Symbol("list".to_string()),
        //             state: State::Evaluated,
        //             file: "main.rs".to_string(),
        //         }],
        //     ),
        //     state: State::Evaluated,
        //     file: "main.rs".to_string(),
        // },
        // // acces cdr of list
        // Expr {
        //     expr: ExprKind::Apply(
        //         Box::new(Expr {
        //             expr: ExprKind::Symbol("cdr".to_string()),
        //             state: State::Evaluated,
        //             file: "main.rs".to_string(),
        //         }),
        //         vec![Expr {
        //             expr: ExprKind::Symbol("list".to_string()),
        //             state: State::Evaluated,
        //             file: "main.rs".to_string(),
        //         }],
        //     ),
        //     state: State::Evaluated,
        //     file: "main.rs".to_string(),
        // },
        // add car and cdr
        Expr {
            expr: ExprKind::Apply(
                Box::new(Expr {
                    expr: ExprKind::Symbol("+".to_string()),
                    state: State::Evaluated,
                    file: "main.rs".to_string(),
                }),
                vec![
                    Expr {
                        expr: ExprKind::Apply(
                            Box::new(Expr {
                                expr: ExprKind::Symbol("car".to_string()),
                                state: State::Evaluated,
                                file: "main.rs".to_string(),
                            }),
                            vec![Expr {
                                expr: ExprKind::Symbol("list".to_string()),
                                state: State::Evaluated,
                                file: "main.rs".to_string(),
                            }],
                        ),
                        state: State::Evaluated,
                        file: "main.rs".to_string(),
                    },
                    Expr {
                        expr: ExprKind::Apply(
                            Box::new(Expr {
                                expr: ExprKind::Symbol("cdr".to_string()),
                                state: State::Evaluated,
                                file: "main.rs".to_string(),
                            }),
                            vec![Expr {
                                expr: ExprKind::Symbol("list".to_string()),
                                state: State::Evaluated,
                                file: "main.rs".to_string(),
                            }],
                        ),
                        state: State::Evaluated,
                        file: "main.rs".to_string(),
                    },
                ],
            ),
            state: State::Evaluated,
            file: "main.rs".to_string(),
        },
        // finished ast
    ];

    // expr.expr = 0
    let mut expr;

    for a in ast {
        // println!("a: {}", a);
        // print_env(env.clone());
        expr = eval_expr(a, env.clone(), 0);

        println!("expr: {}", eval_expr(eval_expr(expr, env.clone(), 0), env.clone(), 0));
    }

    println!("end main");
    let s = "(+ 1 2)";
    let exp = parser::parse(&mut s.chars().peekable());
    let expp = eval_expr(exp, env.clone(), 0);
    println!("expp: {}", expp);
}

// prints user defined variables/functions
fn print_env(env: Env) {
    println!("env:");
    for (k, v) in env.borrow().iter() {
        if k == "cons" || k == "car" || k == "cdr" || "+-*/".contains(k) {
            continue;
        }

        println!("{}: {}", k, v);
    }
    println!("end env");
}
