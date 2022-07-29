#![allow(unused_variables, unreachable_patterns)]
use log::info;

use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use crate::{
    error,
    parser::{
        rules::{IdentifierType, LiteralType, OtherStuff, Stuff},
        Thing,
    },
    token::TokenType,
};
#[derive(PartialEq, Debug, Clone)]
pub struct NewExpression {
    pub inside: LiteralType,
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
                LiteralType::String(String::from("")).to_string()
            }
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum LitOrList {
    Identifier(Box<NewList>),
    Literal(LiteralType),
}

impl Display for LitOrList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LitOrList::Identifier(i) => write!(f, "{}", i),
            LitOrList::Literal(l) => write!(f, "{}", l),
        }
    }
}
#[derive(PartialEq, Clone, Debug)]
pub struct NewList {
    pub car: LitOrList,
    pub cdr: LitOrList,
}

impl NewList {
    pub fn new(thing: &[LiteralType]) -> Self {
        Self {
            car: LitOrList::Literal(thing[0].clone()),
            cdr: LitOrList::Literal(thing[1].clone()),
        }
    }
}

impl Display for NewList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "with: [{}, {}]", self.car, self.cdr)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct NewVairable {
    pub value: LiteralType,
}

impl NewVairable {
    pub const fn new(value: LiteralType) -> Self {
        Self { value }
    }

    pub const fn new_empty(line: i32) -> Self {
        Self {
            value: LiteralType::new_hempty(),
        }
    }
}

impl Display for NewVairable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "with: {}", self.value)
    }
}
#[derive(PartialEq, Clone, Debug)]
pub enum NewIdentifierType {
    List(Box<NewList>),
    Vairable(Box<NewVairable>),
}

