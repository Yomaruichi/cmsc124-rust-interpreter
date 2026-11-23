use std::collections::HashMap;

use crate::evaluator::RuntimeError;
use crate::token::{Token, Value};

/// Each environment owns its local bindings and its enclosing environment.
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { values: HashMap::new(), enclosing: None }
    }

    pub fn enclosed(enclosing: Environment) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    // Only a block environment has a parent.
    pub fn into_enclosing(self) -> Environment {
        *self.enclosing.expect("block environment must have an enclosing scope")
    }

    // A declaration becomes visible after its initializer succeeds.
    // Redeclaring a name replaces its value in the current environment.
    pub fn define(&mut self, name: &Token, value: Value) {
        self.values.insert(name.lexeme.clone(), value);
    }

    pub fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.get(name),
                None => Err(RuntimeError::new(name, &format!("Undefined variable '{}'.", name.lexeme))),
            },
        }
    }

    // Assignment updates an existing binding and never creates one.
    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), RuntimeError> {
        match self.values.get_mut(&name.lexeme) {
            Some(binding) => {
                *binding = value;
                Ok(())
            }
            None => match &mut self.enclosing {
                Some(enclosing) => enclosing.assign(name, value),
                None => Err(RuntimeError::new(name, &format!("Undefined variable '{}'.", name.lexeme))),
            },
        }
    }
}
