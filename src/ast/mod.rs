pub use crate::{cst::{Boolean, Varidiac, PrintType}, interior_mut::{RC, MUTEX}};
pub mod lexer;
#[derive(Debug, Default, Clone, PartialEq)]
pub enum UMPL2Expr {
    Bool(Boolean),
    Number(f64),
    String(RC<str>),
    Scope(Vec<UMPL2Expr>),
    Ident(RC<str>),
    Stop(Box<UMPL2Expr>),
    Skip,
    ContiueDoing(Vec<UMPL2Expr>),
    Fanction(Fanction),
    Quoted(Box<UMPL2Expr>),
    Label(RC<str>),
    FnParam(usize),
    #[default]
    Hempty,
    Link(RC<str>, Vec<RC<str>>),
    Tree(Tree),
    Let(RC<str>, Box<UMPL2Expr>),
    ComeTo(RC<str>),
    Application(Application),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    pub inner: RC<MUTEX<(UMPL2Expr, UMPL2Expr, UMPL2Expr)>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fanction {
    name: Option<char>,
    param_count: usize,
    optinal_params: Option<Varidiac>,
    scope: Vec<UMPL2Expr>,
    lazy: bool,
}

impl Fanction {
    #[must_use]
    pub fn new(
        name: Option<char>,
        param_count: usize,
        optinal_params: Option<Varidiac>,
        scope: Vec<UMPL2Expr>,
    ) -> Self {
        Self {
            name,
            param_count,
            optinal_params,
            scope,
            lazy: true,
        }
    }

    pub fn scope_mut(&mut self) -> &mut Vec<UMPL2Expr> {
        &mut self.scope
    }

    #[must_use]
    pub const fn name(&self) -> Option<char> {
        self.name
    }

    #[must_use]
    pub const fn optinal_params(&self) -> Option<&Varidiac> {
        self.optinal_params.as_ref()
    }

    #[must_use]
    pub const fn param_count(&self) -> usize {
        self.param_count
    }

    #[must_use]
    pub fn scope(&self) -> &[UMPL2Expr] {
        self.scope.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Application {
    args: Vec<UMPL2Expr>,
    print: PrintType,
}

impl Application {
    #[must_use]
    pub fn new(args: Vec<UMPL2Expr>, print: PrintType) -> Self {
        Self { args, print }
    }

    pub fn args_mut(&mut self) -> &mut Vec<UMPL2Expr> {
        &mut self.args
    }

    #[must_use]
    pub fn args(&self) -> &[UMPL2Expr] {
        self.args.as_ref()
    }
}
