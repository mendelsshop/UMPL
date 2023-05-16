use crate::{
    ast::{Expr, ExprKind, State},
    Env,
};

pub fn eval_expr(epr: Expr, vars: Env) -> Expr {
    match epr.expr {
        // case: self-evaluating
        // if its a lambda we wan't to give it the current environment
        // so the closure properly captures the environment
        ExprKind::Nil
        | ExprKind::Word(_)
        | ExprKind::Number(_)
        | ExprKind::Bool(_)
        | ExprKind::PrimitiveLambda(..)
        | ExprKind::Lambda(_, _, _, _, _) => epr.initialize(&vars),
        // case: lookup
        ExprKind::Symbol(s) => vars
            .get(&s)
            .unwrap_or_else(|| panic!("Symbol not found: `{s}` in {}", vars.backtrace())),
        // case: define variable
        ExprKind::Var(s, i) => {
            let v = eval_expr(*i, vars.clone());
            vars.insert(s, v);
            Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "var.rs".to_string(),
            }
        }
        ExprKind::Begin(exprs) => {
            let mut final_val = Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "begin.rs".to_string(),
            };
            for expr in exprs {
                final_val = eval_expr(expr, vars.clone());
            }
            final_val
        }
        ExprKind::Quote(expr) => *expr,
        ExprKind::Def(name, lambda) => {
            let mut lambda = eval_expr(*lambda, vars.clone());
            match &lambda.expr {
                ExprKind::PrimitiveLambda(..) | ExprKind::Lambda(..) => {
                    lambda.set_name(name.clone());
                    vars.insert(name, lambda);
                }
                _ => panic!("Not a lambda: {lambda} in {}", vars.backtrace()),
            }
            Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "def.rs".to_string(),
            }
        }
        ExprKind::Set(name, expr) => {
            let v = eval_expr(*expr, vars.clone());
            vars.set(name, v);
            Expr {
                expr: ExprKind::Nil,
                state: State::Evaluated,
                file: "set.rs".to_string(),
            }
        }
        // have list of primitives, so that we will only evaluate the arguments of primitives
        // and we can have lazy evaluation for user-defined functions (and possibly some primitives)
        ExprKind::List(args) => apply(args[0].clone(), vars, args[1..].to_vec()),
        ExprKind::If(predicate, consequent, alternative) => {
            if actual_value(*predicate, vars.clone()).expr != ExprKind::Bool(false) {
                eval_expr(*consequent, vars)
            } else {
                eval_expr(*alternative, vars)
            }
        }
    }
}

pub fn apply(func: Expr, vars: Env, args: Vec<Expr>) -> Expr {
    let func = actual_value(func, vars.clone());
    match func.expr {
        ExprKind::Lambda(p, params, closure, mut extra_param, name) => {
            let env = closure.unwrap_or_else(|| vars.clone()).new_child(name);
            let mut params = params.into_iter().peekable();
            let mut args = args
                .into_iter()
                .map(|mut epr| {
                    epr.state = State::Thunk(vars.clone());
                    epr
                })
                .peekable();
            loop {
                match (params.peek(), args.peek()) {
                    (None, None) => break,
                    (None, Some(_)) => {
                        if let Some(extra) = std::mem::take(&mut extra_param) {
                            let val = list_to_cons(&(args.collect::<Vec<_>>()), vars.clone());
                            env.insert(extra, val);
                            break;
                        }
                        panic!("to many parameters given in {}", vars.backtrace())
                    }
                    (Some(_), None) => panic!("to few parameters given in {}", vars.backtrace()),
                    (Some(_), Some(_)) => {
                        env.insert(params.next().unwrap(), args.next().unwrap());
                    }
                }
            }
            if let Some(extra) = extra_param {
                env.insert(
                    extra,
                    Expr {
                        expr: ExprKind::Nil,
                        state: State::Evaluated,
                        file: "extra_param".to_string(),
                    },
                );
            }
            eval_expr(*p, env)
        }
        // then we should evaluate the arguments
        ExprKind::PrimitiveLambda(p, _, _) => p(
            args.into_iter()
                .map(|epr| actual_value(epr, vars.clone()))
                .collect(),
            vars,
        ),
        // any literal or symbol should be evaluat
        e => panic!("Not a lambda: {e} in {}", vars.backtrace()),
    }
}

// used for forcing evaluation of an expression
pub fn actual_value(mut expr: Expr, vars: Env) -> Expr {
    expr = eval_expr(expr, vars);
    expr.eval();
    expr
}

fn list_to_cons(exprs: &[Expr], vars: Env) -> Expr {
    if let Some((first, rest)) = exprs.split_first() {
        // eval list of cons first and list->cons of rest
        eval_expr(
            Expr {
                expr: ExprKind::List(vec![
                    Expr {
                        expr: ExprKind::Symbol("cons".to_string()),
                        state: State::Evaluated,
                        file: "list_to_cons.rs".to_string(),
                    },
                    first.clone(),
                    list_to_cons(rest, vars.clone()),
                ]),
                state: State::Evaluated,
                file: "list_to_cons.rs".to_string(),
            },
            vars,
        )
    } else {
        Expr {
            expr: ExprKind::Nil,
            state: State::Evaluated,
            file: "list_to_cons.rs".to_string(),
        }
    }
}
