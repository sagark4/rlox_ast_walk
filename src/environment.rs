use crate::{
    interpreter::RuntimeError,
    token::{Literal, Token},
};
use std::collections::HashMap;

pub(crate) struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    pub(crate) fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub(crate) fn get(&self, name: &Token) -> Result<Literal, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(literal) => Ok(literal.clone()),
            None => Err(RuntimeError {
                message: format!("Undefined variable '{}'.", &name.lexeme),
                token: name.clone(),
            }),
        }
    }

    pub(crate) fn assign(&mut self, name: &Token, value: Literal) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else {
            Err(RuntimeError {
                message: format!("Undefined variable '{}'.", name.lexeme),
                token: name.clone(),
            })
        }
    }
}
