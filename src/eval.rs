use crate::{
    ast::{Expr, ExprKind, State},
    Env,
};

pub fn eval_expr(epr: Expr, vars: Env) -> Expr {
    match epr.expr {
        // case: self-evaluating
        ExprKind::Nil
        | ExprKind::Word(_)
        | ExprKind::Number(_)
        | ExprKind::Bool(_)
        | ExprKind::Lambda(_, _)
        | ExprKind::UserLambda(_, _, _) => epr.initialize(&vars),
        // case: lookup
        ExprKind::Symbol(s) => vars
            .get(&s)
            .unwrap_or_else(|| panic!("Symbol not found: {s}")),
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
        ExprKind::Def(name, lambda) => {
            match &lambda.expr {
                ExprKind::Lambda(_, _) | ExprKind::UserLambda(_, _, _) => {
                    vars.insert(name, (*lambda).initialize(&vars));
                }
                _ => panic!("Not a lambda: {lambda:?}"),
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
        ExprKind::Apply(func, args) => apply(*func, vars, args),
    }
}

pub(crate) fn apply(func: Expr, vars: Env, args: Vec<Expr>) -> Expr {
    let func = eval_expr(func, vars.clone());
    match func.expr {
        ExprKind::Lambda(p, _) => {
            let args = args
                .into_iter()
                .map(|epr| eval_expr(epr, vars.clone()))
                .collect();
            p(args, vars)
        }
        ExprKind::UserLambda(p, params, closure) => {
            let env = closure.unwrap_or_else(|| vars.new_child());
            args.into_iter()
                .map(|epr| eval_expr(epr, vars.clone()))
                .zip(params.into_iter())
                .for_each(|epr| {
                    let (e, p) = epr;
                    env.insert(p, e);
                });
            eval_expr(*p, env)
        }
        // any literal or symbol should be evaluat
        e => panic!("Not a lambda: {e:?}"),
    }
}
