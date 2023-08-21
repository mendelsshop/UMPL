pub use crate::{
    cst::{Boolean, PrintType, Varidiac},
    interior_mut::{MUTEX, RC},
};
pub mod lexer;

pub trait AstFLatten {
    fn flatten(&self) -> UMPL2Expr;
}
#[derive(Debug, Default, Clone, PartialEq)]
pub enum UMPL2Expr {
    Bool(Boolean),
    Number(f64),
    String(RC<str>),
    #[default]
    Hempty,
    Ident(RC<str>),
    Label(RC<str>),
    FnParam(usize),
    Scope(Vec<UMPL2Expr>),
    // Fanction(Fanction),
    // Let(RC<str>, Box<UMPL2Expr>),
    Link(RC<str>, Vec<RC<str>>),
    // control flow
    Skip,
    Stop(Box<UMPL2Expr>),
    ComeTo(RC<str>),

    Application(Application),
}

impl AstFLatten for UMPL2Expr {
    fn flatten(&self) -> UMPL2Expr {
        match self {
            UMPL2Expr::Bool(_)
            | UMPL2Expr::Number(_)
            | UMPL2Expr::String(_)
            | UMPL2Expr::Hempty
            | UMPL2Expr::Ident(_)
            | UMPL2Expr::Label(_)
            | UMPL2Expr::FnParam(_) => self.clone(),
            UMPL2Expr::Scope(s) => UMPL2Expr::Application(Application::new(
                s.iter().map(AstFLatten::flatten).collect(),
                PrintType::None,
            )),
            // UMPL2Expr::Fanction(_) => todo!(),
            // UMPL2Expr::Let(_, _) => todo!(),
            UMPL2Expr::Link(_, _) => todo!(),
            UMPL2Expr::Skip => todo!(),
            UMPL2Expr::Stop(_) => todo!(),
            UMPL2Expr::ComeTo(_) => todo!(),
            UMPL2Expr::Application(a) => UMPL2Expr::Application(Application::new(
                a.args().iter().map(AstFLatten::flatten).collect(),
                a.print,
            )),
        }
    }
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
