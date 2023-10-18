use inkwell::values::{BasicValue, BasicValueEnum, StructValue};

use crate::ast::{FlattenAst, UMPL2Expr};

use super::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn special_form_quote(
        &mut self,
        exprs: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        if exprs.len() != 1 {
            Err("quoted expressiion needs to have one expression")?;
        }
        Ok(Some(exprs[0].clone().flatten(self).as_basic_value_enum()))
    }
    pub fn special_form_unquote(
        _: &mut Compiler<'_, '_>,
        _: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        Err("unquote outside of a quasiquote expression".to_string())
    }
    pub fn special_form_quasiquote(
        &mut self,
        exprs: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        let list = return_none!(exprs
            .iter()
            .map(|expr| {
                if let UMPL2Expr::Application(list) = expr {
                    if list.get(0) == Some(&"unquote".into()) {
                        list.get(1)
                            .map(|expr| self.compile_expr(expr))
                            .ok_or("unquote without expression found")?
                    } else {
                        self.special_form_quasiquote(list)
                    }
                } else {
                    Ok(Some(expr.clone().flatten(self).into()))
                }
            })
            .collect::<Result<Option<Vec<BasicValueEnum<'_>>>, String>>()?);

        let n = list.len();
        let partial_tree = list_to_tree(list, self, n);

        Ok(Some(partial_tree.0.into()))
    }
}
fn list_to_tree<'ctx>(
    list: Vec<BasicValueEnum<'ctx>>,
    compiler: &mut Compiler<'_, 'ctx>,
    n: usize,
) -> (StructValue<'ctx>, Vec<BasicValueEnum<'ctx>>) {
    if n == 0 {
        (compiler.hempty(), list)
    } else {
        let left_size = (n - 1) / 2;
        let (left_tree, mut non_left_tree) = list_to_tree(list, compiler, left_size);

        let this = non_left_tree.remove(0).into_struct_value();

        let right_size = n - (left_size + 1);
        let (right_tree, remaining) = list_to_tree(non_left_tree, compiler, right_size);
        (compiler.const_cons(left_tree, this, right_tree), remaining)
    }
}
