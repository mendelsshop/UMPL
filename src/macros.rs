//! code for macro parsing, expansion, and primitive macros

// TODO: create macro syntax
// TODO: quotation, list manipulation, ...
// TODO: define primitive macros for cond, link, let, ... (and maybe special way for primitive macros)
// primitive macro link needs some additional context to save what labels are linked to other labels which then gets passed to the compiler to make global goto possible

// Macros have their own phase called macro expansion
// it is kind of like an evaluation in that macros are scoped to a certain envoirment kept only for macro expansion
// macros use quotation and list manipulation to tranform syntax making it easier to create syntatic abstraction
// some implementations will recursively expand macros, like when a user-defined macro calls a primitive macro 
// for now this will be a non-hygenic macro expander

use std::collections::HashMap;

use crate::{interior_mut::RC, ast::UMPL2Expr};

pub struct MacroError {}

#[derive(Default)]
pub struct MacroExpander {
    // TODO: make macro type
    macro_env: Vec<HashMap<RC<str>, ()>>,
    links: HashMap<RC<str>, Vec<RC<str>>>
}

impl  MacroExpander {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn expand(&mut self, exprs: Vec<UMPL2Expr>) -> Result<Vec<UMPL2Expr>, MacroError> {
        todo!()
    }
}