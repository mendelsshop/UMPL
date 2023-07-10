use crate::interior_mut::{MUTEX, RC};

#[derive(Clone, Debug, PartialEq)]
pub struct Tree {
    pub inner: RC<MUTEX<(UMPL2Expr, UMPL2Expr, UMPL2Expr)>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum UMPL2Expr {
    Bool(Boolean),
    Number(f64),
    String(RC<str>),
    Scope(Vec<UMPL2Expr>),
    Ident(RC<str>),
    // second 2 are scopes
    If(Box<If>),
    // second 2 are scopes
    Unless(Box<Unless>),
    Stop(Box<UMPL2Expr>),
    Skip,
    // last one is scope
    Until(Box<Until>),
    // last one is scope
    GoThrough(Box<GoThrough>),
    // last one is scope
    ContiueDoing(Vec<UMPL2Expr>),
    // last one is scope
    Fanction(Fanction),
    Application(Application),
    Quoted(Box<UMPL2Expr>),
    Label(RC<str>),
    FnParam(usize),
    #[default]
    Hempty,
    Link(RC<str>, Vec<RC<str>>),
    Tree(Tree),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fanction {
    name: char,
    param_count: usize,
    optinal_params: Option<Varidiac>,
    scope: Vec<UMPL2Expr>,
}

impl Fanction {
    pub fn new(
        name: char,
        param_count: usize,
        optinal_params: Option<Varidiac>,
        scope: Vec<UMPL2Expr>,
    ) -> Self {
        Self {
            name,
            param_count,
            optinal_params,
            scope,
        }
    }

    pub fn scope_mut(&mut self) -> &mut Vec<UMPL2Expr> {
        &mut self.scope
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Application {
    args: Vec<UMPL2Expr>,
    print: PrintType,
}

impl Application {
    pub fn new(args: Vec<UMPL2Expr>, print: PrintType) -> Self {
        Self { args, print }
    }

    pub fn args_mut(&mut self) -> &mut Vec<UMPL2Expr> {
        &mut self.args
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GoThrough {
    ident: RC<str>,
    iter: UMPL2Expr,
    scope: Vec<UMPL2Expr>,
}

impl GoThrough {
    pub fn new(ident: RC<str>, iter: UMPL2Expr, scope: Vec<UMPL2Expr>) -> Self {
        Self { ident, iter, scope }
    }

    pub fn scope_mut(&mut self) -> &mut Vec<UMPL2Expr> {
        &mut self.scope
    }

    pub fn iter_mut(&mut self) -> &mut UMPL2Expr {
        &mut self.iter
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Until {
    cond: UMPL2Expr,
    scope: Vec<UMPL2Expr>,
}

impl Until {
    pub fn new(cond: UMPL2Expr, scope: Vec<UMPL2Expr>) -> Self {
        Self { cond, scope }
    }

    pub fn scope_mut(&mut self) -> &mut Vec<UMPL2Expr> {
        &mut self.scope
    }

    pub fn cond_mut(&mut self) -> &mut UMPL2Expr {
        &mut self.cond
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct If {
    cond: UMPL2Expr,
    cons: Vec<UMPL2Expr>,
    alt: Vec<UMPL2Expr>,
}

impl If {
    pub fn new(cond: UMPL2Expr, cons: Vec<UMPL2Expr>, alt: Vec<UMPL2Expr>) -> Self {
        Self { cond, cons, alt }
    }

    pub fn alt_mut(&mut self) -> &mut Vec<UMPL2Expr> {
        &mut self.alt
    }

    pub fn cons_mut(&mut self) -> &mut Vec<UMPL2Expr> {
        &mut self.cons
    }

    pub fn cond_mut(&mut self) -> &mut UMPL2Expr {
        &mut self.cond
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Unless {
    cond: UMPL2Expr,
    cons: Vec<UMPL2Expr>,
    alt: Vec<UMPL2Expr>,
}

impl Unless {
    pub fn new(cond: UMPL2Expr, alt: Vec<UMPL2Expr>, cons: Vec<UMPL2Expr>) -> Self {
        Self { cond, cons, alt }
    }

    pub fn alt_mut(&mut self) -> &mut Vec<UMPL2Expr> {
        &mut self.alt
    }

    pub fn cons_mut(&mut self) -> &mut Vec<UMPL2Expr> {
        &mut self.cons
    }

    pub fn cond_mut(&mut self) -> &mut UMPL2Expr {
        &mut self.cond
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Boolean {
    /// &
    True,
    /// |
    False,
    /// ?
    Maybee,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Varidiac {
    /// denotes that besides the usual arg count function will take extra args
    /// in form of tree (requires at least 1 arg)
    AtLeast1,
    /// denotes that besides the usual arg count function will take extra args
    /// in form of tree (requires at least 0 args)
    AtLeast0,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrintType {
    None,
    Print,
    PrintLN,
}

macro_rules! get_expr {
    ($type:ident, $ret:ty, $method_name:ident) => {
        impl UMPL2Expr {
            pub fn $method_name(&self) -> Option<&$ret> {
                match self {
                    Self::$type(t) => Some(t),
                    _ => None,
                }
            }
        }
    };
}

get_expr! {Scope, Vec<UMPL2Expr>, get_scope}

macro_rules! get_expr_owned {
    ($type:ident, $ret:ty, $method_name:ident) => {
        impl UMPL2Expr {
            pub fn $method_name(self) -> Option<$ret> {
                match self {
                    Self::$type(t) => Some(t),
                    _ => None,
                }
            }
        }
    };
}

get_expr_owned! {Scope, Vec<UMPL2Expr>, get_scope_owned}
