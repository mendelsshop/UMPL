use std::{
    cell::{Ref, RefCell},
    fmt::{self, Debug, Display},
    rc::Rc,
};

use crate::{
    error::error,
    token::{BuiltinFunction, Info},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr<'a> {
    pub info: Info<'a>,
    pub expr: ExprType<'a>,
}

impl<'a> Default for Expr<'a> {
    fn default() -> Self {
        Self {
            info: Info::default(),
            expr: ExprType::Literal(Lit::new_hempty(Info::default())),
        }
    }
}

pub fn to_string(code: &[Expr<'_>]) -> String {
    let mut to_return = String::new();
    for expr in code {
        to_return.push_str(&format!("{expr}"));
    }
    to_return
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprType<'a> {
    Literal(Lit<'a>),
    /// function definition
    Fn(FnDef<'a>),
    Call(Box<FnCall<'a>>),
    If(Box<If<'a>>),
    Loop(Loop<'a>),
    /// variable declarations
    Var(Box<Var<'a>>),
    Lambda(Lambda<'a>),
    Return(Box<Expr<'a>>),
    Break(Box<Expr<'a>>),
    /// variable, function, refrences
    Identifier(Ident<'a>),
    Cons(Cons<'a>),
    Module(Module<'a>),
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

    pub const fn new_cons(info: Info<'a>, value: Cons<'a>) -> Self {
        Self {
            info,
            expr: ExprType::Cons(value),
        }
    }

    pub const fn new_mod(info: Info<'a>, value: Module<'a>) -> Self {
        Self {
            info,
            expr: ExprType::Module(value),
        }
    }
}

impl<'a> Display for ExprType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ExprType::Literal(lit) => write!(f, "{lit}"),
            ExprType::Fn(fn_def) => write!(f, "{fn_def}"),
            ExprType::Call(call) => write!(f, "{call}"),
            ExprType::If(if_) => write!(f, "{if_}"),
            ExprType::Loop(loop_def) => write!(f, "{loop_def}"),
            ExprType::Var(var) => write!(f, "{var}"),
            ExprType::Lambda(lambda) => write!(f, "{lambda}"),
            ExprType::Return(return_) => write!(f, "{return_}"),
            ExprType::Break(break_) => write!(f, "{break_}"),
            ExprType::Continue => write!(f, "continue"),
            ExprType::Identifier(ident) => write!(f, "{ident}"),
            ExprType::Cons(cons) => write!(f, "{cons}"),
            ExprType::Module(module) => write!(f, "{module}"),
        }
    }
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lit<'a> {
    pub info: Info<'a>,
    pub value: LitType<'a>,
}

impl<'a> Lit<'a> {
    pub const fn new_number(info: Info<'a>, value: f64) -> Self {
        Self {
            info,
            value: LitType::Number(value),
        }
    }

    pub const fn new_string(info: Info<'a>, value: &'a str) -> Self {
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

    pub const fn new_file(info: Info<'a>, value: &'a str) -> Self {
        Self {
            info,
            value: LitType::File(value),
        }
    }
}

impl<'a> Display for Lit<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LitType<'a> {
    String(&'a str),
    Number(f64),
    Boolean(bool),
    File(&'a str),
    Hempty,
}

impl fmt::Display for LitType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(s) | Self::File(s) => write!(f, "{s}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Hempty => write!(f, "hemty"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cons<'a> {
    pub info: Info<'a>,
    pub car: Rc<RefCell<Expr<'a>>>,
    pub cdr: Rc<RefCell<Expr<'a>>>,
}

// TODO: impl Iterator for Cons<'a, Expr>
impl<'a> Cons<'a> {
    pub fn new(info: Info<'a>, car: Expr<'a>, cdr: Expr<'a>) -> Self {
        Self {
            info,
            car: Rc::new(RefCell::new(car)),
            cdr: Rc::new(RefCell::new(cdr)),
        }
    }

    pub fn car(&self) -> Ref<'_, Expr<'a>> {
        self.car.borrow()
    }

    pub fn cdr(&self) -> Ref<'_, Expr<'a>> {
        self.cdr.borrow()
    }

    pub fn len(&self) -> usize {
        // recursively get the length of the list
        match &self.cdr.borrow().expr {
            ExprType::Cons(cons) => 1 + cons.len(),
            _ => 1,
        }
    }

    pub fn new_cdr_empty(info: Info<'a>, car: Expr<'a>) -> Self {
        Self {
            info,
            car: Rc::new(RefCell::new(car)),
            cdr: Rc::new(RefCell::new(Expr::new_literal(info, Lit::new_hempty(info)))),
        }
    }

    pub fn set_cdr(&mut self, cdr: Expr<'a>, recursive: bool) {
        // check if cdr is a list
        // if it is, if recursive, set go to the end of the list and set the cdr
        // if not recursive, set the cdr to the list
        // if it is not a list, set the cdr to a new list with the cdr as the car
        let cdr = match cdr.expr {
            ExprType::Cons(_) => cdr,
            _ => Expr::new_cons(self.info, Cons::new_cdr_empty(self.info, cdr)),
        };
        match match self.cdr.try_borrow_mut() {
            Ok(val) => val,
            Err(err) => error(self.info, format!("refcell borrow error: {err}")),
        }
        .expr
        {
            ExprType::Cons(ref mut cons) => {
                if recursive {
                    cons.set_cdr(cdr, recursive);
                } else {
                    self.cdr.replace(cdr);
                }
            }
            _ => {
                self.cdr.replace(cdr);
            }
        }
    }
}

impl<'a> From<Cons<'a>> for Vec<Expr<'a>> {
    fn from(value: Cons<'a>) -> Self {
        let mut list = value;
        let mut vec = Self::new();
        vec.push(list.car.replace(Expr::default()));
        while let ExprType::Cons(cons) = list.cdr.replace(Expr::default()).expr {
            list = cons;
            vec.push(list.car.replace(Expr::default()));
        }
        vec
    }
}

impl<'a> Display for Cons<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({} . {})",
            match self.car.try_borrow() {
                Ok(value) => value,
                Err(err) => error(self.info, format!("refcell borrow error: {err}")),
            },
            match self.cdr.try_borrow() {
                Ok(value) => value,
                Err(err) => error(self.info, format!("refcell borrow error: {err}")),
            }
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Lambda<'a> {
    pub info: Info<'a>,
    pub param_count: u64,
    pub extra_params: bool,
    body: Vec<Expr<'a>>,
}

impl<'a> Lambda<'a> {
    pub const fn new(
        info: Info<'a>,
        param_count: u64,
        extra_params: bool,
        body: Vec<Expr<'a>>,
    ) -> Self {
        Self {
            info,
            param_count,
            extra_params,
            body,
        }
    }

    pub const fn body(&self) -> &Vec<Expr<'a>> {
        &self.body
    }
}

impl<'a> Display for Lambda<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "lambda{} {}: {}",
            if self.extra_params { " at least" } else { "" },
            self.param_count,
            to_string(&self.body),
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

    // not allowed to be const because it destroys Self
    #[allow(clippy::missing_const_for_fn)]
    pub fn take_inner(self) -> Lambda<'a> {
        self.inner
    }
}

impl<'a> Display for FnDef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}: [{}]", self.name, self.inner, self.info)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            "fn call with {} {}",
            self.args
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<String>(),
            self.args.len(),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct If<'a> {
    pub info: Info<'a>,
    pub condition: Expr<'a>,
    pub then: Vec<Expr<'a>>,
    pub otherwise: Vec<Expr<'a>>,
}

impl<'a> If<'a> {
    pub const fn new(
        info: Info<'a>,
        condition: Expr<'a>,
        then: Vec<Expr<'a>>,
        otherwise: Vec<Expr<'a>>,
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
            "if {} then  {} otherwise {} [{}]",
            self.condition,
            to_string(&self.then),
            to_string(&self.otherwise),
            self.info,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Loop<'a> {
    pub info: Info<'a>,
    pub body: Vec<Expr<'a>>,
}

impl<'a> Loop<'a> {
    pub const fn new(info: Info<'a>, body: Vec<Expr<'a>>) -> Self {
        Self { info, body }
    }
}

impl<'a> Display for Loop<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "loop {} [{}]", to_string(&self.body), self.info)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var<'a> {
    pub info: Info<'a>,
    pub name: &'a str,
    pub value: Expr<'a>,
}

impl<'a> Var<'a> {
    pub const fn new(info: Info<'a>, name: &'a str, value: Expr<'a>) -> Self {
        Self { info, name, value }
    }
}

impl<'a> Display for Var<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "var {} value {} [{}]", self.name, self.value, self.info)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    pub fn is_empty(&self) -> bool {
        self.interlaced.is_empty()
    }

    pub fn len(&self) -> usize {
        self.interlaced.len()
    }

    // takes a self and a method/closure to change the main value and retruns an owned Self with the modified value
    pub fn changed<C: Debug + Clone + Display, F: FnOnce(A) -> C>(
        self,
        new: F,
    ) -> Interlaced<C, B> {
        Interlaced::new(new(self.main), self.interlaced)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
pub enum IdentType<'a> {
    Var(Interlaced<&'a str, Accesor>),
    FnIdent(Interlaced<char, char>),
    Builtin(BuiltinFunction),
    // TODO: if function in function, we loose original funtion parameters
    FnParam(Interlaced<u64, Accesor>),
}

impl Display for IdentType<'_> {
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
            Self::FnParam(param) => write!(
                f,
                "function parameter {}",
                match param.interlaced_to_string(".") {
                    s if s.is_empty() => String::new(),
                    s => format!(".{s}"),
                }
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident<'a> {
    pub info: Info<'a>,
    pub ident_type: IdentType<'a>,
}

impl<'a> Ident<'a> {
    pub const fn new(info: Info<'a>, ident_type: IdentType<'a>) -> Self {
        Self { info, ident_type }
    }
}

impl<'a> Display for Ident<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}]", self.ident_type, self.info)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module<'a> {
    info: Info<'a>,
    name: char,
    mod_type: ModuleType<'a>,
}

impl<'a> fmt::Display for Module<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "module {} type {} [{}]",
            self.name,
            match self.mod_type {
                ModuleType::Inline(_) => "in file".to_string(),
                ModuleType::File(file) => format!("external file: {file}"),
            },
            self.info
        )
    }
}

impl<'a> Module<'a> {
    pub const fn new(info: Info<'a>, name: char, mod_type: ModuleType<'a>) -> Self {
        Self {
            info,
            name,
            mod_type,
        }
    }

    pub const fn get_name(&self) -> char {
        self.name
    }

    pub const fn get_type(&self) -> &ModuleType<'a> {
        &self.mod_type
    }

    pub const fn get_info(&self) -> &Info<'a> {
        &self.info
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModuleType<'a> {
    Inline(Vec<Expr<'a>>),
    File(&'a str),
}
