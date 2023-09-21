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
    Application(Vec<UMPL2Expr>),
    Label(RC<str>),
    // should simlify to ident or the like ...
    FnParam(usize),
    #[default]
    Hempty,
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
            Self::Application(a) => a.flatten(compiler),
            Self::Label(_) => todo!(),
            Self::FnParam(p) => compiler.const_symbol(&format!("'{p}'").into()),
            Self::Hempty => compiler.hempty(),
        }
    }
}

impl<'a, 'ctx> FlattenAst<'a, 'ctx> for Vec<UMPL2Expr> {
    fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx> {
        fn list_to_tree<'ctx>(
            list: Vec<UMPL2Expr>,
            compiler: &mut Compiler<'_, 'ctx>,
            n: usize,
        ) -> (StructValue<'ctx>, Vec<UMPL2Expr>) {
            if n == 0 {
                (compiler.hempty(), list)
            } else {
                let left_size = (n - 1) / 2;
                let (left_tree, mut non_left_tree) = list_to_tree(list, compiler, left_size);

                let this = non_left_tree.remove(0).flatten(compiler);

                let right_size = n - (left_size + 1);
                let (right_tree, remaining) = list_to_tree(non_left_tree, compiler, right_size);
                (compiler.const_cons(left_tree, this, right_tree), remaining)
            }
        }
        let n = self.len();
        let partial_tree = list_to_tree(self, compiler, n);

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
            Self::Application(f0) => f.debug_tuple("Application").field(&f0).finish(),
            Self::Label(f0) => write!(f, "@{f0}"),
            Self::FnParam(f0) => write!(f, "'{f0}"),
            Self::Hempty => write!(f, "hempty"),
        }
    }
}

impl<A: Into<RC<str>>> From<A> for UMPL2Expr {
    fn from(value: A) -> Self {
        Self::Ident(value.into())
    }
}

// impl From<Vec<UMPL2Expr>> for UMPL2Expr {
//     fn from(value: Vec<UMPL2Expr>) -> Self {
//         todo!()
//     }
// }

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
