//! code for macro parsing, expansion, and primitive macros

// TODO: create macro syntax
// TODO: quotation, list manipulation, ...
// TODO: define primitive macros for cond, link, let, ... (and maybe special way for primitive macros)
// primitive macro link needs some additional context to save what labels are linked to other labels which then gets passed to the compiler to make global goto possible

// Macros have their own phase called macro expansion
// it is kind of like an evaluation but macros can only currently be defined in the global scope
// macros use quotation and list manipulation to tranform syntax making it easier to create syntatic abstraction
// some implementations will recursively expand macros, like when a user-defined macro calls a primitive macro
// for now this will be a non-hygenic macro expander

use std::collections::HashMap;

use crate::{ast::UMPL2Expr, interior_mut::RC};

#[derive(Debug)]
pub struct MacroError {
    kind: MacroErrorKind,
}

#[derive(Debug)]
pub enum MacroErrorKind {
    InvalidForm(RC<str>),
    InvalidMacroCase,
    NoMacroCases,
    InvalidMacroCondition,
    NoMacroForms
}

#[derive(Default)]
pub struct MacroExpander {
    // TODO: make macro type
    macro_env: HashMap<RC<str>, MacroType>,
    links: HashMap<RC<str>, Vec<RC<str>>>,
}

impl MacroExpander {
    pub fn new() -> Self {
        let mut this = Self::default();
        this.macro_env.insert(
            "link".into(),
            MacroType::SpecialForm(Self::special_form_link),
        );
        this.macro_env.insert(
            "defmacro".into(),
            MacroType::SpecialForm(Self::special_form_macro),
        );
        this
    }

    pub fn expand(&mut self, exprs: &[UMPL2Expr]) -> Result<Vec<UMPL2Expr>, MacroError> {
        let mut res = vec![];
        for expr in exprs {
            match expr {
                UMPL2Expr::Scope(s) => {
                    res.push(UMPL2Expr::Scope(self.expand(s)?));
                }
                UMPL2Expr::Application(a) => {
                    // we expand all the sub expressions of the application before expanding the application itself
                    let a = self.expand(a)?;
                    if let Some(UMPL2Expr::Ident(op)) = a.first() {
                        if let Some(r#macro) = self.macro_env.get(op) {
                            match r#macro {
                                MacroType::SpecialForm(sf) => {
                                    res.extend(sf(self, &a[1..])?);
                                }
                                MacroType::UserDefined(cases) => {
                                   
                                    let expander = cases.iter().find(|case|case.0.len() + 1 == a.len()).ok_or(MacroError {
                                        kind: MacroErrorKind::InvalidForm(format!("arrity mismatch for macro {op}").into()),
                                    })?.clone();

                                    res.extend(Self::expand_macro(expander.0, a[1..].to_vec(), expander.1))
                                }
                            }
                        } else {
                            res.push(expr.clone());
                        }
                    } else {
                        res.push(expr.clone());
                    }
                }
                _ => {
                    res.push(expr.clone());
                }
            }
        }
        Ok(res)
    }

    fn expand_macro(params: Vec<RC<str>>, args: Vec<UMPL2Expr>, expansion: Vec<UMPL2Expr>) -> Vec<UMPL2Expr> {
        let env = params.into_iter().zip(args.into_iter()).collect::<HashMap<_,_>>();
        fn expand(exprs: Vec<UMPL2Expr>, env: &HashMap<std::rc::Rc<str>, UMPL2Expr>) -> Vec<UMPL2Expr> {
            exprs.into_iter().map(|expr| match &expr {
                UMPL2Expr::Ident(i) => env.get(i).cloned().unwrap_or(expr),
                UMPL2Expr::Application(a) =>UMPL2Expr::Application(expand(a.to_vec(), env)),
                UMPL2Expr::Scope(a) =>UMPL2Expr::Scope(expand(a.to_vec(), env)),
                _ => expr,
            }).collect()
        }
        
        expand(expansion, &env)
    }
}


// special forms
impl MacroExpander {
    fn special_form_link(&mut self, exprs: &[UMPL2Expr]) -> Result<Vec<UMPL2Expr>, MacroError> {
        let Some(UMPL2Expr::Label(linked)) = exprs.get(0) else {
            return Err(MacroError {
                kind: MacroErrorKind::InvalidForm("first expr of link must be a label".into()),
            });
        };
        let links = if let Some(links) = exprs.get(1..) {
            links
                .iter()
                .map(|expr| match expr {
                    UMPL2Expr::Label(l) => Some(l.clone()),
                    _ => None,
                })
                .collect::<Option<Vec<RC<str>>>>()
                .ok_or(MacroError {
                    kind: MacroErrorKind::InvalidForm(
                        "the elements of the link are not all labels".into(),
                    ),
                })?
        } else {
            return Err(MacroError {
                kind: MacroErrorKind::InvalidForm(
                    "there must at least one other expr in a link".into(),
                ),
            });
        };
        self.links.insert(linked.clone(), links);
        Ok(vec![])
    }

    fn special_form_macro(&mut self, exprs: &[UMPL2Expr]) -> Result<Vec<UMPL2Expr>, MacroError> {
        let Some(UMPL2Expr::Ident(macro_name)) = exprs.get(0) else {
            return Err(MacroError {
                kind: MacroErrorKind::InvalidForm("defmacro expects a name for the macro".into()),
            });
        };
        if let Some(cases) = exprs.get(1..) {
            let cases = MacroType::UserDefined(cases
                .into_iter()
                .map(|case| {
                    let UMPL2Expr::Application(case) = case else {
                        return Err(MacroError {
                            kind: MacroErrorKind::InvalidMacroCase,
                        });
                    };
                    let vars = match case.get(0) {
                        Some(UMPL2Expr::Application(cond)) => cond
                            .into_iter()
                            .map(|ident| match ident {
                                UMPL2Expr::Ident(cond) => Ok(cond.clone()),
                                _ => {
                                    return Err(MacroError {
                                        kind: MacroErrorKind::InvalidMacroCondition,
                                    })
                                }
                            })
                            .collect::<Result<_, _>>()?,
                        Some(UMPL2Expr::Ident(cond)) => {
                            vec![cond.clone()]
                        }
                        _ => {
                            return Err(MacroError {
                                kind: MacroErrorKind::InvalidMacroCondition,
                            })
                        }
                    };
                    let expand = case.get(1..).ok_or(MacroError {
                        kind: MacroErrorKind::NoMacroForms,
                    })?.to_vec();
                    Ok((vars, expand))
                    // TODO: parse the expansion part
                })
                .collect::<Result<Vec<_>,_>>()?);
            self.macro_env.insert(macro_name.clone(), cases);
            Ok(vec![])
        } else {
            Err(MacroError {
                kind: MacroErrorKind::NoMacroCases
            })
        }
        
    }
}

pub enum MacroType {
    SpecialForm(fn(&mut MacroExpander, &[UMPL2Expr]) -> Result<Vec<UMPL2Expr>, MacroError>),
    UserDefined(Vec<(Vec<RC<str>>, Vec<UMPL2Expr>)>, )
}