impl NewIdentifierType {
    pub fn new(thing: &[LiteralType]) -> Self {
        match thing.len() {
            0 => error::error(0, "expected Identifier, got empty list"),
            1 => Self::Vairable(Box::new(NewVairable::new(thing[0].clone()))),
            2 => Self::List(Box::new(NewList::new(thing))),
            _ => error::error(0, "expected Identifier, got list with more than 2 elements"),
        }
    }
}
#[derive(PartialEq, Clone, Debug)]
pub struct Scope {
    pub vars: HashMap<String, NewIdentifierType>,
    pub function: HashMap<char, (Vec<Thing>, f64)>,
    pub parent_scope: Option<Box<Scope>>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            function: HashMap::new(),
            parent_scope: None,
        }
    }
    pub fn new_with_parent(parent: Box<Self>) -> Self {
        Self {
            vars: HashMap::new(),
            function: HashMap::new(),
            parent_scope: Some(parent),
        }
    }
    pub fn set_var(&mut self, name: &str, value: &[LiteralType], recurse: bool) {
        // the reason for this being its own method vs using the set method is because it will be easier to use/implemnet getting variable from different scopes
        // and also less typing instead of creating a NewIdentifierType you just pass in a vector of LiteralType
        let new_val = match value.len() {
            0 => error::error(0, "expected Identifier, got empty list"),
            1 => NewIdentifierType::Vairable(Box::new(NewVairable::new(value[0].clone()))),
            2 => NewIdentifierType::List(Box::new(NewList::new(value))),
            _ => error::error(0, "expected Identifier, got list with more than 2 elements"),
        };
        match name {
            name if name.ends_with(".car") | name.ends_with(".cdr") => {
                let new_name = name.trim_end_matches(".car").trim_end_matches(".cdr");
                if recurse {
                    if self.has_var(new_name, false) {
                        let mut new_var = match self.get_var(new_name) {
                            NewIdentifierType::List(list) => list,
                            _ => error::error(0, "expected list"),
                        };

                        if name.ends_with(".cdr") {
                            new_var.cdr = match new_val {
                                NewIdentifierType::List(list) => LitOrList::Identifier(list),
                                NewIdentifierType::Vairable(var) => {
                                    LitOrList::Literal(var.value.clone())
                                }
                            };
                        } else {
                            new_var.car = match new_val {
                                NewIdentifierType::List(list) => LitOrList::Identifier(list),
                                NewIdentifierType::Vairable(var) => {
                                    LitOrList::Literal(var.value.clone())
                                }
                            };
                        }
                        self.vars
                            .insert(new_name.to_string(), NewIdentifierType::List(new_var));
                    } else {
                        match self.parent_scope.as_mut() {
                            Some(parent) => parent.set_var(name, value, recurse),
                            None => error::error(1, "variable not found"),
                        }
                    }
                } else {
                    let mut new_var = match self.get_var(new_name) {
                        NewIdentifierType::List(list) => list,
                        _ => error::error(0, "expected list"),
                    };

                    if name.ends_with(".cdr") {
                        new_var.cdr = match new_val {
                            NewIdentifierType::List(list) => LitOrList::Identifier(list),
                            NewIdentifierType::Vairable(var) => {
                                LitOrList::Literal(var.value.clone())
                            }
                        };
                    } else {
                        new_var.car = match new_val {
                            NewIdentifierType::List(list) => LitOrList::Identifier(list),
                            NewIdentifierType::Vairable(var) => {
                                LitOrList::Literal(var.value.clone())
                            }
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
                        match self.parent_scope.as_mut() {
                            Some(parent) => parent.set_var(name, value, recurse),
                            None => error::error(1, "variable not found"),
                        }
                    }
                } else {
                    self.vars.insert(name.to_string(), new_val);
                }
            }
        }
    }
    pub fn get_var(&self, name: &str) -> NewIdentifierType {
        // the reason for this being its own method vs using the get method is because it will be easier to use/implemnet getting variable from different scopes

        if name.ends_with(".car") || name.ends_with(".cdr") {
            if let NewIdentifierType::List(list) = self.get_var(&name[..name.len() - 4]) {
                if name.ends_with(".car") {
                    match list.car {
                        LitOrList::Identifier(list2) => {
                            return NewIdentifierType::List(list2);
                        }
                        LitOrList::Literal(var) => {
                            return NewIdentifierType::Vairable(Box::new(NewVairable::new(var)));
                        }
                    }
                }
                match list.cdr {
                    LitOrList::Identifier(list2) => {
                        return NewIdentifierType::List(list2);
                    }
                    LitOrList::Literal(var) => {
                        return NewIdentifierType::Vairable(Box::new(NewVairable::new(var)));
                    }
                }
            }
            error::error(1, "expected list, got something else");
        }
        match self.vars.get(name) {
            Some(v) => v.clone(),
            None => match &self.parent_scope {
                Some(parent) => parent.get_var(name),
                None => error::error(1, format!("variable not found {}", name)),
            },
        }
    }
    pub fn set_function(&mut self, name: char, args: Vec<Thing>, body: f64) {
        self.function.insert(name, (args, body));
    }
    pub fn get_function(&self, name: char) -> Option<(Vec<Thing>, f64)> {
        match self.function.get(&name) {
            Some((args, body)) => Some(((*args).clone(), *body)),
            None => match &self.parent_scope {
                Some(parent) => parent.get_function(name),
                None => None,
            },
        }
    }
    pub fn delete_var(&mut self, name: &str) -> Option<NewIdentifierType> {
        self.vars.remove(name)
    }
    pub fn has_var(&self, name: &str, recurse: bool) -> bool {
        let name = if name.ends_with(".car") || name.ends_with(".cdr") {
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
            match &self.parent_scope {
                Some(parent) => parent.has_var(name, recurse),
                None => false,
            }
        }
    }
    pub fn drop_scope(&mut self) {
        self.function = self.parent_scope.as_ref().unwrap().function.clone();
        self.vars = self.parent_scope.as_ref().unwrap().vars.clone();
        self.parent_scope = self.parent_scope.as_ref().unwrap().parent_scope.clone();
    }
    pub fn has_function(&self, name: char) -> bool {
        self.function.contains_key(&name)
    }
}

pub enum Stopper {
    Break,
    Continue,
    Return(LiteralType),
}
#[derive(PartialEq, Clone)]
pub struct Eval {
    pub level: i32,
    pub scope: Scope,
    pub in_function: bool,
    pub in_loop: bool,
}

