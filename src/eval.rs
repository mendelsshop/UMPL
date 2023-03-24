// use log::info;

// use std::{
//     cell::{RefCell, RefMut},
//     collections::HashMap,
//     fmt::{self, Display},
//     fs::{self, File, OpenOptions},
//     io::{Read, Write},
//     mem::swap,
//     rc::Rc,
// };

// use crate::{
//     error::{arg_error, error},
//     parser::{
//         // rules::{IdentifierType, LiteralType, OtherStuff, Stuff},
//         // Thing,
//     },
//     token::TokenType,
// };

// pub fn read_file(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let mut file = OpenOptions::new().read(true).open(file_name)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     drop(file);
//     Ok(contents.clone())
// }

// pub fn write_file(
//     file_name: &str,
//     contents: &str,
//     mode: &str,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     match mode {
//         "w" => {
//             // overwrite existing contents
//             // clear file contents
//             let mut file = OpenOptions::new().write(true).open(file_name)?;
//             file.set_len(0)?;
//             Ok(file.write_all(contents.as_bytes())?)
//         }
//         "a" => {
//             let mut file = OpenOptions::new().append(true).open(file_name)?;
//             Ok(file.write_all(contents.as_bytes())?)
//         }
//         _ => Err("Invalid mode")?,
//     }
// }
// #[derive(PartialEq, Debug, Clone)]
// pub struct NewExpression {
//     pub inside: LiteralOrFile,
//     pub print: bool,
//     pub line: u32,
//     pub new_line: bool,
// }

// impl Display for NewExpression {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             if self.print {
//                 if self.new_line {
//                     format!("{}\n", self.inside)
//                 } else {
//                     format!("{}", self.inside)
//                 }
//             } else {
//                 String::new()
//             }
//         )
//     }
// }

// #[derive(PartialEq, Clone, Debug)]
// pub enum LitOrList {
//     Identifier(Rc<RefCell<NewList>>),
//     Literal(LiteralOrFile),
// }

// impl Display for LitOrList {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Identifier(i) => write!(f, "{}", i.borrow()),
//             Self::Literal(l) => write!(f, "{l}"),
//         }
//     }
// }
// #[derive(PartialEq, Clone, Debug)]
// pub struct NewList {
//     pub car: LitOrList,
//     pub cdr: LitOrList,
// }

// impl NewList {
//     pub fn new(mut thing: Vec<LiteralOrFile>) -> Self {
//         Self {
//             car: LitOrList::Literal(thing.remove(0)),
//             cdr: LitOrList::Literal(thing.remove(0)),
//         }
//     }

//     pub fn set_last(&mut self, new_item: LitOrList) {
//         match &self.cdr {
//             LitOrList::Identifier(i) => i.borrow_mut().set_last(new_item),
//             LitOrList::Literal(i) => {
//                 // check if cdr is hempty
//                 if i == &LiteralOrFile::Literal(LiteralType::Hempty) {
//                     self.cdr = new_item;
//                 }
//             }
//         }
//     }
// }

// impl Display for NewList {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "with: [{}, {}]", self.car, self.cdr)
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum LiteralOrFile {
//     Literal(LiteralType),
//     File(String),
// }

// impl LiteralOrFile {
//     pub fn get_file(self, line: u32, keyword: &TokenType) -> String {
//         match self {
//             Self::File(file) => file,
//             _ => {
//                 error(line, format!("{keyword} requires a file"));
//             }
//         }
//     }
//     pub fn get_string(self, line: u32, keyword: &TokenType) -> String {
//         match self {
//             Self::Literal(LiteralType::String(lit)) => lit,
//             _ => {
//                 error(line, format!("{keyword} requires a string"));
//             }
//         }
//     }
//     pub fn get_number(self, line: u32, keyword: &TokenType) -> f64 {
//         match self {
//             Self::Literal(LiteralType::Number(lit)) => lit,
//             _ => {
//                 error(line, format!("{keyword} requires a number"));
//             }
//         }
//     }
//     pub fn get_bool(self, line: u32, keyword: &TokenType) -> bool {
//         match self {
//             Self::Literal(LiteralType::Boolean(lit)) => lit,
//             _ => {
//                 error(line, format!("{keyword} requires a boolean"));
//             }
//         }
//     }
// }

// impl Display for LiteralOrFile {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Literal(lit) => write!(f, "{lit}"),
//             Self::File(file) => write!(f, "{file}"),
//         }
//     }
// }

// #[derive(PartialEq, Clone, Debug)]
// pub struct NewVairable {
//     pub value: LiteralOrFile,
// }

// impl NewVairable {
//     pub const fn new(value: LiteralOrFile) -> Self {
//         Self { value }
//     }
// }

// impl Display for NewVairable {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "with: {}", self.value)
//     }
// }
// #[derive(PartialEq, Clone, Debug)]
// pub enum NewIdentifierType {
//     List(Rc<RefCell<NewList>>),
//     Vairable(NewVairable),
// }

// impl NewIdentifierType {
//     pub fn to_vec_literaltype(self, line: u32) -> Vec<LiteralOrFile> {
//         match self {
//             Self::Vairable(v) => vec![v.value],
//             _ => error(line, "cannot convert to vec"),
//         }
//     }
// }
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self},
    fs::File,
    mem::swap,
    rc::Rc,
};

use crate::{
    error::error,
    lexer::Lexer,
    parser::{
        rules::{
            Accesor, Expr, ExprType, FnDef, Interlaced, Lambda, Lit, LitType, Module, ModuleType,
            Var,
        },
        Parser,
    },
    token::Info,
};

