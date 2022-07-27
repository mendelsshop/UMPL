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
            match self.print {
                true => self.inside.clone(),
                false => LiteralType::String(String::from("")),
            }
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct NewList {
    pub first: LiteralType,
    pub second: LiteralType,
}

impl NewList {
    pub fn new(thing: Vec<LiteralType>) -> Self {
        Self {
            first: thing[0].clone(),
            second: thing[1].clone(),
        }
    }
}

impl Display for NewList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "with: [{}, {}]", self.first, self.second)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct NewVairable {
    pub value: LiteralType,
}

impl NewVairable {
    pub fn new(value: LiteralType) -> Self {
        Self { value }
    }

    pub fn new_empty(line: i32) -> Self {
        Self {
            value: LiteralType::new_hempty(),
        }
    }
}

impl Display for NewVairable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "with: {}", self.value)
    }
}

pub enum NewIdentifierType {
    List(Box<NewList>),
    Vairable(Box<NewVairable>),
}

impl NewIdentifierType {
    pub fn new(thing: Vec<LiteralType>) -> NewIdentifierType {
        match thing.len() {
            0 => error::error(0, "expected Identifier, got empty list"),
            1 => NewIdentifierType::Vairable(Box::new(NewVairable::new(thing[0].clone()))),
            2 => NewIdentifierType::List(Box::new(NewList::new(thing))),
            _ => error::error(0, "expected Identifier, got list with more than 2 elements"),
        }
    }
}

pub struct Scope {
    pub vars: HashMap<String, NewIdentifierType>,
    pub function: HashMap<char, (Vec<Thing>, f64)>,
    pub body: Vec<NewExpression>,
    pub level: i32,
}

impl Scope {
    pub fn new(body: Vec<Thing>) -> Scope {
        let mut scope = Self {
            vars: HashMap::new(),
            function: HashMap::new(),
            body: Vec::new(),
            level: 0,
        };
        scope.find_functions(body.clone());
        scope.find_variables(body);
        scope
    }

    pub fn find_functions(&mut self, body: Vec<Thing>) {
        for thing in body.iter() {
            if let Thing::Function(function) = thing {
                self.function.insert(
                    function.name,
                    (function.body.clone(), function.num_arguments),
                );
            }
        }
    }

