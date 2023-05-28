use crate::{
    interpreter::RuntimeError,
    token::{Literal, Token},
};
use std::collections::HashMap;

pub(crate) struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub(crate) fn new(enclosing: Option<Box<Environment>>) -> Box<Self> {
        Box::new(Self {
            values: HashMap::new(),
            enclosing,
        })
    }
    pub(crate) fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub(crate) fn get(&self, name: &Token) -> Result<Literal, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(literal) => Ok(literal.clone()),
            None => {
                if let Some(parent_env) = &self.enclosing {
                    parent_env.get(name)
                } else {
                    Err(RuntimeError {
                        message: format!("Undefined variable '{}'.", &name.lexeme),
                        token: name.clone(),
                    })
                }
            }
        }
    }

    pub(crate) fn assign(&mut self, name: &Token, value: Literal) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else if let Some(parent_env) = &mut self.enclosing {
            parent_env.assign(name, value)
        } else {
            Err(RuntimeError {
                message: format!("Undefined variable '{}'.", name.lexeme),
                token: name.clone(),
            })
        }
    }
}
