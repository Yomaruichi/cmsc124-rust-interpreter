use std::collections::HashMap;

use crate::evaluator::RuntimeError;
use crate::token::{Token, Value};

/// Week 2 bindings live for the file execution or the entire REPL session.
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { values: HashMap::new() }
    }

    // A declaration becomes visible after its initializer succeeds.
    // Redeclaring a name replaces its value in the current environment.
    pub fn define(&mut self, name: &Token, value: Value) {
        self.values.insert(name.lexeme.clone(), value);
    }

    pub fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(RuntimeError::new(name, &format!("Undefined variable '{}'.", name.lexeme))),
        }
    }

    // Assignment updates an existing binding and never creates one.
    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        match self.values.get_mut(&name.lexeme) {
            Some(binding) => {
                *binding = value;
                Ok(())
            }
            None => Err(RuntimeError::new(name, &format!("Undefined variable '{}'.", name.lexeme))),
        }
    }
}