impl Eval {
    pub fn new(body: &[Thing]) -> Self {
        let mut self_ = Self {
            level: 0,
            scope: Scope::new(),
            in_function: false,
            in_loop: false,
        };
        self_.find_functions(body);
        self_.find_variables(body);
        self_
    }

    pub fn find_functions(&mut self, body: &[Thing]) {
        for thing in body {
            if let Thing::Function(function) = thing {
                self.scope.set_function(
                    function.name,
                    function.body.clone(),
                    function.num_arguments,
                );
            }
        }
    }
    #[allow(clippy::too_many_lines)]
    pub fn find_variables(&mut self, body: &[Thing]) -> Option<Stopper> {
        // create a vector to return instead of inplace modification
        // well have globa/local scope when we check for variables we check for variables in the current scope and then check the parent scope and so on until we find a variable or we reach the top of the scope stack (same for functions)
        // we can have two different variables with the same name in different scopes, the scope of a variable is determined by where it is declared in the code
        for thing in body {
            match thing.clone() {
                Thing::Identifier(ref variable) => match variable.value {
                    IdentifierType::Vairable(ref name) => {
                        match self.find_pointer_in_other_stuff(&name.value) {
                            Some(pointer) => {
                                self.scope.set_var(&variable.name, &[pointer], false);
                            }
                            None => {
                                self.scope.set_var(
                                    &variable.name,
                                    &[LiteralType::from_other_stuff(name.value.clone())],
                                    false,
                                );
                            }
                        }
                    }
                    IdentifierType::List(ref list) => {
                        let car = match self.find_pointer_in_other_stuff(&list.car) {
                            Some(pointer) => pointer,
                            None => LiteralType::from_other_stuff(list.car.clone()),
                        };

                        let cdr = match self.find_pointer_in_other_stuff(&list.cdr) {
                            Some(pointer) => pointer,
                            None => LiteralType::from_other_stuff(list.cdr.clone()),
                        };
                        self.scope.set_var(&variable.name, &[car, cdr], false);
                    }
                },
                Thing::Return(os, line) => {
                    let ret = match os {
                        Some(os) => match self.find_pointer_in_other_stuff(&os) {
                            Some(identifier) => identifier,
                            None => LiteralType::from_other_stuff(os),
                        },
                        None => LiteralType::Hempty,
                    };
                    return Some(Stopper::Return(ret));
                }
                Thing::Expression(expr) => match self.find_pointer_in_stuff(&expr.inside) {
                    Some(exprs) => {
                        print!(
                            "{}",
                            NewExpression {
                                inside: exprs,
                                print: expr.print,
                                line: expr.line,
                                new_line: expr.new_line,
                            }
                        );
                    }
                    None => {}
                },
                Thing::IfStatement(if_statement) => {
                    let conditon = match self.find_pointer_in_other_stuff(&if_statement.condition) {
                        Some(pointer) => {
                            info!("if {:?}", pointer);
                            pointer
                        }
                        None => LiteralType::from_other_stuff(if_statement.condition),
                    };
                    if conditon.type_eq(&LiteralType::Boolean(true)) {
                    } else {
                        error::error(if_statement.line, "expected boolean, got something else");
                    }
                    self.scope = Scope::new_with_parent(Box::new(self.scope.clone()));

                    if conditon == LiteralType::Boolean(true) {
                        self.find_functions(&if_statement.body_true);
                        let body_true = self.find_variables(&if_statement.body_true);

                        self.scope.drop_scope();

                        match body_true {
                            Some(stop) => match stop {
                                Stopper::Break | Stopper::Continue => {
                                    if self.in_loop {
                                        return Some(stop);
                                    }
                                    error::error(
                                        if_statement.line,
                                        "break or continue outside of loop",
                                    );
                                }
                                Stopper::Return(ret) => {
                                    if self.in_function {
                                        return Some(Stopper::Return(ret));
                                    }
                                    error::error(if_statement.line, "return outside of function");
                                }
                            },
                            None => {}
                        }
                    } else {
                        self.find_functions(&if_statement.body_false);
                        let z = self.find_variables(&if_statement.body_false);

                        self.scope.drop_scope();
                        match z {
                            Some(stop) => match stop {
                                Stopper::Break | Stopper::Continue => {
                                    if self.in_loop {
                                        return Some(stop);
                                    }
                                    error::error(
                                        if_statement.line,
                                        "break or continue outside of loop",
                                    );
                                }
                                Stopper::Return(ret) => {
                                    if self.in_function {
                                        return Some(Stopper::Return(ret));
                                    }
                                    error::error(if_statement.line, "return outside of function");
                                }
                            },
                            None => {}
                        }
                    }
                }
                Thing::LoopStatement(loop_statement) => {
                    loop {
                        self.scope = Scope::new_with_parent(Box::new(self.scope.clone()));

                        self.find_functions(&loop_statement.body);
                        self.in_loop = true;
                        // TODO: find out when break/continue is called
                        let z = self.find_variables(&loop_statement.body);

                        self.scope.drop_scope();

                        match z {
                            Some(stop) => match stop {
                                Stopper::Break => break,
                                Stopper::Continue => continue,
                                Stopper::Return(ret) => {
                                    if self.in_function {
                                        return Some(Stopper::Return(ret));
                                    }
                                    error::error(loop_statement.line, "return outside of function");
                                }
                            },
                            None => {}
                        }
                    }
                    self.in_loop = false;
                }
                Thing::Break(_) => {
                    return Some(Stopper::Break);
                }
                Thing::Continue(_) => {
                    return Some(Stopper::Continue);
                }
                _ => {}
            }
        }
        None
    }