    pub fn find_variables(&mut self, body: Vec<Thing>) {
        // create a vector to return instead of inplace modification
        // well have globa/local scope when we check for variables we check for variables in the current scope and then check the parent scope and so on until we find a variable or we reach the top of the scope stack (same for functions)
        // we can have two different variables with the same name in different scopes, the scope of a variable is determined by where it is declared in the code
        let mut new_body: Vec<NewExpression> = Vec::new();
        for thing in &body {
            match thing.clone() {
                Thing::Identifier(ref variable) => match variable.value {
                    IdentifierType::Vairable(ref name) => {
                        match self.find_pointer_in_other_stuff(&name.value) {
                            Some(pointer) => {
                                println!("{}", pointer);
                                self.vars.insert(
                                    variable.name.clone(),
                                    NewIdentifierType::Vairable(Box::new(NewVairable {
                                        value: pointer,
                                    })),
                                );
                            }
                            None => {
                                println!("{}", variable.name);
                                self.vars.insert(
                                    variable.name.clone(),
                                    NewIdentifierType::Vairable(Box::new(NewVairable {
                                        value: LiteralType::from_other_stuff(name.value.clone()),
                                    })),
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
                    }
                },
                Thing::Return(oos, line) => match oos {
                    Some(os) => match self.find_pointer_in_other_stuff(&os) {
                        Some(identifier) => {}
                        None => {}
                    },
                    None => {}
                },
                Thing::Function(function) => {}
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
                }
                Thing::LoopStatement(loop_statement) => {}
                Thing::Identifier(_) => {}
                Thing::Break(_) => {}
                Thing::Continue(_) => {}
            }
        }
        self.body = new_body;
    }

    fn find_pointer_in_other_stuff(&mut self, other_stuff: &OtherStuff) -> Option<LiteralType> {
        match other_stuff {
            OtherStuff::Identifier(ident) => {
                if self.vars.contains_key(&ident.name) {
                    match self.vars.get(&ident.name).unwrap() {
                        NewIdentifierType::List(..) => {
                            error::error(ident.line, "whole list not supported in call")
                        }
                        NewIdentifierType::Vairable(var) => Some(var.value.clone()),
                    }
                } else {
                    error::error(
                        ident.line,
                        format!("Variable {} is not defined", ident.name),
                    );
                }
            }
            OtherStuff::Expression(expr) => match self.find_pointer_in_stuff(&expr.inside) {
                Some(new_expr) => Some(new_expr),
                None => Some(LiteralType::from_stuff(expr.inside.clone())),
            },
            _ => None,
        }
    }

    fn find_pointer_in_stuff(&mut self, stuff: &Stuff) -> Option<LiteralType> {
        // need to make ways to extract values from literaltypes/literal/vars easy with function
        match stuff {
            Stuff::Identifier(ident) => {
                if self.vars.contains_key(&ident.name) {
                    match self.vars.get(&ident.name).unwrap() {
                        NewIdentifierType::List(..) => {
                            error::error(ident.line, "whole list not supported in call")
                        }
                        NewIdentifierType::Vairable(var) => Some(var.value.clone()),
                    }
                } else {
                    error::error(
                        ident.line,
                        format!("Variable {} is not defined", ident.name),
                    );
                }
            }
            Stuff::Call(call) => {
                match &call.keyword {
                    TokenType::FunctionIdentifier { name } => {
                        if self.function.contains_key(name) {
                            let function = self.function.get(name).unwrap().clone();
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
                                if self.vars.contains_key(&ident.name) {
                                    self.vars.remove(&ident.name);
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
                    | TokenType::Set => {
                        match &call.arguments[0] {
                            Stuff::Identifier(ident) => {
                                if self.vars.contains_key(&ident.name) {
                                    let mut new_stuff = Vec::new();
                                    for thing in call.arguments.iter().skip(1) {
                                        match self.find_pointer_in_stuff(thing) {
                                            Some(new_thing) => new_stuff.push(new_thing.clone()),
                                            None => new_stuff
                                                .push(LiteralType::from_stuff(thing.clone())),
                                        }
                                    }
                                    match new_stuff.len() {
                                        1 => {
                                            let literal = &new_stuff[0];
                                            let var =
                                                match self.vars.get(&ident.name.clone()).unwrap() {
                                                    NewIdentifierType::Vairable(v) => {
                                                        v.value.clone()
                                                    }
                                                    NewIdentifierType::List(..) => {
                                                        error::error(
                                                            ident.line,
                                                            "Cannot change entire list",
                                                        );
                                                    }
                                                };

                                            match call.keyword {
                                                TokenType::Set => {
                                                    self.vars.insert(
                                                        ident.name.clone(),
                                                        NewIdentifierType::Vairable(Box::new(
                                                            NewVairable::new(literal.clone()),
                                                        )),
                                                    );
                                                    None
                                                }
                                                TokenType::AddWith => {
                                                    match var {
                                                        LiteralType::Number(num) => {
                                                            match literal {
                                                                LiteralType::Number(num2) => {
                                                                    let new_val = num + num2;
                                                                    self.vars.insert(ident.name.clone(), NewIdentifierType::Vairable(Box::new(NewVairable::new(
                                                                    LiteralType::Number(new_val)
                                                                ))));
                                                                    None
                                                                }
                                                                _ => {
                                                                    error::error(
                                                                    call.line,
                                                                    format!("Variable {} is not a number", ident.name),
                                                                );
                                                                }
                                                            }
                                                        }
                                                        LiteralType::String(mut s) => {
                                                            match literal {
                                                                LiteralType::String(s2) => {
                                                                    s.push_str(s2);
                                                                    self.vars.insert(ident.name.clone(), NewIdentifierType::Vairable(Box::new(NewVairable::new(
                                                                    LiteralType::String(s)
                                                                ))));
                                                                    None
                                                                }
                                                                LiteralType::Number(n) => {
                                                                    s.push_str(&n.to_string());
                                                                    self.vars.insert(ident.name.clone(), NewIdentifierType::Vairable(Box::new(NewVairable::new(
                                                                    LiteralType::String(s)
                                                                ))));
                                                                    None
                                                                }
                                                                LiteralType::Boolean(boolean) => {
                                                                    s.push_str(
                                                                        &boolean.to_string(),
                                                                    );
                                                                    self.vars.insert(ident.name.clone(), NewIdentifierType::Vairable(Box::new(NewVairable::new(
                                                                    LiteralType::String(s)
                                                                ))));
                                                                    None
                                                                }
                                                                LiteralType::Hempty => {
                                                                    s.push_str("null");
                                                                    self.vars.insert(ident.name.clone(), NewIdentifierType::Vairable(Box::new(NewVairable::new(
                                                                    LiteralType::String(s)
                                                                ))));
                                                                    None
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            error::error(
                                                                call.line,
                                                                format!("Variable {} is not a number/string", ident.name),
                                                            );
                                                        }
                                                    }
                                                }
                                                TokenType::MultiplyWith => {
                                                    match var {
                                                        LiteralType::Number(num) => {
                                                            match literal {
                                                                LiteralType::Number(num2) => {
                                                                    let new_val = num * num2;
                                                                    self.vars.insert(ident.name.clone(), NewIdentifierType::Vairable(Box::new(NewVairable::new(
                                                                    LiteralType::Number(new_val)
                                                                ))));
                                                                    None
                                                                }
                                                                _ => {
                                                                    error::error(
                                                                    call.line,
                                                                    format!("Variable {} is not a number", ident.name),
                                                                );
                                                                }
                                                            }
                                                        }
                                                        LiteralType::String(ref s) => {
                                                            match literal {
                                                                LiteralType::Number(num) => {
                                                                    let new_string =
                                                                        String::from_iter(
                                                                            // create iterator that repeats the string num times
                                                                            (0..*num as i32)
                                                                                .map(|_| s.clone()),
                                                                        );
                                                                    self.vars.insert(ident.name.clone(), NewIdentifierType::Vairable(Box::new(NewVairable::new(
                                                                        LiteralType::String(new_string)
                                                                    ))));
                                                                    None
                                                                }
                                                                _ => {
                                                                    error::error(
                                                                    call.line,
                                                                    format!("Variable {} is not a number", ident.name),
                                                                );
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            error::error(
                                                                call.line,
                                                                format!("Variable {} is not a number/string", ident.name),
                                                            );
                                                        }
                                                    }
                                                }
                                                TokenType::SubtractWith | TokenType::DivideWith => {
                                                    match var {
                                                        LiteralType::Number(nums) => {
                                                            match literal {
                                                                LiteralType::Number(num) => {
                                                                    if call.keyword
                                                                        == TokenType::SubtractWith
                                                                    {
                                                                        let new_val = nums - num;
                                                                        self.vars.insert(ident.name.clone(), NewIdentifierType::Vairable(Box::new(NewVairable::new(
                                                                            LiteralType::Number(new_val)
                                                                        ))));

                                                                        None
                                                                    } else {
                                                                        let new_val = nums / num;
                                                                        self.vars.insert(ident.name.clone(), NewIdentifierType::Vairable(Box::new(NewVairable::new(
                                                                            LiteralType::Number(new_val)
                                                                        ))));
                                                                        None
                                                                    }
                                                                }
                                                                _ => {
                                                                    error::error(
                                                                        call.line,
                                                                        format!("Variable {} is not a number", ident.name),
                                                                    );
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            error::error(
                                                                call.line,
                                                                format!("Variable {} is not a number/string", ident.name),
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
                                                self.vars.insert(
                                                    ident.name.clone(),
                                                    NewIdentifierType::List(Box::new(
                                                        NewList::new(new_stuff),
                                                    )),
                                                );
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
                                            format!(
                                                "Too many arguments for function {}",
                                                call.keyword
                                            )
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
                                    format!(
                                        "First argument of {} must be an identifier",
                                        call.keyword
                                    )
                                    .as_str(),
                                );
                            }
                        }
                    }

                    t => {
                        let mut new_stuff = Vec::new();
                        for thing in &call.arguments {
                            match self.find_pointer_in_stuff(thing) {
                                Some(new_thing) => new_stuff.push(new_thing.clone()),
                                None => new_stuff.push(LiteralType::from_stuff(thing.clone())),
                            }
                        }
                        let lit = t.r#do(new_stuff, call.line);
                        Some(lit)
                    }
                }
            }

            _ => Some(LiteralType::from_stuff(stuff.clone())),
        }
    }
}

impl fmt::Debug for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "variables:")?;
        for (key, value) in &self.vars {
            writeln!(
                f,
                "\t{}: {}",
                key,
                match value {
                    NewIdentifierType::List(list) => list.to_string(),
                    NewIdentifierType::Vairable(variable) => variable.to_string(),
                }
            )?;
        }
        writeln!(f, "Functions:")?;
        for (key, value) in &self.function {
            writeln!(
                f,
                "\t{} {}: body: {}",
                key,
                value.1,
                value
                    .0
                    .iter()
                    .map(|thing| thing.to_string())
                    .collect::<Vec<String>>()
                    .join("\n\t")
            )?;
        }

        write!(
            f,
            "Body: \n\t{}",
            self.body
                .iter()
                .map(|thing| thing.to_string())
                .collect::<Vec<String>>()
                .join("\n\t")
        )
        .unwrap();
        Ok(())
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.body
                .iter()
                .map(|thing| thing.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}
