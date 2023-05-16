use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::{
    ast::{Args, Expr, ExprKind, State},
    eval::{apply, eval_expr},
    parser,
};

// for + - * / % < > <= >= and or: eventually, we want to make these accept any number of arguments
macro_rules! add_binary_fn {
    ($symbol:literal, $env:expr, $res:expr, $op:tt, $mod:expr) => {{
        add_fn!(
            $symbol,
            $env,
            |args: Vec<Expr>, _env| {
                Expr {
                    expr: $res(($mod(&args[0]) $op $mod(&args[1]))),
                    state: State::Evaluated,
                    file: format!("{}.rs", $symbol),
                }
            },
            Args::Count(2)
        );
    }};
}

macro_rules! add_fn {
    ($symbol:literal, $env:expr, $todo:expr, $arg_count:expr) => {{
        let e = Expr {
            expr: ExprKind::PrimitiveLambda(
                |args, env| {
                    if !$arg_count.compare(args.len()) {
                        panic!("Expected {} arguments, got {}", $arg_count, args.len());
                    } else {
                        // we have to use closures as opposed to whatver macro calling the args from |args, _env|
                        // because of macro hygiene
                        #[allow(clippy::redundant_closure_call)]
                        ($todo)(args, env)
                    }
                },
                $arg_count,
                $symbol.to_string(),
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
    name: String,
    scope: Rc<RefCell<HashMap<String, Expr>>>,
    pub parent: Option<Box<Env>>,
}

impl Env {
    #[must_use]
    pub fn new() -> Self {
        let env = Self {
            name: "primitive".to_string(),
            scope: Rc::new(RefCell::new(HashMap::new())),
            parent: None,
        };
        setup_envoirment(&env);
        // we give back a child of the env, so we can detect primitive functions
        // because the user can't modify the "primitive" env, because it's child is returned
        env.new_child(Some("global".to_string()))
    }

    pub fn insert(&self, key: String, val: Expr) -> bool {
        self.scope.borrow_mut().insert(key, val).is_some()
    }

    pub fn set(&self, key: String, val: Expr) {
        if self.scope.borrow().contains_key(&key) {
            self.scope.borrow_mut().insert(key, val);
        } else if let Some(parent) = &self.parent {
            parent.set(key, val);
        } else {
            panic!("Variable {key} not found in {}", self.backtrace());
        }
    }

    pub fn backtrace(&self) -> String {
        let mut s = String::new();
        let mut env = self;
        while let Some(parent) = &env.parent {
            s.push_str(&format!("{} -> ", env.name));
            env = parent;
        }
        s.push_str(&env.name.to_string());
        s
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
    pub fn new_child(&self, name: Option<String>) -> Self {
        Self {
            scope: Rc::new(RefCell::new(HashMap::new())),
            parent: Some(Box::new(self.clone())),
            name: name.unwrap_or_else(|| "lambda".to_string()),
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
    add_primitives(env);
    eval_to_env!("(define (null? x) (eq x nil))", env);
    eval_to_env!("(define (list . d) d)", env);
    eval_to_env!(
        "
        (define (foldl-short-circuit f short init args) 
    (if (null? args) init 
        (if (short (f init (car args)))
            (foldl-short-circuit f short (f init (car args)) (cdr args))
            (f init (car args)))))",
        env
    );
    eval_to_env!(
        "(define (foldl-math f init args)
            (cond ((null? args) init)
                  (else 
                    (define (foldl-inner init args) 
                        (if (null? args) 
                            init
                            (foldl-inner (f init (car args)) (cdr args))))
                    (foldl-inner (f (car args) init) (cdr args)))))",
        env
    );
    eval_to_env!("(define (not x) (if x false true))", env);
    eval_to_env!("(define (newline) (display \"\n\"))", env);
    eval_to_env!("(define (displayln b) (display b) (newline))", env);

    // eval_to_env!("(define (foldl f init args) (if (null? args) init (foldl f (f init (car args)) (cdr args))))", env);
    // foldl doesn't work for - / % if since the most inner call (f init (car args)) with 0 or as init
    // for - the innermost thing will be the negation of the first thing
    // for / the innermost thing will be 1 / the first thing, and in both cases, we want the first thing to be the first thing
    // in order to do this, we need to have three cases:
    // - if there is no args, return init
    // - if there is one arg, return that arg
    // - if there is more than one arg
    // define an inner function foldl-inner imitating (define (foldl f init args) (if (null? args) init (foldl f (f init (car args)) (cdr args))))
    // and then call it with (foldl-inner (f (car args) init) (cdr args)
    // this way, the first thing will be the first thing

    eval_to_env!("(define (+ . args) (foldl-math add 0 args))", env);
    // - needs special case if only one thing for negation
    eval_to_env!("(define (- . args) (cond ((null? args) 0) ((null? (cdr args)) (sub 0 (car args))) (else (foldl-math sub (car args) (cdr args)))))", env);
    eval_to_env!("(define (* . args) (foldl-math mul 1 args))", env);
    eval_to_env!("(define (/ . args) (foldl-math div 1 args))", env);
    // % needs to has to know the first thing in the list or else it will always be 0
    // this is because the innermost call will be (rem 1 (car args)) which will always be 0
    eval_to_env!("(define (identity x) x)", env);
    eval_to_env!(
        "(define (% . args) (cond ((null? args) 1) (else (foldl-math rem (car args) (cdr args)))))",
        env
    );
    eval_to_env!(
        "(define (&& . args) (foldl-short-circuit (lambda (a b) (if a b)) identity true args))",
        env
    );
    eval_to_env!(
        "(define (|| . args) (foldl-short-circuit (lambda (a b) (if a true b)) not false args))",
        env
    );
    // equalities
    // = should fold into either a the first thing or false
    // use a pair so each iteration (return true|false, thing if not false|false)
    eval_to_env!("(define (foldl-compare f args) (if (null? args) true (car (foldl-short-circuit (lambda (a b) (if (f (cdr a) (cdr b)) (cons true (cdr a)) (cons false false))) (lambda (val) (car val)) (cons true (car args)) (map (lambda (arg) (cons true arg)) (cdr args))))))", env);
    eval_to_env!("(define (= . args) (foldl-compare eq args))", env);
    eval_to_env!("(define (< . args) (foldl-compare lt args))", env);
    eval_to_env!("(define (> . args) (foldl-compare gt args))", env);
    eval_to_env!(
        "(define (<= . args) (foldl-compare (lambda (a b) (not (gt a b))) args))",
        env
    );
    eval_to_env!(
        "(define (>= . args) (foldl-compare (lambda (a b) (not (lt a b))) args))",
        env
    );
    eval_to_env!("(define (cons x y) (lambda (m) (m x y)))", env);
    eval_to_env!("(define (car z) (z (lambda (p q) p)))", env);
    eval_to_env!("(define (cdr z) (z (lambda (p q) q)))", env);
    eval_to_env!("(define (set-car! z x) (set! z (cons x (cdr z))))", env);
    eval_to_env!("(define (set-cdr! z x) (set! z (cons (car z) x)))", env);

    eval_to_env!("(define (display-list c) (define (inner c) (if (null? c) (display \"]\") (begin (display (car c)) (display \" \") (inner (cdr c))))) (display \"[\") (inner c))", env);
    // map
    eval_to_env!(
        "(define (map f args) (if (null? args) nil (cons (f (car args)) (map f (cdr args)))))",
        env
    );
    // length
    eval_to_env!(
        "(define (length args) (if (null? args) 0 (+ 1 (length (cdr args)))))",
        env
    );
}

fn add_primitives(env: &Env) {
    add_binary_fn!("add",env,ExprKind::Number, +,Expr::get_number);
    add_binary_fn!("sub",env,ExprKind::Number, -,Expr::get_number);
    add_binary_fn!("mul",env,ExprKind::Number, *,Expr::get_number);
    add_binary_fn!("div",env,ExprKind::Number, /,Expr::get_number);
    add_binary_fn!("rem",env,ExprKind::Number, %,Expr::get_number);
    // identity function:
    // the reason we have to use the |e|e is because we need to have something
    // that when we call it, it returns the value of the expression modified so it should work with the operator
    // but in the case of the boolean operators, we don't need to do anything to the expression
    // so we end up with |e|e weirdness
    add_binary_fn!("eq",env,ExprKind::Bool, ==, |e|e);
    add_binary_fn!("gt",env,ExprKind::Bool, >,Expr::get_number);
    add_binary_fn!("lt",env,ExprKind::Bool, <,Expr::get_number);
    add_fn!(
        "display",
        env,
        |args: Vec<Expr>, _env| {
            print!("{}", args[0]);
            Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "display.rs".to_string(),
            }
        },
        Args::Count(1)
    );
    let apply = Expr {
        expr: ExprKind::PrimitiveLambda(
            |args, env| apply(args[0].clone(), env, args[1..].to_vec()),
            Args::Count(2),
            "apply".to_string(),
        ),
        state: State::Evaluated,
        file: "apply.rs".to_string(),
    };
    env.insert("apply".to_string(), apply);
    let sleep = Expr {
        expr: ExprKind::PrimitiveLambda(
            |args, _| {
                std::thread::sleep(std::time::Duration::from_millis(args[0].get_number() as u64));
                Expr {
                    expr: ExprKind::Nil,
                    state: State::Evaluated,
                    file: "sleep.rs".to_string(),
                }
            },
            Args::Count(1),
            "sleep".to_string(),
        ),
        state: State::Evaluated,
        file: "sleep.rs".to_string(),
    };
    env.insert("sleep".to_string(), sleep);
    add_literal!("nil", ExprKind::Nil, env.clone());
    add_literal!("true", ExprKind::Bool(true), env.clone());
    add_literal!("false", ExprKind::Bool(false), env.clone());
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nScope:\n")?;
        writeln!(f, "{}", self.name)?;
        for (k, v) in self.scope.borrow().iter() {
            writeln!(f, "{k:?}: {v:?}")?;
        }
        if let Some(p) = &self.parent {
            write!(f, "Parent:{p}")?;
        }
        write!(f, "\n--end--\n")
    }
}
