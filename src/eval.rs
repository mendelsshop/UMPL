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
#[derive(PartialEq, Clone)]
pub struct NewExpression {
    pub inside: LiteralType,
    pub print: bool,
    pub line: i32,
}

impl Display for NewExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            if self.print {
                self.inside.clone()
            } else {
                LiteralType::String(String::from(""))
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
    pub first: LitOrList,
    pub second: LitOrList,
}

impl NewList {
    pub fn new(thing: &[LiteralType]) -> Self {
        Self {
            first: LitOrList::Literal(thing[0].clone()),
            second: LitOrList::Literal(thing[1].clone()),
        }
    }
}

impl Display for NewList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "with: [{}, {}]", self.first, self.second)
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
    pub fn set_var(&mut self, name: &str, value: &[LiteralType]) {
        // the reason for this being its own method vs using the set method is because it will be easier to use/implemnet getting variable from different scopes
        // and also less typing instead of creating a NewIdentifierType you just pass in a vector of LiteralType
        let new_val = match value.len() {
            0 => error::error(0, "expected Identifier, got empty list"),
            1 => NewIdentifierType::Vairable(Box::new(NewVairable::new(value[0].clone()))),
            2 => NewIdentifierType::List(Box::new(NewList::new(value))),
            _ => error::error(0, "expected Identifier, got list with more than 2 elements"),
        };
        match name {
            name if name.ends_with(".first") | name.ends_with(".second") => {
                if let Some(NewIdentifierType::List(mut list)) =
                    self.get_var(&name[..name.len() - 6])
                {
                    if name.ends_with(".first") {
                        {
                            // check if new value is a list or a variable
                            match &new_val {
                                NewIdentifierType::List(list2) => {
                                    list.first = LitOrList::Identifier(list2.clone());
                                }
                                NewIdentifierType::Vairable(var) => {
                                    list.first = LitOrList::Literal(var.value.clone());
                                }
                            }
                        }
                    } else {
                        // check if new value is a list or a variable
                        match &new_val {
                            NewIdentifierType::List(list2) => {
                                list.second = LitOrList::Identifier(list2.clone());
                            }
                            NewIdentifierType::Vairable(var) => {
                                list.second = LitOrList::Literal(var.value.clone());
                            }
                        }
                    }
                } else {
                    error::error(0, "expected list, got something else");
                }
            }
            _ => {
                self.vars.insert(name.to_string(), new_val);
            }
        }
    }

    pub fn get_var(&self, name: &str) -> Option<NewIdentifierType> {
        // the reason for this being its own method vs using the get method is because it will be easier to use/implemnet getting variable from different scopes
        self.vars.get(name).cloned()
    }
    pub fn set_function(&mut self, name: char, args: Vec<Thing>, body: f64) {
        self.function.insert(name, (args, body));
    }
    pub fn get_function(&self, name: char) -> Option<(Vec<Thing>, f64)> {
        self.function.get(&name).cloned()
    }

    pub fn delete_var(&mut self, name: &str) -> Option<NewIdentifierType> {
        self.vars.remove(name)
    }

    pub fn has_var(&self, name: &str) -> bool {
        self.vars.contains_key(name)
    }

    pub fn has_function(&self, name: char) -> bool {
        self.function.contains_key(&name)
    }
}

#[derive(PartialEq, Clone)]
pub struct Eval {
    pub body: Vec<NewExpression>,
    pub level: i32,
    pub scope: Scope,
}

