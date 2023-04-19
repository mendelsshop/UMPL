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
                        // todo!("use accesor to get value from list")
                        let mut expr = Rc::clone(v);
                        for (_, accesor) in name.interlaced.iter().enumerate() {
                            if let ExprType::Cons(ref list) = match v.try_borrow() {
                                Ok(val) => val,
                                Err(err) => error(info, format!("refcell borrow error: {err}")),
                            }
                            .expr
                            {
                                match accesor {
                                    Accesor::Car => {
                                        // expr = Rc::clone(&list.car);
                                        // expr;
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
                        return Some(Stopper::Return(*value));
                    }
                    error(expr.info, "return statement outside of function");
                }
                // implicit return should be checked before any other expression kind
                _ if idx == len - 1 && (self.in_function || self.in_if) => {
                    // if the last expression is not a return statement then we return the last expression
                    println!("implicit return");
                    return Some(Stopper::End(expr));
                }
                _ => match self.eval_expr(expr) {
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
    pub fn eval_expr(&mut self, expr: Expr<'a>) -> Result<Expr<'a>, Stopper<'a>> {
        match expr.expr {
            ExprType::Return(value) => {
                if self.in_function {
                    return Err(Stopper::Return(*value));
                }
                error(expr.info, "return statement outside of function");
            }
            // implicit return should be checked before any other expression kind
            ExprType::Var(var) => self.add_variable(*var),
            ExprType::Continue => {
                if self.in_loop {
                    return Err(Stopper::Continue);
                }
                error(expr.info, "continue statement outside of loop");
            }
            ExprType::Break(value) => {
                if self.in_loop {
                    return Err(Stopper::Break(*value));
                }
                error(expr.info, "break statement outside of loop");
            }
            // if its a literal, lambda or cons then its reduced enough
            ExprType::Literal(_)
            | ExprType::Lambda(_)
            | ExprType::Cons(_)
            | ExprType::Identifier(_) => Ok(expr.clone()),
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
                    let evaled_thing = self.eval_expr(arg)?;

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
                                IdentType::Var(var) => {
                                    let var =
                                        self.scope.get_var(&var.changed(VarType::Var), call.info);
                                    let x = self.eval_expr(var.borrow().clone());
                                    x
                                }
                                IdentType::FnIdent(fn_ident) => {
                                    error(
                                        call.info,
                                        format!(
                                            "user defined functions like `{}` cannot be called without new",
                                            fn_ident.main
                                        ),
                                    );
                                }
                                // same as var
                                IdentType::FnParam(fn_param) => {
                                    let var = self
                                        .scope
                                        .get_var(&fn_param.changed(VarType::FnArg), call.info);
                                    let x = self.eval_expr(var.borrow().clone());
                                    x
                                }
                            }
                        }
                        // if its a cons then we need to evaluate the arguments and then apply them to the cons
                        ExprType::Cons(cons) => {
                            // use temporary variables to avoid borrowing issues
                            let car = self.eval_expr(cons.car().clone())?;
                            let cdr = self.eval_expr(cons.cdr().clone())?;
                            Ok(Expr::new_cons(expr.info, Cons::new(cons.info, car, cdr)))
                        }
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
                let exprs = match self.eval_expr(if_statement.condition) {
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
        self.in_function = false;
        for (i, arg) in args.into_iter().enumerate() {
            self.scope.set_var(
                &Interlaced::new(VarType::FnArg(i as u64), vec![]),
                arg,
                false,
                info,
            );
        }

        let res = self.find_variables(body);
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
            BuiltinFunction::StrToNum => todo!(),
            BuiltinFunction::StrToBool => todo!(),
            BuiltinFunction::StrToHempty => todo!(),
            BuiltinFunction::RunCommand => todo!(),
            BuiltinFunction::Open => todo!(),
            BuiltinFunction::Close => todo!(),
            BuiltinFunction::Write => todo!(),
            BuiltinFunction::Read => todo!(),
            BuiltinFunction::ReadLine => todo!(),
            BuiltinFunction::Exit => todo!(),
            BuiltinFunction::Error => todo!(),
            BuiltinFunction::Delete => todo!(),
            BuiltinFunction::SplitOn => todo!(),
            BuiltinFunction::WriteLine => todo!(),
            BuiltinFunction::CreateFile => todo!(),
            BuiltinFunction::DeleteFile => todo!(),
            BuiltinFunction::Type => todo!(),
            BuiltinFunction::Input => todo!(),
            BuiltinFunction::Plus => todo!(),
            BuiltinFunction::Minus => todo!(),
            BuiltinFunction::Divide => todo!(),
            BuiltinFunction::Multiply => todo!(),
            BuiltinFunction::Equal => todo!(),
            BuiltinFunction::NotEqual => todo!(),
            BuiltinFunction::GreaterEqual => todo!(),
            BuiltinFunction::LessEqual => todo!(),
            BuiltinFunction::GreaterThan => todo!(),
            BuiltinFunction::LessThan => todo!(),
            BuiltinFunction::And => todo!(),
            BuiltinFunction::Or => todo!(),
            BuiltinFunction::Not => todo!(),
            BuiltinFunction::New => {
                if let Some(expr) = args.pop() {
                    match self.eval_expr(expr)?.expr {
                        ExprType::Lambda(lambda) => self.eval_lambda(lambda, args, call),
                        ExprType::Identifier(Ident {
                            ident_type: IdentType::FnIdent(value),
                            ..
                        }) => {
                            if let Some(lambda) = self.scope.get_function(&value) {
                                self.eval_lambda(lambda, args, call)
                            } else {
                                error(
                                    call,
                                    // TODO: show full path of function
                                    format!("function {} not found", value.main),
                                );
                            }
                        }
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
            BuiltinFunction::Set => todo!(),
            BuiltinFunction::AddWith => todo!(),
            BuiltinFunction::SubtractWith => todo!(),
            BuiltinFunction::DivideWith => todo!(),
            BuiltinFunction::MultiplyWith => todo!(),
        }
    }
}

impl fmt::Debug for Eval<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "scope {:?}", self.scope)?;
        Ok(())
    }
}

fn print_and_pass<'b>(
    e: Result<Expr<'b>, Stopper<'b>>,
    print_type: PrintType,
) -> Result<Expr<'b>, Stopper<'b>> {
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