    fn find_pointer_in_other_stuff(&mut self, other_stuff: &OtherStuff) -> Option<LiteralType> {
        match other_stuff {
            OtherStuff::Identifier(ident) => match self.scope.get_var(&ident.name) {
                NewIdentifierType::List(..) => {
                    error::error(ident.line, "whole list not supported in call")
                }
                NewIdentifierType::Vairable(var) => Some(var.value.clone()),
            },

            OtherStuff::Expression(expr) => match self.find_pointer_in_stuff(&expr.inside) {
                Some(new_expr) => Some(new_expr),
                None => Some(LiteralType::from_stuff(expr.inside.clone())),
            },
            _ => None,
        }
    }
    #[allow(clippy::too_many_lines)]
    fn find_pointer_in_stuff(&mut self, stuff: &Stuff) -> Option<LiteralType> {
        // need to make ways to extract values from literaltypes/literal/vars easy with function
        match stuff {
            Stuff::Identifier(ident) => match self.scope.get_var(&ident.name) {
                NewIdentifierType::List(..) => {
                    error::error(ident.line, "whole list not supported in call")
                }
                NewIdentifierType::Vairable(var) => Some(var.value.clone()),
            },

            Stuff::Call(call) => match &call.keyword {
                TokenType::FunctionIdentifier { name } => {
                    if let Some(function) = self.scope.get_function(*name) {
                        let mut new_stuff = Vec::new();
                        for (index, thing) in call.arguments.iter().enumerate() {
                            if index > function.1 as usize {
                                error::error(
                                    call.line,
                                    format!("Too many arguments for function {}", call.keyword)
                                        .as_str(),
                                );
                            }
                            match self.find_pointer_in_stuff(thing) {
                                Some(new_thing) => {
                                    new_stuff.push(new_thing.clone());
                                }
                                None => new_stuff.push(LiteralType::from_stuff(thing.clone())),
                            }
                        }
                        if new_stuff.len() != function.1 as usize {
                            error::error(
                                    call.line,
                                    format!("Too few or too many arguments for function {} expected: {}, found: {}", call.keyword, function.1, new_stuff.len()),
                                );
                        }
                        self.scope = Scope::new_with_parent(Box::new(self.scope.clone()));

                        self.find_functions(&function.0);
                        // TODO: find if function has return in it and act accordingly
                        self.in_function = true;

                        for (i, l) in new_stuff.iter().enumerate() {
                            self.scope
                                .set_var(format!("${}", i + 1).as_str(), &[l.clone()], false);
                        }

                        let z = self.find_variables(&function.0);
                        self.in_function = false;

                        self.scope.drop_scope();
                        match z {
                            Some(v) => match v {
                                Stopper::Return(a) => Some(a),
                                _ => {
                                    error::error(
                                        call.line,
                                        "cannot call break/continue at end of function",
                                    );
                                }
                            },
                            None => Some(LiteralType::Hempty),
                        }
                    } else {
                        error::error(call.line, format!("Function {} is not defined", name));
                    }
                }
                TokenType::Delete => {
                    if call.arguments.len() != 1 {
                        error::error(call.line, "delete takes one argument");
                    }
                    match &call.arguments[0] {
                        Stuff::Identifier(ident) => {
                            if self.scope.delete_var(&ident.name).is_some() {
                                None
                            } else {
                                error::error(
                                    ident.line,
                                    format!("Variable {} is not defined", ident.name),
                                );
                            }
                        }
                        _ => error::error(call.line, "delete only takes a variable name"),
                    }
                }
                TokenType::AddWith
                | TokenType::SubtractWith
                | TokenType::DivideWith
                | TokenType::MultiplyWith
                | TokenType::Set => match &call.arguments[0] {
                    Stuff::Identifier(ident) => {
                        if self.scope.has_var(&ident.name, true) {
                            let mut new_stuff = Vec::new();
                            for thing in call.arguments.iter().skip(1) {
                                match self.find_pointer_in_stuff(thing) {
                                    Some(new_thing) => new_stuff.push(new_thing.clone()),
                                    None => new_stuff.push(LiteralType::from_stuff(thing.clone())),
                                }
                            }
                            match new_stuff.len() {
                                1 => {
                                    let literal = &new_stuff[0];
                                    let var = match self.scope.get_var(&ident.name) {
                                        NewIdentifierType::Vairable(v) => v.value.clone(),
                                        NewIdentifierType::List(..) => {
                                            error::error(ident.line, "Cannot change entire list");
                                        }
                                    };

                                    match call.keyword {
                                        TokenType::Set => {
                                            self.scope.set_var(
                                                &ident.name.clone(),
                                                &[literal.clone()],
                                                true,
                                            );
                                            None
                                        }
                                        TokenType::AddWith => match var {
                                            LiteralType::Number(num) => match literal {
                                                LiteralType::Number(num2) => {
                                                    let new_val = num + num2;
                                                    self.scope.set_var(
                                                        &ident.name,
                                                        &[LiteralType::Number(new_val)],
                                                        true,
                                                    );
                                                    None
                                                }
                                                _ => {
                                                    error::error(
                                                        call.line,
                                                        format!(
                                                            "Variable {} is not a number",
                                                            ident.name
                                                        ),
                                                    );
                                                }
                                            },
                                            LiteralType::String(mut s) => match literal {
                                                LiteralType::String(s2) => {
                                                    s.push_str(s2);
                                                    self.scope.set_var(
                                                        &ident.name,
                                                        &[LiteralType::String(s)],
                                                        true,
                                                    );
                                                    None
                                                }
                                                LiteralType::Number(n) => {
                                                    s.push_str(&n.to_string());
                                                    self.scope.set_var(
                                                        &ident.name,
                                                        &[LiteralType::String(s)],
                                                        true,
                                                    );
                                                    None
                                                }
                                                LiteralType::Boolean(boolean) => {
                                                    s.push_str(&boolean.to_string());
                                                    self.scope.set_var(
                                                        &ident.name,
                                                        &[LiteralType::String(s)],
                                                        true,
                                                    );
                                                    None
                                                }
                                                LiteralType::Hempty => {
                                                    s.push_str("null");
                                                    self.scope.set_var(
                                                        &ident.name,
                                                        &[LiteralType::String(s)],
                                                        true,
                                                    );
                                                    None
                                                }
                                            },
                                            _ => {
                                                error::error(
                                                    call.line,
                                                    format!(
                                                        "Variable {} is not a number/string",
                                                        ident.name
                                                    ),
                                                );
                                            }
                                        },
                                        TokenType::MultiplyWith => match var {
                                            LiteralType::Number(num) => match literal {
                                                LiteralType::Number(num2) => {
                                                    let new_val = num * num2;
                                                    self.scope.set_var(
                                                        &ident.name,
                                                        &[LiteralType::Number(new_val)],
                                                        true,
                                                    );
                                                    None
                                                }
                                                _ => {
                                                    error::error(
                                                        call.line,
                                                        format!(
                                                            "Variable {} is not a number",
                                                            ident.name
                                                        ),
                                                    );
                                                }
                                            },
                                            LiteralType::String(ref s) => match literal {
                                                LiteralType::Number(num) => {
                                                    let new_string = (0..*num as i32)
                                                        .map(|_| s.clone())
                                                        .collect::<String>();
                                                    self.scope.set_var(
                                                        &ident.name,
                                                        &[LiteralType::String(new_string)],
                                                        true,
                                                    );
                                                    None
                                                }
                                                _ => {
                                                    error::error(
                                                        call.line,
                                                        format!(
                                                            "Variable {} is not a number",
                                                            ident.name
                                                        ),
                                                    );
                                                }
                                            },
                                            _ => {
                                                error::error(
                                                    call.line,
                                                    format!(
                                                        "Variable {} is not a number/string",
                                                        ident.name
                                                    ),
                                                );
                                            }
                                        },
                                        TokenType::SubtractWith | TokenType::DivideWith => {
                                            match var {
                                                LiteralType::Number(nums) => match literal {
                                                    LiteralType::Number(num) => {
                                                        if call.keyword == TokenType::SubtractWith {
                                                            let new_val = nums - num;
                                                            self.scope.set_var(
                                                                &ident.name,
                                                                &[LiteralType::Number(new_val)],
                                                                true,
                                                            );

                                                            None
                                                        } else {
                                                            let new_val = nums / num;
                                                            self.scope.set_var(
                                                                &ident.name,
                                                                &[LiteralType::Number(new_val)],
                                                                true,
                                                            );
                                                            None
                                                        }
                                                    }
                                                    _ => {
                                                        error::error(
                                                            call.line,
                                                            format!(
                                                                "Variable {} is not a number",
                                                                ident.name
                                                            ),
                                                        );
                                                    }
                                                },
                                                _ => {
                                                    error::error(
                                                        call.line,
                                                        format!(
                                                            "Variable {} is not a number/string",
                                                            ident.name
                                                        ),
                                                    );
                                                }
                                            }
                                        }
                                        _ => {
                                            error::error(
                                                call.line,
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
                                        self.scope.set_var(&ident.name, &new_stuff, true);
                                        None
                                    } else {
                                        error::error(
                                            call.line,
                                            format!(
                                                "Too many arguments for function {}",
                                                call.keyword
                                            )
                                            .as_str(),
                                        );
                                    }
                                }
                                _ => error::error(
                                    call.line,
                                    format!("Too many arguments for function {}", call.keyword)
                                        .as_str(),
                                ),
                            }
                        } else {
                            error::error(
                                ident.line,
                                format!("Variable {} is not defined", ident.name),
                            );
                        }
                    }
                    _ => {
                        error::error(
                            call.line,
                            format!("First argument of {} must be an identifier", call.keyword)
                                .as_str(),
                        );
                    }
                },

                t => {
                    let mut new_stuff = Vec::new();
                    for thing in &call.arguments {
                        match self.find_pointer_in_stuff(thing) {
                            Some(new_thing) => new_stuff.push(new_thing.clone()),
                            None => new_stuff.push(LiteralType::from_stuff(thing.clone())),
                        }
                    }
                    let lit = t.r#do(&new_stuff, call.line);
                    Some(lit)
                }
            },

            _ => Some(LiteralType::from_stuff(stuff.clone())),
        }
    }
}

impl fmt::Debug for Eval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "scope {:?}", self.scope).unwrap();
        Ok(())
    }
}
