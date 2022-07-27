#![allow(unused_variables, unreachable_patterns)]
use log::info;

use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use crate::{
    error,
    parser::{
        rules::{
            Call, Expression, IdentifierType, IfStatement, List, Literal, LiteralType, OtherStuff,
            Stuff, Vairable,
        },
        Thing,
    },
    token::TokenType,
};
#[derive(PartialEq, Clone)]
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
        // well have globa/local scope when we check for variables we check for variables in the current scope and then check the parent scope and so on until we find a variable or we reach the top of the scope stack (same for functions)
        // we can have two different variables with the same name in different scopes, the scope of a variable is determined by where it is declared in the code
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
                    None => {}
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

    fn find_pointer_in_other_stuff(&mut self, other_stuff: &OtherStuff) -> Option<OtherStuff> {
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
                        format!("Variable {} is not defined", ident.name),
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

    fn find_pointer_in_stuff(&mut self, stuff: &Stuff) -> Option<Stuff> {
        // need to make ways to extract values from literaltypes/literal/vars easy with function
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
                        format!("Variable {} is not defined", ident.name),
                    );
                }
            }
            Stuff::Call(call) => match &call.keyword {
                TokenType::FunctionIdentifier { name } => {
                    if self.function.contains_key(name) {
                        let function = self.function.get(name).unwrap().clone();
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
                                    format!("Too few or too many arguments for function {} expected: {}, found: {}", call.keyword, function.1, new_stuff.len()),
                                );
                        }
                        Some(Stuff::Call(Call::new(
                            new_stuff,
                            call.line,
                            TokenType::FunctionIdentifier { name: *name },
                        )))
                    } else {
                        error::error(call.line, format!("Function {} is not defined", name));
                    }
                }
                TokenType::AddWith
                | TokenType::SubtractWith
                | TokenType::DivideWith
                | TokenType::MultiplyWith
                | TokenType::Set => match &call.arguments[0] {
                    Stuff::Identifier(ident) => {
                        if self.vars.contains_key(&ident.name) {
                            let mut new_stuff: Vec<OtherStuff> = Vec::new();
                            for thing in call.arguments.iter().skip(1) {
                                match self.find_pointer_in_stuff(thing) {
                                    Some(new_thing) => {
                                        new_stuff.push(OtherStuff::from_stuff(&new_thing.clone()))
                                    }
                                    None => new_stuff.push(OtherStuff::from_stuff(thing)),
                                }
                            }
                            match new_stuff.len() {
                                1 => {
                                    let new_val = match &new_stuff[0] {
                                        OtherStuff::Literal(val) => val.literal.clone(),
                                        _ => {
                                            error::error(
                                                call.line,
                                                format!("Expected literal for {}", call.keyword),
                                            );
                                        }
                                    };
                                    let var = match self.vars.get(&ident.name.clone()).unwrap() {
                                        IdentifierType::Vairable(v) => match &v.value {
                                            OtherStuff::Literal(val) => val.literal.clone(),
                                            _ => {
                                                error::error(
                                                    call.line,
                                                    format!(
                                                        "Variable {} is not a literal",
                                                        ident.name
                                                    ),
                                                );
                                            }
                                        },
                                        IdentifierType::List(..) => {
                                            error::error(ident.line, "Cannot change entire list");
                                        }
                                    };
                                    match &new_stuff[0] {
                                        OtherStuff::Literal(literal) => {
                                            match call.keyword {
                                                TokenType::Set => {
                                                    self.vars.insert(
                                                        ident.name.clone(),
                                                        IdentifierType::Vairable(Box::new(
                                                            Vairable::new(OtherStuff::from_stuff(
                                                                &Stuff::Literal(literal.clone()),
                                                            )),
                                                        )),
                                                    );
                                                    None
                                                }
                                                TokenType::AddWith => match var {
                                                    LiteralType::Number(num) => {
                                                        match new_val {
                                                            LiteralType::Number(num2) => {
                                                                let new_val = num + num2;
                                                                self.vars.insert(ident.name.clone(), IdentifierType::Vairable(Box::new(Vairable::new(
                                                                    OtherStuff::from_stuff(&Stuff::Literal(Literal { literal: LiteralType::Number(new_val), line: call.line })),
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
                                                    LiteralType::String(mut s) => match new_val {
                                                        LiteralType::String(s2) => {
                                                            s.push_str(&s2);
                                                            self.vars.insert(ident.name.clone(), IdentifierType::Vairable(Box::new(Vairable::new(
                                                                    OtherStuff::from_stuff(&Stuff::Literal(Literal { literal: LiteralType::String(s), line: call.line })),
                                                                ))));
                                                            None
                                                        }
                                                        LiteralType::Number(n) => {
                                                            s.push_str(&n.to_string());
                                                            self.vars.insert(ident.name.clone(), IdentifierType::Vairable(Box::new(Vairable::new(
                                                                    OtherStuff::from_stuff(&Stuff::Literal(Literal { literal: LiteralType::String(s), line: call.line })),
                                                                ))));
                                                            None
                                                        }
                                                        LiteralType::Boolean(boolean) => {
                                                            s.push_str(&boolean.to_string());
                                                            self.vars.insert(ident.name.clone(), IdentifierType::Vairable(Box::new(Vairable::new(
                                                                    OtherStuff::from_stuff(&Stuff::Literal(Literal { literal: LiteralType::String(s), line: call.line })),
                                                                ))));
                                                            None
                                                        }
                                                        LiteralType::Hempty => {
                                                            s.push_str("null");
                                                            self.vars.insert(ident.name.clone(), IdentifierType::Vairable(Box::new(Vairable::new(
                                                                    OtherStuff::from_stuff(&Stuff::Literal(Literal { literal: LiteralType::String(s), line: call.line })),
                                                                ))));
                                                            None
                                                        }
                                                    },
                                                    _ => {
                                                        error::error(
                                                                call.line,
                                                                format!("Variable {} is not a number/string", ident.name),
                                                            );
                                                    }
                                                },
                                                TokenType::MultiplyWith => {
                                                    match var {
                                                        LiteralType::Number(num) => match new_val {
                                                            LiteralType::Number(num2) => {
                                                                let new_val = num * num2;
                                                                self.vars.insert(ident.name.clone(), IdentifierType::Vairable(Box::new(Vairable::new(
                                                                    OtherStuff::from_stuff(&Stuff::Literal(Literal { literal: LiteralType::Number(new_val), line: call.line })),
                                                                ))));
                                                                None
                                                            }
                                                            _ => {
                                                                error::error(
                                                                    call.line,
                                                                    format!("Variable {} is not a number", ident.name),
                                                                );
                                                            }
                                                        },
                                                        LiteralType::String(ref s) => match new_val
                                                        {
                                                            LiteralType::Number(num) => {
                                                                let new_string = String::from_iter(
                                                                    s.chars().map(|c| {
                                                                        c.to_string()
                                                                            .repeat(num as usize)
                                                                    }),
                                                                );
                                                                self.vars.insert(ident.name.clone(), IdentifierType::Vairable(Box::new(Vairable::new(
                                                                        OtherStuff::from_stuff(&Stuff::Literal(Literal { literal: LiteralType::String(new_string), line: call.line })),
                                                                    ))));
                                                                None
                                                            }
                                                            _ => {
                                                                error::error(
                                                                    call.line,
                                                                    format!("Variable {} is not a number", ident.name),
                                                                );
                                                            }
                                                        },
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
                                                            match new_val {
                                                                LiteralType::Number(num) => {
                                                                    if call.keyword
                                                                        == TokenType::SubtractWith
                                                                    {
                                                                        let new_val = nums - num;
                                                                        self.vars.insert(ident.name.clone(), IdentifierType::Vairable(Box::new(Vairable::new(
                                                                            OtherStuff::from_stuff(&Stuff::Literal(Literal { literal: LiteralType::Number(new_val), line: call.line })),
                                                                        ))));

                                                                        None
                                                                    } else {
                                                                        let new_val = nums / num;
                                                                        self.vars.insert(ident.name.clone(), IdentifierType::Vairable(Box::new(Vairable::new(
                                                                            OtherStuff::from_stuff(&Stuff::Literal(Literal { literal: LiteralType::Number(new_val), line: call.line })),
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
                                        _ => {
                                            error::error(
                                                call.line,
                                                "Only literals can be used in set",
                                            );
                                        }
                                    }
                                }
                                2 => {
                                    if call.keyword == TokenType::Set {
                                        self.vars.insert(
                                            ident.name.clone(),
                                            IdentifierType::List(Box::new(List::new(new_stuff))),
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
                    let mut new_stuff: Vec<Stuff> = Vec::new();
                    for thing in &call.arguments {
                        match self.find_pointer_in_stuff(thing) {
                            Some(new_thing) => new_stuff.push(new_thing.clone()),
                            None => new_stuff.push(thing.clone()),
                        }
                    }
                    let lit = t.r#do(new_stuff, call.line);
                    Some(Stuff::Literal(Literal {
                        line: call.line,
                        literal: lit,
                    }))
                }
            },

            _ => Some(stuff.clone()),
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
