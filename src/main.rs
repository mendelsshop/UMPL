use std::{
    cell::RefCell,
    collections::HashMap,
    fmt,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

// pub struct eval {
//     vars: HashMap<String, ExprKind>,
// }

// impl eval {
//     pub fn new(vars: HashMap<String, i32>) -> eval {
//         eval {
//             vars: HashMap::new(),
//         }
//     }

//     pub fn eval(&mut self, ast: Vec<Ast>) -> Vec<Expr> {
//         let mut result = Vec::new();
//         for expr in ast {
//             result.push(eval_expr(expr, Rc::new(RefCell::new(self.vars.clone())), 0));
//         }
//         result;
//         Vec::new()
//     }
// }

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
        ExprKind::Nil | ExprKind::Word(_) | ExprKind::Number(_) | ExprKind::Bool(_) => epr,
        // case: lookup
        ExprKind::Symbol(s) => {
            let borrow = &vars.borrow();

            // println!("{indent}(symbol {s}", s = s);
            //    println!("{indent}(symbol {s} {:?})", borrow);
            let expr = borrow.get(&s).unwrap();
            // println!("{indent}(symbol {s} {expr:?})", s = s, expr = expr);
            expr.clone()
        }
        // case: define variable
        ExprKind::Var(s, i) => {
            vars.borrow_mut().insert(s, *i);
            Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "main.rs".to_string(),
            }
        }
        ExprKind::Begin(exprs) => {
            let mut final_val = Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "main.rs".to_string(),
            };
            for expr in exprs {
                final_val = eval_expr(expr, vars.clone(), ident);
            }
            final_val
        }
        ExprKind::Lambda(proc, params) => Expr {
            expr: ExprKind::Lambda(proc, params),
            state: State::Evaluated,
            file: "main.rs".to_string(),
        },
        ExprKind::Def(name, lambda) => {
            if let ExprKind::Lambda(proc, params) = lambda.expr {
                vars.borrow_mut().insert(
                    name,
                    Expr {
                        expr: ExprKind::Lambda(proc, params),
                        state: State::Evaluated,
                        file: "main.rs".to_string(),
                    },
                );
            } else {
                panic!("Not a lambda");
            }
            Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "main.rs".to_string(),
            }
        }

        ExprKind::Apply(func, args) => {
            // println!("{indent}(thunk apply {func:?}");

            let func = eval_expr(*func, vars.clone(), ident);
            // let (body, params) =
            let s = match func.expr {
                ExprKind::Lambda(p, params) => {
                    let args = args
                        .into_iter()
                        .map(|epr| eval_expr(epr, vars.clone(), ident))
                        .collect();
                    p(args, vars.clone())
                }
                e => panic!("Not a lambda: {:?}", e),
            };
            if let ExprKind::Lambda(_, _) = s.expr {
                s
            } else {
                eval_expr(s, vars.clone(), ident)
            }
        }
    }
}
#[derive(Clone, Debug)]
pub enum State
// <'a, F: FnOnce(usize) -> Expr>
{
    // Thunk(Ast, usize, F),
    Thunk(Env),
    Evaluated,
}
#[derive(Debug, Clone)]
pub struct Expr {
    pub expr: ExprKind,
    pub state: State,
    file: String,
}
#[derive(Clone)]
pub enum ExprKind {
    Number(i32),
    Word(String),
    Bool(bool),
    Nil,
    // use real lambda takes nothing return expr (hypothetically)

    // Lambda(Box<dyn FnOnce() -> Expr>),
    Lambda(
        fn(Vec<Expr>, Env) -> Expr,
        Vec<String>,
    ),
    Def(String, Box<Expr>),
    Begin(Vec<Expr>),
    Apply(Box<Expr>, Vec<Expr>),
    Symbol(String),
    Var(String, Box<Expr>),
}

