use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    interpreter::{ExprVisitorResult, RuntimeErrorOrReturn},
    lox_callable::LoxClass,
    token::{Literal, Token},
};

pub(crate) struct LoxInstance {
    class: Rc<RefCell<LoxClass>>,
    fields: HashMap<String, Literal>,
}

impl LoxInstance {
    pub(crate) fn new(class: Rc<RefCell<LoxClass>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            class,
            fields: HashMap::new(),
        }))
    }

    pub(crate) fn stringify(&self) -> String {
        format!("<cls {}> instance", self.class.borrow().class.name.lexeme).to_string()
    }

    pub(crate) fn get(
        &self,
        self_rc_rfc: Rc<RefCell<LoxInstance>>,
        name: &Token,
    ) -> ExprVisitorResult {
        if self.fields.contains_key(&name.lexeme) {
            Ok(self.fields.get(&name.lexeme).unwrap().clone())
        } else if let Some(method) = (*self.class).borrow().find_method(&name.lexeme) {
            Ok(method.bind(self_rc_rfc))
        } else {
            Err(RuntimeErrorOrReturn {
                message: format!("Undefined property '{}'.", &name.lexeme),
                token: name.clone(),
                return_flag: false,
            })
        }
    }

    pub(crate) fn set(&mut self, name: &Token, value: Literal) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}
