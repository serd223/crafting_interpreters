use std::collections::HashMap;

use crate::{
    interpreter::RuntimeError,
    token::{LiteralVal, Token},
};

pub struct Environment {
    values: HashMap<String, LiteralVal>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralVal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<LiteralVal, RuntimeError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else {
            Err(RuntimeError(
                name.clone(),
                format!("Undefined variable '{}'.", &name.lexeme),
            ))
        }
    }

    pub fn assign(&mut self, name: &Token, value: LiteralVal) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else {
            Err(RuntimeError(
                name.clone(),
                format!("Undefined variable '{}'", &name.lexeme),
            ))
        }
    }
}
