use std::{ops::{Div, Mul, Add, Sub}, fmt};

use crate::Env;

#[derive(Clone, Debug)]
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
            file: "default.rs".to_string(),
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
        match &self.expr {
            ExprKind::Number(n) => *n,
            other => panic!("Not a number: {:?}", other),
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
            file: "add.rs".to_string(),
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
            file: "sub.rs".to_string(),
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
            file: "mul.rs".to_string(),
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
            file: "div.rs".to_string(),
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
