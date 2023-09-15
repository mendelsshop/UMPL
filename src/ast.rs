use std::fmt::Display;

use inkwell::values::StructValue;

use crate::{
    codegen::Compiler,
    interior_mut::{MUTEX, RC},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Tree {
    pub inner: RC<MUTEX<(UMPL2Expr, UMPL2Expr, UMPL2Expr)>>,
}

// TODO: flatten trait for quotation
pub trait FlattenAst<'a, 'ctx> {
    fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx>;
}

#[derive(Clone, Default, PartialEq, Debug)]
pub enum UMPL2Expr {
    Bool(Boolean),
    Number(f64),
    String(RC<str>),
    // could become (begin exprs)
    Scope(Vec<UMPL2Expr>),
    Ident(RC<str>),
    // // second 2 are scopes
    // If(Box<If>),
    // // second 2 are scopes
    // Unless(Box<Unless>),
    // could become (stop expr)
    Stop(Box<UMPL2Expr>),
    // could become (skip)
    Skip,
    // // last one is scope
    // Until(Box<Until>),
    // // last one is scope
    // GoThrough(Box<GoThrough>),
    // // last one is scope
    // ContiueDoing(Vec<UMPL2Expr>),
    // // last one is scope
    // Fanction(Fanction),
    Application(Application),
    // could become (quote expr)
    Quoted(Box<UMPL2Expr>),
    Label(RC<str>),
    FnParam(usize),
    #[default]
    Hempty,
    // Link(RC<str>, Vec<RC<str>>),
    // Let(RC<str>, Box<UMPL2Expr>),
    ComeTo(RC<str>),
    // Module(Module),
}
#[derive(Clone, Default, PartialEq, Debug)]
pub struct Module {
    name: String,
    inner: Vec<UMPL2Expr>,
}

impl Module {
    #[must_use]
    pub fn new(name: String, inner: Vec<UMPL2Expr>) -> Self {
        Self { name, inner }
    }

    pub fn inner_mut(&mut self) -> &mut Vec<UMPL2Expr> {
        &mut self.inner
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    #[must_use]
    pub fn inner(&self) -> &[UMPL2Expr] {
        self.inner.as_ref()
    }
}

impl<'a, 'ctx> FlattenAst<'a, 'ctx> for UMPL2Expr {
    fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx> {
        match self {
            Self::Bool(b) => compiler.const_boolean(b),
            Self::Number(n) => compiler.const_number(n),
            Self::String(c) => compiler.const_string(&c),
            Self::Scope(_) => unreachable!(),
            Self::Ident(i) => compiler.const_symbol(&i),
            // Self::If(_) => todo!(),
            // Self::Unless(_) => todo!(),
            Self::Stop(_) => todo!(),
            Self::Skip => todo!(),
            // Self::Until(_) => todo!(),
            // Self::GoThrough(_) => todo!(),
            // Self::ContiueDoing(_) => todo!(),
            // Self::Fanction(_) => todo!(),
            Self::Application(a) => a.flatten(compiler),
            Self::Quoted(_) => todo!(),
            Self::Label(_) => todo!(),
            Self::FnParam(p) => compiler.const_symbol(&format!("'{p}'").into()),
            Self::Hempty => compiler.hempty(),
            // Self::Link(_, _) => todo!(),
            // Self::Let(_, _) => todo!(),
            Self::ComeTo(_) => todo!(),
            // Self::Module(_) => todo!(),
        }
    }
}

impl<'a, 'ctx> FlattenAst<'a, 'ctx> for Vec<UMPL2Expr> {
    fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx> {
        fn fun_name<'ctx>(
            list: Vec<UMPL2Expr>,
            compiler: &mut Compiler<'_, 'ctx>,
            n: usize,
        ) -> (StructValue<'ctx>, Vec<UMPL2Expr>) {
            if n == 0 {
                (compiler.hempty(), list)
            } else {
                let left_size = (n - 1) / 2;
                let (left_tree, mut non_left_tree) = fun_name(list, compiler, left_size);

                let this = non_left_tree.remove(0).flatten(compiler);

                let right_size = n - (left_size + 1);
                let (right_tree, remaining) = fun_name(non_left_tree, compiler, right_size);
                (compiler.const_cons(left_tree, this, right_tree), remaining)
            }
        }
        let n = self.len();
        let partial_tree = fun_name(self, compiler, n);

        partial_tree.0
    }
}

impl core::fmt::Display for UMPL2Expr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Bool(f0) => write!(f, "{f0}"),
            Self::Number(f0) => write!(f, "{f0}"),
            Self::String(f0) => write!(f, "{f0}"),
            Self::Scope(f0) => f.debug_tuple("Scope").field(&f0).finish(),
            Self::Ident(f0) => write!(f, "{f0}"),
            // Self::If(f0) => f.debug_tuple("If").field(&f0).finish(),
            // Self::Unless(f0) => f.debug_tuple("Unless").field(&f0).finish(),
            Self::Stop(f0) => f.debug_tuple("Stop").field(&f0).finish(),
            Self::Skip => write!(f, "skip"),
            // Self::Until(f0) => f.debug_tuple("Until").field(&f0).finish(),
            // Self::GoThrough(f0) => f.debug_tuple("GoThrough").field(&f0).finish(),
            // Self::ContiueDoing(f0) => f.debug_tuple("ContiueDoing").field(&f0).finish(),
            // Self::Fanction(f0) => f.debug_tuple("Fanction").field(&f0).finish(),
            Self::Application(f0) => f.debug_tuple("Application").field(&f0).finish(),
            Self::Quoted(f0) => f.debug_tuple("Quoted").field(&f0).finish(),
            Self::Label(f0) => write!(f, "@{f0}"),
            Self::FnParam(f0) => write!(f, "'{f0}"),
            Self::Hempty => write!(f, "hempty"),
            // Self::Link(f0, f1) => f.debug_tuple("Link").field(&f0).field(&f1).finish(),
            // Self::Let(f0, f1) => f.debug_tuple("Let").field(&f0).field(&f1).finish(),
            Self::ComeTo(f0) => write!(f, "Come to {f0}"),
            // Self::Module(arg0) => f.debug_tuple("Module").field(arg0).finish(),
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
}

