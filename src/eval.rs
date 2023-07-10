use std::collections::HashMap;

use crate::{ast::UMPL2Expr, interior_mut::RC};

#[derive(Default)]
pub struct Eval {
    env: (),
    link_table: HashMap<RC<str>, Vec<RC<str>>>,
    links: HashMap<RC<str>, (Vec<UMPL2Expr>, usize)>,
    being_evaled: Vec<UMPL2Expr>,
}
pub struct EvalError {}

pub enum Stopper {
    Stop(UMPL2Expr),
    Skip,
    Goto(RC<str>),
}
impl Eval {
    pub fn eval(mut input: Vec<UMPL2Expr>) {
        let mut this = Self {
            ..Default::default()
        };
        this.find_links(&mut input);
        this.find_labels(&mut input);
        println!("{:?}", this.links);

        this.run_all(input);
    }
    fn run_all(&mut self, input: Vec<UMPL2Expr>) {
        for expr in input {
            println!("{expr:?}");
            if let Ok(Err(Stopper::Goto(s))) = self.eval_expr(expr) {
                let new_input = self.links.get(&s).unwrap();
                let new_input = new_input.clone().0.into_iter().skip(new_input.1).collect();
                self.run_all(new_input);
                break;
            }
        }
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
                UMPL2Expr::Link(linked, linkers) => {
                    self.link_table.insert(linked.clone(), linkers.clone());
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
            | UMPL2Expr::Label(_)
            | UMPL2Expr::Quoted(_) => {}
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
        });
    }

    fn find_label(&mut self, val: &mut UMPL2Expr) {
        let val_vec = &mut vec![val.clone()];
        self.find_labels(val_vec);
        *val = val_vec.pop().unwrap_or_default();
    }

    fn find_labels(&mut self, input: &mut Vec<UMPL2Expr>) {
        let mut link_table = HashMap::new();
        *input = input
            .iter()
            .enumerate()
            .filter(|(idx, expr)| match expr {
                UMPL2Expr::Label(l) => match self.link_table.get(l) {
                    Some(_) => {
                        self.link_table.remove(l);
                        link_table.insert(l.clone(), *idx);
                        false
                    }
                    None => true,
                },
                _ => true,
            })
            .map(|i| i.1)
            .cloned()
            .collect::<Vec<_>>();
        input.iter_mut().for_each(|expr| match expr {
            UMPL2Expr::Fanction(fanction) => {
                self.find_labels(fanction.scope_mut());
            }
            UMPL2Expr::ContiueDoing(scope) => {
                self.find_labels(scope);
            }
            UMPL2Expr::GoThrough(go_through) => {
                self.find_labels(go_through.scope_mut());
                self.find_label(go_through.iter_mut())
            }
            UMPL2Expr::Until(until) => {
                self.find_labels(until.scope_mut());
                self.find_label(until.cond_mut())
            }
            UMPL2Expr::Unless(unless) => {
                self.find_labels(unless.cons_mut());
                self.find_labels(unless.alt_mut());
                self.find_label(unless.cond_mut())
            }
            UMPL2Expr::If(if_stmt) => {
                self.find_labels(if_stmt.cons_mut());
                self.find_labels(if_stmt.alt_mut());
                self.find_label(if_stmt.cond_mut())
            }
            UMPL2Expr::Application(aplication) => self.find_labels(aplication.args_mut()),
            UMPL2Expr::Stop(s) => {
                self.find_label(s);
            }
            _ => {}
        });
        self.links.extend(
            link_table
                .iter()
                .map(|(k, v)| (k.clone(), (input.clone(), *v))),
        )
    }

    fn eval_expr(&mut self, input: UMPL2Expr) -> Result<Result<UMPL2Expr, Stopper>, EvalError> {
        match input {
            UMPL2Expr::Bool(_)
            | UMPL2Expr::Number(_)
            | UMPL2Expr::Tree(_)
            | UMPL2Expr::String(_)
            | UMPL2Expr::Hempty
            | UMPL2Expr::Skip
            | UMPL2Expr::Fanction(_) => Ok(Ok(input)),
            UMPL2Expr::Scope(_) => unreachable!(),
            UMPL2Expr::Ident(_) => todo!(),
            UMPL2Expr::If(if_stmt) => {
                todo!()
            }
            UMPL2Expr::Unless(_) => todo!(),
            UMPL2Expr::Stop(s) => todo!(),
            UMPL2Expr::Until(until) => todo!(),
            UMPL2Expr::GoThrough(go_through) => todo!(),
            UMPL2Expr::ContiueDoing(_) => todo!(),
            UMPL2Expr::Application(_) => todo!(),
            UMPL2Expr::Quoted(q) => Ok(Ok(*q)),
            UMPL2Expr::FnParam(_) => todo!(),
            UMPL2Expr::Link(_, _) => unreachable!(),
            UMPL2Expr::Label(l) => Ok(Err(Stopper::Goto(l))),
        }
    }

    fn eval_scope(&mut self, input: Vec<UMPL2Expr>) -> Result<Stopper, EvalError> {
        let len = input.len();
        for (idx, expr) in input.into_iter().enumerate() {
            match (idx, expr) {
                (_, UMPL2Expr::Stop(s)) => return Ok(Stopper::Stop(*s)),
                (_, UMPL2Expr::Skip) => return Ok(Stopper::Skip),
                (i, s) if i + 1 == len => return Ok(Stopper::Stop(s)),
                (_, s) => match self.eval_expr(s)? {
                    Ok(_) => {}
                    Err(stopper) => return Ok(stopper),
                },
            };
        }
        Ok(Stopper::Stop(UMPL2Expr::Hempty))
    }
}

#[cfg(test)]
mod eval_tests {
    use crate::lexer::umpl_parse;

    use super::Eval;

    #[test]
    fn find_links() {
        let tree = umpl_parse("link @a @1 @2 55 if ? do ᚜2 6 @1᚛  otherwise ᚜@a 3 5᚛  3").unwrap();
        Eval::eval(tree);
    }
}
