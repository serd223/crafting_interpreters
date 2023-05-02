use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    interpreter::RuntimeError,
    token::{LiteralVal, Token},
};

#[derive(Clone)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Self>>>,
    values: HashMap<String, LiteralVal>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn with_enclosing(enclosing: Rc<RefCell<Self>>) -> Self {
        Self {
            enclosing: Some(enclosing),
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
            if let Some(enclosing) = &self.enclosing {
                return enclosing.borrow().get(name);
            }

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
            if let Some(enclosing) = &mut self.enclosing {
                return enclosing.borrow_mut().assign(name, value);
            }
            Err(RuntimeError(
                name.clone(),
                format!("Undefined variable '{}'", &name.lexeme),
            ))
        }
    }
}
