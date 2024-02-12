use log::{debug, info};

use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    fmt::{self, Display},
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    mem::swap,
    rc::Rc,
};

use crate::{
    error::{arg_error, error},
    parser::rules::{
        Ast, Call, Declaration, DeclarationType, Function, Identifier, If, LiteralNode, Return,
    },
    token::TokenType,
};

pub fn read_file(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().read(true).open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.clone())
}

pub fn write_file(
    file_name: &str,
    contents: &str,
    mode: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match mode {
        "w" => {
            // overwrite existing contents
            // clear file contents
            let mut file = OpenOptions::new().write(true).open(file_name)?;
            file.set_len(0)?;
            Ok(file.write_all(contents.as_bytes())?)
        }
        "a" => {
            let mut file = OpenOptions::new().append(true).open(file_name)?;
            Ok(file.write_all(contents.as_bytes())?)
        }
        _ => Err("Invalid mode")?,
    }
}
#[derive(PartialEq, Debug, Clone)]
pub struct NewExpression {
    pub inside: LiteralOrFile,
    pub print: bool,
    pub line: i32,
    pub new_line: bool,
}

impl Display for NewExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            if self.print {
                if self.new_line {
                    format!("{}\n", self.inside)
                } else {
                    format!("{}", self.inside)
                }
            } else {
                String::new()
            }
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum LitOrList {
    Identifier(Rc<RefCell<NewList>>),
    Literal(LiteralOrFile),
}

impl Display for LitOrList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(i) => write!(f, "{}", i.borrow()),
            Self::Literal(l) => write!(f, "{l}"),
        }
    }
}
#[derive(PartialEq, Clone, Debug)]
pub struct NewList {
    pub car: LitOrList,
    pub cdr: LitOrList,
}

impl NewList {
    pub fn new(mut thing: Vec<LiteralOrFile>) -> Self {
        Self {
            car: LitOrList::Literal(thing.remove(0)),
            cdr: LitOrList::Literal(thing.remove(0)),
        }
    }

    pub fn set_last(&mut self, new_item: LitOrList) {
        match &self.cdr {
            LitOrList::Identifier(i) => i.borrow_mut().set_last(new_item),
            LitOrList::Literal(i) => {
                // check if cdr is hempty
                if i == &LiteralOrFile::Literal(LiteralNode::Hempty) {
                    self.cdr = new_item;
                }
            }
        }
    }
}

impl Display for NewList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "with: [{}, {}]", self.car, self.cdr)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralOrFile {
    Literal(LiteralNode),
    File(String),
}

impl LiteralOrFile {
    pub fn get_file(self, line: i32, keyword: &TokenType) -> String {
        match self {
            Self::File(file) => file,
            _ => {
                error(line, format!("{keyword} requires a file"));
            }
        }
    }
    pub fn get_string(self, line: i32, keyword: &TokenType) -> String {
        match self {
            Self::Literal(LiteralNode::String(lit)) => lit,
            _ => {
                error(line, format!("{keyword} requires a string"));
            }
        }
    }
    pub fn get_number(self, line: i32, keyword: &TokenType) -> f64 {
        match self {
            Self::Literal(LiteralNode::Number(lit)) => lit,
            _ => {
                error(line, format!("{keyword} requires a number"));
            }
        }
    }
    pub fn get_bool(self, line: i32, keyword: &TokenType) -> bool {
        match self {
            Self::Literal(LiteralNode::Boolean(lit)) => lit,
            _ => {
                error(line, format!("{keyword} requires a boolean"));
            }
        }
    }
}

impl Display for LiteralOrFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(lit) => write!(f, "{lit}"),
            Self::File(file) => write!(f, "{file}"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct NewVairable {
    pub value: LiteralOrFile,
}

impl NewVairable {
    pub const fn new(value: LiteralOrFile) -> Self {
        Self { value }
    }
}

impl Display for NewVairable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "with: {}", self.value)
    }
}
#[derive(PartialEq, Clone, Debug)]
pub enum NewIdentifierType {
    List(Rc<RefCell<NewList>>),
    Vairable(NewVairable),
}

impl NewIdentifierType {
    pub fn to_vec_literal(self, line: i32) -> Vec<LiteralOrFile> {
        match self {
            Self::Vairable(v) => vec![v.value],
            _ => error(line, "cannot convert to vec"),
        }
    }
}

#[derive(Debug)]
pub struct Scope {
    pub vars: HashMap<String, NewIdentifierType>,
    pub function: HashMap<char, (Vec<Ast>, f64, bool)>,
    pub parent_scope: Option<Box<Scope>>,
    pub files: HashMap<String, File>,
    pub open_modules: [Option<Box<Module>>; 26],
}

#[derive(Debug)]
pub struct Module {
    /// parent scope will be empty unless its an inline module
    pub scope: Scope,
    pub missing_function_handler: Option<(Vec<Ast>, f64, bool)>,
}

impl Module {
    pub fn new(scope: Scope) -> Self {
        Self {
            scope,
            missing_function_handler: None,
        }
    }

    pub fn set_missing_function_handler(
        &mut self,
        missing_function_handler: Option<(Vec<Ast>, f64, bool)>,
    ) {
        self.missing_function_handler = missing_function_handler;
    }

    pub fn get_function(&self, name: char, path: &[char]) -> (Vec<Ast>, f64, bool) {
        self.scope.get_function(name, path).unwrap_or(
            self.missing_function_handler
                .clone()
                .unwrap_or_else(|| todo!("defualt handler")),
        )
    }
}