impl  fmt::Debug for ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprKind::Number(n) => write!(f, "{}", n),
            ExprKind::Word(s) => write!(f, "{}", s),
            ExprKind::Bool(b) => write!(f, "{}", b),
            ExprKind::Nil => write!(f, "nil"),
            ExprKind::Lambda(proc, params) => {
                write!(f, "(lambda)  of {:?}", params)
            }
            ExprKind::Def(name, lambda) => write!(f, "(def {} {:#?})", name, lambda),
            ExprKind::Begin(exprs) => {
                write!(f, "(begin {:#?})", exprs)
            }
            ExprKind::Apply(func, args) => {
                write!(f, "(apply {:#?} to {:#?})", func, args)
            }
            ExprKind::Symbol(s) => write!(f, "(symbol {})", s),
            ExprKind::Var(s, i) => write!(f, "(var {} {})", s, i),
        }
    }
}

impl ExprKind {
    pub fn get_number(&self) -> i32 {
        match self {
            ExprKind::Number(n) => *n,
            _ => panic!("Not a number"),
        }
    }
}

impl fmt::Display for ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprKind::Number(n) => write!(f, "{}", n),
            ExprKind::Word(s) => write!(f, "{}", s),
            ExprKind::Bool(b) => write!(f, "{}", b),
            ExprKind::Nil => write!(f, "nil"),
            ExprKind::Lambda(proc, params) => {
                write!(f, "(lambda of {params:?}")
            }
            ExprKind::Def(name, lambda) => write!(f, "(def {} {:#?})", name, lambda),
            ExprKind::Begin(exprs) => write!(f, "(begin {:#?})", exprs),
            ExprKind::Apply(func, args) => write!(f, "(apply {:#?} {:#?})", func, args),
            ExprKind::Symbol(s) => write!(f, "{}", s),
            ExprKind::Var(s, e) => write!(f, "(var {} {:#?})", s, e),
        }
    }
}

impl Expr {
    pub fn new(expr: i32) -> Expr {
        Expr {
            expr: ExprKind::Number(expr),
            state: State::Evaluated,
            file: "main.rs".to_string(),
        }
    }

    pub fn eval(self) -> Self {
        let val = match self.state {
            State::Thunk(vars) => {
                // let thunk = self.expr;
                // let thunk = match thunk {
                //     ExprKind::Lambda(params, ) => {
                //         println!("(thunk lambda");

                //         eval_expr(params(vec![]), vars, 0)
                //     }
                //     _ => panic!("Not a thunk"),
                // };
                // Expr {
                //     expr: thunk.expr,
                //     state: State::Evaluated,
                //     file: "main.rs".to_string(),
                // }
                todo!()
            }
            State::Evaluated => self,
        };
        val
    }

    pub fn get_number(&self) -> i32 {
        match self.expr {
            ExprKind::Number(n) => n,
            _ => panic!("Not a number"),
        }
    }
}

impl Add for Expr {
    type Output = Expr;

    fn add(self, other: Expr) -> Expr {
        // get the value of the first expression
        let expr1 = Expr::eval(self).get_number();
        // get the value of the second expression
        let expr2 = Expr::eval(other).get_number();
        // return the sum of the two expressions
        Expr {
            expr: ExprKind::Number(expr1 + expr2),
            state: State::Evaluated,
            file: "main.rs".to_string(),
        }
    }
}

impl Sub for Expr {
    type Output = Expr;

    fn sub(self, other: Expr) -> Expr {
        // get the value of the first expression
        let expr1 = Expr::eval(self).get_number();
        // get the value of the second expression
        let expr2 = Expr::eval(other).get_number();
        // return the sum of the two expressions
        Expr {
            expr: ExprKind::Number(expr1 - expr2),
            state: State::Evaluated,
            file: "main.rs".to_string(),
        }
    }
}

impl Mul for Expr {
    type Output = Expr;

    fn mul(self, other: Expr) -> Expr {
        // get the value of the first expression
        let expr1 = Expr::eval(self).get_number();
        // get the value of the second expression
        let expr2 = Expr::eval(other).get_number();
        // return the sum of the two expressions
        Expr {
            expr: ExprKind::Number(expr1 * expr2),
            state: State::Evaluated,
            file: "main.rs".to_string(),
        }
    }
}

impl Div for Expr {
    type Output = Expr;

