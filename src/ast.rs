use std::{
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
    Apply(Box<Expr>, Vec<Expr>),
    Symbol(String),
    Var(String, Box<Expr>),
    UserLambda(Box<Expr>, Vec<String>, Option<Env>),
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
    pub(crate) file: String,
}

impl ExprKind {
    pub fn get_number(&self) -> i32 {
        match self {
            Self::Number(n) => *n,
            _ => panic!("Not a number"),
        }
    }
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
            Self::Apply(func, args) => write!(
                f,
                "(apply {} {})",
                func,
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
            Self::Apply(func, args) => write!(f, "apply ({func:?}, {args:?})"),
            Self::Symbol(s) => write!(f, "symbol ({s:?})"),
            Self::Var(s, e) => write!(f, "var ({s:?}, {e:?})"),
            Self::UserLambda(body, params, _) => {
                write!(f, "user lambda ({body:?}, {params:?})")
            }
        }
    }
}
impl Expr {
    pub fn new(expr: i32) -> Self {
        Self {
            expr: ExprKind::Number(expr),
            state: State::Evaluated,
            file: "default.rs".to_string(),
        }
    }

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

    pub fn get_number(&self) -> i32 {
        match &self.expr {
            ExprKind::Number(n) => *n,
            other => panic!("Not a number: {other:?}"),
        }
    }
}

impl Add for Expr {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // get the value of the first expression
        let expr1 = Self::eval(self).get_number();
        // get the value of the second expression
        let expr2 = Self::eval(other).get_number();
        // return the sum of the two expressions
        Self {
            expr: ExprKind::Number(expr1 + expr2),
            state: State::Evaluated,
            file: "add.rs".to_string(),
        }
    }
}

impl Sub for Expr {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // get the value of the first expression
        let expr1 = Self::eval(self).get_number();
        // get the value of the second expression
        let expr2 = Self::eval(other).get_number();
        // return the sum of the two expressions
        Self {
            expr: ExprKind::Number(expr1 - expr2),
            state: State::Evaluated,
            file: "sub.rs".to_string(),
        }
    }
}

impl Mul for Expr {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // get the value of the first expression
        let expr1 = Self::eval(self).get_number();
        // get the value of the second expression
        let expr2 = Self::eval(other).get_number();
        // return the sum of the two expressions
        Self {
            expr: ExprKind::Number(expr1 * expr2),
            state: State::Evaluated,
            file: "mul.rs".to_string(),
        }
    }
}

impl Div for Expr {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        // get the value of the first expression
        let expr1 = Self::eval(self).get_number();
        // get the value of the second expression
        let expr2 = Self::eval(other).get_number();
        // return the sum of the two expressions
        Self {
            expr: ExprKind::Number(expr1 / expr2),
            state: State::Evaluated,
            file: "div.rs".to_string(),
        }
    }
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
