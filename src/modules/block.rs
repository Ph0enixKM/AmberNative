use std::collections::VecDeque;

use heraclitus_compiler::prelude::*;
use crate::docs::module::DocumentationModule;
use crate::utils::{metadata::ParserMetadata, TranslateMetadata};
use crate::translate::module::TranslateModule;
use super::statement::stmt::Statement;

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>
}

impl Block {
    // Get whether this block is empty
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }

    // Push a parsed statement into the block
    pub fn push_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }

    // Check for compile time issues 
    pub fn check(&self) {

    }
}

impl SyntaxModule<ParserMetadata> for Block {
    syntax_name!("Block");

    fn new() -> Self {
        Block {
            statements: vec![]
        }
    }

    fn parse(&mut self, meta: &mut ParserMetadata) -> SyntaxResult {
        meta.push_scope();
        while let Some(token) = meta.get_current_token() {
            // Handle the end of line or command
            if ["\n", ";"].contains(&token.word.as_str()) {
                meta.increment_index();
                continue;
            }
            // Handle block end
            else if token.word == "}" {
                break;
            }
            let mut statemant = Statement::new();
            if let Err(failure) = statemant.parse(meta) {
                return match failure {
                    Failure::Quiet(pos) => error_pos!(meta, pos, "Unexpected token"),
                    Failure::Loud(err) => return Err(Failure::Loud(err))
                }
            }
            self.statements.push(statemant);
        }
        for var in meta.get_all_vars() {
            if var.is_empty().unwrap() {
                return error_pos!(meta, var.declared_at.clone(), "Variable declared but not assigned!");
            }
        }
        meta.pop_scope();
        Ok(())
    }
}

impl TranslateModule for Block {
    fn translate(&self, meta: &mut TranslateMetadata) -> String {
        // Save the current statement queue and create a new one
        let mut new_queue = VecDeque::new();
        std::mem::swap(&mut meta.stmt_queue, &mut new_queue);
        meta.increase_indent();
        let result = if self.is_empty() {
            ":".to_string()
        }
        else {
            self.statements.iter()
                .map(|statement| statement.translate(meta))
                .filter(|translation| !translation.trim().is_empty())
                .collect::<Vec<_>>().join("\n")
        };
        meta.decrease_indent();
        // Restore the old statement queue
        std::mem::swap(&mut meta.stmt_queue, &mut new_queue);
        result
    }
}

impl DocumentationModule for Block {
    fn document(&self, meta: &ParserMetadata) -> String {
        self.statements.iter()
            .map(|statement| statement.document(meta))
            .collect::<Vec<_>>().join("")
    }
}
