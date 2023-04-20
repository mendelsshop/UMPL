use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self},
    fs::File,
    mem::swap,
    rc::Rc,
};

use crate::{
    error::{arg_error, error},
    lexer::Lexer,
    parser::{
        rules::{
            Accesor, Cons, Expr, ExprType, FnDef, Ident, IdentType, Interlaced, Lambda, Lit,
            LitType, Module, ModuleType, PrintType, Var,
        },
        Parser,
    },
    token::{BuiltinFunction, Info},
};

// used to store function arguments and normal variables in the same scope
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VarType<'a> {
    FnArg(u64),
    Var(&'a str),
}

impl<'a> fmt::Display for VarType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FnArg(i) => write!(f, "function argument ${i}"),
            Self::Var(v) => write!(f, "variable {v}",),
        }
    }
}

#[derive(Debug, Default)]
pub struct Scope<'a> {
    vars: HashMap<VarType<'a>, Rc<RefCell<Expr<'a>>>>,
    // fn_params: HashMap<u64, Rc<RefCell<Expr<'a>>>>,
    functions: HashMap<Interlaced<char, char>, Lambda<'a>>,
    files: HashMap<String, File>,
    parent_scope: Option<Box<Scope<'a>>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            functions: HashMap::new(),
            parent_scope: None,
            files: HashMap::new(),
            // fn_params: HashMap::new(),
        }
    }
    pub fn new_with_parent(parent: Box<Self>) -> Self {
        Self {
            parent_scope: Some(parent),
            ..Default::default()
        }
    }
    pub fn set_var(
        &mut self,
        name: &Interlaced<VarType<'a>, Accesor>,
        value: Expr<'a>,
        recurse: bool,
        info: Info<'_>,
    ) {
        // if we are setting the car or cdr of a list
        if !name.is_empty() {
            if recurse {
                todo!("set var in parent scope");
            } else {
                // check if the var exists because we are attempting to change it
                if self.has_var(name.main, false) {
                    // get the var
                    let var = self.get_var(name, info);
                    var.replace(value);
                } else {
                    // if the var does not exist then we error
                    error(info, format!("variable {} does not exist", name.main));
                }
            }
        } else if recurse {
            todo!("set var in parent scope");
        } else {
            self.vars.insert(name.main, Rc::new(RefCell::new(value)));
        }
    }

    // TODO: possibly make this _mut and have another get_var that doesn't return a mutable reference
    pub fn get_var(
        &mut self,
        name: &Interlaced<VarType<'_>, Accesor>,
        info: Info<'_>,
    ) -> Rc<RefCell<Expr<'a>>> {
        match self.vars.get(&name.main) {
            Some(v) => {
                match (v, name.len()) {
                    // match the type of the variable and accesor length
                    // if the accesor length is 0 then it can be any type
                    // if the accesor length is > 0 then it must be a list
                    (v, 0) => Rc::clone(v),
                    (v, _)
                        if matches!(
                            match v.try_borrow() {
                                Ok(val) => val,
                                Err(err) => error(info, format!("refcell borrow error: {err}")),
                            }
                            .expr,
                            ExprType::Cons(_)
                        ) =>
                    {
                        // TODO potenially eval expr here for example: a = [1 2] b = [3 4] c = [a b]
                        // because c is a list of lists once we access c.car (a) we need to eval it
                        let mut expr = Rc::clone(v);
                        let mut old_expr;
                        for (_, accesor) in name.interlaced.iter().enumerate() {
                            old_expr = Rc::clone(&expr);
                            if let ExprType::Cons(ref list) = match old_expr.try_borrow() {
                                Ok(val) => val,
                                Err(err) => error(info, format!("refcell borrow error: {err}")),
                            }
                            .expr
                            {
                                match accesor {
                                    Accesor::Car => {
                                        expr = Rc::clone(&list.car);
                                    }
                                    Accesor::Cdr => {
                                        expr = Rc::clone(&list.cdr);
                                    }
                                }
                            } else {
                                error(
                                    info,
                                    format!(
                                        "only lists can be accessed with car and cdr, got {}",
                                        match expr.try_borrow() {
                                            Ok(val) => val,
                                            Err(err) =>
                                                error(info, format!("refcell borrow error: {err}")),
                                        }
                                    ),
                                );
                            }
                        }
                        expr
                    }
                    (expr, _) => error(
                        info,
                        format!(
                            "only lists can be accessed with car and cdr, got {}",
                            match expr.try_borrow() {
                                Ok(val) => val,
                                Err(err) => error(info, format!("refcell borrow error: {err}")),
                            }
                        ),
                    ),
                }
            }
            None => self.parent_scope.as_mut().map_or_else(
                || error(info, format!("variable not found {}", name.main)),
                |parent| parent.get_var(name, info),
            ),
        }
    }
    pub fn set_function(&mut self, name: Interlaced<char, char>, body: Lambda<'a>) {
        self.functions.insert(name, body);
    }
    pub fn get_function(&self, name: &Interlaced<char, char>) -> Option<Lambda<'a>> {
        self.functions.get(name).map_or_else(
            || {
                self.parent_scope
                    .as_ref()
                    .and_then(|parent| parent.get_function(name))
            },
            |func| Some(func.clone()),
        )
    }
    //     pub fn delete_var(&mut self, name: &str) -> Option<NewIdentifierType> {
    //         self.vars.remove(name)
    //     }
    pub fn has_var(&self, name: VarType<'_>, recurse: bool) -> bool {
        if !recurse {
            return self.vars.contains_key(&name);
        }
        if self.vars.contains_key(&name) {
            true
        } else {
            self.parent_scope
                .as_ref()
                .map_or(false, |parent| parent.has_var(name, recurse))
        }
    }
    pub fn drop_scope(&mut self) {
        let p_scope: Self = self
            .parent_scope
            .take()
            .map_or_else(|| error(Info::default(), "no parent scope"), |scope| *scope);
        *self = p_scope;
    }

    pub fn from_parent(&mut self) {
        let mut temp_scope = Self::new();
        swap(&mut temp_scope, self);
        *self = Self::new_with_parent(Box::new(temp_scope));
    }
    //     pub fn has_function(&self, name: &str) -> bool {
    //         self.function.contains_key(name)
    //     }
}

