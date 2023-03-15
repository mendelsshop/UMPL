use std::fmt::{self, Debug, Display};

use crate::token::{BuiltinFunction, Info};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr<'a> {
    pub info: Info<'a>,
    pub expr: ExprType<'a>,
}

impl<'a> ListUtils<'a, Expr<'a>> for Expr<'a> {
    fn get_list(&self) -> Option<&Cons<'a, Expr<'a>>> {
        match &self.expr {
            ExprType::Cons(cons) => Some(cons),
            _ => None,
        }
    }

    fn get_list_mut(&mut self) -> Option<&mut Cons<'a, Expr<'a>>> {
        match &mut self.expr {
            ExprType::Cons(cons) => Some(cons),
            _ => None,
        }
    }

    fn new_list(info: Info<'a>, list: Cons<'a, Expr<'a>>) -> Expr<'a> {
        Expr {
            info,
            expr: ExprType::Cons(list),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprType<'a> {
    Literal(Lit<'a>),
    Fn(FnDef<'a>),
    Call(Box<FnCall<'a>>),
    If(Box<If<'a>>),
    Loop(Loop<'a>),
    Var(Box<Var<'a>>),
    Lambda(Lambda<'a>),
    Return(Box<Expr<'a>>),
    Break(Box<Expr<'a>>),
    Identifier(Ident<'a>),
    Cons(Cons<'a, Expr<'a>>),
    Continue,
}

impl<'a> Expr<'a> {
    pub const fn new_literal(info: Info<'a>, value: Lit<'a>) -> Self {
        Self {
            info,
            expr: ExprType::Literal(value),
        }
    }

    pub const fn new_fn(info: Info<'a>, value: FnDef<'a>) -> Self {
        Self {
            info,
            expr: ExprType::Fn(value),
        }
    }

    pub fn new_call(info: Info<'a>, value: FnCall<'a>) -> Self {
        Self {
            info,
            expr: ExprType::Call(Box::new(value)),
        }
    }

    pub fn new_if(info: Info<'a>, value: If<'a>) -> Self {
        Self {
            info,
            expr: ExprType::If(Box::new(value)),
        }
    }

    pub const fn new_loop(info: Info<'a>, value: Loop<'a>) -> Self {
        Self {
            info,
            expr: ExprType::Loop(value),
        }
    }

    pub fn new_var(info: Info<'a>, value: Var<'a>) -> Self {
        Self {
            info,
            expr: ExprType::Var(Box::new(value)),
        }
    }

    pub const fn new_lambda(info: Info<'a>, value: Lambda<'a>) -> Self {
        Self {
            info,
            expr: ExprType::Lambda(value),
        }
    }

    pub fn new_return(info: Info<'a>, value: Expr<'a>) -> Self {
        Self {
            info,
            expr: ExprType::Return(Box::new(value)),
        }
    }

    pub fn new_break(info: Info<'a>, value: Expr<'a>) -> Self {
        Self {
            info,
            expr: ExprType::Break(Box::new(value)),
        }
    }

    pub const fn new_continue(info: Info<'a>) -> Self {
        Self {
            info,
            expr: ExprType::Continue,
        }
    }

    pub const fn new_identifier(info: Info<'a>, value: Ident<'a>) -> Self {
        Self {
            info,
            expr: ExprType::Identifier(value),
        }
    }

    pub const fn new_cons(info: Info<'a>, value: Cons<'a, Expr<'a>>) -> Self {
        Self {
            info,
            expr: ExprType::Cons(value),
        }
    }
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.expr {
            ExprType::Literal(lit) => write!(f, "literal [{lit}]"),
            ExprType::Fn(fn_def) => write!(f, "fn [{fn_def}]"),
            ExprType::Call(call) => write!(f, "call [{call}]"),
            ExprType::If(if_) => write!(f, "if [{if_}]"),
            ExprType::Loop(loop_def) => write!(f, "loop [{loop_def}]"),
            ExprType::Var(var) => write!(f, "var [{var}]"),
            ExprType::Lambda(lambda) => write!(f, "lambda [{lambda}]"),
            ExprType::Return(return_) => write!(f, "return [{return_}]"),
            ExprType::Break(break_) => write!(f, "break [{break_}]"),
            ExprType::Continue => write!(f, "continue"),
            ExprType::Identifier(ident) => write!(f, "ident [{ident}]"),
            ExprType::Cons(cons) => write!(f, "cons [{cons}]"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lit<'a> {
    pub info: Info<'a>,
    pub value: LitType,
}

impl<'a> Lit<'a> {
    pub const fn new_number(info: Info<'a>, value: f64) -> Self {
        Self {
            info,
            value: LitType::Number(value),
        }
    }

    pub const fn new_string(info: Info<'a>, value: String) -> Self {
        Self {
            info,
            value: LitType::String(value),
        }
    }

    pub const fn new_boolean(info: Info<'a>, value: bool) -> Self {
        Self {
            info,
            value: LitType::Boolean(value),
        }
    }

    pub const fn new_hempty(info: Info<'a>) -> Self {
        Self {
            info,
            value: LitType::Hempty,
        }
    }

    pub const fn new_file(info: Info<'a>, value: String) -> Self {
        Self {
            info,
            value: LitType::File(value),
        }
    }
}

impl<'a> Display for Lit<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at [{}]", self.value, self.info)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LitType {
    String(String),
    Number(f64),
    Boolean(bool),
    File(String),
    Hempty,
}

impl fmt::Display for LitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(s) => write!(f, "string: {s}"),
            Self::Number(n) => write!(f, "number: {n}"),
            Self::Boolean(b) => write!(f, "boolean: {b}"),
            Self::File(s) => write!(f, "file: {s}"),
            Self::Hempty => write!(f, "hemty"),
        }
    }
}

pub trait ListUtils<'a, A> {
    fn get_list(&self) -> Option<&Cons<'a, A>>;
    fn get_list_mut(&mut self) -> Option<&mut Cons<'a, A>>;
    fn new_list(info: Info<'a>, list: Cons<'a, A>) -> A;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cons<'a, A> {
    pub info: Info<'a>,
    pub car: Box<A>,
    pub cdr: Option<Box<A>>,
}

// TODO: impl Iterator for Cons<'a, A>
impl<'a, A: ListUtils<'a, A>> Cons<'a, A> {
    pub fn new(info: Info<'a>, car: A, cdr: Option<A>) -> Self {
        Self {
            info,
            car: Box::new(car),
            cdr: cdr.map(|cdr| Box::new(cdr)),
        }
    }

    pub const fn car(&self) -> &A {
        &self.car
    }

    pub const fn cdr(&self) -> &Option<Box<A>> {
        &self.cdr
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut list = self;
        while let Some(list_) = &list.cdr {
            if let Some(list_) = list_.get_list() {
                len += 1;
                list = list_;
            }
        }
        len
    }

    pub fn new_cdr_empty(info: Info<'a>, car: A) -> Self {
        Self {
            info,
            car: Box::new(car),
            cdr: None,
        }
    }

    pub fn set_cdr(&mut self, cdr: A) {
        if let Some(cdr_) = &mut self.cdr {
            if let Some(cdr_) = cdr_.get_list_mut() {
                cdr_.set_cdr(cdr);
            } else {
                *cdr_ = Box::new(A::new_list(self.info, Self::new_cdr_empty(self.info, cdr)));
            }
        } else {
            self.cdr = Some(Box::new(A::new_list(
                self.info,
                Self::new_cdr_empty(self.info, cdr),
            )));
        }
    }
}

impl<'a, A: Display> Display for Cons<'a, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({car}", car = self.car)?;
        if let Some(cdr) = &self.cdr {
            write!(f, " {cdr}")?;
        }
        write!(f, ")")
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Lambda<'a> {
    pub info: Info<'a>,
    pub param_count: usize,
    pub extra_params: bool,
    pub body: Cons<'a, Expr<'a>>,
}

impl<'a> Lambda<'a> {
    pub const fn new(
        info: Info<'a>,
        param_count: usize,
        extra_params: bool,
        body: Cons<'a, Expr<'a>>,
    ) -> Self {
        Self {
            info,
            param_count,
            extra_params,
            body,
        }
    }
}

impl<'a> Display for Lambda<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "fn: [with {} {} parameters {} [{}]]",
            if self.extra_params { "at least" } else { "" },
            self.param_count,
            self.body,
            self.info
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDef<'a> {
    pub info: Info<'a>,
    pub name: char,
    pub modules: Vec<char>,
    inner: Lambda<'a>,
}

impl<'a> FnDef<'a> {
    pub const fn new(info: Info<'a>, name: char, modules: Vec<char>, inner: Lambda<'a>) -> Self {
        Self {
            info,
            name,
            modules,
            inner,
        }
    }
}

impl<'a> Display for FnDef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} defined as {} at [{}]",
            self.name, self.inner, self.info
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrintType {
    Newline,
    NoNewline,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnCall<'a> {
    pub info: Info<'a>,
    pub args: Vec<Expr<'a>>,
    pub print_type: PrintType,
}

impl<'a> FnCall<'a> {
    pub fn new(info: Info<'a>, args: Vec<Expr<'a>>, print_type: PrintType) -> Self {
        Self {
            info,
            args,
            print_type,
        }
    }
}

impl<'a> Display for FnCall<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "fn: {}() with {} args [{}]",
            self.args
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<String>(),
            self.args.len(),
            self.info
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct If<'a> {
    pub info: Info<'a>,
    pub condition: Expr<'a>,
    pub then: Cons<'a, Expr<'a>>,
    pub otherwise: Cons<'a, Expr<'a>>,
}

impl<'a> If<'a> {
    pub const fn new(
        info: Info<'a>,
        condition: Expr<'a>,
        then: Cons<'a, Expr<'a>>,
        otherwise: Cons<'a, Expr<'a>>,
    ) -> Self {
        Self {
            info,
            condition,
            then,
            otherwise,
        }
    }
}

impl<'a> Display for If<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "if {} then {} else {} [{}]",
            self.condition, self.then, self.otherwise, self.info,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Loop<'a> {
    pub info: Info<'a>,
    pub body: Cons<'a, Expr<'a>>,
}

impl<'a> Loop<'a> {
    pub const fn new(info: Info<'a>, body: Cons<'a, Expr<'a>>) -> Self {
        Self { info, body }
    }
}

impl<'a> Display for Loop<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "loop {} [{}]", self.body, self.info)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var<'a> {
    pub info: Info<'a>,
    pub name: String,
    pub value: Expr<'a>,
}

impl<'a> Var<'a> {
    pub const fn new(info: Info<'a>, name: String, value: Expr<'a>) -> Self {
        Self { info, name, value }
    }
}

impl<'a> Display for Var<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "var {} = {} [{}]", self.name, self.value, self.info)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interlaced<A: Debug + Display + Clone, B: Debug + Clone + Display> {
    pub main: A,
    pub interlaced: Vec<B>,
}

impl<A: Debug + Clone + Display, B: Debug + Clone + Display> Interlaced<A, B> {
    pub fn new(main: A, interlaced: Vec<B>) -> Self {
        Self { main, interlaced }
    }

    pub fn interlaced_to_string(&self, sep: &str) -> String {
        self.interlaced
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(sep)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Accesor {
    Car,
    Cdr,
}

impl Display for Accesor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Car => write!(f, "car"),
            Self::Cdr => write!(f, "cdr"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentType {
    Var(Interlaced<String, Accesor>),
    FnIdent(Interlaced<char, char>),
    Builtin(BuiltinFunction),
}

impl Display for IdentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Var(name) => write!(
                f,
                "var {}{}",
                name.main,
                match name.interlaced_to_string(".") {
                    s if s.is_empty() => String::new(),
                    s => format!(".{s}"),
                }
            ),
            Self::FnIdent(ident) => write!(
                f,
                "defined function {}{}",
                ident.main,
                match ident.interlaced_to_string("+") {
                    s if s.is_empty() => String::new(),
                    s => format!(" in module {s}"),
                }
            ),
            Self::Builtin(builtin) => write!(f, "builtin function {builtin}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident<'a> {
    pub info: Info<'a>,
    pub ident_type: IdentType,
}

impl<'a> Ident<'a> {
    pub const fn new(info: Info<'a>, ident_type: IdentType) -> Self {
        Self { info, ident_type }
    }
}

impl<'a> Display for Ident<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}]", self.ident_type, self.info)
    }
}