impl Scope {
    const EMPTY_MODULE: Option<Box<Module>> = None;
    pub fn new() -> Self {
        let open_modules = [Self::EMPTY_MODULE; 26];
        Self {
            vars: HashMap::new(),
            function: HashMap::new(),
            parent_scope: None,
            files: HashMap::new(),
            open_modules,
        }
    }
    pub fn new_with_parent(parent: Box<Self>) -> Self {
        let open_modules = [Self::EMPTY_MODULE; 26];
        Self {
            vars: HashMap::new(),
            function: HashMap::new(),
            parent_scope: Some(parent),
            files: HashMap::new(),
            open_modules,
        }
    }
    pub fn set_var(
        &mut self,
        name: &str,
        value: &mut Vec<LiteralOrFile>,
        recurse: bool,
        line: i32,
    ) {
        // the reason for this being its own method vs using the set method is because it will be easier to use/implemnet getting variable from different scopes
        // and also less typing instead of creating a NewIdentifierType you just pass in a vector of LiteralNode
        debug!("setting var: {} to: {:?}", name, value);
        let new_val: NewIdentifierType = match value.len() {
            0 => error(line, "expected Identifier, got empty list"),
            1 => NewIdentifierType::Vairable(NewVairable::new(value.clone().remove(0))),
            2 => NewIdentifierType::List(Rc::new(RefCell::new(NewList::new(value.clone())))),
            _ => error(
                line,
                "expected Identifier, got list with more than 2 elements",
            ),
        };
        match name {
            name if name.ends_with(".car") | name.ends_with(".cdr") => {
                let new_name = name.trim_end_matches(".car").trim_end_matches(".cdr");
                if recurse {
                    if self.has_var(new_name, false) {
                        let new_var = match self.get_var(new_name, line) {
                            NewIdentifierType::List(list) => list,
                            _ => error(line, "expected list"),
                        };
                        if name.ends_with(".cdr") {
                            new_var.borrow_mut().cdr = match new_val {
                                NewIdentifierType::List(list) => LitOrList::Identifier(list),
                                NewIdentifierType::Vairable(var) => LitOrList::Literal(var.value),
                            };
                        } else {
                            new_var.borrow_mut().car = match new_val {
                                NewIdentifierType::List(list) => LitOrList::Identifier(list),
                                NewIdentifierType::Vairable(var) => LitOrList::Literal(var.value),
                            };
                        }
                        self.vars
                            .insert(new_name.to_string(), NewIdentifierType::List(new_var));
                    } else {
                        self.parent_scope.as_mut().map_or_else(
                            || error(line, "variable not found"),
                            |parent| {
                                parent.set_var(
                                    name,
                                    &mut new_val.to_vec_literal(line),
                                    recurse,
                                    line,
                                );
                            },
                        );
                    }
                } else {
                    let new_var: Rc<RefCell<NewList>> = match self.get_var(new_name, line) {
                        NewIdentifierType::List(list) => list,
                        _ => error(line, "expected list"),
                    };
                    if name.ends_with(".cdr") {
                        new_var.borrow_mut().cdr = match new_val {
                            NewIdentifierType::List(list) => LitOrList::Identifier(list),
                            NewIdentifierType::Vairable(var) => LitOrList::Literal(var.value),
                        };
                    } else {
                        new_var.borrow_mut().car = match new_val {
                            NewIdentifierType::List(list) => LitOrList::Identifier(list),
                            NewIdentifierType::Vairable(var) => LitOrList::Literal(var.value),
                        };
                    }
                    self.vars
                        .insert(new_name.to_string(), NewIdentifierType::List(new_var));
                }
            }
            _ => {
                if recurse {
                    if self.has_var(name, false) {
                        self.vars.insert(name.to_string(), new_val);
                    } else {
                        self.parent_scope.as_mut().map_or_else(
                            || error(line, "variable not found"),
                            |parent| parent.set_var(name, value, recurse, line),
                        );
                    }
                } else {
                    self.vars.insert(name.to_string(), new_val);
                }
            }
        }
    }
    pub fn set_list(&mut self, name: &str, value: NewList, recurse: bool, line: i32) {
        if recurse {
            if self.has_var(name, false) {
                self.vars.insert(
                    name.to_string(),
                    NewIdentifierType::List(Rc::new(RefCell::new(value))),
                );
            } else {
                self.parent_scope.as_mut().map_or_else(
                    || error(line, "variable not found"),
                    |parent| parent.set_list(name, value, recurse, line),
                );
            }
        } else {
            self.vars.insert(
                name.to_string(),
                NewIdentifierType::List(Rc::new(RefCell::new(value))),
            );
        }
    }
    pub fn get_var(&mut self, name: &str, line: i32) -> NewIdentifierType {
        // the reason for this being its own method vs using the get method is because it will be easier to use/implemnet getting variable from different scopes
        if name.ends_with(".car") || name.ends_with(".cdr") {
            if let NewIdentifierType::List(list) = self.get_var(&name[..name.len() - 4], line) {
                if name.ends_with(".car") {
                    match &list.borrow_mut().car {
                        LitOrList::Identifier(list2) => {
                            return NewIdentifierType::List(Rc::clone(list2));
                        }
                        LitOrList::Literal(var) => match var {
                            LiteralOrFile::Literal(_) => {
                                return NewIdentifierType::Vairable(NewVairable::new(var.clone()));
                            }
                            _ => error(line, "expected literal"),
                        },
                    }
                }
                match &list.borrow_mut().cdr {
                    LitOrList::Identifier(list2) => {
                        return NewIdentifierType::List(Rc::clone(&list2));
                    }
                    LitOrList::Literal(var) => match var {
                        LiteralOrFile::Literal(_) => {
                            return NewIdentifierType::Vairable(NewVairable::new(var.clone()));
                        }
                        _ => error(line, "expected literal"),
                    },
                }
            }
            error(line, "expected list, got something else");
        }
        match self.vars.get(name) {
            Some(v) => match v {
                NewIdentifierType::Vairable(_) => v.clone(),
                NewIdentifierType::List(list) => NewIdentifierType::List(list.clone()),
            },
            None => self.parent_scope.as_mut().map_or_else(
                || error(line, format!("variable not found {name}")),
                |parent| parent.get_var(name, line),
            ),
        }
    }
    pub fn set_function(&mut self, name: char, body: Vec<Ast>, args: f64, extra: bool) {
        self.function.insert(name, (body, args, extra));
    }
    pub fn get_function(&self, name: char, path: &[char]) -> Option<(Vec<Ast>, f64, bool)> {
        if let Some(m) = path.first() {
            // TODO: turn oprion into result b/c we could not find functions b/c module doesn't
            // exist, not b/c function doesnt't exist needs differenent handler
            Some(
                self.open_modules[m.to_ascii_uppercase() as usize - 65]
                    .as_ref()?
                    .get_function(name, &path[1..])
                    .clone(),
            )
        } else {
            match self.function.get(&name) {
                Some((body, args, extra)) => Some((body.clone(), *args, *extra)),
                None => self
                    .parent_scope
                    .as_ref()
                    .and_then(|parent| parent.get_function(name, path)),
            }
        }
    }
    pub fn delete_var(&mut self, name: &str) -> Option<NewIdentifierType> {
        self.vars.remove(name)
    }
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
            .map_or_else(|| error(0, "no parent scope"), |scope| *scope);
        *self = p_scope;
    }

    pub fn from_parent(&mut self) {
        let mut temp_scope = Self::new();
        swap(&mut temp_scope, self);
        *self = Self::new_with_parent(Box::new(temp_scope));
    }
    pub fn has_function(&self, name: char) -> bool {
        self.function.contains_key(&name)
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

pub enum Stopper {
    Break,
    Continue,
    Return(LiteralOrFile),
}

pub struct Eval {
    pub scope: Scope,
    pub in_function: bool,
    pub in_loop: bool,
    pub files: HashMap<String, Rc<RefCell<File>>>,
}

impl Eval {
    pub fn new() -> Self {
        Self {
            scope: Scope::new(),
            in_function: false,
            in_loop: false,
            files: HashMap::new(),
        }
    }

    pub fn eval_scope(&mut self, body: Vec<Ast>) -> Option<Stopper> {
        let body = self.find_functions(body);
        self.eval_expression(body)
    }

    pub fn get_file(&self, name: &str) -> Option<RefMut<'_, File>> {
        self.files.get(name).map(|file| file.borrow_mut())
    }

    pub fn find_functions(&mut self, body: Vec<Ast>) -> Vec<Ast> {
        let body = body
            .into_iter()
            .filter(|thing| -> bool {
                if let Ast::Function(Function { node: function, .. }) = thing {
                    self.scope.set_function(
                        function.name.clone(),
                        function.body.node.0.clone(),
                        function.num_arguments,
                        function.extra_arguments,
                    );
                    false
                } else {
                    true
                }
            })
            .collect();
        self.find_imports(body)
    }

    // fn eval_module(&mut self) {
    //     if args.len() != 2 {
    //         error::error(line, format!("Expected 2 arguments for {self:?} operator"));
    //     }
    //     let module_name = match &args[0] {
    //         LiteralNode::String(string) if string.is_ascii() && string.len() == 1 => string,
    //         _ => error::error(line, format!("Expected string for {self:?} operator")),
    //     };
    //     match &args[1] {
    //         LiteralNode::String(filename) => {
    //             let prev_module_name = scope.module_name.clone();
    //             if scope.module_name.is_empty() {
    //                 scope.module_name = module_name.clone();
    //             } else {
    //                 scope.module_name = scope.module_name.clone() + "$" + module_name;
    //             }
    //             let _module_name = scope.module_name.clone();
    //             let file = File::open(filename);
    //             if let Ok(mut file) = file {
    //                 let mut buf = String::new();
    //                 if let Err(err) = file.read_to_string(&mut buf) {
    //                     error::error(line, format!("Failed to read file: {err}"));
    //                 }
    //                 let lexer = Lexer::new(buf, filename.clone());
    //                 let lexed = lexer.scan_tokens();
    //                 let mut parsed = Parser::new(lexed, filename.clone());
    //                 let body = parsed.parse();
    //                 scope.find_functions(body);
    //                 scope.module_name = prev_module_name;
    //             } else {
    //                 error::error(line, format!("Could not open file {filename:?}"));
    //             };
    //         }
    //         _ => error::error(line, format!("Expected string for {self:?} operator")),
    //     };
    //
    //     // see if there is the second argument is a string
    //     LiteralNode::String(module_name.to_string())
    // }

    #[allow(clippy::too_many_lines)]
    pub fn eval_expression(&mut self, body: Vec<Ast>) -> Option<Stopper> {
        // create a vector to return instead of inplace modification
        // well have globa/local scope when we check for variables we check for variables in the current scope and then check the parent scope and so on until we find a variable or we reach the top of the scope stack (same for functions)
        // we can have two different variables with the same name in different scopes, the scope of a variable is determined by where it is declared in the code
        debug!("find variables in scope");
        // print variables in scope
        for (name, var) in &self.scope.vars {
            debug!("{}: {:?}", name, var);
        }
        for thing in body {
            match thing {
                Ast::Declaration(Declaration {
                    node: ref variable,
                    start_line: line,
                    ..
                }) => match variable.value {
                    DeclarationType::Variable(ref name) => {
                        if let Some(pointer) = self.find_pointer_in_other_stuff(&name) {
                            self.scope
                                .set_var(&variable.name, &mut vec![pointer], false, line);
                        } else {
                            self.scope.set_var(
                                &variable.name,
                                &mut vec![LiteralOrFile::Literal(LiteralNode::get_from_ast(
                                    &name, line,
                                ))],
                                false,
                                line,
                            );
                        }
                    }
                    DeclarationType::Cons(ref list) => {
                        let car: LiteralOrFile =
                            self.find_pointer_in_other_stuff(&list.car).map_or_else(
                                || {
                                    LiteralOrFile::Literal(LiteralNode::get_from_ast(
                                        &list.car, line,
                                    ))
                                },
                                |pointer| pointer,
                            );
                        let cdr: LiteralOrFile =
                            self.find_pointer_in_other_stuff(&list.cdr).map_or_else(
                                || {
                                    LiteralOrFile::Literal(LiteralNode::get_from_ast(
                                        &list.cdr, line,
                                    ))
                                },
                                |pointer| pointer,
                            );
                        self.scope
                            .set_var(&variable.name, &mut vec![car, cdr], false, line);
                    }
                },
                Ast::Return(Return {
                    node,
                    start_line: line,
                    ..
                }) => {
                    let ret: LiteralOrFile =
                        node.0
                            .map_or(LiteralOrFile::Literal(LiteralNode::Hempty), |os| {
                                self.find_pointer_in_other_stuff(&os).map_or_else(
                                    || LiteralOrFile::Literal(LiteralNode::get_from_ast(&os, line)),
                                    |identifier| identifier,
                                )
                            });
                    return Some(Stopper::Return(ret));
                }
                // TODO: print exrpessions
                // Ast::Expression(expr) => {
                //     let exprs = self.find_pointer_in_stuff(&expr.inside);
                //     print!(
                //         "{}",
                //         NewExpression {
                //             inside: exprs,
                //             print: expr.print,
                //             line: expr.start_line,
                //             new_line: expr.new_line,
                //         }
                //     );
                // }
                Ast::If(If {
                    node: mut if_statement,
                    start_line,
                    ..
                }) => {
                    let conditon: LiteralNode =
                        match self.find_pointer_in_other_stuff(&if_statement.condition) {
                            Some(pointer) => {
                                info!("if {:?}", pointer);
                                match pointer {
                                    LiteralOrFile::Literal(literal) => literal,
                                    _ => error(start_line, "cannot compare files"),
                                }
                            }
                            None => LiteralNode::get_from_ast(&if_statement.condition, start_line),
                        };
                    if conditon.type_eq(&LiteralNode::Boolean(true)) {
                    } else {
                        error(start_line, "expected boolean, got something else");
                    }
                    self.scope.from_parent();
                    if conditon == LiteralNode::Boolean(true) {
                        if_statement.body_true.node.0 =
                            self.find_functions(if_statement.body_true.node.0);
                        let body_true: Option<Stopper> =
                            self.eval_expression(if_statement.body_true.node.0);
                        self.scope.drop_scope();
                        if let Some(stop) = body_true {
                            match stop {
                                Stopper::Break | Stopper::Continue => {
                                    if self.in_loop {
                                        return Some(stop);
                                    }
                                    error(start_line, "break or continue outside of loop");
                                }
                                Stopper::Return(ret) => {
                                    if self.in_function {
                                        return Some(Stopper::Return(ret));
                                    }
                                    error(start_line, "return outside of function");
                                }
                            }
                        }
                    } else {
                        if_statement.body_false.node.0 =
                            self.find_functions(if_statement.body_false.node.0);
                        let z = self.eval_expression(if_statement.body_false.node.0);
                        self.scope.drop_scope();
                        if let Some(stop) = z {
                            if let Stopper::Return(ret) = stop {
                                if self.in_function {
                                    return Some(Stopper::Return(ret));
                                }
                                error(start_line, "return outside of function");
                            } else {
                                if self.in_loop {
                                    return Some(stop);
                                }
                                error(start_line, "break or continue outside of loop");
                            }
                        }
                    }
                }
                Ast::Loop(loop_statement) => {
                    'l: loop {
                        self.scope.from_parent();
                        let loop_body = self.find_functions(loop_statement.node.0.node.0.clone());
                        self.in_loop = true;
                        let z: Option<Stopper> = self.eval_expression(loop_body.clone());
                        self.scope.drop_scope();
                        if let Some(stop) = z {
                            match stop {
                                Stopper::Break => break 'l,
                                Stopper::Continue => continue 'l,
                                Stopper::Return(ret) => {
                                    if self.in_function {
                                        return Some(Stopper::Return(ret));
                                    }
                                    error(loop_statement.start_line, "return outside of function");
                                }
                            }
                        }
                    }
                    self.in_loop = false;
                }
                Ast::Break(..) => {
                    return Some(Stopper::Break);
                }
                Ast::Continue(..) => {
                    return Some(Stopper::Continue);
                }
                _ => {}
            }
        }
        None
    }

    pub fn find_imports(&mut self, body: Vec<Ast>) -> Vec<Ast> {
        body.into_iter()
            .filter(|thing| match thing {
                // Ast::Expression(e) => match &e.inside {
                Ast::Call(call) => match call.node.keyword {
                    TokenType::Module => {
                        let mut new_stuff: Vec<LiteralNode> = Vec::new();
                        call.node.arguments.iter().for_each(|thing| {
                            match self.find_pointer_in_stuff(thing) {
                                LiteralOrFile::Literal(literal) => {
                                    new_stuff.push(literal);
                                }
                                LiteralOrFile::File(_) => {
                                    error(
                                        call.start_line,
                                        format!(
                                            "Cannot use file as argument for function {}",
                                            call.node.keyword
                                        )
                                        .as_str(),
                                    );
                                }
                            }
                        });
                        TokenType::Module.r#do(&new_stuff, call.start_line);
                        false
                    }
                    _ => true,
                },
                _ => true,
                // },
                // _ => true,
            })
            .collect()
    }

    fn find_pointer_in_other_stuff(&mut self, other_stuff: &Ast) -> Option<LiteralOrFile> {
        match other_stuff {
            Ast::Identifier(ident) => match self.scope.get_var(&ident.node.0, ident.start_line) {
                NewIdentifierType::List(..) => {
                    error(ident.start_line, "whole list not supported in call")
                }
                NewIdentifierType::Vairable(var) => match var.value {
                    LiteralOrFile::Literal(_) => Some(var.value),
                    _ => error(ident.start_line, "variable is not a literal"),
                },
            },
            expr => Some(self.find_pointer_in_stuff(&expr)),
            // _ => None,
        }
    }
    #[allow(clippy::too_many_lines)]
    fn find_pointer_in_stuff(&mut self, stuff: &Ast) -> LiteralOrFile {
        // need to make ways to extract values from literaltypes/literal/vars easy with function
        match stuff {
            Ast::Identifier(ident) => match self.scope.get_var(&ident.node.0, ident.start_line) {
                NewIdentifierType::List(..) => {
                    error(ident.start_line, "whole list not supported in call")
                }
                NewIdentifierType::Vairable(var) => var.value,
            },
            Ast::Call(Call {
                node: call,
                start_line,
                ..
            }) => {
                let start_line = *start_line;
                match &call.keyword {
                    TokenType::FunctionIdentifier { name, path } => {
                        if let Some(mut function) = self.scope.get_function(*name, path) {
                            let new_stuff: Vec<LiteralOrFile> = call
                                .arguments
                                .iter()
                                .map(|thing| self.find_pointer_in_stuff(thing))
                                .collect();
                            arg_error(
                                function.1 as u32,
                                new_stuff.len() as u32,
                                &call.keyword,
                                function.2,
                                start_line,
                            );
                            self.scope.from_parent();
                            function.0 = self.find_functions(function.0);
                            self.in_function = true;
                            let mut extra_args: Option<NewList> = None;

                            // TODO: once we have more than ammount of arguments specified in function we should label the rest as under one variable $n which is a list
                            new_stuff.into_iter().enumerate().for_each(|(i, l)| {
                                if i as f64 >= function.1 {
                                    if let Some(ref mut list) = extra_args {
                                        list.set_last(LitOrList::Identifier(Rc::new(
                                            RefCell::new(NewList {
                                                car: LitOrList::Literal(l),
                                                cdr: LitOrList::Literal(LiteralOrFile::Literal(
                                                    LiteralNode::Hempty,
                                                )),
                                            }),
                                        )));
                                    } else {
                                        extra_args = Some(NewList {
                                            car: LitOrList::Literal(l),
                                            cdr: LitOrList::Literal(LiteralOrFile::Literal(
                                                LiteralNode::Hempty,
                                            )),
                                        });
                                    }
                                } else {
                                    self.scope.set_var(
                                        format!("${}", i + 1).as_str(),
                                        &mut vec![(l)],
                                        false,
                                        start_line,
                                    );
                                }
                            });
                            if let Some(list) = extra_args {
                                self.scope.set_list(
                                    format!("${}", function.1 as usize + 1).as_str(),
                                    list,
                                    false,
                                    start_line,
                                );
                            }
                            let z: Option<Stopper> = self.eval_expression(function.0);
                            self.in_function = false;
                            self.scope.drop_scope();
                            z.map_or(LiteralOrFile::Literal(LiteralNode::Hempty), |v| {
                                if let Stopper::Return(a) = v {
                                    a
                                } else {
                                    error(
                                        start_line,
                                        "cannot call break/continue at end of function",
                                    );
                                }
                            })
                        } else {
                            error(start_line, format!("Function {name} is not defined"));
                        }
                    }
                    TokenType::Type => {
                        arg_error(
                            1,
                            call.arguments.len() as u32,
                            &call.keyword,
                            false,
                            start_line,
                        );
                        match self.find_pointer_in_stuff(&call.arguments[0]) {
                            LiteralOrFile::Literal(a) => {
                                LiteralOrFile::Literal(LiteralNode::String(a.get_type()))
                            }
                            LiteralOrFile::File(_) => {
                                LiteralOrFile::Literal(LiteralNode::String("file".to_string()))
                            }
                        }
                    }
                    TokenType::Delete => {
                        if call.arguments.len() != 1 {
                            error(start_line, "delete takes one argument");
                        }
                        if let Ast::Identifier(Identifier {
                            node: ident,
                            start_line,
                            ..
                        }) = &call.arguments[0]
                        {
                            if self.scope.delete_var(&ident.0).is_some() {
                                LiteralOrFile::Literal(LiteralNode::Hempty)
                            } else {
                                error(*start_line, format!("Variable {} is not defined", ident.0));
                            }
                        } else {
                            error(start_line, "delete only takes a variable name")
                        }
                    }
                    TokenType::AddWith
                    | TokenType::SubtractWith
                    | TokenType::DivideWith
                    | TokenType::MultiplyWith
                    | TokenType::Set => {
                        debug!("{} {:?}", call.keyword, call.arguments);
                        if let Ast::Identifier(Identifier {
                            node: ident,
                            start_line: ident_line,
                            ..
                        }) = &call.arguments[0]
                        {
                            if self.scope.has_var(&ident.0, true) {
                                let mut new_stuff: Vec<LiteralOrFile> = Vec::new();
                                call.arguments.iter().skip(1).for_each(|thing| {
                                    new_stuff.push(self.find_pointer_in_stuff(thing));
                                });
                                match new_stuff.len() {
                                    1 => {
                                        let literal: LiteralOrFile = new_stuff.remove(0);
                                        let var: LiteralOrFile =
                                            match self.scope.get_var(&ident.0, start_line) {
                                                NewIdentifierType::Vairable(v) => v.value,
                                                NewIdentifierType::List(..) => {
                                                    error(*ident_line, "Cannot change entire list");
                                                }
                                            };
                                        match call.keyword {
                                            TokenType::Set => {
                                                self.scope.set_var(
                                                    &ident.0,
                                                    &mut vec![literal.clone()],
                                                    true,
                                                    start_line,
                                                );
                                                literal
                                            }
                                            TokenType::AddWith => match var {
                                                LiteralOrFile::Literal(LiteralNode::Number(
                                                    num,
                                                )) => {
                                                    if let LiteralOrFile::Literal(
                                                        LiteralNode::Number(num2),
                                                    ) = literal
                                                    {
                                                        let new_val = num + num2;
                                                        self.scope.set_var(
                                                            &ident.0,
                                                            &mut vec![LiteralOrFile::Literal(
                                                                LiteralNode::Number(new_val),
                                                            )],
                                                            true,
                                                            start_line,
                                                        );
                                                        LiteralOrFile::Literal(LiteralNode::Number(
                                                            new_val,
                                                        ))
                                                    } else {
                                                        error(
                                                            start_line,
                                                            format!(
                                                                "Variable {} is not a number",
                                                                ident.0
                                                            ),
                                                        );
                                                    }
                                                }
                                                LiteralOrFile::Literal(LiteralNode::String(
                                                    mut s,
                                                )) => match literal {
                                                    LiteralOrFile::Literal(
                                                        LiteralNode::String(s2),
                                                    ) => {
                                                        s.push_str(s2.as_str());
                                                        self.scope.set_var(
                                                            &ident.0,
                                                            &mut vec![LiteralOrFile::Literal(
                                                                LiteralNode::String(s.clone()),
                                                            )],
                                                            true,
                                                            start_line,
                                                        );
                                                        LiteralOrFile::Literal(LiteralNode::String(
                                                            s,
                                                        ))
                                                    }
                                                    LiteralOrFile::Literal(
                                                        LiteralNode::Number(n),
                                                    ) => {
                                                        s.push_str(&n.to_string());
                                                        self.scope.set_var(
                                                            &ident.0,
                                                            &mut vec![LiteralOrFile::Literal(
                                                                LiteralNode::String(s.clone()),
                                                            )],
                                                            true,
                                                            start_line,
                                                        );
                                                        LiteralOrFile::Literal(LiteralNode::String(
                                                            s,
                                                        ))
                                                    }
                                                    LiteralOrFile::Literal(
                                                        LiteralNode::Boolean(boolean),
                                                    ) => {
                                                        s.push_str(&boolean.to_string());
                                                        self.scope.set_var(
                                                            &ident.0,
                                                            &mut vec![LiteralOrFile::Literal(
                                                                LiteralNode::String(s.clone()),
                                                            )],
                                                            true,
                                                            start_line,
                                                        );
                                                        LiteralOrFile::Literal(LiteralNode::String(
                                                            s,
                                                        ))
                                                    }
                                                    LiteralOrFile::Literal(LiteralNode::Hempty) => {
                                                        s.push_str("hempty");
                                                        self.scope.set_var(
                                                            &ident.0,
                                                            &mut vec![LiteralOrFile::Literal(
                                                                LiteralNode::String(s.clone()),
                                                            )],
                                                            true,
                                                            start_line,
                                                        );
                                                        LiteralOrFile::Literal(LiteralNode::String(
                                                            s,
                                                        ))
                                                    }
                                                    _ => {
                                                        error(
                                                            start_line,
                                                            format!(
                                                                "Variable {} is not a string",
                                                                ident.0
                                                            ),
                                                        );
                                                    }
                                                },
                                                _ => {
                                                    error(
                                                        start_line,
                                                        format!(
                                                            "Variable {} is not a number/string",
                                                            ident.0
                                                        ),
                                                    );
                                                }
                                            },
                                            TokenType::MultiplyWith => match var {
                                                LiteralOrFile::Literal(LiteralNode::Number(
                                                    num,
                                                )) => {
                                                    if let LiteralOrFile::Literal(
                                                        LiteralNode::Number(num2),
                                                    ) = literal
                                                    {
                                                        let new_val: f64 = num * num2;
                                                        self.scope.set_var(
                                                            &ident.0,
                                                            &mut vec![LiteralOrFile::Literal(
                                                                LiteralNode::Number(new_val),
                                                            )],
                                                            true,
                                                            start_line,
                                                        );
                                                        LiteralOrFile::Literal(LiteralNode::Number(
                                                            new_val,
                                                        ))
                                                    } else {
                                                        error(
                                                            start_line,
                                                            format!(
                                                                "Variable {} is not a number",
                                                                ident.0
                                                            ),
                                                        );
                                                    }
                                                }
                                                LiteralOrFile::Literal(LiteralNode::String(
                                                    ref s,
                                                )) => {
                                                    if let LiteralOrFile::Literal(
                                                        LiteralNode::Number(num),
                                                    ) = literal
                                                    {
                                                        let new_string: String = (0..num as i32)
                                                            .map(|_| s.to_string())
                                                            .collect();
                                                        self.scope.set_var(
                                                            &ident.0,
                                                            &mut vec![LiteralOrFile::Literal(
                                                                LiteralNode::String(
                                                                    new_string.clone(),
                                                                ),
                                                            )],
                                                            true,
                                                            start_line,
                                                        );
                                                        LiteralOrFile::Literal(LiteralNode::String(
                                                            new_string,
                                                        ))
                                                    } else {
                                                        error(
                                                            start_line,
                                                            format!(
                                                                "Variable {} is not a number",
                                                                ident.0
                                                            ),
                                                        );
                                                    }
                                                }
                                                _ => {
                                                    error(
                                                        start_line,
                                                        format!(
                                                            "Variable {} is not a number/string",
                                                            ident.0
                                                        ),
                                                    );
                                                }
                                            },
                                            TokenType::SubtractWith | TokenType::DivideWith => {
                                                if let LiteralOrFile::Literal(
                                                    LiteralNode::Number(nums),
                                                ) = var
                                                {
                                                    if let LiteralOrFile::Literal(
                                                        LiteralNode::Number(num),
                                                    ) = literal
                                                    {
                                                        if call.keyword == TokenType::SubtractWith {
                                                            let new_val = nums - num;
                                                            self.scope.set_var(
                                                                &ident.0,
                                                                &mut vec![LiteralOrFile::Literal(
                                                                    LiteralNode::Number(new_val),
                                                                )],
                                                                true,
                                                                start_line,
                                                            );
                                                            LiteralOrFile::Literal(
                                                                LiteralNode::Number(new_val),
                                                            )
                                                        } else {
                                                            let new_val = nums / num;
                                                            self.scope.set_var(
                                                                &ident.0,
                                                                &mut vec![LiteralOrFile::Literal(
                                                                    LiteralNode::Number(new_val),
                                                                )],
                                                                true,
                                                                start_line,
                                                            );
                                                            LiteralOrFile::Literal(
                                                                LiteralNode::Number(new_val),
                                                            )
                                                        }
                                                    } else {
                                                        error(
                                                            start_line,
                                                            format!(
                                                                "Variable {} is not a number",
                                                                ident.0
                                                            ),
                                                        );
                                                    }
                                                } else {
                                                    error(
                                                        start_line,
                                                        format!(
                                                            "Variable {} is not a number/string",
                                                            ident.0
                                                        ),
                                                    );
                                                }
                                            }
                                            _ => {
                                                error(
                                                    start_line,
                                                    format!(
                                                        "Invalid operator for literal {}",
                                                        call.keyword
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                    2 => {
                                        if call.keyword == TokenType::Set {
                                            self.scope.set_var(
                                                &ident.0,
                                                &mut new_stuff,
                                                true,
                                                start_line,
                                            );
                                            LiteralOrFile::Literal(LiteralNode::String(
                                                new_stuff
                                                    .iter()
                                                    .map(std::string::ToString::to_string)
                                                    .collect(),
                                            ))
                                        } else {
                                            error(
                                                start_line,
                                                format!(
                                                    "Too many arguments for function {}",
                                                    call.keyword
                                                )
                                                .as_str(),
                                            );
                                        }
                                    }
                                    _ => error(
                                        start_line,
                                        format!("Too many arguments for function {}", call.keyword)
                                            .as_str(),
                                    ),
                                }
                            } else {
                                error(*ident_line, format!("Variable {} is not defined", ident.0));
                            }
                        } else {
                            error(
                                start_line,
                                format!("First argument of {} must be an identifier", call.keyword)
                                    .as_str(),
                            );
                        }
                    }
                    TokenType::Open => {
                        arg_error(
                            1,
                            call.arguments.len() as u32,
                            &call.keyword,
                            false,
                            start_line,
                        );
                        // check if the first argument is a string
                        let arg = self
                            .find_pointer_in_stuff(&call.arguments[0])
                            .get_string(start_line, &call.keyword);
                        if std::path::Path::new(&arg).exists() {
                            LiteralOrFile::File(arg)
                        } else {
                            error(
                                start_line,
                                format!("Could not open file {arg}: does not exist").as_str(),
                            );
                        }
                    }
                    TokenType::Close | TokenType::Read => {
                        arg_error(
                            1,
                            call.arguments.len() as u32,
                            &call.keyword,
                            false,
                            start_line,
                        );
                        // evalute args[0] and check if it is a file
                        match &call.arguments[0] {
                            Ast::Identifier(Identifier { node: ident, .. }) => {
                                let files = self.scope.get_var(&ident.0, start_line);
                                match files {
                                    NewIdentifierType::Vairable(var) => {
                                        match var.value {
                                            LiteralOrFile::File(file) => {
                                                // set idnetifier to nothing
                                                match call.keyword {
                                                    TokenType::Close => {
                                                        self.scope.set_var(
                                                            &ident.0,
                                                            &mut vec![LiteralOrFile::Literal(
                                                                LiteralNode::Hempty,
                                                            )],
                                                            true,
                                                            start_line,
                                                        );
                                                    }
                                                    TokenType::Read => {
                                                        let contents = // create a string to hold the contents of the file
                                                                match read_file(&file) {
                                                                    Ok(contents) => contents,
                                                                    Err(err) => {
                                                                        error(start_line, format!("{err}"));
                                                                    }
                                                                }; // read the file into the string
                                                        return LiteralOrFile::Literal(
                                                            LiteralNode::String(contents),
                                                        ); // return the string
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            _ => error(
                                                start_line,
                                                format!("{} is not a file", ident.0).as_str(),
                                            ),
                                        }
                                    }
                                    NewIdentifierType::List(_) => error(
                                        start_line,
                                        format!("Variable {} is not a file", ident.0).as_str(),
                                    ),
                                }
                            }
                            other => {
                                match self.find_pointer_in_stuff(other) {
                                    LiteralOrFile::File(file) => {
                                        match call.keyword {
                                            TokenType::Close => {
                                                self.scope.files.remove(&file);
                                            }
                                            TokenType::Read => {
                                                let contents: String = String::new(); // create a string to hold the contents of the file
                                                match read_file(&file) {
                                                    Ok(contents) => contents,
                                                    Err(err) => {
                                                        error(start_line, format!("{err}"));
                                                    }
                                                }; // read the file into the string
                                                return LiteralOrFile::Literal(
                                                    LiteralNode::String(contents),
                                                ); // return the string
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {
                                        error(
                                            start_line,
                                            format!(
                                                "First argument of {} must be a file",
                                                call.keyword
                                            )
                                            .as_str(),
                                        );
                                    }
                                };
                            }
                        }
                        LiteralOrFile::Literal(LiteralNode::Hempty)
                    }
                    TokenType::Write => {
                        //takes 3 arguments:
                        // 1. the file
                        // 2. the string to write
                        // 3. the mode (a: append or w: overwrite)
                        arg_error(
                            3,
                            call.arguments.len() as u32,
                            &call.keyword,
                            false,
                            start_line,
                        );
                        // get the file
                        let file = self
                            .find_pointer_in_stuff(&call.arguments[0])
                            .get_file(start_line, &call.keyword);
                        // get the string
                        let string = self
                            .find_pointer_in_stuff(&call.arguments[1])
                            .get_string(start_line, &call.keyword);
                        // get the mode
                        let mode = self
                            .find_pointer_in_stuff(&call.arguments[2])
                            .get_string(start_line, &call.keyword);
                        // write the string to the file
                        match write_file(&file, &string, &mode) {
                            Ok(_) => LiteralOrFile::Literal(LiteralNode::Hempty),
                            Err(err) => {
                                error(start_line, format!("{err}").as_str());
                            }
                        }
                    }
                    TokenType::ReadLine => {
                        // takes 2 arguments: file, line
                        arg_error(
                            2,
                            call.arguments.len() as u32,
                            &call.keyword,
                            false,
                            start_line,
                        );
                        // get the file
                        let file = self
                            .find_pointer_in_stuff(&call.arguments[0])
                            .get_file(start_line, &call.keyword);
                        // get the line
                        let line = self
                            .find_pointer_in_stuff(&call.arguments[1])
                            .get_number(start_line, &call.keyword);
                        // read the the file
                        match read_file(&file) {
                            Ok(contents) => {
                                let lines = contents.split('\n').collect::<Vec<&str>>();
                                if line as usize > lines.len() {
                                    error(
                                        start_line,
                                        format!("Line {line} does not exist in file {file}"),
                                    );
                                }
                                LiteralOrFile::Literal(LiteralNode::String(
                                    lines[line as usize - 1].to_string(),
                                ))
                            }
                            Err(err) => {
                                error(start_line, format!("{err}").as_str());
                            }
                        }
                    }
                    TokenType::WriteLine => {
                        // takes 4 arguments: file, line, string and mode (a: append to beging of line or w: overwrite)
                        arg_error(
                            4,
                            call.arguments.len() as u32,
                            &call.keyword,
                            false,
                            start_line,
                        );
                        // get the file
                        let file = self
                            .find_pointer_in_stuff(&call.arguments[0])
                            .get_file(start_line, &call.keyword);
                        // get the string
                        let mut string = self
                            .find_pointer_in_stuff(&call.arguments[1])
                            .get_string(start_line, &call.keyword);
                        // get the line
                        let line = self
                            .find_pointer_in_stuff(&call.arguments[2])
                            .get_number(start_line, &call.keyword);
                        // get the mode
                        let mode = self
                            .find_pointer_in_stuff(&call.arguments[3])
                            .get_string(start_line, &call.keyword);
                        // read the file
                        let mut contents = match read_file(&file) {
                            Ok(contents) => contents,
                            Err(err) => {
                                error(start_line, format!("{err}").as_str());
                            }
                        };
                        // split the contents into lines
                        let mut lines = contents.split('\n').collect::<Vec<&str>>();
                        // if the line is greater than the number of lines, add a new line
                        if line as usize > lines.len() {
                            error(start_line, "Line does not exist in file");
                        }
                        string = match mode.as_str() {
                            "a" => {
                                format!("{string}{}", lines[line as usize - 1],)
                            }
                            "w" => string,
                            _ => {
                                error(
                                    start_line,
                                    format!("Mode {mode} is not a valid mode").as_str(),
                                );
                            }
                        };
                        lines[line as usize - 1] = string.as_str();
                        // collect all lines
                        contents = lines.join("\n");
                        // write the file
                        match write_file(&file, &contents, "w") {
                            Ok(_) => LiteralOrFile::Literal(LiteralNode::Hempty),
                            Err(err) => {
                                error(start_line, format!("{err}").as_str());
                            }
                        }
                    }
                    TokenType::DeleteFile | TokenType::CreateFile => {
                        // takes 1 argument: file
                        arg_error(
                            1,
                            call.arguments.len() as u32,
                            &call.keyword,
                            false,
                            start_line,
                        );
                        // get the file
                        let file = self
                            .find_pointer_in_stuff(&call.arguments[0])
                            .get_file(start_line, &call.keyword);
                        // match delete or create file
                        match call.keyword {
                            TokenType::DeleteFile => match fs::remove_file(&file) {
                                Ok(_) => LiteralOrFile::Literal(LiteralNode::Hempty),
                                Err(err) => {
                                    error(start_line, format!("{err}").as_str());
                                }
                            },
                            TokenType::CreateFile => {
                                // create the file
                                match OpenOptions::new().create(true).open(&file) {
                                    Ok(_) => LiteralOrFile::File(file),
                                    Err(err) => {
                                        error(start_line, format!("{err}").as_str());
                                    }
                                }
                            }
                            _ => LiteralOrFile::Literal(LiteralNode::Hempty),
                        }
                    }
                    t => {
                        let mut new_stuff: Vec<LiteralNode> = Vec::new();
                        call.arguments.iter().for_each(|thing| {
                            match self.find_pointer_in_stuff(thing) {
                                LiteralOrFile::Literal(literal) => {
                                    new_stuff.push(literal);
                                }
                                LiteralOrFile::File(_) => {
                                    error(
                                        start_line,
                                        format!(
                                            "Cannot use file as argument for function {}",
                                            call.keyword
                                        )
                                        .as_str(),
                                    );
                                }
                            }
                        });
                        LiteralOrFile::Literal(t.r#do(&new_stuff, start_line))
                    }
                }
            }
            Ast::Literal(lit) => LiteralOrFile::Literal(lit.node.clone()),
            Ast::Block(block) => {
                let block = block.clone();
                self.eval_scope(block.node.0);
                LiteralOrFile::Literal(LiteralNode::Hempty)
            }
            Ast::Function(_) => todo!(),
            Ast::If(_) => todo!(),
            Ast::Loop(_) => todo!(),
            Ast::Break(_) => todo!(),
            Ast::Continue(_) => todo!(),
            Ast::Return(_) => todo!(),
            Ast::Declaration(_) => todo!(),
        }
    }
}

impl fmt::Debug for Eval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "scope {:?}", self.scope)?;
        Ok(())
    }
}
