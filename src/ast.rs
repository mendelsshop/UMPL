use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, Div, Mul, Sub},
};

use crate::Env;

#[derive(Clone)]
pub enum ExprKind {
    Number(i32),
    Word(String),
    Bool(bool),
    Nil,
    Lambda(fn(Vec<Expr>, Env) -> Expr, Vec<String>),
    Def(String, Box<Expr>),
    Begin(Vec<Expr>),
    List(Vec<Expr>),
    Symbol(String),
    Var(String, Box<Expr>),
    UserLambda(Box<Expr>, Vec<String>, Option<Env>),
    Set(String, Box<Expr>),
}

impl PartialOrd for ExprKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Number(n), Self::Number(m)) => n.partial_cmp(m),
            (Self::Word(n), Self::Word(m)) => n.partial_cmp(m),
            (Self::Bool(n), Self::Bool(m)) => n.partial_cmp(m),
            (Self::Nil, Self::Nil) => Some(Ordering::Equal),
            _ => None,
        }
    }
}

impl PartialEq for ExprKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(n), Self::Number(m)) => n == m,
            (Self::Word(n), Self::Word(m)) => n == m,
            (Self::Bool(n), Self::Bool(m)) => n == m,
            (Self::Nil, Self::Nil) => true,
            _ => false,
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

#[derive(Clone, Debug)]
pub struct Expr {
    pub expr: ExprKind,
    pub state: State,
    pub(crate) file: String,
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.expr == other.expr
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        println!("{:?}", self.expr);
        println!("{:?}", other.expr);
        self.expr.partial_cmp(&other.expr)
    }
}
impl Expr {
    pub fn initialize(mut self, env: &Env) -> Self {
        if let ExprKind::UserLambda(_, _, ref mut closure) = self.expr {
            if closure.is_none() {
                *closure = Some(env.new_child());
            }
        }
        self
    }
}

macro_rules! get_type {
    ($self:ident, $type:ident, $ret:ty, $func:ident) => {
        pub fn $func(&self) -> $ret {
            match self {
                Self::$type(n) => n.clone(),
                other => panic!("Expected {}, got {:?}", stringify!($type), other),
            }
        }
    };
}
macro_rules! impl_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for Expr {
            type Output = Expr;
            fn $method(self, other: Self) -> Self::Output {
                match (self.expr, other.expr) {
                    (ExprKind::Number(n), ExprKind::Number(m)) => Expr {
                        expr: ExprKind::Number(n $op m),
                        state: State::Evaluated,
                        file: String::new(),
                    },
                    _ => panic!("Invalid operation"),
                }
            }
        }
    };
}

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
impl_op!(Div, div, /);

impl ExprKind {
    get_type!(self, Number, i32, get_number);
    get_type!(self, Word, String, get_word);
    get_type!(self, Bool, bool, get_bool);
    get_type!(self, Symbol, String, get_symbol);
}

impl fmt::Display for ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Word(s) => write!(f, "{s}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Nil => write!(f, "nil"),
            Self::Lambda(_, params) => {
                write!(f, "(lambda of {})", params.join(", "))
            }
            Self::Def(name, lambda) => write!(f, "(def {name} {lambda})"),
            Self::Begin(exprs) => write!(
                f,
                "(begin {})",
                exprs
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            Self::List(args) => write!(
                f,
                "(list {})",
                args.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Symbol(s) => write!(f, "symbol: {s}"),
            Self::Var(s, e) => write!(f, "(var {s} {e})"),
            Self::UserLambda(body, params, _) => {
                write!(f, "(lambda {body} of {})", params.join(", "))
            }
            Self::Set(s, e) => write!(f, "(set {s} {e})"),
        }
    }
}

impl fmt::Debug for ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "number: ({n:?})"),
            Self::Word(s) => write!(f, "word: ({s:?})"),
            Self::Bool(b) => write!(f, "bool: ({b:?})"),
            Self::Nil => write!(f, "nil"),
            Self::Lambda(proc, params) => {
                write!(f, "lambda ({proc:?}, {params:?})")
            }
            Self::Def(name, lambda) => write!(f, "def ({name:?}, {lambda:?})"),
            Self::Begin(exprs) => write!(f, "begin ({exprs:?})"),
            Self::List(args) => write!(f, "list ({args:?})"),
            Self::Symbol(s) => write!(f, "symbol ({s:?})"),
            Self::Var(s, e) => write!(f, "var ({s:?}, {e:?})"),
            Self::UserLambda(body, params, _) => {
                write!(f, "user lambda ({body:?}, {params:?})")
            }
            Self::Set(s, e) => write!(f, "set ({s:?}, {e:?})"),
        }
    }
}

// defines the function that calls the inner (Exprkind) function of the same name
/// impl Expr {
///    `call_inner!(self`, `get_number`, i32);
/// }
macro_rules! call_inner {
    ($self:ident, $func:ident, $ret:ty) => {
        pub fn $func(&self) -> $ret {
            self.expr.$func()
        }
    };
}

impl Expr {
    pub fn eval(self) -> Self {
        match self.state {
            State::Thunk(_vars) => {
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
        }
    }

    call_inner!(self, get_number, i32);
    call_inner!(self, get_word, String);
    call_inner!(self, get_bool, bool);
    call_inner!(self, get_symbol, String);
}

#[allow(clippy::match_same_arms)]
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.state {
            State::Thunk(_) => write!(f, "{}", self.expr),
            State::Evaluated => write!(f, "{}", self.expr),
        }
    }
}
