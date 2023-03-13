use std::fmt::{self, Debug, Display};

use crate::token::{BuiltinFunction, Info};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr<'a> {
    pub info: Info<'a>,
    pub expr: ExprType<'a>,
}
#[derive(Debug, Clone, PartialEq)]
pub enum ExprType<'a> {
    Literal(Lit<'a>),
    List(List<'a>),
    Fn(FnDef<'a>),
    Call(Box<FnCall<'a>>),
    If(Box<If<'a>>),
    Loop(Loop<'a>),
    Var(Box<Var<'a>>),
    Lambda(Lambda<'a>),
    Return(Box<Expr<'a>>),
    Break(Box<Expr<'a>>),
    Identifier(Ident<'a>),
    Continue,
}

impl<'a> Expr<'a> {
    pub const fn new_literal(info: Info<'a>, value: Lit<'a>) -> Self {
        Self {
            info,
            expr: ExprType::Literal(value),
        }
    }

    pub const fn new_list(info: Info<'a>, value: List<'a>) -> Self {
        Self {
            info,
            expr: ExprType::List(value),
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
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.expr {
            ExprType::Literal(lit) => write!(f, "literal [{lit}]"),
            ExprType::List(list) => write!(f, "list [{list}]"),
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

#[derive(Debug, Clone, PartialEq)]
pub struct List<'a> {
    pub info: Info<'a>,
    pub car: Box<Expr<'a>>,
    pub cdr: Box<Expr<'a>>,
}

impl<'a> List<'a> {
    pub fn new(info: Info<'a>, car: Expr<'a>, cdr: Expr<'a>) -> Self {
        Self {
            info,
            car: Box::new(car),
            cdr: Box::new(cdr),
        }
    }

    pub const fn car(&self) -> &Expr<'a> {
        &self.car
    }

    pub const fn cdr(&self) -> &Expr<'a> {
        &self.cdr
    }

    pub const fn len(&self) -> usize {
        let mut len = 0;
        let mut list = self;
        while let ExprType::List(list_) = &list.cdr.expr {
            len += 1;
            list = list_;
        }
        len
    }

    pub fn new_cdr_empty(info: Info<'a>, car: Expr<'a>) -> Self {
        Self {
            info,
            car: Box::new(car),
            cdr: Box::new(Expr::new_literal(info, Lit::new_hempty(info))),
        }
    }

    pub fn push(&mut self, car: Expr<'a>) {
        self.cdr = Box::new(Expr::new_list(
            car.info,
            Self::new_cdr_empty(self.info, car),
        ));
    }

    pub fn from_vec(info: Info<'a>, mut exprs: Vec<Expr<'a>>) -> Option<List<'a>> {
        // should be a recursive function
        // to create the list
        // if the exprs is empty, set the cdr to be a hempty
        // otherwise, set the cdr to be a list
        if exprs.is_empty() {
            None
        } else {
            let first = exprs.first().unwrap().clone();
            let cdr = if exprs.len() == 1 {
                Expr::new_literal(first.info, Lit::new_hempty(first.info))
            } else {
                exprs.remove(0);
                Expr::new_list(
                    first.info,
                    Self::from_vec(first.info, exprs).expect("failed to create list from vec"),
                )
            };
            Some(Self::new(info, first.clone(), cdr))
        }
    }
}

impl Default for List<'_> {
    fn default() -> Self {
        let info = Info::default();
        Self::new(
            info,
            Expr::new_literal(info, Lit::new_hempty(info)),
            Expr::new_literal(info, Lit::new_hempty(info)),
        )
    }
}

impl<'a> TryFrom<Vec<Expr<'a>>> for List<'a> {
    type Error = String;

    fn try_from(exprs: Vec<Expr<'a>>) -> Result<Self, Self::Error> {
        // this will have be a recursive function
        // to create the list
        if exprs.is_empty() {
            Ok(Self::default())
        } else {
            let first = exprs.first().unwrap();
            Self::from_vec(first.info, exprs)
                .ok_or_else(|| "failed to create list from vec".to_string())
        }
    }
}

impl<'a> Iterator for List<'a> {
    type Item = Expr<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let ExprType::List(list) = &self.cdr.expr {
            let car = self.car.clone();
            self.car = list.car.clone();
            self.cdr = list.cdr.clone();
            Some(*car)
        } else {
            None
        }
    }
}

impl<'a> Display for List<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "list: [car: {} cdr: {} [{}]]",
            self.car, self.cdr, self.info
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lambda<'a> {
    pub info: Info<'a>,
    pub param_count: usize,
    pub extra_params: bool,
    pub body: List<'a>,
}

impl<'a> Lambda<'a> {
    pub const fn new(
        info: Info<'a>,
        param_count: usize,
        extra_params: bool,
        body: List<'a>,
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
            self.body
                .clone()
                .map(|e| format!("{e}"))
                .collect::<Vec<String>>()
                .join(" "),
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
    pub then: List<'a>,
    pub otherwise: List<'a>,
}

impl<'a> If<'a> {
    pub const fn new(
        info: Info<'a>,
        condition: Expr<'a>,
        then: List<'a>,
        otherwise: List<'a>,
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
            self.condition,
            self.then
                .clone()
                .map(|e| format!("{e}"))
                .collect::<Vec<String>>()
                .join(" "),
            self.otherwise
                .clone()
                .map(|e| format!("{e}"))
                .collect::<Vec<String>>()
                .join(" "),
            self.info
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Loop<'a> {
    pub info: Info<'a>,
    pub body: List<'a>,
}

impl<'a> Loop<'a> {
    pub const fn new(info: Info<'a>, body: List<'a>) -> Self {
        Self { info, body }
    }
}

impl<'a> Display for Loop<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "loop {} [{}]",
            self.body
                .clone()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(" "),
            self.info
        )
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
