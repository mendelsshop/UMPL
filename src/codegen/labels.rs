use inkwell::values::BasicValueEnum;

use crate::{ast::UMPL2Expr, interior_mut::RC};

use super::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    // maybe should be macro (im probably delagateing/delaying to much until macro is implemented)
    pub fn special_form_link(
        &mut self,
        exprs: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        if exprs.len() < 2 {
            return Err("special for link requires at least 2 labels".to_string());
        }
        let labels = exprs
            .iter()
            .map(|expr| {
                if let UMPL2Expr::Label(l) = expr {
                    Ok(l)
                } else {
                    Err("special for link only accepts labels")?
                }
                .cloned()
            })
            .collect::<Result<Vec<RC<str>>, String>>()?;
        let link = &labels[0];
        let linkers = &labels[1..];
        Ok(Some(self.hempty().into()))
    }
}
