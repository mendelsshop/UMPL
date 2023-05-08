use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::{
    ast::{Expr, ExprKind, State},
    eval::{apply, eval_expr},
    parser,
};

// for + - * / % < > <= >= and or: eventually, we want to make these accept any number of arguments
macro_rules! add_binary_fn {
    ($symbol:literal, $env:expr, $res:expr, $op:tt, $mod:expr) => {{
        add_fn!(
            $symbol,
            $env,
            |args: Vec<Expr>| {
                Expr {
                    expr: $res($mod(&args[0]) $op $mod(&args[1])),
                    state: State::Evaluated,
                    file: format!("{}.rs", $symbol),
                }
            },
            2
        );
    }};
}

macro_rules! add_fn {
    ($symbol:literal, $env:expr, $todo:expr, $arg_count:literal) => {{
        let args = (0..$arg_count)
            .map(|i| format!("x{}", i))
            .collect::<Vec<String>>();
        let v = args.clone();
        let e = Expr {
            expr: ExprKind::Lambda(
                |args, _env| {
                    if args.len() != $arg_count {
                        panic!("Expected {} arguments, got {}", $arg_count, args.len());
                    } else {
                        // we have to use closures as opposed to whatver macro calling the args from |args, _env|
                        // because of macro hygiene
                        #[allow(clippy::redundant_closure_call)]
                        ($todo)(args)
                    }
                },
                v,
            ),
            state: State::Evaluated,
            file: format!("{}.rs", $symbol),
        };
        $env.insert($symbol.to_string(), e);
    }};
}

macro_rules! add_literal {
    ($symbol:literal, $expr:expr, $env:expr) => {{
        $env.insert(
            $symbol.to_string(),
            Expr {
                expr: $expr,
                state: State::Evaluated,
                file: format!("{}.rs", $symbol),
            },
        );
    }};
}

#[derive(Debug, Clone, Default)]
pub struct Env {
    scope: Rc<RefCell<HashMap<String, Expr>>>,
    pub parent: Option<Box<Env>>,
}

impl Env {
    #[must_use]
    pub fn new() -> Self {
        let env = Self {
            scope: Rc::new(RefCell::new(HashMap::new())),
            parent: None,
        };
        setup_envoirment(&env);
        env
    }

    pub fn insert(&self, key: String, val: Expr) {
        self.scope.borrow_mut().insert(key, val);
    }

    pub fn set(&self, key: String, val: Expr) {
        if self.scope.borrow().contains_key(&key) {
            self.scope.borrow_mut().insert(key, val);
        } else if let Some(parent) = &self.parent {
            parent.set(key, val);
        } else {
            panic!("Variable {key} not found");
        }
    }

    #[must_use]
    pub fn get(&self, key: &String) -> Option<Expr> {
        self.scope
            .borrow()
            .get(key)
            .cloned()
            .or_else(|| self.parent.as_ref().and_then(|p| p.get(key)))
    }

    #[must_use]
    pub fn new_child(&self) -> Self {
        Self {
            scope: Rc::new(RefCell::new(HashMap::new())),
            parent: Some(Box::new(self.clone())),
        }
    }
}

macro_rules! eval_to_env {
    ($definition:literal, $env:expr) => {
        eval_expr(
            parser::parse(&mut $definition.chars().peekable()),
            $env.clone(),
        )
    };
}

fn setup_envoirment(env: &Env) {
    add_binary_fn!("+",env,ExprKind::Number, +,Expr::get_number);
    add_binary_fn!("-",env,ExprKind::Number, -,Expr::get_number);
    add_binary_fn!("*",env,ExprKind::Number, *,Expr::get_number);
    add_binary_fn!("/",env,ExprKind::Number, /,Expr::get_number);
    add_binary_fn!("%",env,ExprKind::Number, %,Expr::get_number);
    // the reason we have to use the |e|e is because we need to have something
    // that when we call it, it returns the value of the expression modified so it should work with the operator
    // but in the case of the boolean operators, we don't need to do anything to the expression
    // so we end up with |e|e weirdness
    add_binary_fn!("=",env,ExprKind::Bool,  == , |e|e);
    add_binary_fn!("<",env,ExprKind::Bool, <, |e|e);
    add_binary_fn!(">",env,ExprKind::Bool, >, |e|e);
    add_binary_fn!("<=",env,ExprKind::Bool, <= , |e|e);
    add_binary_fn!(">=",env,ExprKind::Bool, >= , |e|e);
    add_binary_fn!("!=",env,ExprKind::Bool, != , |e|e);
    add_binary_fn!("and",env,ExprKind::Bool, &&,Expr::get_bool);
    add_binary_fn!("or",env,ExprKind::Bool, ||,Expr::get_bool);
    add_fn!(
        "not",
        env,
        |args: Vec<Expr>| Expr {
            expr: ExprKind::Bool(!args[0].get_bool()),
            state: State::Evaluated,
            file: "not.rs".to_string(),
        },
        1
    );
    add_fn!(
        "display",
        env,
        |args: Vec<Expr>| {
            print!("{}", args[0]);
            Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "display.rs".to_string(),
            }
        },
        1
    );
    eval_to_env!("(define (newline) (display \"\\n\"))", env);
    eval_to_env!("(define (cons x y) (lambda (m) (m x y)))", env);
    eval_to_env!("(define (car z) (z (lambda (p q) p)))", env);
    eval_to_env!("(define (cdr z) (z (lambda (p q) q)))", env);
    eval_to_env!("(define (set-car! z x) (set! z (cons x (cdr z))))", env);
    eval_to_env!("(define (set-cdr! z x) (set! z (cons (car z) x)))", env);
    let apply = Expr {
        expr: ExprKind::Lambda(
            |args, env| apply(args[0].clone(), env, args[1..].to_vec()),
            vec![],
        ),
        state: State::Evaluated,
        file: "apply.rs".to_string(),
    };
    env.insert("apply".to_string(), apply);
    add_literal!("nil", ExprKind::Nil, env.clone());
    add_literal!("true", ExprKind::Bool(true), env.clone());
    add_literal!("false", ExprKind::Bool(false), env.clone());
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
