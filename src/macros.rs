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

// TODO more builtins an cleanup better error handling removing explicit panics
use std::collections::HashMap;

use itertools::Itertools;

use crate::{ast::UMPL2Expr, interior_mut::RC, lexer::umpl_parse};

#[derive(Debug)]
pub enum ParseError<'a> {
    Macro(MacroError),
    Parse(super::pc::ParseError<'a>),
}

pub fn parse_and_expand(input: &str) -> Result<Vec<UMPL2Expr>, ParseError<'_>> {
    let ast = umpl_parse(input).map_err(ParseError::Parse)?;
    let mut expander = MacroExpander::new();
    let expanded = expander.expand(&ast).map_err(ParseError::Macro)?;
    Ok(expanded)
}

trait HashMapExtend {
    fn push_nested(&mut self, nested: Self);
    fn merge(&mut self, other: Self);
    fn get(&self, key: &RC<str>) -> Result<&UMPL2Expr, MacroExpansionError>;
}

impl HashMapExtend for HashMap<RC<str>, MacroBinding> {
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

    fn merge(&mut self, other: Self) {
        // rn just just extend but might not be that simple
        self.extend(other);
    }

    fn get(&self, key: &RC<str>) -> Result<&UMPL2Expr, MacroExpansionError> {
        let b = self
            .get(key)
            .ok_or(MacroExpansionError::MetaVariableNotFound(key.clone()))?;

        match b {
            MacroBinding::Expr(it) => Ok(it),
            MacroBinding::List(_) => Err(MacroExpansionError::DepthAndVariableDepthNoMatch),
        }
    }
}

#[derive(Debug)]
pub enum MacroError {
    InvalidForm(RC<str>),
    InvalidMacroCase,
    NoMacroCases,
    InvalidMacroCondition,
    NoMacroForms,
    MacroExpansion(MacroExpansionError),
}

#[derive(Debug, Clone)]
pub enum MacroExpansionError {
    MetaVariableNotFound(RC<str>),
    KleeneClosureWithoutNestedBindings,
    DepthAndVariableDepthNoMatch,
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
                                    let expander = cases
                                        .into_iter()
                                        .find_map(|(case, res)| {
                                            // case.
                                            let MacroArg::List(case) = case else {
                                                unreachable!()
                                            };
                                            matches(&a[1..], case)
                                                .map(|bindings| (bindings, res))
                                                .ok()
                                        })
                                        .unwrap();
                                    res.extend(
                                        expand_macro(&expander.0, expander.1.to_vec())
                                            .map_err(|e| (MacroError::MacroExpansion(e)))?,
                                    );
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
}
fn expand_macro(
    bindings: &HashMap<RC<str>, MacroBinding>,
    expansion: Vec<UMPL2Expr>,
) -> Result<Vec<UMPL2Expr>, MacroExpansionError> {
    fn expand_macro_inner(
        bindings: &HashMap<RC<str>, MacroBinding>,
        expansion: UMPL2Expr,
    ) -> Result<UMPL2Expr, MacroExpansionError> {
        match expansion {
            UMPL2Expr::Application(a) => expand_macro(bindings, a).map(UMPL2Expr::Application),
            UMPL2Expr::Scope(s) => expand_macro(bindings, s).map(UMPL2Expr::Scope),
            UMPL2Expr::Ident(i) => match HashMapExtend::get(bindings, &i) {
                Ok(res) => Ok(res.clone()),
                Err(MacroExpansionError::MetaVariableNotFound(_)) => Ok(UMPL2Expr::Ident(i)),
                Err(e) => Err(e),
            },
            other => Ok(other),
        }
    }
    let mut res = Vec::new();
    let mut expansion = expansion.into_iter().peekable();
    while let Some(expander) = expansion.next() {
        // TODO: handle more than one kleene expansion ie (a * *)
        if expansion.peek() == Some(&UMPL2Expr::Ident("*".into())) {
            expansion.next(); // swallow kleene closure

            let mut expander = vec![expander];
            while expansion.peek() == Some(&UMPL2Expr::Ident("*".into())) {
                expander.push(expansion.next().unwrap())
            }
            let expander = expander;

            // instead we need to know how many expressions connected to the idents in expansion and loop that many times
            // after each kleene closure is found remove a level for each binding which is a list
            // to make this work in reality after we hit kleene closure we check if there are any bindings that are lists otherwise error
            // we then iterate n times where n is the length of the list of a binding such that the list is the longest of all the binding lists
            // during each iteration we create new bindings by going through the binding and
            // if binding is expression in new bindings binding will same expression
            // if list if nth element of binding is present then insert nth element of binding as binding in new bindings binding otherwise do not bind in new bindings binding

            // still has problems when were at a depth in expansion where there is kleene closure but the binding for kleene closure is not one being used
            // like             "(defmacro test [((c * b ) * a) (display b ) * *  ])"
            let largest = bindings
                .into_iter()
                .filter_map(|binding| {
                    if let MacroBinding::List(l) = binding.1 {
                        Some(l.len())
                    } else {
                        None
                    }
                })
                .max()
                .ok_or(MacroExpansionError::KleeneClosureWithoutNestedBindings)?;

            let (errs, oks): (Vec<_>, Vec<_>) = (0..largest)
                .map(move |n| {
                    let bindings: HashMap<_, _> = bindings
                        .clone()
                        .into_iter()
                        .filter_map(|binding| {
                            if let MacroBinding::List(l) = binding.1 {
                                Some((binding.0, l.get(n)?.clone()))
                            } else {
                                Some(binding)
                            }
                        })
                        .collect();
                    expand_macro(&bindings, expander.clone())
                })
                .partition_map(From::from);
            if oks.is_empty() {
                return Err(errs[0].clone());
            }
            res.extend(oks.into_iter().flatten());
        } else {
            res.push(expand_macro_inner(bindings, expander)?)
        }
    }

    Ok(res)
}

