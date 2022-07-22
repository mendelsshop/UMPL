#![allow(unused_variables, unreachable_patterns)]
use log::info;
use std::{collections::HashMap, fmt::Display};

use crate::{
    error,
    parser::{
        rules::{IdentifierType, IfStatement, OtherStuff},
        Thing,
    },
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
        scope.find_variables();
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

    pub fn find_variables(&mut self) {
        // create a vector to return instead of inplace modification
        let mut new_body = Vec::new();
        for thing in &self.body {
            match thing.clone() {
                Thing::Identifier(ref variable) => {
                    self.vars
                        .insert(variable.name.clone(), variable.value.clone());
                }
                Thing::Return(oos, _) => match oos {
                    Some(os) => match os {
                        OtherStuff::Identifier(ident) => {}
                        OtherStuff::Expression(expr) => {}
                        _ => {}
                    },
                    None => {
                        new_body.push(thing.clone());
                    }
                },
                Thing::Function(function) => {}
                Thing::Expression(expr) => {}
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
                Thing::Identifier(_) => todo!(),
                Thing::Break(_) => todo!(),
                Thing::Continue(_) => todo!(),
            }
        }
        self.body = new_body;
    }

    fn find_pointer_in_other_stuff(&self, other_stuff: &OtherStuff) -> Option<OtherStuff> {
        match other_stuff {
            OtherStuff::Identifier(ident) => {
                if self.vars.contains_key(&ident.name) {
                    match self.vars.get(&ident.name).unwrap() {
                        IdentifierType::List(..) => error::error(ident.line, ""),
                        IdentifierType::Vairable(var) => Some(var.value.clone()),
                    }
                } else {
                    error::error(
                        ident.line,
                        format!("Variable {} is not defined", ident.name).as_str(),
                    );
                }
            }
            OtherStuff::Expression(expr) => None,
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
            writeln!(f, "\t{}: {:?}", key, value)?;
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
