use std::{collections::HashMap, fmt::Display};

use crate::parser::{rules::IdentifierType, Thing};
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
        for thing in &self.body {
            if let Thing::Function(function) = thing {
                // self.body.remove(index);
                self.function.insert(
                    function.name,
                    (function.body.body.clone(), function.num_arguments),
                );
            }
        }
    }

    pub fn find_variables(&mut self) {
        for thing in &self.body {
            if let Thing::Identifier(variable) = thing {
                self.vars
                    .insert(variable.name.clone(), variable.value.clone());
            }
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
            "Body: \n{}",
            self.body
                .iter()
                .map(|thing| thing.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
        .unwrap();

        // .join("\n\t")?;
        Ok(())
    }
}
