use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::{
    ast::{Expr, ExprKind, State},
    eval::eval_expr,
    parser,
};

macro_rules! add_math_fn {
    ($symbol:literal, $op:tt, $env:expr) => {
        {
            let v = vec![String::from("x"), String::from("y")];
            let e = Expr {
                expr: ExprKind::Lambda(|args, _env|
                    if args.len() != 2 {
                        panic!("Expected 2 arguments, got {}", args.len());
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

fn setup_envoirment(env: &Env) {
    add_math_fn!("+", +, env);
    add_math_fn!("-", -, env);
    add_math_fn!("*", *, env);
    add_math_fn!("/", /, env);
    let display = Expr {
        expr: ExprKind::Lambda(
            |args, _env| {
                println!("{}", args[0]);
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
    eval_expr(
        parser::parse(
            &mut "(define (cons x y) (lambda (m) (m x y)))"
                .chars()
                .peekable(),
        ),
        env.clone(),
    );
    eval_expr(
        parser::parse(&mut "(define (car z) (z (lambda (p q) p)))".chars().peekable()),
        env.clone(),
    );
    eval_expr(
        parser::parse(&mut "(define (cdr z) (z (lambda (p q) q)))".chars().peekable()),
        env.clone(),
    );
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