impl fmt::Display for Scope<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // display each variable : value
        // display each function : lambda
        write!(
            f,
            "Scope {{ \nvars:\n{},\nfunctions:\n{} }}",
            self.vars
                .iter()
                .map(|(k, v)| format!(
                    "{k}: {}",
                    match v.try_borrow() {
                        Ok(val) => val,
                        Err(err) => error(Info::default(), format!("refcell borrow error: {err}")),
                    }
                ))
                .collect::<Vec<String>>()
                .join(",\n\n"),
            self.functions
                .iter()
                .map(|(k, v)| format!(
                    "{}{}: {}",
                    match k.interlaced_to_string("+") {
                        string if string.is_empty() => string,
                        string => string + "+",
                    },
                    k.main,
                    v
                ))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Clone)]
pub enum Stopper<'a> {
    Break(Expr<'a>),
    Continue,
    Return(Expr<'a>),
    End(Expr<'a>),
}

pub struct Eval<'a> {
    pub scope: Scope<'a>,
    pub in_function: bool,
    pub in_loop: bool,
    pub in_if: bool,
    pub files: HashMap<String, Rc<RefCell<File>>>,
    pub module_name: String,
    modules: Vec<char>,
}

impl<'a> Eval<'a> {
    pub fn new(mut body: Vec<Expr<'a>>) -> Self {
        let mut self_ = Self {
            scope: Scope::new(),
            in_function: false,
            in_loop: false,
            in_if: false,
            files: HashMap::new(),
            module_name: String::new(),
            modules: Vec::new(),
        };
        body = self_.find_functions(body);
        self_.find_variables(body);
        self_
    }

    //     pub fn get_file(&self, name: &str) -> Option<RefMut<'_, File>> {
    //         self.files.get(name).map(|file| file.borrow_mut())
    //     }

    // this only finds functions defined in outermost scope, it doesn not find functions defined in calls ie in "()" parentheses
    // this means that functions defined in the most outer scope can be called from anywhere in the file even before they are defined in the file
    // but functions defined in calls can only be called after they are defined
    pub fn find_functions(&mut self, body: Vec<Expr<'a>>) -> Vec<Expr<'a>> {
        let body = body
            .into_iter()
            .filter(|thing| -> bool {
                if let ExprType::Fn(function) = &thing.expr {
                    self.scope.set_function(
                        Interlaced::new(function.name, self.modules.clone()),
                        function.clone().take_inner(),
                    );
                    false
                } else {
                    true
                }
            })
            .collect();
        self.find_imports(body)
    }

    pub fn find_variables(&mut self, body: Vec<Expr<'a>>) -> Option<Stopper<'a>> {
        let len = body.len();
        // create a vector to return instead of inplace modification
        // well have globa/local scope when we check for variables we check for variables in the current scope and then check the parent scope and so on until we find a variable or we reach the top of the scope stack (same for functions)
        // we can have two different variables with the same name in different scopes, the scope of a variable is determined by where it is declared in the code
        let in_if = self.in_if;
        let in_function = self.in_function;
        let in_loop = self.in_loop;
        for (idx, expr) in body.into_iter().enumerate() {
            self.in_if = in_if;
            self.in_loop = in_loop;
            self.in_function = in_function;
            match expr.expr {
                // we explicity match the return statement so that if we are on the last expression of a function
                // that we dont end up falling into the implicit return and returning a return statement
                ExprType::Return(value) => {
                    if self.in_function {
                        return Some(Stopper::Return(match self.eval_expr(*value, true) {
                            Ok(value) => value,
                            Err(stopper) => return Some(stopper),
                        }));
                    }
                    error(expr.info, "return statement outside of function");
                }
                // implicit return should be checked before any other expression kind
                _ if idx == len - 1 && (self.in_function || self.in_if) => {
                    // if the last expression is not a return statement then we return the last expression
                    println!("implicit return");
                    return Some(Stopper::End(match self.eval_expr(expr, true) {
                        Ok(value) => value,
                        Err(stopper) => return Some(stopper),
                    }));
                }
                _ => match self.eval_expr(expr, true) {
                    // TODO: proper formatting
                    Ok(_) => {}
                    Err(stopper) => return Some(stopper),
                },
            }
        }
        None
    }

    pub fn find_imports(&mut self, body: Vec<Expr<'a>>) -> Vec<Expr<'a>> {
        body.into_iter()
            .filter(|expr| {
                if let ExprType::Module(module) = &expr.expr {
                    self.add_module(module);
                    false
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn add_module(&mut self, module: &Module<'a>) -> Expr<'a> {
        self.modules.push(module.get_name());
        match module.get_type() {
            ModuleType::Inline(code) => {
                self.find_functions(code.clone());
            }
            ModuleType::File(file) => {
                let contents = std::fs::read_to_string(file).unwrap_or_else(|_| {
                    error(*module.get_info(), format!("Could not read file {file}"));
                });
                let mut lexer: Lexer<'a, 'a> = Lexer::new(contents, file);
                let mut parser = Parser::new(lexer.scan_tokens(), file);
                let parsed = parser.parse();
                self.find_functions(parsed);
            }
        }
        self.modules.pop();
        Expr::new_literal(
            *module.get_info(),
            Lit::new_string(*module.get_info(), "module added"),
        )
    }

    pub fn add_function(&mut self, function: FnDef<'a>) -> Expr<'a> {
        let info = function.info;
        self.scope.set_function(
            Interlaced::new(function.name, function.modules.clone()),
            function.take_inner(),
        );
        Expr::new_literal(info, Lit::new_string(info, "function added"))
    }

    pub fn add_variable(&mut self, variable: Var<'a>) -> Result<Expr<'a>, Stopper<'a>> {
        self.scope.set_var(
            &Interlaced::new(VarType::Var(variable.name), vec![]),
            variable.value,
            false,
            variable.info,
        );
        Ok(Expr::new_literal(
            variable.info,
            Lit::new_string(variable.info, "variable added"),
        ))
    }

    // attempts to simplify an expression to its simplest form
    pub fn eval_expr(&mut self, expr: Expr<'a>, force: bool) -> Result<Expr<'a>, Stopper<'a>> {
        match expr.expr {
            ExprType::Return(value) => Err(Stopper::Return(self.eval_expr(*value, true)?)),
            ExprType::Var(var) => self.add_variable(*var),
            ExprType::Continue => Err(Stopper::Continue),
            ExprType::Break(value) => Err(Stopper::Break(self.eval_expr(*value, true)?)),
            // if its a literal, lambda or cons then its reduced enough
            ExprType::Identifier(Ident {
                ident_type: IdentType::Builtin(_),
                ..
            })
            | ExprType::Literal(_)
            | ExprType::Lambda(_) => Ok(expr.clone()),
            ExprType::Cons(_) | ExprType::Identifier(_) if !force => Ok(expr.clone()),
            ExprType::Cons(cons) => {
                let car = self.eval_expr(cons.car().clone(), true)?;
                let cdr = self.eval_expr(cons.cdr().clone(), true)?;
                Ok(Expr::new_cons(expr.info, Cons::new(cons.info, car, cdr)))
            }
            ExprType::Identifier(ident) => match &ident.ident_type {
                IdentType::Builtin(_) => unreachable!(),
                IdentType::FnParam(_) | IdentType::Var(_) => self.get_var(ident, expr.info),
                IdentType::FnIdent(fn_name) => match self.scope.get_function(fn_name) {
                    Some(expr) => Ok(Expr::new_lambda(expr.info, expr)),
                    None => error(expr.info, "function not bound"),
                },
            },
            ExprType::Call(call) => {
                // first remove and eval the first argument
                // if its a builtin then apply it to the rest of the arguments
                // otherwise if there are no additional done
                // if there are more arguments then error out
                // now check what type of print type the call want and act accordingly
                // then return the value back to the caller
                let mut args = call.args;
                // we need to reverse the arguments so we can pop them off the end (which when reversed is the front)
                args.reverse();
                let call_eval = if let Some(arg) = args.pop() {
                    let evaled_thing = self.eval_expr(arg, true)?;

                    match evaled_thing.expr {
                        ExprType::Literal(lit) => Ok(Expr::new_literal(expr.info, lit)),

                        ExprType::Lambda(_) => {
                            error(expr.info, "lambda expressions cannot be called without new")
                        }
                        ExprType::Identifier(ident) => {
                            match ident.ident_type {
                                IdentType::Builtin(builtin) => {
                                    self.eval_builtin(builtin, &mut args, call.info)
                                }
                                IdentType::Var(_)
                                | IdentType::FnParam(_)
                                | IdentType::FnIdent(_) => {
                                    unreachable!("should be evaluated before this point")
                                } // same as var
                            }
                        }
                        // if its a cons then we need to evaluate the arguments and then apply them to the cons
                        ExprType::Cons(cons) => Ok(Expr::new_cons(expr.info, cons)),
                        ExprType::Module(_)
                        | ExprType::Fn(_)
                        | ExprType::Call(_)
                        | ExprType::If(_)
                        | ExprType::Loop(_)
                        | ExprType::Var(_) => unreachable!("should be evaluated before this point"),
                        ExprType::Return(_) | ExprType::Break(_) | ExprType::Continue => {
                            unreachable!("return, break and continue should be handled via stopper")
                        }
                    }
                } else {
                    // if there are no arguments then we just return hempty
                    return Ok(Expr::new_literal(call.info, Lit::new_hempty(call.info)));
                };
                if !args.is_empty() {
                    error(
                        call.info,
                        format!(
                            "too many arguments to function, expected 1 found {args}",
                            args = args.len() + 1
                        ),
                    );
                }
                return print_and_pass(call_eval, call.print_type);
            }
            // if statements and loops are not lazily evaluated
            ExprType::If(if_statement) => {
                self.in_if = true;
                let cond_info = if_statement.condition.info;
                let exprs = match self.eval_expr(if_statement.condition, true) {
                    Ok(Expr {
                        expr:
                            ExprType::Literal(Lit {
                                value: LitType::Boolean(true),
                                ..
                            }),
                        ..
                    }) => if_statement.then,
                    Ok(Expr {
                        expr:
                            ExprType::Literal(Lit {
                                value: LitType::Boolean(false),
                                ..
                            }),
                        ..
                    }) => if_statement.otherwise,
                    Err(e) => return Err(e),
                    Ok(expr) => error(
                        cond_info,
                        format!("condition of expression must be true or false found {expr}"),
                    ),
                };
                // TODO: create new scope
                let other_exprs = self.find_functions(exprs);
                let evaled = self.find_variables(other_exprs);
                self.in_if = false;
                match evaled {
                    Some(Stopper::End(expr)) => Ok(expr),
                    Some(stopper) => Err(stopper),
                    None => Ok(Expr::new_literal(
                        if_statement.info,
                        Lit::new_hempty(if_statement.info),
                    )),
                }
            }
            ExprType::Loop(loop_statement) => {
                // create new scope
                let loop_exprs = self.find_functions(loop_statement.body);
                loop {
                    self.in_loop = true;
                    let evaled = self.find_variables(loop_exprs.clone());
                    self.in_loop = false;
                    match evaled {
                        Some(Stopper::Break(expr)) => return Ok(expr),
                        Some(Stopper::End(_)) => unreachable!(),
                        None | Some(Stopper::Continue) => continue,
                        Some(e) => return Err(e),
                    }
                }
            }
            ExprType::Fn(fndef) => Ok(self.add_function(fndef)),
            ExprType::Module(module) => Ok(self.add_module(&module)),
        }
    }

    fn eval_lambda(
        &mut self,
        lambda: Lambda<'a>,
        given_args: &mut Vec<Expr<'a>>,
        info: Info<'a>,
    ) -> Result<Expr<'a>, Stopper<'a>> {
        arg_error(
            lambda.param_count,
            given_args.len() as u64,
            "lambda",
            lambda.extra_params,
            info,
        );
        let args = given_args.clone();
        given_args.clear();
        // eval rest of arguments
        // if there are extra arguments then add them to the scope
        self.scope.from_parent();
        // add the extra arguments to the scope as fn params
        self.in_function = true;
        let body = self.find_functions(lambda.body());
        for (i, arg) in args.into_iter().enumerate() {
            self.scope.set_var(
                &Interlaced::new(VarType::FnArg(i as u64), vec![]),
                arg,
                false,
                info,
            );
        }
        let res = self.find_variables(body);
        self.in_function = false;
        match res {
            Some(Stopper::Return(expr) | Stopper::End(expr)) => Ok(expr),
            Some(res) => Err(res),
            None => unreachable!(),
        }
    }

    fn eval_builtin(
        &mut self,
        builtin_function: BuiltinFunction,
        args: &mut Vec<Expr<'a>>,
        call: Info<'a>,
    ) -> Result<Expr<'a>, Stopper<'a>> {
        match builtin_function {
            // returns string
            // takes number
            BuiltinFunction::StrToNum => todo!(),
            // takes bool
            BuiltinFunction::StrToBool => todo!(),
            // takes hempty
            BuiltinFunction::StrToHempty => todo!(),
            // returns string takes string
            BuiltinFunction::RunCommand => todo!(),
            // returns `file`
            BuiltinFunction::Open => todo!(),
            // return file contents
            BuiltinFunction::Close => todo!(),
            // returns string takes string
            BuiltinFunction::Read => todo!(),
            BuiltinFunction::ReadLine => todo!(),
            BuiltinFunction::Exit | BuiltinFunction::Error => todo!(),
            // returns cons takes string
            BuiltinFunction::SplitOn => todo!(),
            // returns string (file contents) takes string (file name) and string (file mode)
            BuiltinFunction::Write => todo!(),
            // takes line to
            BuiltinFunction::WriteLine => todo!(),
            // returns string takes string
            BuiltinFunction::CreateFile => todo!(),
            BuiltinFunction::DeleteFile => todo!(),
            // returns string takes any
            BuiltinFunction::Type => todo!(),
            // returns string takes string
            BuiltinFunction::Input => todo!(),
            BuiltinFunction::Plus
            | BuiltinFunction::Minus
            | BuiltinFunction::Divide
            | BuiltinFunction::Multiply => {
                let mut args = self.eval_args(args, true)?;
                if let Some(expr) = args.pop() {
                    match expr.expr {
                        ExprType::Literal(Lit {
                            value: LitType::Number(mut num),
                            ..
                        }) => {
                            if args.is_empty() && builtin_function == BuiltinFunction::Minus {
                                num = -num;
                            } else {
                                num = args.iter().fold(num, |acc, expr| match expr.expr {
                                    ExprType::Literal(Lit {
                                        value: LitType::Number(num),
                                        ..
                                    }) => match builtin_function {
                                        BuiltinFunction::Plus => acc + num,
                                        BuiltinFunction::Minus => acc - num,
                                        BuiltinFunction::Divide => acc / num,
                                        BuiltinFunction::Multiply => acc * num,
                                        _ => unreachable!(),
                                    },
                                    _ => error(
                                        expr.info,
                                        format!(
                                            "expected number for {builtin_function} but found {}",
                                            expr.expr
                                        ),
                                    ),
                                });
                            }
                            Ok(Expr::new_literal(call, Lit::new_number(call, num)))
                        }
                        ExprType::Literal(Lit {
                            value: LitType::String(mut string),
                            ..
                        }) => {
                            if builtin_function == BuiltinFunction::Plus {
                                let string_modified = args.iter().fold(
                                    string.to_string(),
                                    |acc, expr|
                                    // if its add then anythign can be added to a string
                                    format!(
                                        "{acc}{expr}",
                                    ), // if its multiply then only a number can be multiplied
                                );
                                let boxed = Box::new(string_modified);
                                string = Box::leak(boxed);
                            } else if builtin_function == BuiltinFunction::Multiply {
                                let mult = args.iter().fold(1, |acc, expr| match expr.expr {
                                    ExprType::Literal(Lit {
                                        value: LitType::Number(num),
                                        ..
                                    }) => acc * num as usize,
                                    _ => error(
                                        expr.info,
                                        format!(
                                            "expected number for {builtin_function} but found {}",
                                            expr.expr
                                        ),
                                    ),
                                });
                                let string_modified = string.repeat(mult);
                                let boxed = Box::new(string_modified);
                                string = Box::leak(boxed);
                            } else {
                                error(
                                    expr.info,
                                    format!("cannot use {builtin_function} with strings",),
                                );
                            }
                            Ok(Expr::new_literal(call, Lit::new_string(call, string)))
                        }
                        _ => error(
                            expr.info,
                            format!(
                                "expected number or string for {builtin_function} but found {}",
                                expr.expr
                            ),
                        ),
                    }
                } else {
                    error(
                        call,
                        format!("expected at least one argument for {builtin_function}"),
                    )
                }
            }
            // two or more args returns bool
            BuiltinFunction::Equal | BuiltinFunction::NotEqual => todo!(),
            // two args returns bool
            BuiltinFunction::GreaterEqual
            | BuiltinFunction::LessEqual
            | BuiltinFunction::GreaterThan
            | BuiltinFunction::LessThan => todo!(),
            // 2 or more args (booleans only) returns bool
            BuiltinFunction::And | BuiltinFunction::Or => todo!(),
            BuiltinFunction::Not => {
                let mut args = self.eval_args(args, true)?;
                // arg_error(num_args, given_args, function, at_least, info)
                arg_error(1, args.len() as u64, "not", false, call);
                let expr = args.pop().unwrap();
                match expr.expr {
                    ExprType::Literal(Lit {
                        value: LitType::Boolean(bool),
                        ..
                    }) => Ok(Expr::new_literal(call, Lit::new_boolean(call, !bool))),
                    _ => error(
                        expr.info,
                        format!("expected boolean for not but found {}", expr.expr),
                    ),
                }
            }
            BuiltinFunction::New => {
                if let Some(expr) = args.pop() {
                    match self.eval_expr(expr, true)?.expr {
                        ExprType::Lambda(lambda) => self.eval_lambda(lambda, args, call),
                        // builtin function this allows for new to be used as a constructor for builtin types
                        // so that higher order functions can be used with builtin types as well as user defined types
                        // if not for this you would accept a builtin type or a user defined type but not both
                        // but also allows for (new new ...) ie you could have infinite news
                        ExprType::Identifier(Ident {
                            ident_type: IdentType::Builtin(value),
                            ..
                        }) => self.eval_builtin(value, args, call),
                        expr => error(
                            call,
                            format!("first argument of new must be a lambda function found {expr}"),
                        ),
                    }
                } else {
                    error(
                        call,
                        "first argument of new must be a lambda function".to_string(),
                    );
                }
            }
            // takes variable name and returns value
            BuiltinFunction::Delete => todo!(),
            // takes variable name and value and sets it
            // returns value
            BuiltinFunction::Set => todo!(),
            // follow same rules as regular math operators (from above)
            // but the first argument is the variable to set
            // returns new value
            BuiltinFunction::AddWith
            | BuiltinFunction::SubtractWith
            | BuiltinFunction::DivideWith
            | BuiltinFunction::MultiplyWith => {
                arg_error(2, args.len() as u64, "set", true, call);
                let var = args.pop().unwrap();
                // if its a variable or parameter we first need to get the value
                if !matches!(
                    var.expr,
                    ExprType::Identifier(Ident {
                        ident_type: IdentType::Var(_) | IdentType::FnParam(_),
                        ..
                    })
                ) {
                    error(
                        var.info,
                        format!(
                            "expected variable or parameter for {builtin_function} but found {}",
                            var.expr
                        ),
                    );
                }
                let mut args = self.eval_args(args, true)?;
                args.push(var.clone());
                let value = self.eval_builtin(builtin_function, &mut args, call)?;
                // self.scope.set_var(var, value.clone(), true, call);\
                let var = match var.expr {
                    ExprType::Identifier(Ident {
                        ident_type: IdentType::Var(value),
                        ..
                    }) => value.changed(VarType::Var),
                    ExprType::Identifier(Ident {
                        ident_type: IdentType::FnParam(value),
                        ..
                    }) => value.changed(VarType::FnArg),
                    _ => unreachable!(),
                };
                self.scope.set_var(&var, value.clone(), true, call);
                Ok(value)
            }
        }
    }

    fn get_var(&mut self, var: Ident<'a>, info: Info<'_>) -> Result<Expr<'a>, Stopper<'a>> {
        // TODO: macrobitize this
        match var.ident_type {
            IdentType::Var(value) => {
                let expr = self
                    .scope
                    .get_var(&value.changed(VarType::Var), info)
                    .borrow()
                    .clone();
                self.eval_expr(expr, true)
            }
            IdentType::FnParam(value) => {
                let expr = self
                    .scope
                    .get_var(&value.changed(VarType::FnArg), info)
                    .borrow()
                    .clone();
                self.eval_expr(expr, true)
            }
            _ => error(
                info,
                format!("expected variable or parameter but found {var}"),
            ),
        }
    }

    fn eval_args(
        &mut self,
        args: &mut Vec<Expr<'a>>,
        force: bool,
    ) -> Result<Vec<Expr<'a>>, Stopper<'a>> {
        // we need to force the identifiers to be evaluated
        // maybe force in get_var
        args.drain(..)
            .map(|arg| self.eval_expr(arg, force))
            .collect()
    }
}

impl fmt::Debug for Eval<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "scope {:?}", self.scope)?;
        Ok(())
    }
}

fn print_and_pass<'a>(
    e: Result<Expr<'a>, Stopper<'a>>,
    print_type: PrintType,
) -> Result<Expr<'a>, Stopper<'a>> {
    match e {
        Ok(expr) => {
            match print_type {
                PrintType::Newline => println!("{expr}"),
                PrintType::NoNewline => print!("{expr}"),
                PrintType::None => (),
            }
            Ok(expr)
        }

        Err(e) => Err(e),
    }
}
