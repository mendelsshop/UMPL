use std::{collections::HashMap, fmt::Display};

use crate::parser::{Thing, rules::IdentifierType};

pub struct Scope {
    pub vars: HashMap<String, IdentifierType>,
    pub function: HashMap<char, (Vec<Thing>, f64)>,
    pub body: Vec<Thing>,
}

impl Scope  {
    pub fn new(body: Vec<Thing>) -> Scope {
        Scope {
            vars: HashMap::new(),
            function: HashMap::new(),
            body
        }
    }

    pub fn find_functions(&mut self)   {
        for thing in &self.body {
            if let Thing::Function(function) = thing {
                self.function.insert(function.name, (function.body.clone(), function.num_arguments));
            }
        }
    }

    pub fn find_variables(&mut self) {
        for thing in &self.body {
            if let Thing::Identifier(variable) = thing {
                self.vars.insert(variable.name.clone(), variable.value.clone());
            }
        }
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "variables: \n")?;
        for (key, value) in &self.vars {
            write!(f, "\t{}: {}\n", key, match value {
                IdentifierType::List(list) => list.to_string(),
                IdentifierType::Vairable(variable) => variable.to_string(),
            })?;
        }
        write!(f, "Functions: \n")?;
        for (key, value) in &self.function {
            write!(f, "\t{}: {:?}\n", key, value)?;
        }
        Ok(())
    }
}

