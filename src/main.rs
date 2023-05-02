use std::{
    cell::RefCell,
    collections::HashMap,
    fmt,
    ops::{Add, Div, Mul, Sub},
    rc::Rc, f32::consts::E,
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

//     pub fn eval<'a>(&mut self, ast: Vec<Ast>) -> Vec<Expr<'a>> {
//         let mut result = Vec::new();
//         for expr in ast {
//             result.push(eval_expr(expr, Rc::new(RefCell::new(self.vars.clone())), 0));
//         }
//         result;
//         Vec::new()
//     }
// }

fn eval_expr<'a>(epr: Expr<'a>, vars: Rc<RefCell<HashMap<String, Expr<'a>>>>, ident: usize) -> Expr<'a> {
    println!("eval_expr");
    println!("epr: {:?}", epr);
    let indent = (0..ident).map(|_| " ").collect::<String>();
    
    let var1 = vars.clone();
   
    match epr.expr {
        // ExprType::Symbol(s) => self.vars.get(&s).unwrap(),
        // ExprType::Var(s, i) => self.vars.insert(s, i),
        // ExprType::Add(e1, e2) => || eval_expr(e1) + eval_expr(e2),
        // ExprType::Sub(e1, e2) => || eval_expr(e1) - eval_expr(e2),
        // ExprType::Mul(e1, e2) => || eval_expr(e1) * eval_expr(e2),
        // ExprType::Div(e1, e2) => || eval_expr(e1) / eval_expr(e2),
        ExprKind::Nil | ExprKind::Word(_) | ExprKind::Number(_) | ExprKind::Bool(_) => epr,
        ExprKind::Symbol(s) => {
            
            
           
            let borrow = &vars.borrow();
            
            
           println!("{indent}(symbol {s}", s = s);
        //    println!("{indent}(symbol {s} {:?})", borrow);
            let v = Box::leak(Box::new(borrow.get(&s).unwrap().clone()));
            println!("{indent}(symbol {s} {v})", s = s, v = v);
            (*v).clone().eval()
        }
        ExprKind::Var(s, i) => {
            vars.borrow_mut().insert(s, *i.clone());
                  Expr {
                    expr: ExprKind::Nil,
                    state: State::Evaluated,
                    file: "main.rs",
            }
        }
        // Ast::Add(e1, e2) =>
        //     {
        //         println!("{indent}(thunk +");
        //         let e1 = eval_expr(*e1, vars.clone(), ident + 1);
        //         let e2 = eval_expr(*e2, vars.clone(), ident + 1);
        //         println!("{indent})");
        //         e1 + e2
                
        //     },
        //     // state: State::Thunk(Box::new(|ident| {
        //     //     let indent = Box::leak(indent.into_boxed_str());
        //     //     println!("{indent}(thunk +");
        //     //     let expr1 = Expr::eval(eval_expr(*e1, vars, ident + 1));
        //     //     let expr2 = Expr::eval(eval_expr(*e2, var1, ident + 1));
        //     //     println!("{indent})");
        //     //     expr1 + expr2
        //     // }), ident),
           
        // // },
        // Ast::Sub(e1, e2) =>  {
        //     println!("{indent}(thunk -");
        //     let e1 = eval_expr(*e1, vars.clone(), ident + 1);
        //     let e2 = eval_expr(*e2, vars.clone(), ident + 1);
        //     println!("{indent})");
        //     e1 - e2
        // },
        // // Expr {
        // //     expr: ExprKind::Nil,
        // //     state: State::Thunk(Box::new(|ident| {
        // //         let indent = Box::leak(indent.into_boxed_str());
        // //         println!("{indent}(thunk -");
        // //         let expr1 = Expr::eval(eval_expr(*e1, vars, ident + 1));
        // //         let expr2 = Expr::eval(eval_expr(*e2, var1, ident + 1));
        // //         println!("{indent})");
        // //         expr1 - expr2
        // //     }), ident),
        // //     file: "main.rs",
        // // },
        // Ast::Mul(e1, e2) => {
        //     println!("{indent}(thunk *");
        //     let e1 = eval_expr(*e1, vars.clone(), ident + 1);
        //     let e2 = eval_expr(*e2, vars.clone(), ident + 1);
        //     println!("{indent})");
        //     e1 * e2
        // },
        // //  Expr {
        // //     expr:  ExprKind::Nil,
        // //     state: State::Thunk(Box::new(|ident| {
        // //         let indent = Box::leak(indent.into_boxed_str());
        // //         println!("{indent}(thunk *");
        // //         let expr1 = Expr::eval(eval_expr(*e1, vars, ident + 1));
        // //         let expr2 = Expr::eval(eval_expr(*e2, var1, ident + 1));
        // //         println!("{indent})");
        // //         expr1 * expr2
        // //     }), ident),
        // //     file: "main.rs",
        // // },
        // Ast::Div(e1, e2) =>  {
        //     println!("{indent}(thunk /");
        //     let e1 = eval_expr(*e1, vars.clone(), ident + 1);
        //     let e2 = eval_expr(*e2, vars.clone(), ident + 1);
        //     println!("{indent})");
        //     e1 / e2
        // },
        // // Expr {
        // //     expr: ExprKind::Nil,
        // //     state: State::Thunk(Box::new(|ident| {
        // //         let indent = Box::leak(indent.into_boxed_str());

        // //         println!("{indent}(thunk ");
        // //         let expr1 = Expr::eval(eval_expr(*e1, vars, ident + 1));
        // //         let expr2 = Expr::eval(eval_expr(*e2, var1, ident + 1));
        // //         println!("{indent})");
        // //         expr1 / expr2
        // //     }), ident),
        // //     file: "main.rs",
        // // },
        // Ast::Exp(e1) => {
        //     println!("{indent}(thunk **");
        //     eval_expr(Ast::Mul(e1.clone(), e1), vars, ident + 1)
        // }
        // Ast::Display(e1) => {
        //     // Expr {
        //     //     expr: ExprKind::Nil,
        //     //     state: State::Thunk(Box::new(|ident| {
        //     //         let indent = Box::leak(indent.into_boxed_str());
    
        //     //         println!("{indent}(thunk display");
        //     //         let expr1 = Expr::eval(eval_expr(*e1, vars, ident + 1));
        //     //         println!("{indent})");
        //     //         expr1
        //     //     }), ident),
        //     //     file: "main.rs",
        //     // }
        //     println!("{}", eval_expr(*e1, vars, ident + 1));
        //     Expr {
        //             expr: ExprKind::Nil,
        //             state: State::Evaluated,
        //             file: "main.rs",
        //     }

        // }
        ExprKind::Begin(exprs) => {
            let mut final_val = Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "main.rs",
            };
            for expr in exprs {
                final_val = eval_expr(expr, vars.clone(), ident);
            }
            final_val
        }
        ExprKind::Lambda(params, body) => Expr {
            expr: ExprKind::Lambda(params, body),
            state: State::Evaluated,
            file: "main.rs",
        },
     ExprKind::Apply(func, args) => {
            println!("{indent}(thunk apply");
            let params;
            let body;
            let func = eval_expr(*func, vars.clone(), ident);
            match func.expr {
                ExprKind::Lambda(p, b) => {
                    params = p;
                    body = b;
                }
                e => panic!("Not a lambda: {:?}", e),
            }
            let vars = vars.clone();
            for (param, arg) in params.iter().zip(args.iter()) {
                vars.borrow_mut().insert(param.clone(), Expr {expr: ExprKind::Lambda(vec![], Box::new(arg.clone())), state: State::Thunk(vars.clone()), file: "main.rs" }
            );
            }
            eval_expr(*body, vars, ident)
        }
        ExprKind::Def(name, lambda) => {
            if let ExprKind::Lambda(params, body) = lambda.expr {
                vars.borrow_mut().insert(name, Expr {expr: ExprKind::Lambda(params, body), state: State::Evaluated, file: "main.rs" });
            } else {
                panic!("Not a lambda");
            }
            Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "main.rs",
            }
        }
    }

}
#[derive(Clone, Debug)]
pub enum State <'a>
// <'a, F: FnOnce(usize) -> Expr<'a>> 
{
    // Thunk(Ast, usize, F),
    Thunk(Rc<RefCell<HashMap<String, Expr<'a>>>>),
    Evaluated,
}
#[derive(Clone, Debug)]
pub struct Expr<'a> {
    pub expr: ExprKind<'a>,
    pub state: State<'a>,
    file: &'a str,
}
pub struct Thunk<'a, F: ?Sized + Fn() -> Expr<'a>> {
    pub expr: Expr<'a>,
    pub state: State<'a>,
    pub file: &'a str,
    pub f: F,
}

