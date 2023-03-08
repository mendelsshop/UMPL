
use std::fmt::{self, Debug, Display};



    use crate::{
        token::{Info}, 
    };

    #[derive(Debug, Clone, PartialEq)]
    pub struct Expr<'a> {
        pub info: Info<'a>,
        pub expr: ExprType<'a>,
    }
    #[derive(Debug, Clone, PartialEq)]
    pub enum ExprType<'a> {
        Literal(Lit<'a>),
        List(Box<List<'a>>),
        Fn(Box<FnDef<'a>>),
        Call(Box<FnCall<'a>>),
        If(Box<If<'a>>),
        Loop(Box<Loop<'a>>),
        Var(Box<Var<'a>>),
        Lambda(Lambda<'a>),
        Return(Box<Expr<'a>>),
        Break(Box<Expr<'a>>),
        Continue,
    }

    impl<'a> Expr<'a> {
        pub fn new_literal(info: Info<'a>, value: Lit<'a>) -> Self {
            Self {
                info,
                expr: ExprType::Literal(value),
            }
        }

        pub fn new_list(info: Info<'a>, value: List<'a>) -> Self {
            Self {
                info,
                expr: ExprType::List(Box::new(value)),
            }
        }

        pub fn new_fn(info: Info<'a>, value: FnDef<'a>) -> Self {
            Self {
                info,
                expr: ExprType::Fn(Box::new(value)),
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

        pub fn new_loop(info: Info<'a>, value: Loop<'a>) -> Self {
            Self {
                info,
                expr: ExprType::Loop(Box::new(value)),
            }
        }

        pub fn new_var(info: Info<'a>, value: Var<'a>) -> Self {
            Self {
                info,
                expr: ExprType::Var(Box::new(value)),
            }
        }

        pub fn new_lambda(info: Info<'a>, value: Lambda<'a>) -> Self {
            Self {
                info,
                expr: ExprType::Lambda(value),
            }
        }

        pub fn new_return(info: Info<'a>, value: Option<Expr<'a>>) -> Self {
            Self {
                info,
                expr: ExprType::Return(Box::new(value.unwrap_or_else(|| Expr::new_literal(info, Lit::new_hempty(info))))),
            }
        }

        pub fn new_break(info: Info<'a>, value: Option<Expr<'a>>) -> Self {
            Self {
                info,
                expr: ExprType::Break(Box::new(value.unwrap_or_else(|| Expr::new_literal(info, Lit::new_hempty(info))))),
            }
        }

        pub fn new_continue(info: Info<'a>) -> Self {
            Self {
                info,
                expr: ExprType::Continue,
            }
        }

    }

    impl<'a> Display for Expr<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.expr {
                ExprType::Literal(lit) => write!(f, "literal [{}]", lit),
                ExprType::List(list) => write!(f, "list [{}]", list),
                ExprType::Fn(fn_def) => write!(f, "fn [{}]", fn_def),
                ExprType::Call(call) => write!(f, "call [{}]", call),
                ExprType::If(if_) => write!(f, "if [{}]", if_),
                ExprType::Loop(loop_def) => write!(f, "loop [{}]", loop_def),
                ExprType::Var(var) => write!(f, "var [{}]", var),
                ExprType::Lambda(lambda) => write!(f, "lambda [{}]", lambda),
                ExprType::Return(return_) => write!(f, "return [{}]", return_),
                ExprType::Break(break_) => write!(f, "break [{}]", break_),
                ExprType::Continue => write!(f, "continue"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Lit<'a> {
        pub info: Info<'a>,
        pub value: LitType,
    }

    impl<'a> Lit<'a> {
        pub fn new_number(info: Info<'a>, value: f64) -> Self {
            Self {
                info,
                value: LitType::Number(value),
            }
        }

        pub fn new_string(info: Info<'a>, value: String) -> Self {
            Self {
                info,
                value: LitType::String(value),
            }
        }

        pub fn new_boolean(info: Info<'a>, value: bool) -> Self {
            Self {
                info,
                value: LitType::Boolean(value),
            }
        }

        pub fn new_hempty(info: Info<'a>) -> Self {
            Self {
                info,
                value: LitType::Hempty,
            }
        }

        pub fn new_file(info: Info<'a>, value: String) -> Self {
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
                LitType::String(s) => write!(f, "string: {}", s),
                LitType::Number(n) => write!(f, "number: {}", n),
                LitType::Boolean(b) => write!(f, "boolean: {}", b),
                LitType::File(s) => write!(f, "file: {}", s),
                LitType::Hempty => write!(f, "hemty"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct List<'a> {
        pub info: Info<'a>,
        pub car: Expr<'a>,
        pub cdr: Expr<'a>,
    }

    impl<'a> List<'a> {
        pub fn new(info: Info<'a>, car: Expr<'a>, cdr: Expr<'a>) -> Self {
            Self { info, car, cdr }
        }

        pub fn car(&self) -> &Expr<'a> {
            &self.car
        }

        pub fn cdr(&self) -> &Expr<'a> {
            &self.cdr
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
        pub body: Vec<Expr<'a>>,
    }

    impl<'a> Lambda<'a> {
        pub fn new(
            info: Info<'a>,
            param_count: usize,
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
    }

    impl<'a> Display for Lambda<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "fn: [with {} {} parameters {} [{}]]",
                if self.extra_params { "at least" } else { "" },
                self.param_count,
                self.body
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                self.info
            )
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FnDef<'a> {
        pub info: Info<'a>,
        pub name: String,
        inner: Lambda<'a>,
    }

    impl<'a> FnDef<'a> {
        pub fn new(
            info: Info<'a>,
            name: String,
            inner: Lambda<'a>,
        ) -> Self {
            Self {
                info,
                name,
                inner,
            }
        }
    }

    impl<'a> Display for FnDef<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{} defined as {} at [{}]", self.name, self.inner , self.info
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

                self.args.iter().map(|e| e.to_string()).collect::<String>(),
                self.args.len(),
                self.info
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
        pub fn new(
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
                "if {} then {} else {} [{}]",
                self.condition,
                self.then
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                self.otherwise
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                self.info
            )
        }
    }


    #[derive(Debug, Clone, PartialEq)]
    pub struct Loop<'a> {
        pub info: Info<'a>,
        pub body: Vec<Expr<'a>>,
    }

    impl<'a> Loop<'a> {
        pub fn new(info: Info<'a>, body: Vec<Expr<'a>>) -> Self {
            Self { info, body }
        }
    }

    impl<'a> Display for Loop<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "loop {} [{}]",
                self.body
                    .iter()
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
        pub name: &'a str,
        pub value: Expr<'a>,
    }

    impl<'a> Var<'a> {
        pub fn new(info: Info<'a>, name: &'a str, value: Expr<'a>) -> Self {
            Self { info, name, value }
        }
    }

    impl<'a> Display for Var<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "var {} = {} [{}]", self.name, self.value, self.info)
        }
    }

    