    fn div(self, other: Expr) -> Expr {
        // get the value of the first expression
        let expr1 = Expr::eval(self).get_number();
        // get the value of the second expression
        let expr2 = Expr::eval(other).get_number();
        // return the sum of the two expressions
        Expr {
            expr: ExprKind::Number(expr1 / expr2),
            state: State::Evaluated,
            file: "main.rs".to_string(),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.state {
            State::Thunk(_) => write!(f, "Expr: {}, file: {}", self.expr, self.file),
            State::Evaluated => write!(f, "Expr: {}, file: {}", self.expr, self.file),
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
                file: "main.rs".to_string(),
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
    let v = vec![String::from("x"), String::from("y")];
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
                                    Expr {
                                        expr: ExprKind::Apply(
                                            Box::new(new_env.get(&"m".to_string()).unwrap().clone()),
                                            vec![
                                                new_env.get(&"x".to_string()).unwrap().clone(),
                                                new_env.get(&"y".to_string()).unwrap().clone(),
                                            ],
                                        ),
                                        state: State::Evaluated,
                                        file: "main.rs".to_string(),
                                    }
                                }
                            },
                            vec!["m".to_string()],
                        ),
                        state: State::Evaluated,
                        file: "main.rs".to_string(),
                    }
                }
            },
            v,
        ),
        state: State::Evaluated,
        file: "main.rs".to_string(),
    };
    env.borrow_mut().insert("cons".to_string(), e);

    // car
    // (define (car x) (x (lambda (p q) p)))
    let v = vec![String::from("x")];
    let e = Expr {
        expr: ExprKind::Lambda(
            |args, env| {
                if args.len() != 1 {
                    panic!()
                } else {
                    let new_env = Env::from(env);
                    new_env.insert("x".to_string(), args[0].clone());
                    Expr {
                        expr: ExprKind::Apply(
                            Box::new(new_env.get(&"x".to_string()).unwrap().clone()),
                            vec![
                                Expr {
                                    expr: ExprKind::Lambda(
                                        |args, env| {
                                            if args.len() != 2 {
                                                panic!()
                                            } else {
                                                let new_env = Env::from(env);
                                                new_env.insert("p".to_string(), args[0].clone());
                                                new_env.insert("q".to_string(), args[1].clone());
                                                new_env.get(&"p".to_string()).unwrap().clone()
                                            }
                                        },
                                        vec!["p".to_string(), "q".to_string()],
                                    ),
                                    state: State::Evaluated,
                                    file: "main.rs".to_string(),
                                },
                            ],
                        ),
                        state: State::Evaluated,
                        file: "main.rs".to_string(),
                    }
                }
            },
            v,
        ),
        state: State::Evaluated,
        file: "main.rs".to_string(),
    };
    env.borrow_mut().insert("car".to_string(), e);

    // cdr
    // (define (cdr x) (x (lambda (p q) q)))
    let v = vec![String::from("x")];
    let e = Expr {
        expr: ExprKind::Lambda(
            |args, env| {
                if args.len() != 1 {
                    panic!()
                } else {
                    let new_env = Env::from(env);
                    new_env.insert("x".to_string(), args[0].clone());
                    Expr {
                        expr: ExprKind::Apply(
                            Box::new(new_env.get(&"x".to_string()).unwrap().clone()),
                            vec![
                                Expr {
                                    expr: ExprKind::Lambda(
                                        |args, env| {
                                            if args.len() != 2 {
                                                panic!()
                                            } else {
                                                let new_env = Env::from(env);
                                                new_env.insert("p".to_string(), args[0].clone());
                                                new_env.insert("q".to_string(), args[1].clone());
                                                new_env.get(&"q".to_string()).unwrap().clone()
                                            }
                                        },
                                        vec!["p".to_string(), "q".to_string()],
                                    ),
                                    state: State::Evaluated,
                                    file: "main.rs".to_string(),
                                },
                            ],
                        ),
                        state: State::Evaluated,
                        file: "main.rs".to_string(),
                    }
                }
            },
            v,
        ),
        state: State::Evaluated,
        file: "main.rs".to_string(),
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
        // acces car of list
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
        // acces cdr of list
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
