use std::collections::{HashMap, HashSet};

use crate::{ast::UMPL2Expr, interior_mut::RC};

#[derive(Default, Debug)]
pub struct Analyzer {
    link_set: HashMap<RC<str>, Vec<RC<str>>>,
}
#[derive(Debug)]
pub struct EvalError {}

impl Analyzer {
    pub fn analyze(input: &[UMPL2Expr]) -> (HashSet<RC<str>>, Vec<UMPL2Expr>) {
        let mut this = Self {
            ..Default::default()
        };
        let mut input = input.to_vec();
        this.find_links(&mut input);
        this.find_labels(&mut input);
        (this.link_set.into_keys().collect(), input)
    }

    /// finds links insert them in the list of links and replaces them with null
    fn find_link(&mut self, val: &mut UMPL2Expr) {
        match val {
            UMPL2Expr::Bool(_)
            | UMPL2Expr::Number(_)
            | UMPL2Expr::String(_)
            | UMPL2Expr::Ident(_)
            | UMPL2Expr::FnParam(_)
            | UMPL2Expr::Hempty
            | UMPL2Expr::FnKW(_)
            // even thougth scope contains more expressions it should not make it into the ast so we just ignore it (arguabbly we should put an unreachable)
            | UMPL2Expr::Scope(_)
            | UMPL2Expr::Skip
            | UMPL2Expr::Label(_) => {}

            UMPL2Expr::Link(label, jump_from) => {
                self.link_set.insert(label.clone(), jump_from.to_vec());
                *val = UMPL2Expr::Hempty
            }
            UMPL2Expr::Quoted(v) => self.find_link(v),
            UMPL2Expr::Tree(_) => unreachable!(),
            UMPL2Expr::Application(aplication) => self.find_links(aplication.args_mut()),
            UMPL2Expr::Stop(s) => {
                self.find_link(s);
            }
            UMPL2Expr::If(if_stmt) => {
                self.find_links(if_stmt.cons_mut());
                self.find_links(if_stmt.alt_mut());
                self.find_link(if_stmt.cond_mut())
            }
            UMPL2Expr::Unless(unless) => {
                self.find_links(unless.cons_mut());
                self.find_links(unless.alt_mut());
                self.find_link(unless.cond_mut())
            }

            UMPL2Expr::Until(until) => {
                self.find_links(until.scope_mut());
                self.find_link(until.cond_mut())
            }
            UMPL2Expr::GoThrough(go_through) => {
                self.find_links(go_through.scope_mut());
                self.find_link(go_through.iter_mut())
            }
            UMPL2Expr::ContiueDoing(scope) => {
                self.find_links(scope);
            }
            UMPL2Expr::Fanction(fanction) => {
                self.find_links(fanction.scope_mut());
            }
            UMPL2Expr::Let(_, v) => self.find_link(v),
            UMPL2Expr::ComeTo(_) => {}
        }
    }

    fn find_links(&mut self, input: &mut [UMPL2Expr]) {
        input.iter_mut().for_each(|expr| self.find_link(expr));
    }

    /// find_label* methods replace any item in a link stmt label list with the corresponding label
    /// and any link first labels get replaced with the cometo stmt
    fn find_label(&mut self, input: &mut UMPL2Expr) {
        match input {
            UMPL2Expr::Bool(_)
            | UMPL2Expr::Number(_)
            | UMPL2Expr::String(_)
            | UMPL2Expr::Ident(_)
            | UMPL2Expr::FnParam(_)
            | UMPL2Expr::Hempty
            | UMPL2Expr::FnKW(_)
            // even thougth scope contains more expressions it should not make it into the ast so we just ignore it (arguabbly we should put an unreachable)
            | UMPL2Expr::Scope(_)
            | UMPL2Expr::Skip  => {}
            | UMPL2Expr::Label(label) => {
                if let Some(link) = self.link_set.iter().find_map(|link|
                    if link.0 == label {
                        Some(UMPL2Expr::ComeTo(label.clone()))
                    } else if link.1.contains(label) {
                        Some(UMPL2Expr::Label(link.0.clone()))
                    } else {
                        None
                    }
                ) {
                    *input = link
                }
            }

            UMPL2Expr::Link(_, _) => {
              unreachable!()
            }
            UMPL2Expr::Quoted(v) => self.find_label(v),
            UMPL2Expr::Tree(_) => unreachable!(),
            UMPL2Expr::Application(aplication) => self.find_labels(aplication.args_mut()),
            UMPL2Expr::Stop(s) => {
                self.find_label(s);
            }
            UMPL2Expr::If(if_stmt) => {
                self.find_labels(if_stmt.cons_mut());
                self.find_labels(if_stmt.alt_mut());
                self.find_label(if_stmt.cond_mut())
            }
            UMPL2Expr::Unless(unless) => {
                self.find_labels(unless.cons_mut());
                self.find_labels(unless.alt_mut());
                self.find_label(unless.cond_mut())
            }

            UMPL2Expr::Until(until) => {
                self.find_labels(until.scope_mut());
                self.find_label(until.cond_mut())
            }
            UMPL2Expr::GoThrough(go_through) => {
                self.find_labels(go_through.scope_mut());
                self.find_label(go_through.iter_mut())
            }
            UMPL2Expr::ContiueDoing(scope) => {
                self.find_labels(scope);
            }
            UMPL2Expr::Fanction(fanction) => {
                self.find_labels(fanction.scope_mut());
            }
            UMPL2Expr::Let(_, v) => self.find_link(v),
            UMPL2Expr::ComeTo(_) => todo!(),
        }
    }

    fn find_labels(&mut self, input: &mut [UMPL2Expr]) {
        input.iter_mut().for_each(|expr| self.find_label(expr));
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{lexer::umpl_parse, analyzer::Analyzer};

    #[test]
    fn find_links() {
        let program = umpl_parse("
        @a
        link @foo @bar @baz
        (add 1 2)<
        if & do ᚜link @set @get᚛ otherwise ᚜4᚛
        @set
        @c
        (f link @a @b)<
        @b
        ").unwrap();
        println!("{:?}", program);
        let (analyzed, program) = Analyzer::analyze(&program);
        assert_eq!(analyzed, HashSet::from_iter(vec!["foo".into(), "set".into(), "a".into()]));
        println!("{:?}", program);
    }
}