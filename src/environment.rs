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
    pub(crate) fn get<'a> (&'a self, name: &Token) -> Result<&'a Literal, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(literal) => Ok(literal),
            None => {
                let mut message = String::from("Undefined variable '");
                message.push_str(&name.lexeme);
                message.push_str("'.");
                Err(RuntimeError {
                    message,
                    token: name.clone(),
                })
            }
        }
    }
}