impl<'ctx, 'a> FlattenAst<'a, 'ctx> for Application {
    fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx> {
        self.args.flatten(compiler)
    }
}

impl Application {
    #[must_use]
    pub fn new(args: Vec<UMPL2Expr>) -> Self {
        Self { args }
    }

    pub fn args_mut(&mut self) -> &mut Vec<UMPL2Expr> {
        &mut self.args
    }

    #[must_use]
    pub fn args(&self) -> &[UMPL2Expr] {
        self.args.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GoThrough {
    ident: RC<str>,
    iter: UMPL2Expr,
    scope: Vec<UMPL2Expr>,
}

impl GoThrough {
    #[must_use]
    pub fn new(ident: RC<str>, iter: UMPL2Expr, scope: Vec<UMPL2Expr>) -> Self {
        Self { ident, iter, scope }
    }

    pub fn scope_mut(&mut self) -> &mut Vec<UMPL2Expr> {
        &mut self.scope
    }

    pub fn iter_mut(&mut self) -> &mut UMPL2Expr {
        &mut self.iter
    }

    #[must_use]
    pub fn scope(&self) -> &[UMPL2Expr] {
        self.scope.as_ref()
    }

    #[must_use]
    pub const fn iter(&self) -> &UMPL2Expr {
        &self.iter
    }

    #[must_use]
    pub fn ident(&self) -> &str {
        self.ident.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Until {
    cond: UMPL2Expr,
    scope: Vec<UMPL2Expr>,
}

impl Until {
    #[must_use]
    pub fn new(cond: UMPL2Expr, scope: Vec<UMPL2Expr>) -> Self {
        Self { cond, scope }
    }

    pub fn scope_mut(&mut self) -> &mut Vec<UMPL2Expr> {
        &mut self.scope
    }

    pub fn cond_mut(&mut self) -> &mut UMPL2Expr {
        &mut self.cond
    }

    #[must_use]
    pub const fn cond(&self) -> &UMPL2Expr {
        &self.cond
    }

    #[must_use]
    pub fn scope(&self) -> &[UMPL2Expr] {
        self.scope.as_ref()
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct If {
    cond: UMPL2Expr,
    cons: Vec<UMPL2Expr>,
    alt: Vec<UMPL2Expr>,
}

impl If {
    #[must_use]
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

    #[must_use]
    pub const fn cond(&self) -> &UMPL2Expr {
        &self.cond
    }

    #[must_use]
    pub fn cons(&self) -> &[UMPL2Expr] {
        self.cons.as_ref()
    }

    #[must_use]
    pub fn alt(&self) -> &[UMPL2Expr] {
        self.alt.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Unless {
    cond: UMPL2Expr,
    cons: Vec<UMPL2Expr>,
    alt: Vec<UMPL2Expr>,
}

impl Unless {
    #[must_use]
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

    #[must_use]
    pub const fn cond(&self) -> &UMPL2Expr {
        &self.cond
    }

    #[must_use]
    pub fn cons(&self) -> &[UMPL2Expr] {
        self.cons.as_ref()
    }

    #[must_use]
    pub fn alt(&self) -> &[UMPL2Expr] {
        self.alt.as_ref()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boolean {
    /// |
    False = 0,
    /// &
    True = 1,
    /// ?
    Maybee,
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::False => write!(f, "false"),
            Self::True => write!(f, "true"),
            Self::Maybee => write!(f, "maybe"),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Varidiac {
    /// denotes that besides the usual arg count function will take extra args
    /// in form of tree (requires at least 1 arg)
    AtLeast1,
    /// denotes that besides the usual arg count function will take extra args
    /// in form of tree (requires at least 0 args)
    AtLeast0,
}

macro_rules! get_expr {
    ($type:ident, $ret:ty, $method_name:ident) => {
        impl UMPL2Expr {
            #[must_use]
            pub const fn $method_name(&self) -> Option<&$ret> {
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
            #[allow(clippy::missing_const_for_fn)] // taking self doesnt work well with const fn (something about destructors)
            #[must_use]
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
