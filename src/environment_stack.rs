use crate::{
    interpreter::RuntimeErrorOrReturn,
    token::{Literal, Token},
};
use std::collections::HashMap;

pub(crate) struct EnvironmentStack {
    stack: Vec<HashMap<String, Literal>>,
    parent_id: Vec<usize>, //actually, it's parent id plus one so I can use usize and range
}

impl EnvironmentStack {
    pub(crate) fn new() -> Self {
        Self {
            stack: vec![HashMap::new()],
            parent_id: vec![0usize],
        }
    }

    pub(crate) fn define(&mut self, name: String, value: Literal) {
        self.stack.last_mut().unwrap().insert(name, value);
    }

    pub(crate) fn define_global(&mut self, name: String, value: Literal) {
        self.stack.first_mut().unwrap().insert(name, value);
    }

    pub(crate) fn get(&self, name: &Token) -> Result<Literal, RuntimeErrorOrReturn> {
        if let Some(literal) = self.stack.last().unwrap().get(&name.lexeme) {
            return Ok(literal.clone());
        }
        for i in (0..*self.parent_id.last().unwrap()).rev() {
            let env = &self.stack[i];
            if let Some(literal) = env.get(&name.lexeme) {
                return Ok(literal.clone());
            }
        }

        return Err(RuntimeErrorOrReturn {
            message: format!("Undefined variable '{}'.", &name.lexeme),
            token: name.clone(),
            return_flag: false,
        });
    }

    pub(crate) fn assign(
        &mut self,
        name: &Token,
        value: Literal,
    ) -> Result<(), RuntimeErrorOrReturn> {
        let env = self.stack.last_mut().unwrap();
        if env.contains_key(&name.lexeme) {
            env.insert(name.lexeme.clone(), value);
            return Ok(());
        }
        for i in (0..*self.parent_id.last().unwrap()).rev() {
            let env = &mut self.stack[i];
            if env.contains_key(&name.lexeme) {
                env.insert(name.lexeme.clone(), value);
                return Ok(());
            }
        }
        return Err(RuntimeErrorOrReturn {
            message: format!("Undefined variable '{}'.", name.lexeme),
            token: name.clone(),
            return_flag: false,
        });
    }

    pub(crate) fn push_new(&mut self) {
        self.parent_id.push(self.stack.len()); //default is enclosing, i.e., previous
        self.stack.push(HashMap::new());
    }

    pub(crate) fn push_with_parent_id(&mut self, parent_id: usize) {
        self.parent_id.push(parent_id + 1);
        self.stack.push(HashMap::new());
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop();
        self.parent_id.pop();
    }
}
