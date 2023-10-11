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
    NoMacroForms,
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
                                    let expander = cases.iter().find_map(|(case, res)| {
                                        // case.
                                        let MacroArg::List(case) = case else {
                                            unreachable!()
                                        };
                                        println!("case: {:?}", case);
                                        println!("expression: {:?}", &a[1..]);
                                        println!("result: {:?}", res);
                                        fun_name(&a[1..], case).map(|bindings| (bindings, res)).ok()
                                    });
                                    println!("expander {:?}", expander);
                                    // let expander = cases
                                    //     .iter()
                                    //     .find(|case| case.0.len() + 1 == a.len())
                                    //     .ok_or(MacroError {
                                    //         kind: MacroErrorKind::InvalidForm(
                                    //             format!("arrity mismatch for macro {op}").into(),
                                    //         ),
                                    //     })?
                                    //     .clone();

                                    // res.extend(Self::expand_macro(
                                    //     // expander.0,
                                    //     todo!(),
                                    //     a[1..].to_vec(),
                                    //     expander.1,
                                    // ))
                                    todo!()
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

    fn expand_macro(
        params: Vec<RC<str>>,
        args: Vec<UMPL2Expr>,
        expansion: Vec<UMPL2Expr>,
    ) -> Vec<UMPL2Expr> {
        let env = params
            .into_iter()
            .zip(args.into_iter())
            .collect::<HashMap<_, _>>();
        fn expand(
            exprs: Vec<UMPL2Expr>,
            env: &HashMap<std::rc::Rc<str>, UMPL2Expr>,
        ) -> Vec<UMPL2Expr> {
            exprs
                .into_iter()
                .map(|expr| match &expr {
                    UMPL2Expr::Ident(i) => env.get(i).cloned().unwrap_or(expr),
                    UMPL2Expr::Application(a) => UMPL2Expr::Application(expand(a.to_vec(), env)),
                    UMPL2Expr::Scope(a) => UMPL2Expr::Scope(expand(a.to_vec(), env)),
                    _ => expr,
                })
                .collect()
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
            let cases = MacroType::UserDefined(
                cases
                    .into_iter()
                    .map(|case| {
                        let UMPL2Expr::Application(rule) = case else {
                            return Err(MacroError {
                                kind: MacroErrorKind::InvalidMacroCase,
                            });
                        };

                        let Some(UMPL2Expr::Application(case)) = rule.first() else {
                            return Err(MacroError {
                                kind: MacroErrorKind::InvalidMacroCase,
                            });
                        };
                        let cond: MacroArg = case.as_slice().try_into()?;

                        let expand = rule
                            .get(1..)
                            .ok_or(MacroError {
                                kind: MacroErrorKind::NoMacroForms,
                            })?
                            .to_vec();
                        Ok((cond, expand))
                        // TODO: parse the expansion part
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            );
            self.macro_env.insert(macro_name.clone(), cases);
            Ok(vec![])
        } else {
            Err(MacroError {
                kind: MacroErrorKind::NoMacroCases,
            })
        }
    }
}

pub enum MacroType {
    SpecialForm(fn(&mut MacroExpander, &[UMPL2Expr]) -> Result<Vec<UMPL2Expr>, MacroError>),
    // the parameters of a macro can nested lists and can have constraints such taht certain symbols are required
    UserDefined(Vec<(MacroArg, Vec<UMPL2Expr>)>),
}

#[derive(Debug)]
pub enum MacroArg {
    // a list of macro arguments
    List(Vec<MacroArg>),
    Ident(RC<str>),
    // when encountred during macro expansion must match specifically this symbol
    Constant(RC<str>),
    // will be some if there is a arg before it
    KleeneClosure(Option<Box<MacroArg>>),
}

impl TryFrom<&[UMPL2Expr]> for MacroArg {
    type Error = MacroError;

    fn try_from(value: &[UMPL2Expr]) -> Result<Self, Self::Error> {
        // TOOD: check for empty kleene closures if they have something before them
        // and if more than one kleene closure error
        let mut ret: Vec<_> = value
            .into_iter()
            .map(TryFrom::try_from)
            .collect::<Result<_, _>>()?;
        let mut temp = MacroArg::KleeneClosure(None);
        println!("{:?}", ret);
        if let Some(MacroArg::KleeneClosure(_)) = ret.first() {
            if ret
                .iter()
                .skip(1)
                .find(|arg| matches!(arg, MacroArg::KleeneClosure(_)))
                .is_some()
            {
                Err(MacroError {
                    kind: MacroErrorKind::InvalidMacroCondition,
                })
            } else {
                Ok(MacroArg::List(ret))
            }
        } else {
            if let Some(pos) = ret
                .iter()
                .position(|arg| matches!(arg, MacroArg::KleeneClosure(_)))
            {
                if ret
                    .iter()
                    .skip(pos + 1)
                    .find(|arg| matches!(arg, MacroArg::KleeneClosure(_)))
                    .is_some()
                {
                    // if we encounter 2 *
                    Err(MacroError {
                        kind: MacroErrorKind::InvalidMacroCondition,
                    })
                } else {
                    std::mem::swap(&mut ret[pos - 1], &mut temp);
                    ret[pos - 1] = MacroArg::KleeneClosure(Some(Box::new(temp)));
                    ret.remove(pos);
                    Ok(MacroArg::List(ret))
                }
            } else {
                Ok(MacroArg::List(ret))
            }
        }
    }
}

impl TryFrom<&UMPL2Expr> for MacroArg {
    type Error = MacroError;

    fn try_from(value: &UMPL2Expr) -> Result<Self, Self::Error> {
        match value {
            UMPL2Expr::Application(a) => a.as_slice().try_into(),
            UMPL2Expr::Ident(i) => Ok(if i == &("*".into()) {
                MacroArg::KleeneClosure(None)
            } else {
                MacroArg::Ident(i.clone())
            }),
            UMPL2Expr::String(s) => Ok(MacroArg::Constant(s.clone())),
            _ => todo!("error"),
        }
    }
}
//https://github.com/rust-lang/rust-analyzer/pull/719/files#diff-ebf19883d233fae7be3aec83af917278e2ed7b79d3847c69941dfdf0728b0583 (could be helpfull)
#[derive(Debug)]
pub enum MacroBinding {
    // generated from matching *
    List(Vec<MacroBinding>),
    Expr(UMPL2Expr),
}

#[derive(Debug)]
pub enum MacroMatchError {}

trait PushNested {
    fn push_nested(&mut self, nested: Self);
}

impl PushNested for HashMap<RC<str>, MacroBinding> {
    fn push_nested(&mut self, nested: Self) {
        for (k, v) in nested {
            if !self.contains_key(&k) {
                self.insert(k.clone(), MacroBinding::List(vec![]));
            }
            match self.get_mut(&k) {
                Some(MacroBinding::List(l)) => l.push(v),
                _ => panic!(),
            }
        }
    }
}

impl MacroArg {
    fn matches(
        &self,
        pattern: &[UMPL2Expr],
    ) -> Result<HashMap<RC<str>, MacroBinding>, MacroMatchError> {
        // let mut bindings = HashMap::new();
        // pattern ((a b *) *)
        // thing   ((1 4 6) (1 5 7))
        // matched (a: (1 1) b: ((4 6) (5 7)))
        // we need to keep some info about pattern to help with matching
        // ((a b *) *) -> (a b) ((a b *) *)
        todo!()
    }

    fn matches_expr(
        &self,
        pattern: &UMPL2Expr,
    ) -> Result<HashMap<RC<str>, MacroBinding>, MacroError> {
        let mut bindings = HashMap::new();
        match (self, pattern) {
            (MacroArg::List(pattern), UMPL2Expr::Application(expr)) => {
                bindings.push_nested(fun_name(expr, pattern)?);
            }
            // error
            (MacroArg::List(_), _) => todo!(),
            (MacroArg::Ident(i), expr) => {
                bindings.insert(i.clone(), MacroBinding::Expr(expr.clone()));
            }

            (MacroArg::Constant(a), UMPL2Expr::Ident(b)) if a == b => todo!(),
            // error
            (MacroArg::Constant(_), _) => todo!(),
            (MacroArg::KleeneClosure(_), UMPL2Expr::Bool(_)) => todo!(),
            (MacroArg::KleeneClosure(_), UMPL2Expr::Number(_)) => todo!(),
            (MacroArg::KleeneClosure(_), UMPL2Expr::String(_)) => todo!(),
            (MacroArg::KleeneClosure(_), UMPL2Expr::Scope(_)) => todo!(),
            (MacroArg::KleeneClosure(_), UMPL2Expr::Ident(_)) => todo!(),
            (MacroArg::KleeneClosure(_), UMPL2Expr::Application(_)) => todo!(),
            (MacroArg::KleeneClosure(_), UMPL2Expr::Label(_)) => todo!(),
            (MacroArg::KleeneClosure(_), UMPL2Expr::FnParam(_)) => todo!(),
            (MacroArg::KleeneClosure(_), UMPL2Expr::Hempty) => todo!(),
        }
        Ok(bindings)
    }
}

fn fun_name(
    expr: &[UMPL2Expr],
    pattern: &[MacroArg],
) -> Result<HashMap<RC<str>, MacroBinding>, MacroError> {
    let mut expr_count = expr.len() - 1;
    let mut pat_count = pattern.len() - 1;
    let mut bindings = HashMap::new();
    let mut expr = expr.into_iter();
    for pat in pattern {
        match pat {
            MacroArg::List(pat) => {
                let Some((UMPL2Expr::Application(expr))) = expr.next() else {
                    panic!()
                };
                bindings.push_nested(fun_name(expr, pat)?);
                expr_count -= 1;
            }
            MacroArg::Ident(i) => {
                let expr = expr.next().unwrap();
                bindings.insert(i.clone(), MacroBinding::Expr(expr.clone()));
                expr_count -= 1;
            }
            MacroArg::Constant(c) => {
                let Some((UMPL2Expr::Ident(expr))) = expr.next() else {
                    panic!()
                };
                if c != expr {
                    panic!()
                }
                expr_count -= 1;
            }
            MacroArg::KleeneClosure(c) => {
                let matched = (&mut expr).take(expr_count.checked_sub(pat_count - 1).unwrap_or(0));
                if let Some(c) = c {
                    for expr in matched {
                        bindings.push_nested(c.matches_expr(expr)?);
                    }
                }
            }
        }
        pat_count -= 1;
    }
    Ok(bindings)
}

#[cfg(test)]
mod tests {
    use crate::{ast::UMPL2Expr, lexer::umpl_parse};

    use super::MacroExpander;

    #[test]
    fn basic_macro_test() {
        let parsed = umpl_parse(
            "(defmacro test [(a) (a a)])
        (test 1)",
        )
        .unwrap();
        let expanded = MacroExpander::new().expand(&parsed).unwrap();
        assert_eq!(
            expanded,
            vec![UMPL2Expr::Application(vec![
                UMPL2Expr::Number(1.0),
                UMPL2Expr::Number(1.0)
            ])]
        )
    }

    #[test]
    fn basic_macro_constant_test() {
        let parsed = umpl_parse(
            "(defmacro test [(.a.) (1 1)])
        (test a)",
        )
        .unwrap();
        let expanded = MacroExpander::new().expand(&parsed).unwrap();
        assert_eq!(
            expanded,
            vec![UMPL2Expr::Application(vec![
                UMPL2Expr::Number(1.0),
                UMPL2Expr::Number(1.0)
            ])]
        )
    }

    #[test]
    fn basic_macro_kleene_test() {
        let parsed = umpl_parse(
            "(defmacro test [(b * a) (display b) *])
        (test 1 2 3 4)",
        )
        .unwrap();
        let expanded = MacroExpander::new().expand(&parsed).unwrap();
        assert_eq!(
            expanded,
            vec![UMPL2Expr::Application(vec![
                UMPL2Expr::Number(1.0),
                UMPL2Expr::Number(1.0)
            ])]
        )
    }
}