// special forms
impl MacroExpander {
    fn special_form_link(&mut self, exprs: &[UMPL2Expr]) -> Result<Vec<UMPL2Expr>, MacroError> {
        let Some(UMPL2Expr::Label(linked)) = exprs.get(0) else {
            return Err(MacroError::InvalidForm(
                "first expr of link must be a label".into(),
            ));
        };
        let links = if let Some(links) = exprs.get(1..) {
            links
                .iter()
                .map(|expr| match expr {
                    UMPL2Expr::Label(l) => Some(l.clone()),
                    _ => None,
                })
                .collect::<Option<Vec<RC<str>>>>()
                .ok_or(MacroError::InvalidForm(
                    "the elements of the link are not all labels".into(),
                ))?
        } else {
            return Err(MacroError::InvalidForm(
                "there must at least one other expr in a link".into(),
            ));
        };
        self.links.insert(linked.clone(), links);
        Ok(vec![])
    }

    fn special_form_macro(&mut self, exprs: &[UMPL2Expr]) -> Result<Vec<UMPL2Expr>, MacroError> {
        let Some(UMPL2Expr::Ident(macro_name)) = exprs.get(0) else {
            return Err(MacroError::InvalidForm(
                "defmacro expects a name for the macro".into(),
            ));
        };
        if let Some(cases) = exprs.get(1..) {
            let cases = MacroType::UserDefined(
                cases
                    .into_iter()
                    .map(|case| {
                        let UMPL2Expr::Application(rule) = case else {
                            return Err(MacroError::InvalidMacroCase);
                        };

                        let Some(UMPL2Expr::Application(case)) = rule.first() else {
                            return Err(MacroError::InvalidMacroCase);
                        };
                        let cond: MacroArg = case.as_slice().try_into()?;

                        let expand = rule.get(1..).ok_or(MacroError::NoMacroForms)?.to_vec();
                        Ok((cond, expand))
                        // TODO: parse the expansion part
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            );
            self.macro_env.insert(macro_name.clone(), cases);
            Ok(vec![])
        } else {
            Err(MacroError::NoMacroCases)
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
        if let Some(MacroArg::KleeneClosure(_)) = ret.first() {
            if ret
                .iter()
                .skip(1)
                .find(|arg| matches!(arg, MacroArg::KleeneClosure(_)))
                .is_some()
            {
                Err(MacroError::InvalidMacroCondition)
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
                    Err(MacroError::InvalidMacroCondition)
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
#[derive(Debug, Clone)]
pub enum MacroBinding {
    // generated from matching *
    List(Vec<MacroBinding>),
    Expr(UMPL2Expr),
}

#[derive(Debug)]
pub enum MacroMatchError {}

impl MacroArg {
    fn matches(&self, pattern: &UMPL2Expr) -> Result<HashMap<RC<str>, MacroBinding>, MacroError> {
        let mut bindings = HashMap::new();
        match (self, pattern) {
            (MacroArg::List(pattern), UMPL2Expr::Application(expr)) => {
                bindings.merge(matches(expr, pattern)?);
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

fn matches(
    expr: &[UMPL2Expr],
    pattern: &[MacroArg],
) -> Result<HashMap<RC<str>, MacroBinding>, MacroError> {
    let mut expr_count = expr.len();
    let mut pat_count = pattern.len();
    let mut bindings = HashMap::new();
    let mut expr = expr.into_iter().peekable();
    for pat in pattern {
        match pat {
            MacroArg::List(pat) => {
                let Some(UMPL2Expr::Application(expr)) = expr.next() else {
                    panic!()
                };
                bindings.merge(matches(expr, pat)?);
                expr_count -= 1;
            }
            MacroArg::Ident(i) => {
                let expr = expr.next().unwrap();
                bindings.insert(i.clone(), MacroBinding::Expr(expr.clone()));
                expr_count -= 1;
            }
            MacroArg::Constant(c) => {
                let Some(UMPL2Expr::Ident(expr)) = expr.next() else {
                    panic!()
                };
                if c != expr {
                    panic!()
                }
                expr_count -= 1;
            }
            MacroArg::KleeneClosure(c) => {
                let taken_count = expr_count.checked_sub(pat_count - 1).unwrap_or(expr_count);
                expr_count -= taken_count;
                let matched = expr.clone().take(taken_count);

                expr.nth(taken_count - 1);

                if let Some(c) = c {
                    for expr in matched {
                        bindings.push_nested(c.matches(&expr)?);
                    }
                }
            }
        }
        pat_count -= 1;
    }
    if expr.len() != 0 {
        panic!()
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
            vec![
                UMPL2Expr::Application(vec![
                    UMPL2Expr::Ident("display".into()),
                    UMPL2Expr::Number(1.0)
                ]),
                UMPL2Expr::Application(vec![
                    UMPL2Expr::Ident("display".into()),
                    UMPL2Expr::Number(2.0)
                ]),
                UMPL2Expr::Application(vec![
                    UMPL2Expr::Ident("display".into()),
                    UMPL2Expr::Number(3.0)
                ]),
            ]
        )
    }

    #[test]
    fn basic_macro_kleene_list_test() {
        let parsed = umpl_parse(
            "(defmacro test [((c * b ) * a) (display c ) * *  ])
        (test (1 4 5) (4 7 8 ) (4 7 7 ) 6) ",
        )
        .unwrap();
        let expanded = MacroExpander::new().expand(&parsed).unwrap();
        assert_eq!(
            expanded,
            vec![
                UMPL2Expr::Application(vec![
                    UMPL2Expr::Ident("display".into()),
                    UMPL2Expr::Number(5.0)
                ]),
                UMPL2Expr::Application(vec![
                    UMPL2Expr::Ident("display".into()),
                    UMPL2Expr::Number(8.0)
                ]),
                UMPL2Expr::Application(vec![
                    UMPL2Expr::Ident("display".into()),
                    UMPL2Expr::Number(7.0)
                ]),
            ]
        )
    }

    #[test]
    fn basic_macro_kleene_ignore_test() {
        let parsed = umpl_parse(
            "(defmacro test [(* a) (display a)])
        (test (1 4 5) (4 7 8 ) (4 7 8 ) 6)",
        )
        .unwrap();
        let expanded = MacroExpander::new().expand(&parsed).unwrap();
        assert_eq!(
            expanded,
            vec![UMPL2Expr::Application(vec![
                UMPL2Expr::Ident("display".into()),
                UMPL2Expr::Number(6.0)
            ]),]
        )
    }

    #[test]
    fn complexish_macro_kleene_test() {
        let parsed = umpl_parse(
            "(defmacro test [((c .ret. b * a ) * q) (display a ) *])
        (test (1 ret 4 y v 5) (4 ret 7 u  8 ) (4 ret 7 8 )  1)",
        )
        .unwrap();
        let expanded = MacroExpander::new().expand(&parsed).unwrap();
        assert_eq!(
            expanded,
            vec![UMPL2Expr::Application(vec![
                UMPL2Expr::Ident("display".into()),
                UMPL2Expr::Number(6.0)
            ]),]
        )
    }
}