#[derive(Debug, Default)]
pub struct Scope<'a> {
    vars: HashMap<&'a str, Expr<'a>>,
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
        }
    }
    pub fn new_with_parent(parent: Box<Self>) -> Self {
        Self {
            vars: HashMap::new(),
            functions: HashMap::new(),
            parent_scope: Some(parent),
            files: HashMap::new(),
        }
    }
    pub fn set_var(
        &mut self,
        name: &Interlaced<&'a str, Accesor>,
        value: Expr<'a>,
        recurse: bool,
        info: Info<'_>,
    ) {
        //         // the reason for this being its own method vs using the set method is because it will be easier to use/implemnet getting variable from different scopes
        //         // and also less typing instead of creating a NewIdentifierType you just pass in a vector of LiteralType
        //         let new_val: NewIdentifierType = match value.len() {
        //             0 => error(line, "expected Identifier, got empty list"),
        //             1 => NewIdentifierType::Vairable(NewVairable::new(value.clone().remove(0))),
        //             2 => NewIdentifierType::List(Rc::new(RefCell::new(NewList::new(value.clone())))),
        //             _ => error(
        //                 line,
        //                 "expected Identifier, got list with more than 2 elements",
        //             ),
        //         };
        // if we are setting the car or cdr of a list
        if !name.is_empty() {
            if recurse {
                todo!("set var in parent scope");
            } else {
                // check if the var exists because we are attempting to change it
                if self.has_var(name.main, false) {
                    // get the var
                    let var = self.get_var(name, info);
                    *var = value;
                } else {
                    // if the var does not exist then we error
                    error(info, format!("variable {} does not exist", name.main));
                }
            }
        } else if recurse {
            todo!("set var in parent scope");
        } else {
            self.vars.insert(name.main, value.clone());
        }

        //         match name {
        //             name if name.ends_with(".car") | name.ends_with(".cdr") => {
        //                 let new_name = name.trim_end_matches(".car").trim_end_matches(".cdr");
        //                 if recurse {
        //                     if self.has_var(new_name, false) {
        //                         let new_var = match self.get_var(new_name, line) {
        //                             NewIdentifierType::List(list) => list,
        //                             _ => error(line, "expected list"),
        //                         };
        //                         if name.ends_with(".cdr") {
        //                             new_var.borrow_mut().cdr = match new_val {
        //                                 NewIdentifierType::List(list) => LitOrList::Identifier(list),
        //                                 NewIdentifierType::Vairable(var) => LitOrList::Literal(var.value),
        //                             };
        //                         } else {
        //                             new_var.borrow_mut().car = match new_val {
        //                                 NewIdentifierType::List(list) => LitOrList::Identifier(list),
        //                                 NewIdentifierType::Vairable(var) => LitOrList::Literal(var.value),
        //                             };
        //                         }
        //                         self.vars
        //                             .insert(new_name.to_string(), NewIdentifierType::List(new_var));
        //                     } else {
        //                         self.parent_scope.as_mut().map_or_else(
        //                             || error(line, "variable not found"),
        //                             |parent| {
        //                                 parent.set_var(
        //                                     name,
        //                                     &mut new_val.to_vec_literaltype(line),
        //                                     recurse,
        //                                     line,
        //                                 );
        //                             },
        //                         );
        //                     }
        //                 } else {
        //                     let new_var: Rc<RefCell<NewList>> = match self.get_var(new_name, line) {
        //                         NewIdentifierType::List(list) => list,
        //                         _ => error(line, "expected list"),
        //                     };
        //                     if name.ends_with(".cdr") {
        //                         new_var.borrow_mut().cdr = match new_val {
        //                             NewIdentifierType::List(list) => LitOrList::Identifier(list),
        //                             NewIdentifierType::Vairable(var) => LitOrList::Literal(var.value),
        //                         };
        //                     } else {
        //                         new_var.borrow_mut().car = match new_val {
        //                             NewIdentifierType::List(list) => LitOrList::Identifier(list),
        //                             NewIdentifierType::Vairable(var) => LitOrList::Literal(var.value),
        //                         };
        //                     }
        //                     self.vars
        //                         .insert(new_name.to_string(), NewIdentifierType::List(new_var));
        //                 }
        //             }
        //             _ => {
        //                 if recurse {
        //                     if self.has_var(name, false) {
        //                         self.vars.insert(name.to_string(), new_val);
        //                     } else {
        //                         self.parent_scope.as_mut().map_or_else(
        //                             || error(line, "variable not found"),
        //                             |parent| parent.set_var(name, value, recurse, line),
        //                         );
        //                     }
        //                 } else {
        //                     self.vars.insert(name.to_string(), new_val);
        //                 }
        //             }
        //         }
    }
    //     pub fn set_list(&mut self, name: &str, value: NewList, recurse: bool, line: u32) {
    //         if recurse {
    //             if self.has_var(name, false) {
    //                 self.vars.insert(
    //                     name.to_string(),
    //                     NewIdentifierType::List(Rc::new(RefCell::new(value))),
    //                 );
    //             } else {
    //                 self.parent_scope.as_mut().map_or_else(
    //                     || error(line, "variable not found"),
    //                     |parent| parent.set_list(name, value, recurse, line),
    //                 );
    //             }
    //         } else {
    //             self.vars.insert(
    //                 name.to_string(),
    //                 NewIdentifierType::List(Rc::new(RefCell::new(value))),
    //             );
    //         }
    //     }

    // TODO: possibly make this _mut and have another get_var that doesn't return a mutable reference
    pub fn get_var(
        &mut self,
        name: &Interlaced<&'a str, Accesor>,
        info: Info<'_>,
    ) -> &mut Expr<'a> {
        match self.vars.get_mut(name.main) {
            Some(v) => {
                match (v, name.len()) {
                    // match the type of the variable and accesor length
                    // if the accesor length is 0 then it can be any type
                    // if the accesor length is > 0 then it must be a list
                    (v, 0) => v,
                    (v, _) if matches!(v.expr, ExprType::Cons(_)) => {
                        // todo!("use accesor to get value from list")
                        let mut expr = v;
                        let mut cloned_expr = expr.clone();
                        for (_, accesor) in name.interlaced.iter().enumerate() {
                            if let ExprType::Cons(ref mut list) = &mut expr.expr {
                                match accesor {
                                    Accesor::Car => {
                                        expr = list.car.as_mut();
                                        cloned_expr = expr.clone();
                                    }
                                    Accesor::Cdr => {
                                        expr = list.cdr.as_mut();
                                        cloned_expr = expr.clone();
                                    }
                                }
                            } else {
                                error(
                                    info,
                                    format!(
                                        "only lists can be accessed with car and cdr, got {cloned_expr}",
                                    ),
                                );
                            }
                        }
                        expr
                    }
                    (expr, _) => error(
                        info,
                        format!("only lists can be accessed with car and cdr, got {expr}"),
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
    //     pub fn get_function(&self, name: String) -> Option<(Vec<Thing>, f64, bool)> {
    //         match self.function.get(&name) {
    //             Some((body, args, extra)) => Some((body.clone(), *args, *extra)),
    //             None => self
    //                 .parent_scope
    //                 .as_ref()
    //                 .and_then(|parent| parent.get_function(name)),
    //         }
    //     }
    //     pub fn delete_var(&mut self, name: &str) -> Option<NewIdentifierType> {
    //         self.vars.remove(name)
    //     }
    pub fn has_var(&self, name: &str, recurse: bool) -> bool {
        let name: &str = if name.ends_with(".car") || name.ends_with(".cdr") {
            &name[..name.len() - 4]
        } else {
            name
        };
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
                .map(|(k, v)| format!("{k}: {v}"))
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
                    return Some(Stopper::End(expr));
                }
                _ => match self.eval_expr(expr) {
                    // TODO: proper formatting
                    Ok(expr) => println!("{expr}"),
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
                    error(
                        *module.get_info(),
                        format!("Could not read file {file}"),
                    );
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

    pub fn add_variable(&mut self, variable: Var<'a>) -> Expr<'a> {
        self.scope.set_var(
            &Interlaced::new(variable.name, vec![]),
            variable.value,
            false,
            variable.info,
        );
        Expr::new_literal(
            variable.info,
            Lit::new_string(variable.info, "variable added"),
        )
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
            ExprType::Var(var) => Ok(self.add_variable(*var)),
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
            // if its a literal then its reduced enough
            ExprType::Literal(_) => Ok(expr),
            ExprType::Call(call) => {
                let mut expr_list = call.args.into_iter().map(|expr| self.eval_expr(expr));
                expr_list.find_map(Result::err).map_or_else(|| todo!(), Err)
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
            _ => todo!(), //                 Thing::Identifier(ref variable) => match variable.value {
                          //                     IdentifierType::Vairable(ref name) => {
                          //                         if let Some(pointer) = self.find_pointer_in_other_stuff(&name.value) {
                          //                             self.scope.set_var(
                          //                                 &variable.name,
                          //                                 &mut vec![pointer],
                          //                                 false,
                          //                                 variable.line,
                          //                             );
                          //                         } else {
                          //                             self.scope.set_var(
                          //                                 &variable.name,
                          //                                 &mut vec![LiteralOrFile::Literal(LiteralType::from_other_stuff(
                          //                                     &name.value,
                          //                                     variable.line,
                          //                                 ))],
                          //                                 false,
                          //                                 variable.line,
                          //                             );
                          //                         }
                          //                     }
                          //                     IdentifierType::List(ref list) => {
                          //                         let car: LiteralOrFile =
                          //                             self.find_pointer_in_other_stuff(&list.car).map_or_else(
                          //                                 || {
                          //                                     LiteralOrFile::Literal(LiteralType::from_other_stuff(
                          //                                         &list.car,
                          //                                         variable.line,
                          //                                     ))
                          //                                 },
                          //                                 |pointer| pointer,
                          //                             );
                          //                         let cdr: LiteralOrFile =
                          //                             self.find_pointer_in_other_stuff(&list.cdr).map_or_else(
                          //                                 || {
                          //                                     LiteralOrFile::Literal(LiteralType::from_other_stuff(
                          //                                         &list.cdr,
                          //                                         variable.line,
                          //                                     ))
                          //                                 },
                          //                                 |pointer| pointer,
                          //                             );
                          //                         self.scope.set_var(
                          //                             &variable.name,
                          //                             &mut vec![car, cdr],
                          //                             false,
                          //                             variable.line,
                          //                         );
                          //                     }
                          //                 },
                          //                 Thing::Return(os, line, _) => {
                          //                     let ret: LiteralOrFile =
                          //                         os.map_or(LiteralOrFile::Literal(LiteralType::Hempty), |os| {
                          //                             self.find_pointer_in_other_stuff(&os).map_or_else(
                          //                                 || LiteralOrFile::Literal(LiteralType::from_other_stuff(&os, line)),
                          //                                 |identifier| identifier,
                          //                             )
                          //                         });
                          //                     return Some(Stopper::Return(ret));
                          //                 }
                          //                 Thing::Expression(expr) => {
                          //                     let exprs = self.find_pointer_in_stuff(&expr.inside);
                          //                     print!(
                          //                         "{}",
                          //                         NewExpression {
                          //                             inside: exprs,
                          //                             print: expr.print,
                          //                             line: expr.line,
                          //                             new_line: expr.new_line,
                          //                         }
                          //                     );
                          //                 }
                          //                 Thing::IfStatement(mut if_statement) => {
                          //                     let conditon: LiteralType =
                          //                         match self.find_pointer_in_other_stuff(&if_statement.condition) {
                          //                             Some(pointer) => {
                          //                                 info!("if {:?}", pointer);
                          //                                 match pointer {
                          //                                     LiteralOrFile::Literal(literal) => literal,
                          //                                     _ => error(if_statement.line, "cannot compare files"),
                          //                                 }
                          //                             }
                          //                             None => LiteralType::from_other_stuff(
                          //                                 &if_statement.condition,
                          //                                 if_statement.line,
                          //                             ),
                          //                         };
                          //                     if conditon.type_eq(&LiteralType::Boolean(true)) {
                          //                     } else {
                          //                         error(if_statement.line, "expected boolean, got something else");
                          //                     }
                          //                     self.scope.from_parent();
                          //                     if conditon == LiteralType::Boolean(true) {
                          //                         if_statement.body_true = self.find_functions(if_statement.body_true);
                          //                         let body_true: Option<Stopper> =
                          //                             self.find_variables(if_statement.body_true);
                          //                         self.scope.drop_scope();
                          //                         if let Some(stop) = body_true {
                          //                             match stop {
                          //                                 Stopper::Break | Stopper::Continue => {
                          //                                     if self.in_loop {
                          //                                         return Some(stop);
                          //                                     }
                          //                                     error(if_statement.line, "break or continue outside of loop");
                          //                                 }
                          //                                 Stopper::Return(ret) => {
                          //                                     if self.in_function {
                          //                                         return Some(Stopper::Return(ret));
                          //                                     }
                          //                                     error(if_statement.line, "return outside of function");
                          //                                 }
                          //                             }
                          //                         }
                          //                     } else {
                          //                         if_statement.body_false = self.find_functions(if_statement.body_false);
                          //                         let z = self.find_variables(if_statement.body_false);
                          //                         self.scope.drop_scope();
                          //                         if let Some(stop) = z {
                          //                             if let Stopper::Return(ret) = stop {
                          //                                 if self.in_function {
                          //                                     return Some(Stopper::Return(ret));
                          //                                 }
                          //                                 error(if_statement.line, "return outside of function");
                          //                             } else {
                          //                                 if self.in_loop {
                          //                                     return Some(stop);
                          //                                 }
                          //                                 error(if_statement.line, "break or continue outside of loop");
                          //                             }
                          //                         }
                          //                     }
                          //                 }
                          //                 Thing::LoopStatement(loop_statement) => {
                          //                     'l: loop {
                          //                         self.scope.from_parent();
                          //                         let loop_body = self.find_functions(loop_statement.body.clone());
                          //                         self.in_loop = true;
                          //                         let z: Option<Stopper> = self.find_variables(loop_body.clone());
                          //                         self.scope.drop_scope();
                          //                         if let Some(stop) = z {
                          //                             match stop {
                          //                                 Stopper::Break => break 'l,
                          //                                 Stopper::Continue => continue 'l,
                          //                                 Stopper::Return(ret) => {
                          //                                     if self.in_function {
                          //                                         return Some(Stopper::Return(ret));
                          //                                     }
                          //                                     error(loop_statement.line, "return outside of function");
                          //                                 }
                          //                             }
                          //                         }
                          //                     }
                          //                     self.in_loop = false;
                          //                 }
                          //                 Thing::Break(..) => {
                          //                     return Some(Stopper::Break);
                          //                 }
                          //                 Thing::Continue(..) => {
                          //     return Some(Stopper::Continue);
                          // }
        }
    }

    //     fn find_pointer_in_other_stuff(&mut self, other_stuff: &OtherStuff) -> Option<LiteralOrFile> {
    //         match other_stuff {
    //             OtherStuff::Identifier(ident) => match self.scope.get_var(&ident.name, ident.line) {
    //                 NewIdentifierType::List(..) => {
    //                     error(ident.line, "whole list not supported in call")
    //                 }
    //                 NewIdentifierType::Vairable(var) => match var.value {
    //                     LiteralOrFile::Literal(_) => Some(var.value),
    //                     _ => error(ident.line, "variable is not a literal"),
    //                 },
    //             },
    //             OtherStuff::Expression(expr) => Some(self.find_pointer_in_stuff(&expr.inside)),
    //             _ => None,
    //         }
    //     }
    //     #[allow(clippy::too_many_lines)]
    //     fn find_pointer_in_stuff(&mut self, stuff: &Stuff) -> LiteralOrFile {
    //         // need to make ways to extract values from literaltypes/literal/vars easy with function
    //         match stuff {
    //             Stuff::Identifier(ident) => match self.scope.get_var(&ident.name, ident.line) {
    //                 NewIdentifierType::List(..) => {
    //                     error(ident.line, "whole list not supported in call")
    //                 }
    //                 NewIdentifierType::Vairable(var) => var.value,
    //             },
    //             Stuff::If(ifs) => {
    //                 todo!("evaluate if statements");
    //             }
    //             Stuff::Call(call) => match &call.keyword {
    //                 TokenType::FunctionIdentifier { name } => {
    //                     if let Some(mut function) = self.scope.get_function(name.to_string()) {
    //                         let new_stuff: Vec<LiteralOrFile> = call
    //                             .arguments
    //                             .iter()
    //                             .map(|thing| self.find_pointer_in_stuff(thing))
    //                             .collect();
    //                         arg_error(
    //                             function.1 as u32,
    //                             new_stuff.len() as u32,
    //                             &call.keyword,
    //                             function.2,
    //                             call.line,
    //                         );
    //                         self.scope.from_parent();
    //                         function.0 = self.find_functions(function.0);
    //                         self.in_function = true;
    //                         let mut extra_args: Option<NewList> = None;

    //                         // TODO: once we have more than ammount of arguments specified in function we should label the rest as under one variable $n which is a list
    //                         new_stuff.into_iter().enumerate().for_each(|(i, l)| {
    //                             if i as f64 >= function.1 {
    //                                 if let Some(ref mut list) = extra_args {
    //                                     list.set_last(LitOrList::Identifier(Rc::new(RefCell::new(
    //                                         NewList {
    //                                             car: LitOrList::Literal(l),
    //                                             cdr: LitOrList::Literal(LiteralOrFile::Literal(
    //                                                 LiteralType::Hempty,
    //                                             )),
    //                                         },
    //                                     ))));
    //                                 } else {
    //                                     extra_args = Some(NewList {
    //                                         car: LitOrList::Literal(l),
    //                                         cdr: LitOrList::Literal(LiteralOrFile::Literal(
    //                                             LiteralType::Hempty,
    //                                         )),
    //                                     });
    //                                 }
    //                             } else {
    //                                 self.scope.set_var(
    //                                     format!("${}", i + 1).as_str(),
    //                                     &mut vec![(l)],
    //                                     false,
    //                                     call.line,
    //                                 );
    //                             }
    //                         });
    //                         if let Some(list) = extra_args {
    //                             self.scope.set_list(
    //                                 format!("${}", function.1 as usize + 1).as_str(),
    //                                 list,
    //                                 false,
    //                                 call.line,
    //                             );
    //                         }
    //                         let z: Option<Stopper> = self.find_variables(function.0);
    //                         self.in_function = false;
    //                         self.scope.drop_scope();
    //                         z.map_or(LiteralOrFile::Literal(LiteralType::Hempty), |v| {
    //                             if let Stopper::Return(a) = v {
    //                                 a
    //                             } else {
    //                                 error(call.line, "cannot call break/continue at end of function");
    //                             }
    //                         })
    //                     } else {
    //                         error(call.line, format!("Function {name} is not defined"));
    //                     }
    //                 }
    //                 TokenType::Type => {
    //                     arg_error(
    //                         1,
    //                         call.arguments.len() as u32,
    //                         &call.keyword,
    //                         false,
    //                         call.line,
    //                     );
    //                     match self.find_pointer_in_stuff(&call.arguments[0]) {
    //                         LiteralOrFile::Literal(a) => {
    //                             LiteralOrFile::Literal(LiteralType::String(a.get_type()))
    //                         }
    //                         LiteralOrFile::File(_) => {
    //                             LiteralOrFile::Literal(LiteralType::String("file".to_string()))
    //                         }
    //                     }
    //                 }
    //                 TokenType::Delete => {
    //                     if call.arguments.len() != 1 {
    //                         error(call.line, "delete takes one argument");
    //                     }
    //                     if let Stuff::Identifier(ident) = &call.arguments[0] {
    //                         if self.scope.delete_var(&ident.name).is_some() {
    //                             LiteralOrFile::Literal(LiteralType::Hempty)
    //                         } else {
    //                             error(
    //                                 ident.line,
    //                                 format!("Variable {} is not defined", ident.name),
    //                             );
    //                         }
    //                     } else {
    //                         error(call.line, "delete only takes a variable name")
    //                     }
    //                 }
    //                 TokenType::AddWith
    //                 | TokenType::SubtractWith
    //                 | TokenType::DivideWith
    //                 | TokenType::MultiplyWith
    //                 | TokenType::Set => {
    //                     if let Stuff::Identifier(ident) = &call.arguments[0] {
    //                         if self.scope.has_var(&ident.name, true) {
    //                             let mut new_stuff: Vec<LiteralOrFile> = Vec::new();
    //                             call.arguments.iter().skip(1).for_each(|thing| {
    //                                 new_stuff.push(self.find_pointer_in_stuff(thing));
    //                             });
    //                             match new_stuff.len() {
    //                                 1 => {
    //                                     let literal: LiteralOrFile = new_stuff.remove(0);
    //                                     let var: LiteralOrFile =
    //                                         match self.scope.get_var(&ident.name, call.line) {
    //                                             NewIdentifierType::Vairable(v) => v.value,
    //                                             NewIdentifierType::List(..) => {
    //                                                 error(ident.line, "Cannot change entire list");
    //                                             }
    //                                         };
    //                                     match call.keyword {
    //                                         TokenType::Set => {
    //                                             self.scope.set_var(
    //                                                 &ident.name,
    //                                                 &mut vec![literal.clone()],
    //                                                 true,
    //                                                 call.line,
    //                                             );
    //                                             literal
    //                                         }
    //                                         TokenType::AddWith => match var {
    //                                             LiteralOrFile::Literal(LiteralType::Number(num)) => {
    //                                                 if let LiteralOrFile::Literal(
    //                                                     LiteralType::Number(num2),
    //                                                 ) = literal
    //                                                 {
    //                                                     let new_val = num + num2;
    //                                                     self.scope.set_var(
    //                                                         &ident.name,
    //                                                         &mut vec![LiteralOrFile::Literal(
    //                                                             LiteralType::Number(new_val),
    //                                                         )],
    //                                                         true,
    //                                                         call.line,
    //                                                     );
    //                                                     LiteralOrFile::Literal(LiteralType::Number(
    //                                                         new_val,
    //                                                     ))
    //                                                 } else {
    //                                                     error(
    //                                                         call.line,
    //                                                         format!(
    //                                                             "Variable {} is not a number",
    //                                                             ident.name
    //                                                         ),
    //                                                     );
    //                                                 }
    //                                             }
    //                                             LiteralOrFile::Literal(LiteralType::String(mut s)) => {
    //                                                 match literal {
    //                                                     LiteralOrFile::Literal(
    //                                                         LiteralType::String(s2),
    //                                                     ) => {
    //                                                         s.push_str(s2.as_str());
    //                                                         self.scope.set_var(
    //                                                             &ident.name,
    //                                                             &mut vec![LiteralOrFile::Literal(
    //                                                                 LiteralType::String(s.clone()),
    //                                                             )],
    //                                                             true,
    //                                                             call.line,
    //                                                         );
    //                                                         LiteralOrFile::Literal(LiteralType::String(
    //                                                             s,
    //                                                         ))
    //                                                     }
    //                                                     LiteralOrFile::Literal(
    //                                                         LiteralType::Number(n),
    //                                                     ) => {
    //                                                         s.push_str(&n.to_string());
    //                                                         self.scope.set_var(
    //                                                             &ident.name,
    //                                                             &mut vec![LiteralOrFile::Literal(
    //                                                                 LiteralType::String(s.clone()),
    //                                                             )],
    //                                                             true,
    //                                                             call.line,
    //                                                         );
    //                                                         LiteralOrFile::Literal(LiteralType::String(
    //                                                             s,
    //                                                         ))
    //                                                     }
    //                                                     LiteralOrFile::Literal(
    //                                                         LiteralType::Boolean(boolean),
    //                                                     ) => {
    //                                                         s.push_str(&boolean.to_string());
    //                                                         self.scope.set_var(
    //                                                             &ident.name,
    //                                                             &mut vec![LiteralOrFile::Literal(
    //                                                                 LiteralType::String(s.clone()),
    //                                                             )],
    //                                                             true,
    //                                                             call.line,
    //                                                         );
    //                                                         LiteralOrFile::Literal(LiteralType::String(
    //                                                             s,
    //                                                         ))
    //                                                     }
    //                                                     LiteralOrFile::Literal(LiteralType::Hempty) => {
    //                                                         s.push_str("hempty");
    //                                                         self.scope.set_var(
    //                                                             &ident.name,
    //                                                             &mut vec![LiteralOrFile::Literal(
    //                                                                 LiteralType::String(s.clone()),
    //                                                             )],
    //                                                             true,
    //                                                             call.line,
    //                                                         );
    //                                                         LiteralOrFile::Literal(LiteralType::String(
    //                                                             s,
    //                                                         ))
    //                                                     }
    //                                                     _ => {
    //                                                         error(
    //                                                             call.line,
    //                                                             format!(
    //                                                                 "Variable {} is not a string",
    //                                                                 ident.name
    //                                                             ),
    //                                                         );
    //                                                     }
    //                                                 }
    //                                             }
    //                                             _ => {
    //                                                 error(
    //                                                     call.line,
    //                                                     format!(
    //                                                         "Variable {} is not a number/string",
    //                                                         ident.name
    //                                                     ),
    //                                                 );
    //                                             }
    //                                         },
    //                                         TokenType::MultiplyWith => match var {
    //                                             LiteralOrFile::Literal(LiteralType::Number(num)) => {
    //                                                 if let LiteralOrFile::Literal(
    //                                                     LiteralType::Number(num2),
    //                                                 ) = literal
    //                                                 {
    //                                                     let new_val: f64 = num * num2;
    //                                                     self.scope.set_var(
    //                                                         &ident.name,
    //                                                         &mut vec![LiteralOrFile::Literal(
    //                                                             LiteralType::Number(new_val),
    //                                                         )],
    //                                                         true,
    //                                                         call.line,
    //                                                     );
    //                                                     LiteralOrFile::Literal(LiteralType::Number(
    //                                                         new_val,
    //                                                     ))
    //                                                 } else {
    //                                                     error(
    //                                                         call.line,
    //                                                         format!(
    //                                                             "Variable {} is not a number",
    //                                                             ident.name
    //                                                         ),
    //                                                     );
    //                                                 }
    //                                             }
    //                                             LiteralOrFile::Literal(LiteralType::String(ref s)) => {
    //                                                 if let LiteralOrFile::Literal(
    //                                                     LiteralType::Number(num),
    //                                                 ) = literal
    //                                                 {
    //                                                     let new_string: String = (0..num as i32)
    //                                                         .map(|_| s.to_string())
    //                                                         .collect();
    //                                                     self.scope.set_var(
    //                                                         &ident.name,
    //                                                         &mut vec![LiteralOrFile::Literal(
    //                                                             LiteralType::String(new_string.clone()),
    //                                                         )],
    //                                                         true,
    //                                                         call.line,
    //                                                     );
    //                                                     LiteralOrFile::Literal(LiteralType::String(
    //                                                         new_string,
    //                                                     ))
    //                                                 } else {
    //                                                     error(
    //                                                         call.line,
    //                                                         format!(
    //                                                             "Variable {} is not a number",
    //                                                             ident.name
    //                                                         ),
    //                                                     );
    //                                                 }
    //                                             }
    //                                             _ => {
    //                                                 error(
    //                                                     call.line,
    //                                                     format!(
    //                                                         "Variable {} is not a number/string",
    //                                                         ident.name
    //                                                     ),
    //                                                 );
    //                                             }
    //                                         },
    //                                         TokenType::SubtractWith | TokenType::DivideWith => {
    //                                             if let LiteralOrFile::Literal(LiteralType::Number(
    //                                                 nums,
    //                                             )) = var
    //                                             {
    //                                                 if let LiteralOrFile::Literal(
    //                                                     LiteralType::Number(num),
    //                                                 ) = literal
    //                                                 {
    //                                                     if call.keyword == TokenType::SubtractWith {
    //                                                         let new_val = nums - num;
    //                                                         self.scope.set_var(
    //                                                             &ident.name,
    //                                                             &mut vec![LiteralOrFile::Literal(
    //                                                                 LiteralType::Number(new_val),
    //                                                             )],
    //                                                             true,
    //                                                             call.line,
    //                                                         );
    //                                                         LiteralOrFile::Literal(LiteralType::Number(
    //                                                             new_val,
    //                                                         ))
    //                                                     } else {
    //                                                         let new_val = nums / num;
    //                                                         self.scope.set_var(
    //                                                             &ident.name,
    //                                                             &mut vec![LiteralOrFile::Literal(
    //                                                                 LiteralType::Number(new_val),
    //                                                             )],
    //                                                             true,
    //                                                             call.line,
    //                                                         );
    //                                                         LiteralOrFile::Literal(LiteralType::Number(
    //                                                             new_val,
    //                                                         ))
    //                                                     }
    //                                                 } else {
    //                                                     error(
    //                                                         call.line,
    //                                                         format!(
    //                                                             "Variable {} is not a number",
    //                                                             ident.name
    //                                                         ),
    //                                                     );
    //                                                 }
    //                                             } else {
    //                                                 error(
    //                                                     call.line,
    //                                                     format!(
    //                                                         "Variable {} is not a number/string",
    //                                                         ident.name
    //                                                     ),
    //                                                 );
    //                                             }
    //                                         }
    //                                         _ => {
    //                                             error(
    //                                                 call.line,
    //                                                 format!(
    //                                                     "Invalid operator for literal {}",
    //                                                     call.keyword
    //                                                 ),
    //                                             );
    //                                         }
    //                                     }
    //                                 }
    //                                 2 => {
    //                                     if call.keyword == TokenType::Set {
    //                                         self.scope.set_var(
    //                                             &ident.name,
    //                                             &mut new_stuff,
    //                                             true,
    //                                             call.line,
    //                                         );
    //                                         LiteralOrFile::Literal(LiteralType::String(
    //                                             new_stuff
    //                                                 .iter()
    //                                                 .map(std::string::ToString::to_string)
    //                                                 .collect(),
    //                                         ))
    //                                     } else {
    //                                         error(
    //                                             call.line,
    //                                             format!(
    //                                                 "Too many arguments for function {}",
    //                                                 call.keyword
    //                                             )
    //                                             .as_str(),
    //                                         );
    //                                     }
    //                                 }
    //                                 _ => error(
    //                                     call.line,
    //                                     format!("Too many arguments for function {}", call.keyword)
    //                                         .as_str(),
    //                                 ),
    //                             }
    //                         } else {
    //                             error(
    //                                 ident.line,
    //                                 format!("Variable {} is not defined", ident.name),
    //                             );
    //                         }
    //                     } else {
    //                         error(
    //                             call.line,
    //                             format!("First argument of {} must be an identifier", call.keyword)
    //                                 .as_str(),
    //                         );
    //                     }
    //                 }
    //                 TokenType::Open => {
    //                     arg_error(
    //                         1,
    //                         call.arguments.len() as u32,
    //                         &call.keyword,
    //                         false,
    //                         call.line,
    //                     );
    //                     // check if the first argument is a string
    //                     let arg = self
    //                         .find_pointer_in_stuff(&call.arguments[0])
    //                         .get_string(call.line, &call.keyword);
    //                     if std::path::Path::new(&arg).exists() {
    //                         LiteralOrFile::File(arg)
    //                     } else {
    //                         error(
    //                             call.line,
    //                             format!("Could not open file {arg}: does not exist").as_str(),
    //                         );
    //                     }
    //                 }
    //                 TokenType::Close | TokenType::Read => {
    //                     arg_error(
    //                         1,
    //                         call.arguments.len() as u32,
    //                         &call.keyword,
    //                         false,
    //                         call.line,
    //                     );
    //                     // evalute args[0] and check if it is a file
    //                     match &call.arguments[0] {
    //                         Stuff::Identifier(ident) => {
    //                             let files = self.scope.get_var(&ident.name, call.line);
    //                             match files {
    //                                 NewIdentifierType::Vairable(var) => {
    //                                     match var.value {
    //                                         LiteralOrFile::File(file) => {
    //                                             // set idnetifier to nothing
    //                                             match call.keyword {
    //                                                 TokenType::Close => {
    //                                                     self.scope.set_var(
    //                                                         &ident.name,
    //                                                         &mut vec![LiteralOrFile::Literal(
    //                                                             LiteralType::Hempty,
    //                                                         )],
    //                                                         true,
    //                                                         call.line,
    //                                                     );
    //                                                 }
    //                                                 TokenType::Read => {
    //                                                     let contents = // create a string to hold the contents of the file
    //                                                     match read_file(&file) {
    //                                                         Ok(contents) => contents,
    //                                                         Err(err) => {
    //                                                             error(call.line, format!("{err}"));
    //                                                         }
    //                                                     }; // read the file into the string
    //                                                     return LiteralOrFile::Literal(
    //                                                         LiteralType::String(contents),
    //                                                     ); // return the string
    //                                                 }
    //                                                 _ => {}
    //                                             }
    //                                         }
    //                                         _ => error(
    //                                             call.line,
    //                                             format!("{} is not a file", ident.name).as_str(),
    //                                         ),
    //                                     }
    //                                 }
    //                                 NewIdentifierType::List(_) => error(
    //                                     call.line,
    //                                     format!("Variable {} is not a file", ident.name).as_str(),
    //                                 ),
    //                             }
    //                         }
    //                         other => {
    //                             match self.find_pointer_in_stuff(other) {
    //                                 LiteralOrFile::File(file) => {
    //                                     match call.keyword {
    //                                         TokenType::Close => {
    //                                             self.scope.files.remove(&file);
    //                                         }
    //                                         TokenType::Read => {
    //                                             let contents: String = String::new(); // create a string to hold the contents of the file
    //                                             match read_file(&file) {
    //                                                 Ok(contents) => contents,
    //                                                 Err(err) => {
    //                                                     error(call.line, format!("{err}"));
    //                                                 }
    //                                             }; // read the file into the string
    //                                             return LiteralOrFile::Literal(LiteralType::String(
    //                                                 contents,
    //                                             )); // return the string
    //                                         }
    //                                         _ => {}
    //                                     }
    //                                 }
    //                                 _ => {
    //                                     error(
    //                                         call.line,
    //                                         format!(
    //                                             "First argument of {} must be a file",
    //                                             call.keyword
    //                                         )
    //                                         .as_str(),
    //                                     );
    //                                 }
    //                             };
    //                         }
    //                     }
    //                     LiteralOrFile::Literal(LiteralType::Hempty)
    //                 }
    //                 TokenType::Write => {
    //                     //takes 3 arguments:
    //                     // 1. the file
    //                     // 2. the string to write
    //                     // 3. the mode (a: append or w: overwrite)
    //                     arg_error(
    //                         3,
    //                         call.arguments.len() as u32,
    //                         &call.keyword,
    //                         false,
    //                         call.line,
    //                     );
    //                     // get the file
    //                     let file = self
    //                         .find_pointer_in_stuff(&call.arguments[0])
    //                         .get_file(call.line, &call.keyword);
    //                     // get the string
    //                     let string = self
    //                         .find_pointer_in_stuff(&call.arguments[1])
    //                         .get_string(call.line, &call.keyword);
    //                     // get the mode
    //                     let mode = self
    //                         .find_pointer_in_stuff(&call.arguments[2])
    //                         .get_string(call.line, &call.keyword);
    //                     // write the string to the file
    //                     match write_file(&file, &string, &mode) {
    //                         Ok(_) => LiteralOrFile::Literal(LiteralType::Hempty),
    //                         Err(err) => {
    //                             error(call.line, format!("{err}").as_str());
    //                         }
    //                     }
    //                 }
    //                 TokenType::ReadLine => {
    //                     // takes 2 arguments: file, line
    //                     arg_error(
    //                         2,
    //                         call.arguments.len() as u32,
    //                         &call.keyword,
    //                         false,
    //                         call.line,
    //                     );
    //                     // get the file
    //                     let file = self
    //                         .find_pointer_in_stuff(&call.arguments[0])
    //                         .get_file(call.line, &call.keyword);
    //                     // get the line
    //                     let line = self
    //                         .find_pointer_in_stuff(&call.arguments[1])
    //                         .get_number(call.line, &call.keyword);
    //                     // read the the file
    //                     match read_file(&file) {
    //                         Ok(contents) => {
    //                             let lines = contents.split('\n').collect::<Vec<&str>>();
    //                             if line as usize > lines.len() {
    //                                 error(
    //                                     call.line,
    //                                     format!("Line {line} does not exist in file {file}"),
    //                                 );
    //                             }
    //                             LiteralOrFile::Literal(LiteralType::String(
    //                                 lines[line as usize - 1].to_string(),
    //                             ))
    //                         }
    //                         Err(err) => {
    //                             error(call.line, format!("{err}").as_str());
    //                         }
    //                     }
    //                 }
    //                 TokenType::WriteLine => {
    //                     // takes 4 arguments: file, line, string and mode (a: append to beging of line or w: overwrite)
    //                     arg_error(
    //                         4,
    //                         call.arguments.len() as u32,
    //                         &call.keyword,
    //                         false,
    //                         call.line,
    //                     );
    //                     // get the file
    //                     let file = self
    //                         .find_pointer_in_stuff(&call.arguments[0])
    //                         .get_file(call.line, &call.keyword);
    //                     // get the string
    //                     let mut string = self
    //                         .find_pointer_in_stuff(&call.arguments[1])
    //                         .get_string(call.line, &call.keyword);
    //                     // get the line
    //                     let line = self
    //                         .find_pointer_in_stuff(&call.arguments[2])
    //                         .get_number(call.line, &call.keyword);
    //                     // get the mode
    //                     let mode = self
    //                         .find_pointer_in_stuff(&call.arguments[3])
    //                         .get_string(call.line, &call.keyword);
    //                     // read the file
    //                     let mut contents = match read_file(&file) {
    //                         Ok(contents) => contents,
    //                         Err(err) => {
    //                             error(call.line, format!("{err}").as_str());
    //                         }
    //                     };
    //                     // split the contents into lines
    //                     let mut lines = contents.split('\n').collect::<Vec<&str>>();
    //                     // if the line is greater than the number of lines, add a new line
    //                     if line as usize > lines.len() {
    //                         error(call.line, "Line does not exist in file");
    //                     }
    //                     string = match mode.as_str() {
    //                         "a" => {
    //                             format!("{string}{}", lines[line as usize - 1],)
    //                         }
    //                         "w" => string,
    //                         _ => {
    //                             error(
    //                                 call.line,
    //                                 format!("Mode {mode} is not a valid mode").as_str(),
    //                             );
    //                         }
    //                     };
    //                     lines[line as usize - 1] = string.as_str();
    //                     // collect all lines
    //                     contents = lines.join("\n");
    //                     // write the file
    //                     match write_file(&file, &contents, "w") {
    //                         Ok(_) => LiteralOrFile::Literal(LiteralType::Hempty),
    //                         Err(err) => {
    //                             error(call.line, format!("{err}").as_str());
    //                         }
    //                     }
    //                 }
    //                 TokenType::DeleteFile | TokenType::CreateFile => {
    //                     // takes 1 argument: file
    //                     arg_error(
    //                         1,
    //                         call.arguments.len() as u32,
    //                         &call.keyword,
    //                         false,
    //                         call.line,
    //                     );
    //                     // get the file
    //                     let file = self
    //                         .find_pointer_in_stuff(&call.arguments[0])
    //                         .get_file(call.line, &call.keyword);
    //                     // match delete or create file
    //                     match call.keyword {
    //                         TokenType::DeleteFile => match fs::remove_file(&file) {
    //                             Ok(_) => LiteralOrFile::Literal(LiteralType::Hempty),
    //                             Err(err) => {
    //                                 error(call.line, format!("{err}").as_str());
    //                             }
    //                         },
    //                         TokenType::CreateFile => {
    //                             // create the file
    //                             match OpenOptions::new().create(true).open(&file) {
    //                                 Ok(_) => LiteralOrFile::File(file),
    //                                 Err(err) => {
    //                                     error(call.line, format!("{err}").as_str());
    //                                 }
    //                             }
    //                         }
    //                         _ => LiteralOrFile::Literal(LiteralType::Hempty),
    //                     }
    //                 }
    //                 t => {
    //                     let mut new_stuff: Vec<LiteralType> = Vec::new();
    //                     call.arguments.iter().for_each(|thing| {
    //                         match self.find_pointer_in_stuff(thing) {
    //                             LiteralOrFile::Literal(literal) => {
    //                                 new_stuff.push(literal);
    //                             }
    //                             LiteralOrFile::File(_) => {
    //                                 error(
    //                                     call.line,
    //                                     format!(
    //                                         "Cannot use file as argument for function {}",
    //                                         call.keyword
    //                                     )
    //                                     .as_str(),
    //                                 );
    //                             }
    //                         }
    //                     });
    //                     LiteralOrFile::Literal(t.r#do(&new_stuff, call.line, self))
    //                 }
    //             },
    //             Stuff::Literal(lit) => LiteralOrFile::Literal(lit.literal.clone()),
    //             Stuff::Function(fn_def) => {
    //                 todo!("evaluate function definition");
    //             }
    //             Stuff::List(list) => {
    //                 todo!("evaluate list definition");
    //             }
    //         }
    //     }
}

impl fmt::Debug for Eval<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "scope {:?}", self.scope)?;
        Ok(())
    }
}
