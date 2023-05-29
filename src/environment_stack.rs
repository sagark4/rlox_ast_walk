use crate::{
    interpreter::RuntimeError,
    token::{Literal, Token},
};
use std::collections::HashMap;

pub(crate) struct EnvironmentStack {
    stack: Vec<HashMap<String, Literal>>,
}

impl EnvironmentStack {
    pub(crate) fn new() -> Self {
        Self {
            stack: vec![HashMap::new()],
        }
    }

    pub(crate) fn define(&mut self, name: String, value: Literal) {
        self.stack.last_mut().unwrap().insert(name, value);
    }

    pub(crate) fn define_global(&mut self, name: String, value: Literal) {
        self.stack.first_mut().unwrap().insert(name, value);
    }

    pub(crate) fn get(&self, name: &Token) -> Result<Literal, RuntimeError> {
        for env in self.stack.iter().rev() {
            if let Some(literal) = env.get(&name.lexeme) {
                return Ok(literal.clone());
            }
        }

        return Err(RuntimeError {
            message: format!("Undefined variable '{}'.", &name.lexeme),
            token: name.clone(),
        });
    }

    pub(crate) fn assign(&mut self, name: &Token, value: Literal) -> Result<(), RuntimeError> {
        for env in self.stack.iter_mut().rev() {
            if env.contains_key(&name.lexeme) {
                env.insert(name.lexeme.clone(), value);
                return Ok(());
            }
        }
        return Err(RuntimeError {
            message: format!("Undefined variable '{}'.", name.lexeme),
            token: name.clone(),
        });
    }

    pub(crate) fn push_new(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop();
    }
}
