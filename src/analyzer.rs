use std::collections::HashSet;

use crate::{ast::UMPL2Expr, interior_mut::RC};

#[derive(Default, Debug)]
pub struct Analyzer {
    link_set: HashSet<RC<str>>, 
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
        (this.link_set, input)
    }

    fn find_link(&mut self, val: &mut UMPL2Expr) {
        let val_vec = &mut vec![val.clone()];
        self.find_links(val_vec);
        *val = val_vec.pop().unwrap_or_default();
    }

    fn find_links(&mut self, input: &mut Vec<UMPL2Expr>) {
        *input = input
            .iter()
            .filter(|expr| match expr {
                UMPL2Expr::Link(linked, _) => {
                    self.link_set.insert(linked.clone());
                    false
                }
                _ => true,
            })
            .cloned()
            .collect();
        input.iter_mut().for_each(|expr| match expr {
            UMPL2Expr::Bool(_)
            | UMPL2Expr::Number(_)
            | UMPL2Expr::String(_)
            | UMPL2Expr::Ident(_)
            | UMPL2Expr::FnParam(_)
            | UMPL2Expr::Hempty
            | UMPL2Expr::Link(_, _)
            | UMPL2Expr::Scope(_)
            | UMPL2Expr::Skip
            | UMPL2Expr::Label(_) => {}
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
            UMPL2Expr::FnKW(_) => todo!(),
            UMPL2Expr::Let(_, v) => self.find_link(v),
        });
    }
}