pub struct ExprThunk <'a> {
    pub f: Thunk<'a, dyn Fn() -> Expr<'a> + 'a>,
}

impl <'a> ExprThunk <'a> {

    pub fn new(f: &Thunk<'a, dyn Fn() -> Expr<'a> + 'a>) ->Self {
        ExprThunk {
            f
        }
    }
    pub fn eval(&self) -> Expr<'a> {
        let f = &self.f;
        let expr = (f.f)();
        expr
    }
}

fn other() {
    ExprThunk {
        f: Thunk {
            expr: Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "main.rs",
            },
            state: State::Evaluated,
            file: "main.rs",
            f: || Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "main.rs",
            },
        },
    };
}
#[derive(Debug, Clone)]
pub enum ExprKind <'a> {
    Number(i32),
    Word(String),
    Bool(bool),
    Nil,
    // use real lambda takes nothing return expr (hypothetically)

    // Lambda(Box<dyn FnOnce() -> Expr<'a>>),
    Lambda(Vec<String>, Box<Expr<'a>>),
    Def(String, Box<Expr<'a>>),
    Begin(Vec<Expr<'a>>),
    Apply(Box<Expr<'a>>, Vec<Expr<'a>>),
    Symbol(String),
    Var(String, Box<Expr<'a>>),
}

impl<'a> ExprKind<'a> {
    pub fn get_number(&self) -> i32 {
        match self {
            ExprKind::Number(n) => *n,
            _ => panic!("Not a number"),
        }
    }
}

impl fmt::Display for ExprKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprKind::Number(n) => write!(f, "{}", n),
            ExprKind::Word(s) => write!(f, "{}", s),
            ExprKind::Bool(b) => write!(f, "{}", b),
            ExprKind::Nil => write!(f, "nil"),
            ExprKind::Lambda(params, body) => write!(f, "(lambda ({}) {:#?})", params.join(" "), body),
            ExprKind::Def(name, lambda) => write!(f, "(def {} {:#?})", name, lambda),
            ExprKind::Begin(exprs) => write!(f, "(begin {:#?})", exprs),
            ExprKind::Apply(func, args) => write!(f, "(apply {:#?} {:#?})", func, args),
            ExprKind::Symbol(s) => write!(f, "{}", s),
            ExprKind::Var(s, e) => write!(f, "(var {} {:#?})", s, e),
        }
    }
}

