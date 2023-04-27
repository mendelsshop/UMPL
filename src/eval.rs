use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    fmt,
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    mem::{self, swap},
    rc::Rc,
};

use crate::{
    error::{arg_error, error},
    lexer::Lexer,
    parser::{
        rules::{
            Cons, Expr, ExprState, ExprType, FileWrapper, FnDef, Ident, IdentType, Interlaced,
            Lambda, Lit, LitType, Module, ModuleType, PrintType, Thunk, Var,
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

#[derive(Debug, Default, Clone)]
pub struct Scope<'a> {
    vars: HashMap<VarType<'a>, Rc<RefCell<Expr<'a>>>>,
    functions: HashMap<Interlaced<char, char>, Lambda<'a>>,
    parent_scope: Option<Box<Scope<'a>>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            functions: HashMap::new(),
            parent_scope: None,
        }
    }
    pub fn new_with_parent(parent: Box<Self>) -> Self {
        Self {
            parent_scope: Some(parent),
            ..Default::default()
        }
    }
    pub fn set_var(&mut self, name: VarType<'a>, value: Expr<'a>, recurse: bool, info: Info<'_>) {
        if recurse {
            if self.has_var(&name, false) {
                self.vars.insert(name, Rc::new(RefCell::new(value)));
            } else {
                self.parent_scope.as_mut().map_or_else(
                    || error(info, format!("variable not found {name}")),
                    |parent| parent.set_var(name, value, recurse, info),
                );
            }
        } else {
            // if were are creating a new variable
            self.vars.insert(name, Rc::new(RefCell::new(value)));
        }
    }

    // TODO: possibly make this _mut and have another get_var that doesn't return a mutable reference
    pub fn get_var(&mut self, name: &VarType<'_>, info: Info<'_>) -> Rc<RefCell<Expr<'a>>> {
        match self.vars.get(name) {
            Some(v) => v.clone(),
            None => self.parent_scope.as_mut().map_or_else(
                || error(info, format!("variable not found {name}")),
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

    pub fn has_var(&self, name: &VarType<'_>, recurse: bool) -> bool {
        if !recurse {
            return self.vars.contains_key(name);
        }
        if self.vars.contains_key(name) {
            true
        } else {
            self.parent_scope
                .as_ref()
                .map_or(false, |parent| parent.has_var(name, recurse))
        }
    }

    fn remove_var(&mut self, var: VarType<'a>) -> Option<Rc<RefCell<Expr<'a>>>> {
        self.vars.remove(&var).or_else(|| {
            self.parent_scope
                .as_mut()
                .map(|scope| scope.remove_var(var))?
        })
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
    pub fn has_function(&self, fn_name: &Interlaced<char, char>) -> bool {
        self.functions.contains_key(fn_name)
    }
}

#[derive(Debug, Clone)]
pub enum Stopper<'a> {
    Break(Expr<'a>),
    Continue,
    Return(Expr<'a>),
    End(Expr<'a>),
}

// macro for getting a literal value from an expression
// $self:ident is the eval struct
macro_rules! get_literal {
    ($self:ident, $expr:ident, $lit_type:ident, $func:ident, $ret_type:ty) => {
        fn $func(&mut self, $expr: Expr<'a>) -> Result<$ret_type, Stopper<'a>> {
            let info = $expr.info;
            // first force the expression to be evaluated
            match self.eval_expr($expr, true)?.expr {
                // then with the provided literal type, get the value
                ExprType::Literal(Lit {
                    value: LitType::$lit_type(lit),
                    ..
                }) => Ok(lit),
                // if the value is not the correct type, error
                e => error(
                    info,
                    format!("expected {} value found {e}", stringify!($lit_type)),
                ),
            }
        }
    };
}
#[derive(Clone)]
pub struct Eval<'a> {
    pub scope: Scope<'a>,
    modules: Vec<char>,
}

impl<'a> Eval<'a> {
    get_literal!(self, expr, Boolean, eval_to_bool, bool);
    get_literal!(self, expr, Number, eval_to_num, f64);
    get_literal!(self, expr, String, eval_to_str, &'a str);
    get_literal!(self, expr, File, eval_to_file, FileWrapper<'a>);

    fn eval_to_non_float(&mut self, expr: Expr<'a>) -> Result<i64, Stopper<'a>> {
        let info = expr.info;
        self.eval_to_num(expr).map(|num| {
            if num.fract() == 0.0 {
                num as i64
            } else {
                error(info, format!("expected non float value found {num}"))
            }
        })
    }

    pub fn new(mut body: Vec<Expr<'a>>) -> Self {
        let mut self_ = Self {
            scope: Scope::new(),
            modules: Vec::new(),
        };
        body = self_.find_functions(body);
        self_.find_variables(body, false);
        self_
    }

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

    pub fn find_variables(
        &mut self,
        body: Vec<Expr<'a>>,
        implicit_return: bool,
    ) -> Option<Stopper<'a>> {
        let len = body.len();
        // create a vector to return instead of inplace modification
        // well have globa/local scope when we check for variables we check for variables in the current scope and then check the parent scope and so on until we find a variable or we reach the top of the scope stack (same for functions)
        // we can have two different variables with the same name in different scopes, the scope of a variable is determined by where it is declared in the code
        for (idx, expr) in body.into_iter().enumerate() {
            match expr.expr {
                // we explicity match the return statement so that if we are on the last expression of a function
                // that we dont end up falling into the implicit return and returning a return statement
                ExprType::Return(value) => {
                    return Some(Stopper::Return(match self.eval_expr(*value, true) {
                        Ok(value) => value,
                        Err(stopper) => return Some(stopper),
                    }));
                }
                // implicit return should be checked before any other expression kind
                _ if idx == len - 1 && implicit_return => {
                    // if the last expression is not a return statement then we return the last expression
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

    pub fn add_variable(&mut self, variable: Var<'a, Expr<'a>>) -> Result<Expr<'a>, Stopper<'a>> {
        self.scope.set_var(
            VarType::Var(variable.name),
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
            ExprType::Call(call) => self.eval_call(*call),
            // if statements and loops are not lazily evaluated
            ExprType::If(if_statement) => self.eval_if(if_statement),
            ExprType::Loop(loop_statement) => {
                // create new scope
                let loop_exprs = self.find_functions(loop_statement.body);
                loop {
                    let evaled = self.find_variables(loop_exprs.clone(), false);
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

    fn eval_if(
        &mut self,
        if_statement: Box<crate::parser::rules::If<'a>>,
    ) -> Result<Expr<'a>, Stopper<'a>> {
        let cond_info = if_statement.condition.info;
        let exprs = match self.eval_expr(if_statement.condition, true) {
            Ok(Expr {
                expr:
                    ExprType::Literal(Lit {
                        value: LitType::Boolean(val),
                        ..
                    }),
                ..
            }) => {
                if val {
                    if_statement.then
                } else {
                    if_statement.otherwise
                }
            }
            Err(e) => return Err(e),
            Ok(expr) => error(
                cond_info,
                format!("condition of expression must be true or false found {expr}"),
            ),
        };
        // TODO: create new scope
        let other_exprs = self.find_functions(exprs);
        let evaled = self.find_variables(other_exprs, true);
        match evaled {
            Some(Stopper::End(expr)) => Ok(expr),
            Some(stopper) => Err(stopper),
            None => Ok(Expr::new_literal(
                if_statement.info,
                Lit::new_hempty(if_statement.info),
            )),
        }
    }

    fn eval_call(
        &mut self,
        call: crate::parser::rules::FnCall<'a>,
    ) -> Result<Expr<'a>, Stopper<'a>> {
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
                ExprType::Literal(lit) => Ok(Expr::new_literal(call.info, lit)),

                ExprType::Lambda(_) => {
                    error(call.info, "lambda expressions cannot be called without new")
                }
                ExprType::Identifier(ident) => {
                    match ident.ident_type {
                        IdentType::Builtin(builtin) => {
                            self.eval_builtin(builtin, &mut args, call.info)
                        }
                        IdentType::Var(_) | IdentType::FnParam(_) | IdentType::FnIdent(_) => {
                            unreachable!("should be evaluated before this point")
                        } // same as var
                    }
                }
                // if its a cons then we need to evaluate the arguments and then apply them to the cons
                ExprType::Cons(cons) => Ok(Expr::new_cons(call.info, cons)),
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
            Ok(Expr::new_literal(call.info, Lit::new_hempty(call.info)))
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
        print_and_pass(call_eval, call.print_type)
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
        given_args.reverse();
        let (args, rest): (Vec<_>, Vec<_>) = given_args
            .drain(..)
            .enumerate()
            .partition(|(i, _)| *i < lambda.param_count as usize);
        // eval rest of arguments
        // if there are extra arguments then add them to the scope
        self.scope.from_parent();
        // add the extra arguments to the scope as fn params
        let body = self.find_functions(lambda.body());
        // TODO: put them as thunks
        for (i, mut arg) in args {
            let cloned = self.clone();
            arg.state = ExprState::Thunk(Thunk::new(Box::new(fun_name(cloned)))).into();
            self.scope
                .set_var(VarType::FnArg(i as u64), arg, false, info);
        }

        let res = self.find_variables(body, true);
        match res {
            Some(Stopper::Return(expr) | Stopper::End(expr)) => Ok(expr),
            Some(res) => Err(res),
            None => unreachable!(),
        }
    }

    // can't be lazy because file can be changed this point and when go to read it it will be different
    fn read_file(
        &mut self,
        file_name: FileWrapper<'a>,
        info: Info<'a>,
        line: Option<u32>,
    ) -> Expr<'a> {
        let mut contents = String::new();
        file_name
            .file
            .borrow_mut()
            .read_to_string(&mut contents)
            .unwrap_or_else(|e| error(info, format!("error reading file {file_name}: {e}")));
        if let Some(line) = line {
            contents.lines().nth(line as usize).unwrap_or_else(|| {
                error(
                    info,
                    format!("error reading file {file_name}: line {line} does not exist",),
                )
            });
        }
        Expr::new_literal(
            info,
            Lit::new_string(info, Box::leak(contents.into_boxed_str())),
        )
    }

    // can't be lazy because file can be changed this point and when go to read it it will be different
    fn write_file(
        &mut self,
        file_name: FileWrapper<'a>,
        mode: &str,
        contents: &str,
        info: Info<'a>,
        line: Option<u32>,
    ) {
        let fc = self.read_file(file_name, info, None);
        let lines = self.eval_to_str(fc).expect("unreachable").lines();
        let contents = match mode {
            "a" => {
                let mut lines = lines.collect::<Vec<_>>();
                if let Some(line) = line {
                    lines.insert(line as usize, contents);
                } else {
                    lines.push(contents);
                }
                lines.join("\n")
            }
            "w" => {
                // if line is none then we just write the contents
                // otherwise we write the contents at the line and delete the rest of the file
                line.map_or_else(
                    || contents.to_string(),
                    |line| {
                        let mut lines = lines.collect::<Vec<_>>();
                        lines.insert(line as usize, contents);
                        lines.truncate(line as usize + 1);
                        lines.join("\n")
                    },
                )
            }
            _ => {
                error(info, format!("invalid mode {mode} for write_file"));
            }
        };
        file_name
            .file
            .borrow_mut()
            .write_all(contents.as_bytes())
            .unwrap_or_else(|e| error(info, format!("error writing file {file_name}: {e}")));
    }

    // never returns a Thunk
    fn eval_builtin(
        &mut self,
        builtin_function: BuiltinFunction,
        args: &mut Vec<Expr<'a>>,
        call: Info<'a>,
    ) -> Result<Expr<'a>, Stopper<'a>> {
        match builtin_function {
            // return file contents
            BuiltinFunction::Close | BuiltinFunction::DeleteFile => {
                arg_error(1, args.len() as u64, builtin_function, false, call);
                let file = self.eval_to_file(args.pop().unwrap())?;
                let contents = self.read_file(file, call, None);
                // self.files.remove(file);
                if builtin_function == BuiltinFunction::DeleteFile {
                    std::fs::remove_file(file.name).unwrap_or_else(|e| {
                        error(call, format!("error deleting file {file}: {e}"))
                    });
                }
                Ok(contents)
            }
            // returns string takes file
            BuiltinFunction::Read => {
                arg_error(1, args.len() as u64, "read", false, call);
                self.eval_to_file(args.pop().unwrap())
                    .map(|file| self.read_file(file, call, None))
            }
            // also takes line number
            BuiltinFunction::ReadLine => {
                arg_error(2, args.len() as u64, "read", false, call);
                let file = self.eval_to_file(args.pop().unwrap())?;
                let line = self.eval_to_non_float(args.pop().unwrap())?;
                Ok(self.read_file(file, call, Some(line as u32)))
            }
            BuiltinFunction::Exit => {
                arg_error(1, args.len() as u64, "exit", false, call);
                let exit_code = self.eval_to_non_float(args.pop().unwrap())?;
                std::process::exit(exit_code as i32);
            }
            // takes string prints it and exits 1
            BuiltinFunction::Error => {
                arg_error(1, args.len() as u64, builtin_function, false, call);
                let error = self.eval_to_str(args.pop().unwrap())?;
                println!("{error}");
                std::process::exit(1);
            }
            // returns cons takes string
            BuiltinFunction::SplitOn => {
                arg_error(1, args.len() as u64, builtin_function, false, call);
                let string = self.eval_to_str(args.pop().unwrap())?;

                let char_iter = string.split_terminator("").skip(1);
                Ok(self.iter_to_cons(char_iter, call))
            }
            // returns string (file contents) takes string (file name) and string (file mode)
            BuiltinFunction::Write => {
                arg_error(3, args.len() as u64, builtin_function, false, call);
                let file = self.eval_to_file(args.pop().unwrap())?;
                let mode = self.eval_to_str(args.pop().unwrap())?;
                let contents = self.eval_to_str(args.pop().unwrap())?;
                self.write_file(file, mode, contents, call, None);
                Ok(Expr::new_literal(call, Lit::new_hempty(call)))
            }
            // takes line to
            BuiltinFunction::WriteLine => {
                // takes arg file (name) mode (a or w) line contents
                arg_error(4, args.len() as u64, builtin_function, false, call);
                let file = self.eval_to_file(args.pop().unwrap())?;
                let mode = self.eval_to_str(args.pop().unwrap())?;
                let line = self.eval_to_non_float(args.pop().unwrap())?;
                let contents = self.eval_to_str(args.pop().unwrap())?;
                self.write_file(file, mode, contents, call, Some(line as u32));
                Ok(Expr::new_literal(call, Lit::new_hempty(call)))
            }

            // takes string returns any
            BuiltinFunction::StrToNum
            | BuiltinFunction::StrToBool
            | BuiltinFunction::StrToHempty => {
                arg_error(1, args.len() as u64, builtin_function, false, call);
                let input = self.eval_to_str(args.pop().unwrap())?;
                match builtin_function {
                    BuiltinFunction::StrToNum => {
                        let num = input.parse::<f64>().unwrap_or_else(|e| {
                            error(call, format!("error parsing {input} to number: {e}"))
                        });
                        Ok(Expr::new_literal(call, Lit::new_number(call, num)))
                    }
                    BuiltinFunction::StrToBool => {
                        let bool = input.parse::<bool>().unwrap_or_else(|e| {
                            error(call, format!("error parsing {input} to bool: {e}"))
                        });
                        Ok(Expr::new_literal(call, Lit::new_boolean(call, bool)))
                    }
                    BuiltinFunction::StrToHempty => {
                        let hempty = if input == "hempty" {
                            Lit::new_hempty(call)
                        } else {
                            error(call, format!("error parsing {input} to hempty"))
                        };
                        Ok(Expr::new_literal(call, hempty))
                    }
                    _ => unreachable!(),
                }
            }
            // returns string takes string
            BuiltinFunction::RunCommand => {
                arg_error(1, args.len() as u64, builtin_function, false, call);
                let cmd = self.eval_to_str(args.pop().unwrap())?;
                let res = run_script::run_script!(cmd)
                    .unwrap_or_else(|e| error(call, format!("error running command {cmd}: {e}")));
                if res.0 != 0 {
                    error(call, format!("error running command {cmd}: {}", res.2));
                }
                Ok(Expr::new_literal(
                    call,
                    Lit::new_string(call, Box::leak(res.1.into_boxed_str())),
                ))
            }
            // returns `file`
            BuiltinFunction::Open | BuiltinFunction::CreateFile => {
                arg_error(1, args.len() as u64, builtin_function, false, call);
                let file_name = self.eval_to_str(args.pop().unwrap())?;
                let file = OpenOptions::new()
                    .write(true)
                    .create(builtin_function == BuiltinFunction::CreateFile)
                    .read(true)
                    .open(file_name)
                    .unwrap_or_else(|e| {
                        error(call, format!("could not open file `{file_name}` {e}"))
                    });
                // self.files
                //     .insert(file_name.to_string(), Rc::new(RefCell::new(file)));
                Ok(Expr::new_literal(
                    call,
                    Lit::new_file(
                        call,
                        FileWrapper::new(Rc::new(RefCell::new(file)), file_name),
                    ),
                ))
            }
            // returns string takes string
            BuiltinFunction::Input => {
                arg_error(1, args.len() as u64, builtin_function, false, call);
                let prompt = self.eval_to_str(args.pop().unwrap())?;
                let mut input = String::new();
                print!("{prompt}");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                // remove the newline
                input.pop();
                Ok(Expr::new_literal(
                    call,
                    Lit::new_string(call, Box::leak(input.into_boxed_str())),
                ))
            }
            // returns string takes any
            BuiltinFunction::Type => {
                if let Some(expr) = args.pop() {
                    Ok(Expr::new_literal(
                        call,
                        Lit::new_string(
                            call,
                            match self.eval_expr(expr, true)?.expr {
                                ExprType::Literal(lit) => match lit.value {
                                    LitType::String(_) => "string",
                                    LitType::Number(_) => "number",
                                    LitType::Boolean(_) => "boolean",
                                    LitType::File(_) => "file",
                                    LitType::Hempty => "hempty",
                                },
                                ExprType::Cons(_) => "list",
                                ExprType::Identifier(ident) => match ident.ident_type {
                                    IdentType::Builtin(_) => "builtin-function",
                                    _ => unreachable!(),
                                },
                                ExprType::Lambda(_) => "lambda",
                                _ => unreachable!(),
                            },
                        ),
                    ))
                } else {
                    error(call, "Type requires at least 1 arguments")
                }
            }
            // returns string takes string
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
                                num = args.into_iter().try_fold(num, |acc, expr| {
                                    let val = self.eval_to_num(expr)?;
                                    match builtin_function {
                                        BuiltinFunction::Plus => Ok(acc + val),
                                        BuiltinFunction::Minus => Ok(acc - val),
                                        BuiltinFunction::Divide => Ok(acc / val),
                                        BuiltinFunction::Multiply => Ok(acc * val),
                                        _ => unreachable!(),
                                    }
                                })?;
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
                                let mult = args.into_iter().try_fold(1, |acc, expr| {
                                    {
                                        // make sure its a non float number
                                        let val = self.eval_to_non_float(expr)?;
                                        Ok(acc * val as usize)
                                    }
                                });
                                let string_modified = string.repeat(mult?);
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
            BuiltinFunction::Not => {
                let mut args = self.eval_args(args, true)?;
                // arg_error(num_args, given_args, function, at_least, info)
                arg_error(1, args.len() as u64, builtin_function, false, call);
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
            // 2 or more args (booleans only) returns bool
            // two or more args returns bool
            // short circuting
            BuiltinFunction::GreaterEqual
            | BuiltinFunction::LessEqual
            | BuiltinFunction::GreaterThan
            | BuiltinFunction::LessThan
            | BuiltinFunction::Equal
            | BuiltinFunction::NotEqual => {
                let op = match builtin_function {
                    BuiltinFunction::Equal => |a, b| a == b,
                    BuiltinFunction::NotEqual => |a, b| a != b,
                    BuiltinFunction::GreaterEqual => |a, b| a >= b,
                    BuiltinFunction::LessEqual => |a, b| a <= b,
                    BuiltinFunction::GreaterThan => |a, b| a > b,
                    BuiltinFunction::LessThan => |a, b| a < b,
                    _ => unreachable!(),
                };
                arg_error(2, args.len() as u64, builtin_function, true, call);
                let expr = args.pop().unwrap();
                let mut first_expr = self.eval_expr(expr, true)?;
                let args = args.drain(..).collect::<Vec<_>>();
                for arg in args {
                    let expr = self.eval_expr(arg, true)?;

                    if !op(first_expr.clone(), expr.clone()) {
                        return Ok(Expr::new_literal(call, Lit::new_boolean(call, false)));
                    }
                    // we need to modify comparator so ordering comparisons work
                    first_expr = expr;
                }
                Ok(Expr::new_literal(call, Lit::new_boolean(call, true)))
            }
            BuiltinFunction::And | BuiltinFunction::Or => {
                arg_error(2, args.len() as u64, builtin_function, true, call);
                let args = args.drain(..).collect::<Vec<_>>();
                for arg in args {
                    let bool_val = self.eval_to_bool(arg)?;
                    if !bool_val && builtin_function == BuiltinFunction::And {
                        return Ok(Expr::new_literal(call, Lit::new_boolean(call, false)));
                    }
                }
                Ok(Expr::new_literal(call, Lit::new_boolean(call, true)))
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
            BuiltinFunction::Delete => {
                arg_error(1, args.len() as u64, builtin_function, false, call);
                let var = match args.pop().expect("should not be empty").expr {
                    ExprType::Identifier(ident) => {
                        // if its a variable or parameter we first need to get the value
                        ident
                    }
                    expr => {
                        error(call, format!("expected variable or parameter for {builtin_function} but found {expr}"));
                    }
                };
                let var = into_var(var, call);
                // go through scope looking for var
                Ok(
                    Rc::try_unwrap(self.scope.remove_var(var).unwrap_or_else(|| {
                        error(
                            call,
                            format!("variable {var} does not exist in current scope"),
                        )
                    }))
                    .unwrap_or_else(|_| {
                        error(
                            call,
                            format!("failed to retrieve value when deleting {var}"),
                        )
                    })
                    .into_inner(),
                )
            }
            // takes variable name and value and sets it
            // returns value
            BuiltinFunction::Set => {
                arg_error(2, args.len() as u64, builtin_function, false, call);
                let var = match args.pop().expect("should not be empty").expr {
                    ExprType::Identifier(ident) => {
                        // if its a variable or parameter we first need to get the value
                        ident
                    }
                    expr => {
                        error(call, format!("expected variable or parameter for {builtin_function} but found {expr}"));
                    }
                };
                let value = args.pop().expect("should not be empty");
                let value = self.eval_expr(value, true)?;
                self.set_var(var, value, call, true);
                Ok(value)
            }
            // follow same rules as regular math operators (from above)
            // but the first argument is the variable to set
            // returns new value
            BuiltinFunction::AddWith
            | BuiltinFunction::SubtractWith
            | BuiltinFunction::DivideWith
            | BuiltinFunction::MultiplyWith => {
                arg_error(2, args.len() as u64, builtin_function, true, call);
                let var = args.pop().unwrap();
                let mut args = self.eval_args(args, true)?;
                args.push(var.clone());
                let builtin_function = match builtin_function {
                    BuiltinFunction::AddWith => BuiltinFunction::Plus,
                    BuiltinFunction::SubtractWith => BuiltinFunction::Minus,
                    BuiltinFunction::DivideWith => BuiltinFunction::Divide,
                    BuiltinFunction::MultiplyWith => BuiltinFunction::Multiply,
                    _ => unreachable!(),
                };
                let value = self.eval_builtin(builtin_function, &mut args, call)?;
                let var = match var.expr {
                    ExprType::Identifier(ident) => {
                        // if its a variable or parameter we first need to get the value
                        ident
                    }
                    _ => {
                        error(call, format!("expected variable or parameter for {builtin_function} but found {var}"));
                    }
                };
                self.set_var(var, value, call, true);
                Ok(value)
            }
        }
    }

    // never returns a thunk
    // if the value is a thunk it will be evaluated
    // and the result will be returned
    // and the thunk in the var hashmap will be replaced with the result
    fn get_var(&mut self, var: Ident<'a>, info: Info<'_>) -> Result<Expr<'a>, Stopper<'a>> {
        let expr = self.get_var_inner(var, info).borrow().clone();
        let var = self.eval_expr(expr, true)?;

        // TODO: use get car/cdrs if needed
        Ok(var)
    }

    fn get_var_inner(&mut self, var: Ident<'_>, info: Info<'_>) -> Rc<RefCell<Expr<'a>>> {
        let var_name = into_var(var, info);
        self.scope.get_var(&var_name, info)
    }

    // first retrive var
    // the fully evaluate it
    // then use accessors (car/cdr) if needed
    // then return the value
    // and also set the value in the var hashmap
    fn set_var(&mut self, var: Ident<'a>, value: Expr<'a>, info: Info<'_>, new: bool) {
        if new {
            self.scope
                .vars
                .insert(into_var(var, info), Rc::new(RefCell::new(value.clone())));
        } else {
            let prev = self.get_var_inner(var, info);
            // TODO: use get car/cdrs if needed
            prev.borrow_mut().expr = value.expr;
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

    fn iter_to_cons(
        &self,
        mut char_iter: impl Iterator<Item = &'a str>,
        call: Info<'a>,
    ) -> Expr<'a> {
        if let Some(first) = char_iter.next() {
            let cons = Expr::new_cons(
                call,
                Cons::new(
                    call,
                    Expr::new_literal(call, Lit::new_string(call, first)),
                    self.iter_to_cons(char_iter, call),
                ),
            );
            // use fold
            // let cons = char_iter.fold(Expr::new_cons(call, Cons::new(call, Expr::new_literal(call, Lit::new_string(call, first)),

            cons
        } else {
            Expr::new_literal(call, Lit::new_hempty(call))
        }
    }
}

fn fun_name(cloned: Eval<'_>) -> impl Fn(Expr<'_>) -> Result<Expr<'_>, Stopper<'_>> + '_ {
    move |expr: Expr<'_>| cloned.clone().eval_expr(expr, true)
}

fn into_var<'a>(var: Ident<'a>, info: Info<'_>) -> VarType<'a> {
    let var_name = match var.ident_type {
        IdentType::Var(value) => VarType::Var(value.main),
        IdentType::FnParam(value) => VarType::FnArg(value.main),
        _ => error(
            info,
            format!("expected variable or parameter but found {var}"),
        ),
    };
    var_name
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
