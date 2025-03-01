use crate::modules::expression::expr::Expr;
use crate::modules::expression::unop::UnOp;
use crate::modules::prelude::*;
use crate::modules::types::{Type, Typed};
use crate::translate::module::TranslateModule;
use crate::utils::{ParserMetadata, TranslateMetadata};
use heraclitus_compiler::prelude::*;

#[derive(Debug, Clone)]
pub struct Len {
    value: Box<Expr>,
}

impl Typed for Len {
    fn get_type(&self) -> Type {
        Type::Num
    }
}

impl UnOp for Len {
    fn set_expr(&mut self, expr: Expr) {
        self.value = Box::new(expr);
    }

    fn parse_operator(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        token(meta, "len")?;
        Ok(())
    }
}

impl SyntaxModule<ParserMetadata> for Len {
    syntax_name!("Length");

    fn new() -> Self {
        Len {
            value: Box::new(Expr::new()),
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        if !matches!(self.value.get_type(), Type::Text | Type::Array(_)) {
            let msg = self
                .value
                .get_error_message(meta)
                .message("Length can only be applied to text or array types");
            return Err(Failure::Loud(msg));
        }
        Ok(())
    }
}

impl TranslateModule for Len {
    fn translate(&self, meta: &mut TranslateMetadata) -> TranslationFragment {
        let value = self.value.translate(meta);
        let id = meta.gen_value_id();
        let var_fragment = meta.push_stmt_variable_lazy("__length", Some(id), Type::Num, value).set_get_length();
        var_fragment.to_frag()
    }
}

impl DocumentationModule for Len {
    fn document(&self, _meta: &ParserMetadata) -> String {
        "".to_string()
    }
}