impl<'a> Expr<'a> {
    pub fn new(expr: i32) -> Expr<'a> {
        Expr {
            expr: ExprKind::Number(expr),
            state: State::Evaluated,
            file: "main.rs",
        }
    }

    pub fn eval(self) -> Self {
        let val = match self.state {
            State::Thunk(vars) => {
                let thunk = self.expr;
                let thunk = match thunk {
                    ExprKind::Lambda(params, body) => {
                        println!("(thunk lambda");
                        if params.len() > 0 {
                            panic!("expected 0 params, got {}", params.len());
                            
                        }
                        let app = Expr {
                            expr: ExprKind::Apply(body, vec![]),
                            state: State::Thunk(vars.clone()),
                            file: "main.rs",
                        };
                        eval_expr(app, vars, 0)
                    }
                    _ => panic!("Not a thunk"),
                };
                Expr {
                    expr: thunk.expr,
                    state: State::Evaluated,
                    file: "main.rs",
                }
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

impl<'a> Add for Expr<'a> {
    type Output = Expr<'a>;

    fn add(self, other: Expr<'a>) -> Expr<'a> {
        // get the value of the first expression
        let expr1 = Expr::eval(self).get_number();
        // get the value of the second expression
        let expr2 = Expr::eval(other).get_number();
        // return the sum of the two expressions
        Expr {
            expr: ExprKind::Number(expr1 + expr2),
            state: State::Evaluated,
            file: "main.rs",
        }
    }
}


impl<'a> Sub for Expr<'a> {
    type Output = Expr<'a>;

    fn sub(self, other: Expr<'a>) -> Expr<'a> {
        // get the value of the first expression
        let expr1 = Expr::eval(self).get_number();
        // get the value of the second expression
        let expr2 = Expr::eval(other).get_number();
        // return the sum of the two expressions
        Expr {
            expr: ExprKind::Number(expr1 - expr2),
            state: State::Evaluated,
            file: "main.rs",
        }
    }
}

impl<'a> Mul for Expr<'a> {
    type Output = Expr<'a>;

    fn mul(self, other: Expr<'a>) -> Expr<'a> {
        // get the value of the first expression
        let expr1 = Expr::eval(self).get_number();
        // get the value of the second expression
        let expr2 = Expr::eval(other).get_number();
        // return the sum of the two expressions
        Expr {
            expr: ExprKind::Number(expr1 * expr2),
            state: State::Evaluated,
            file: "main.rs",
        }
    }
}


impl<'a> Div for Expr<'a> {
    type Output = Expr<'a>;

    fn div(self, other: Expr<'a>) -> Expr<'a> {
        // get the value of the first expression
        let expr1 = Expr::eval(self).get_number();
        // get the value of the second expression
        let expr2 = Expr::eval(other).get_number();
        // return the sum of the two expressions
        Expr {
            expr: ExprKind::Number(expr1 / expr2),
            state: State::Evaluated,
            file: "main.rs",
        }
    }
}







impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.state {
            State::Thunk(_) => write!(f, "Expr: {}, file: {}", self.expr, self.file),
            State::Evaluated => write!(f, "Expr: {}, file: {}", self.expr, self.file),
        }
    }
}
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Ast<> {
//     Add(Box<Ast>, Box<Ast>),
//     Sub(Box<Ast>, Box<Ast>),
//     Mul(Box<Ast>, Box<Ast>),
//     Div(Box<Ast>, Box<Ast>),
//     Exp(Box<Ast>),
//     Display(Box<Ast>),

// }

// env.borrow_mut().insert("+".to_string(),  Expr {
//     expr: ExprKind::Lambda(vec!["x".to_string(), "y".to_string()], Box::new({
//         let x = eval_expr(Expr { expr: ExprKind::Symbol("x".to_string()), state: State::Evaluated, file: "main.rs" }, env.clone(), 0);
//         let y = eval_expr(Expr { expr: ExprKind::Symbol("y".to_string()), state: State::Evaluated, file: "main.rs" }, env.clone(), 0);
//         x + y

//     })),
//     state: State::Evaluated,
//     file: "main.rs",
// });

macro_rules! add_math_fn {
    ($symbol:literal, $op:tt, $env:expr) => {
        {
            let e = Expr {
                expr: ExprKind::Lambda(vec!["x".to_string(), "y".to_string()], Box::new({
                    // get this to be evaluated later, maybe my theoery is wrong about using ekprtype lambda to implement lazy evaluation
                    let x = eval_expr(Expr { expr: ExprKind::Symbol("x".to_string()), state: State::Evaluated, file: "main.rs" }, $env.clone(), 0);
                    let y = eval_expr(Expr { expr: ExprKind::Symbol("y".to_string()), state: State::Evaluated, file: "main.rs" }, $env.clone(), 0);
                    x $op y
    
                })),
                state: State::Evaluated,
                file: "main.rs",
            };
            $env.borrow_mut().insert($symbol.to_string(),  e);
        }
        
    };
}
fn main() {
    let ast = vec![
        Expr {
            expr: ExprKind::Number(1),
            state: State::Evaluated,
            file: "main.rs",
        },
    ];
    let env = Rc::new(RefCell::new(HashMap::new()));

    add_math_fn!("+", +, env);
    add_math_fn!("-", -, env);
    add_math_fn!("*", *, env);
    add_math_fn!("/", /, env);


    // expr.expr = 0
    let mut expr = Expr {
        expr: ExprKind::Nil,
        state: State::Evaluated,
        file: "main.rs",
    };

    for a in ast {
        println!("a: {}", a);
        expr = eval_expr(a, env.clone(), 0);
    
        println!("expr: {}", Expr::eval(expr));
    }
}
