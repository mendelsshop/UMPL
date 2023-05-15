use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, Div, Mul, Sub},
};

use crate::{eval::actual_value, Env};

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Args {
    Count(i32),
    Many,
}

impl Args {
    pub const fn compare(self, len: usize) -> bool {
        match self {
            Self::Count(n) => len == n as usize,
            Self::Many => true,
        }
    }
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Count(n) => write!(f, "{}", n),
            Self::Many => write!(f, "many"),
        }
    }
}

#[derive(Clone)]
pub enum ExprKind {
    Number(f64),
    Word(String),
    Bool(bool),
    Nil,
    PrimitiveLambda(fn(Vec<Expr>, Env) -> Expr, Args, String),
    Def(String, Box<Expr>),
    Begin(Vec<Expr>),
    List(Vec<Expr>),
    Symbol(String),
    Var(String, Box<Expr>),
    Lambda(
        Box<Expr>,
        Vec<String>,
        Option<Env>,
        Option<String>,
        Option<String>,
    ),
    Set(String, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

impl PartialOrd for ExprKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Number(n), Self::Number(m)) => n.partial_cmp(m),
            (Self::Word(n), Self::Word(m)) => n.partial_cmp(m),
            (Self::Bool(n), Self::Bool(m)) => n.partial_cmp(m),
            (Self::Nil, Self::Nil) => Some(Ordering::Equal),
            (Self::List(n), Self::List(m)) => n.partial_cmp(m),
            _ => None,
        }
    }
}

impl PartialEq for ExprKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(n), Self::Number(m)) => n == m,
            (Self::Word(n), Self::Word(m)) | (Self::Symbol(n), Self::Symbol(m)) => n == m,
            (Self::Bool(n), Self::Bool(m)) => n == m,
            (Self::Nil, Self::Nil) => true,
            (Self::List(n), Self::List(m)) => n == m,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum State {
    Thunk(Env),
    #[default]
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
        self.expr.partial_cmp(&other.expr)
    }
}
impl Expr {
    pub(crate) fn set_name(&mut self, name: String) {
        match &mut self.expr {
            ExprKind::Lambda(_, _, _, _, n) => *n = Some(name),
            ExprKind::PrimitiveLambda(_, _, n) => *n = name,
            _ => panic!("Cannot set name of non-lambda"),
        }
    }
}

macro_rules! get_type {
    ($self:ident, $type:ident, $ret:ty, $func:ident) => {
        pub fn $func(&self) -> $ret {
            match self {
                Self::$type(n) => n.clone(),
                other => panic!("Expected {}, got {}", stringify!($type), other),
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
    get_type!(self, Number, f64, get_number);
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
            Self::PrimitiveLambda(_, params, name) => {
                write!(f, "primitive #<procedure:{name} {params}>")
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
            Self::Lambda(body, params, _env, extra_params, name) => {
                write!(
                    f,
                    "(lambda {}({}{}) {body} in <procedure-env>)",
                    name.as_ref()
                        .map_or_else(String::new, |name| format!("{} ", name)),
                    params.join(", "),
                    extra_params
                        .as_ref()
                        .map_or_else(String::new, |extra_params| format!(", {extra_params}")),
                )
            }
            Self::Set(s, e) => write!(f, "(set {s} {e})"),
            Self::If(predicate, consequent, alternative) => {
                write!(f, "(if {predicate} {consequent} else {alternative})")
            }
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
            Self::PrimitiveLambda(proc, params, name) => {
                write!(
                    f,
                    "lambda {name:?} ({params:?}) (native: {proc:?}) in <procedure-env>)"
                )
            }
            Self::Def(name, lambda) => write!(f, "def ({name:?}, {lambda:?})"),
            Self::Begin(exprs) => write!(f, "begin ({exprs:?})"),
            Self::List(args) => write!(f, "list ({args:?})"),
            Self::Symbol(s) => write!(f, "symbol ({s:?})"),
            Self::Var(s, e) => write!(f, "var ({s:?}, {e:?})"),
            Self::Lambda(body, params, _, extra_param, name) => {
                write!(
                    f,
                    "(lambda {name:?} ({params:?} {extra_param:?}) {body:?}) in <procedure-env>)"
                )
            }
            Self::Set(s, e) => write!(f, "set ({s:?}, {e:?})"),
            Self::If(predicate, consequent, alternative) => {
                write!(f, "if {predicate:?} {consequent:?} else {alternative:?}")
            }
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
    pub fn eval(&mut self) {
        if let State::Thunk(vars) = std::mem::take(&mut self.state) {
            self.expr = actual_value(self.clone(), vars).expr;
        }
    }

    pub fn initialize(mut self, env: &Env) -> Self {
        if let ExprKind::Lambda(_, _, ref mut closure, _,_) = self.expr {
            if closure.is_none() {
       
                    *closure = Some(env.clone());
            }
        }
        self
    }

    call_inner!(self, get_number, f64);
    call_inner!(self, get_word, String);
    call_inner!(self, get_bool, bool);
    call_inner!(self, get_symbol, String);
}

#[allow(clippy::match_same_arms)]
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.state {
            State::Thunk(_) => write!(f, "thunk"),
            State::Evaluated => write!(f, "{}", self.expr),
        }
    }
}
