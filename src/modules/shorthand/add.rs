use heraclitus_compiler::prelude::*;
use crate::docs::module::DocumentationModule;
use crate::error_type_match;
use crate::modules::expression::expr::Expr;
use crate::modules::variable::{variable_name_extensions, handle_variable_reference};
use crate::translate::compute::translate_computation_eval;
use crate::utils::{ParserMetadata, TranslateMetadata};
use crate::translate::{module::TranslateModule, compute::{ArithOp, translate_computation}};
use crate::modules::types::{Type, Typed};

#[derive(Debug, Clone)]
pub struct ShorthandAdd {
    var: String,
    expr: Box<Expr>,
    kind: Type,
    global_id: Option<usize>,
    is_ref: bool
}

impl SyntaxModule<ParserMetadata> for ShorthandAdd {
    syntax_name!("Shorthand Add");

    fn new() -> Self {
        Self {
            var: String::new(),
            expr: Box::new(Expr::new()),
            kind: Type::Null,
            global_id: None,
            is_ref: false
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        let var_tok = meta.get_current_token();
        self.var = variable(meta, variable_name_extensions())?;
        token(meta, "+=")?;
        let variable = handle_variable_reference(meta, var_tok, &self.var)?;
        self.kind = variable.kind;
        self.global_id = variable.global_id;
        self.is_ref = variable.is_ref;
        syntax(meta, &mut *self.expr)?;
        if self.kind != self.expr.get_type() || !matches!(self.kind, Type::Num | Type::Text | Type::Array(_)) {
            let msg = self.expr.get_error_message(meta);
            return error_type_match!(meta, msg, "add", self.expr, [Num, Text, Array]);
        }
        Ok(())
    }
}

impl TranslateModule for ShorthandAdd {
    fn translate(&self, meta: &mut TranslateMetadata) -> String {
        let expr = self.is_ref
            .then(|| self.expr.translate_eval(meta, true))
            .unwrap_or_else(|| self.expr.translate(meta));
        let name: String = match self.global_id {
            Some(id) => format!("__{id}_{}", self.var),
            None => if self.is_ref { format!("${{{}}}", self.var) } else { self.var.clone() }
        };
        let stmt = match self.kind {
            Type::Text => format!("{}+={}", name, expr),
            Type::Array(_) => format!("{}+=({})", name, expr),
            _ => {
                let var = if self.is_ref { format!("\\${{{name}}}") } else { format!("${{{name}}}") };
                let translated_computation = if self.is_ref {
                    translate_computation_eval(meta, ArithOp::Add, Some(var), Some(expr))
                } else {
                    translate_computation(meta, ArithOp::Add, Some(var), Some(expr))
                };
                format!("{}={}", name, translated_computation)
            }
        };
        if self.is_ref {
            format!("eval \"{}\"", stmt)
        } else {
            stmt
        }
    }
}

impl DocumentationModule for ShorthandAdd {
    fn document(&self, _meta: &ParserMetadata) -> String {
        "".to_string()
    }
}
