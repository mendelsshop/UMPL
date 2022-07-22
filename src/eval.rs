#![allow(unused_variables, unreachable_patterns)]
use log::info;
use std::{collections::HashMap, fmt::Display};

use crate::{
    error,
    parser::{
        rules::{
            Call, Expression, IdentifierType, IfStatement,  OtherStuff, Stuff,
            Vairable,
        },
        Thing,
    },
    token::TokenType,
};
#[derive(PartialEq, Clone, Debug)]
pub struct Scope {
    pub vars: HashMap<String, IdentifierType>,
    pub function: HashMap<char, (Vec<Thing>, f64)>,

    pub body: Vec<Thing>,
    pub level: i32,
}

impl Scope {
    pub fn new(body: Vec<Thing>) -> Scope {
        let mut scope = Self {
            vars: HashMap::new(),
            function: HashMap::new(),

            body,
            level: 0,
        };
        scope.find_functions();
        scope.find_variables(scope.body.clone());
        scope
    }

    pub fn find_functions(&mut self) {
        for thing in self.body.clone().iter() {
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
        // variables should be immutable, so it fixes the problem of functions being able to change the value of variables
        let mut new_body = Vec::new();
        for thing in &body {
            match thing.clone() {
                Thing::Identifier(ref variable) => match variable.value {
                    IdentifierType::Vairable(ref name) => {
                        match self.find_pointer_in_other_stuff(&name.value) {
                            Some(pointer) => {
                                self.vars.insert(
                                    variable.name.clone(),
                                    IdentifierType::Vairable(Box::new(Vairable { value: pointer })),
                                );
                            }
                            None => {
                                self.vars
                                    .insert(variable.name.clone(), variable.value.clone());
                            }
                        }
                    }
                    IdentifierType::List(ref list) => {}
                },
                Thing::Return(oos, line) => match oos {
                    Some(os) => match self.find_pointer_in_other_stuff(&os) {
                        Some(identifier) => {
                            new_body.push(Thing::Return(Some(identifier), line));
                        }
                        None => {
                            new_body.push(Thing::Return(Some(os), line));
                        }
                    },
                    None => {
                        new_body.push(thing.clone());
                    }
                },
                Thing::Function(function) => {}
                Thing::Expression(expr) => match self.find_pointer_in_stuff(&expr.inside) {
                    Some(exprs) => {
                        new_body.push(Thing::Expression(Expression {
                            inside: exprs,
                            print: expr.print,
                            line: expr.line,
                        }));
                    }
                    None => new_body.push(Thing::Expression(expr)),
                },
                Thing::IfStatement(if_statement) => {
                    let conditon = match self.find_pointer_in_other_stuff(&if_statement.condition) {
                        Some(pointer) => {
                            info!("if {:?}", pointer);
                            pointer
                        }
                        None => if_statement.condition,
                    };
                    new_body.push(Thing::IfStatement(IfStatement::new(
                        conditon,
                        if_statement.body_true.clone(),
                        if_statement.body_false.clone(),
                        if_statement.line,
                    )));
                }
                Thing::LoopStatement(loop_statement) => {}
                Thing::Identifier(_) => {}
                Thing::Break(_) => {}
                Thing::Continue(_) => {}
            }
        }
        self.body = new_body;
    }

    fn find_pointer_in_other_stuff(&self, other_stuff: &OtherStuff) -> Option<OtherStuff> {
        match other_stuff {
            OtherStuff::Identifier(ident) => {
                if self.vars.contains_key(&ident.name) {
                    match self.vars.get(&ident.name).unwrap() {
                        IdentifierType::List(..) => {
                            error::error(ident.line, "whole list not supported in call")
                        }
                        IdentifierType::Vairable(var) => Some(var.value.clone()),
                    }
                } else {
                    error::error(
                        ident.line,
                        format!("Variable {} is not defined", ident.name).as_str(),
                    );
                }
            }
            OtherStuff::Expression(expr) => match self.find_pointer_in_stuff(&expr.inside) {
                Some(new_expr) => Some(OtherStuff::Expression(Expression::new(
                    new_expr, expr.print, expr.line,
                ))),
                None => Some(OtherStuff::Expression(expr.clone())),
            },
            _ => None,
        }
    }

    fn find_pointer_in_stuff(&self, stuff: &Stuff) -> Option<Stuff> {
        match stuff {
            Stuff::Identifier(ident) => {
                if self.vars.contains_key(&ident.name) {
                    match self.vars.get(&ident.name).unwrap() {
                        IdentifierType::List(..) => {
                            error::error(ident.line, "whole list not supported in call")
                        }
                        IdentifierType::Vairable(var) => match var.value.clone() {
                            OtherStuff::Expression(expr) => Some(expr.inside),
                            OtherStuff::Identifier(ident) => Some(Stuff::Identifier(ident)),
                            OtherStuff::Literal(function) => Some(Stuff::Literal(function)),
                        },
                    }
                } else {
                    error::error(
                        ident.line,
                        format!("Variable {} is not defined", ident.name).as_str(),
                    );
                }
            }
            Stuff::Call(call) => match call.keyword {
                TokenType::Plus | TokenType::Minus | TokenType::Divide | TokenType::Multiply => {
                    let mut new_stuff: Vec<Stuff> = Vec::new();
                    for thing in &call.arguments {
                        match self.find_pointer_in_stuff(thing) {
                            Some(new_thing) => new_stuff.push(new_thing.clone()),
                            None => new_stuff.push(thing.clone()),
                        }
                    }
                    Some(Stuff::Call(Call::new(
                        new_stuff,
                        call.line,
                        call.keyword.clone(),
                    )))
                }
                TokenType::Not
                | TokenType::Input
                | TokenType::Error
                | TokenType::Exit
                | TokenType::Negative => {
                    let mut new_stuff: Vec<Stuff> = Vec::new();
                    for (index, thing) in call.arguments.iter().enumerate() {
                        if index > 0 {
                            error::error(
                                call.line,
                                format!("Too many arguments for function {}", call.keyword)
                                    .as_str(),
                            );
                        }
                        match self.find_pointer_in_stuff(thing) {
                            Some(new_thing) => new_stuff.push(new_thing.clone()),
                            None => new_stuff.push(thing.clone()),
                        }
                    }
                    Some(Stuff::Call(Call::new(
                        new_stuff,
                        call.line,
                        call.keyword.clone(),
                    )))
                }
                TokenType::FunctionIdentifier { name } => {
                    if self.function.contains_key(&name) {
                        let function = self.function.get(&name).unwrap();
                        let mut new_stuff: Vec<Stuff> = Vec::new();
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
                                None => new_stuff.push(thing.clone()),
                            }
                        }
                        if new_stuff.len() != function.1 as usize {
                            error::error(
                                    call.line,
                                    format!("Too few or too many arguments for function {} expected: {}, found: {}", call.keyword, function.1, new_stuff.len()).as_str(),
                                );
                        }
                        Some(Stuff::Call(Call::new(
                            new_stuff,
                            call.line,
                            TokenType::FunctionIdentifier { name },
                        )))
                    } else {
                        error::error(
                            call.line,
                            format!("Function {} is not defined", name).as_str(),
                        );
                    }
                }

                _ => None,
            },
            _ => None,
        }
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "variables:")?;
        for (key, value) in &self.vars {
            writeln!(
                f,
                "\t{}: {}",
                key,
                match value {
                    IdentifierType::List(list) => list.to_string(),
                    IdentifierType::Vairable(variable) => variable.to_string(),
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