impl Eval {
    pub fn new(body: &[Thing]) -> Self {
        Self {
            body: Vec::new(),
            level: 0,
            scope: Scope::new(),
        }
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

    pub fn find_variables(&mut self, body: &[Thing]) {
        // create a vector to return instead of inplace modification
        // well have globa/local scope when we check for variables we check for variables in the current scope and then check the parent scope and so on until we find a variable or we reach the top of the scope stack (same for functions)
        // we can have two different variables with the same name in different scopes, the scope of a variable is determined by where it is declared in the code
        let mut new_body: Vec<NewExpression> = Vec::new();
        for thing in body {
            match thing.clone() {
                Thing::Identifier(ref variable) => match variable.value {
                    IdentifierType::Vairable(ref name) => {
                        match self.find_pointer_in_other_stuff(&name.value) {
                            Some(pointer) => {
                                self.scope.set_var(&variable.name, &[pointer]);
                            }
                            None => {
                                self.scope.set_var(
                                    &variable.name,
                                    &[LiteralType::from_other_stuff(name.value.clone())],
                                );
                            }
                        }
                    }
                    IdentifierType::List(ref list) => {
                        let first = match self.find_pointer_in_other_stuff(&list.first) {
                            Some(pointer) => pointer,
                            None => LiteralType::from_other_stuff(list.first.clone()),
                        };
                        let second = match self.find_pointer_in_other_stuff(&list.second) {
                            Some(pointer) => pointer,
                            None => LiteralType::from_other_stuff(list.second.clone()),
                        };
                        self.scope.set_var(&variable.name, &[first, second]);
                    }
                },
                Thing::Return(os, line) => match os {
                    Some(os) => match self.find_pointer_in_other_stuff(&os) {
                        Some(identifier) => {
                            todo!()
                        }
                        None => {
                            todo!()
                        }
                    },
                    None => {
                        todo!()
                    }
                },
                Thing::Expression(expr) => match self.find_pointer_in_stuff(&expr.inside) {
                    Some(exprs) => {
                        new_body.push(NewExpression {
                            inside: exprs,
                            print: expr.print,
                            line: expr.line,
                        });
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
                    todo!()
                }
                Thing::LoopStatement(loop_statement) => {
                    todo!()
                }
                Thing::Break(_) | Thing::Continue(_) => {
                    todo!()
                }
                _ => {}
            }
        }
        self.body = new_body;
    }

    fn find_pointer_in_other_stuff(&mut self, other_stuff: &OtherStuff) -> Option<LiteralType> {
        match other_stuff {
            OtherStuff::Identifier(ident) => self.scope.get_var(&ident.name).map_or_else(
                || {
                    error::error(
                        ident.line,
                        format!("Variable {} is not defined", ident.name),
                    );
                },
                |i| match i {
                    NewIdentifierType::List(..) => {
                        error::error(ident.line, "whole list not supported in call")
                    }
                    NewIdentifierType::Vairable(var) => Some(var.value.clone()),
                },
            ),
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
            Stuff::Identifier(ident) => self.scope.get_var(&ident.name).map_or_else(
                || {
                    error::error(
                        ident.line,
                        format!("Variable {} is not defined", ident.name),
                    );
                },
                |i| match i {
                    NewIdentifierType::List(..) => {
                        error::error(ident.line, "whole list not supported in call")
                    }
                    NewIdentifierType::Vairable(var) => Some(var.value.clone()),
                },
            ),
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
                        todo!()
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
                        if self.scope.has_var(&ident.name) {
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
                                    let var = match self.scope.get_var(&ident.name).unwrap() {
                                        NewIdentifierType::Vairable(v) => v.value.clone(),
                                        NewIdentifierType::List(..) => {
                                            error::error(ident.line, "Cannot change entire list");
                                        }
                                    };

                                    match call.keyword {
                                        TokenType::Set => {
                                            self.scope
                                                .set_var(&ident.name.clone(), &[literal.clone()]);
                                            None
                                        }
                                        TokenType::AddWith => match var {
                                            LiteralType::Number(num) => match literal {
                                                LiteralType::Number(num2) => {
                                                    let new_val = num + num2;
                                                    self.scope.set_var(
                                                        &ident.name,
                                                        &[LiteralType::Number(new_val)],
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
                                                    );
                                                    None
                                                }
                                                LiteralType::Number(n) => {
                                                    s.push_str(&n.to_string());
                                                    self.scope.set_var(
                                                        &ident.name,
                                                        &[LiteralType::String(s)],
                                                    );
                                                    None
                                                }
                                                LiteralType::Boolean(boolean) => {
                                                    s.push_str(&boolean.to_string());
                                                    self.scope.set_var(
                                                        &ident.name,
                                                        &[LiteralType::String(s)],
                                                    );
                                                    None
                                                }
                                                LiteralType::Hempty => {
                                                    s.push_str("null");
                                                    self.scope.set_var(
                                                        &ident.name,
                                                        &[LiteralType::String(s)],
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
                                                            );

                                                            None
                                                        } else {
                                                            let new_val = nums / num;
                                                            self.scope.set_var(
                                                                &ident.name,
                                                                &[LiteralType::Number(new_val)],
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
                                        self.scope.set_var(&ident.name, &new_stuff);
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
        write!(
            f,
            "Body: \n\t{}",
            self.body
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join("\n\t")
        )
        .unwrap();
        Ok(())
    }
}

impl Display for Eval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.body
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
