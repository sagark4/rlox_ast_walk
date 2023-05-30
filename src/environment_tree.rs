use crate::{
    interpreter::{ExprVisitorResult, RuntimeErrorOrReturn},
    token::{Literal, Token},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub(crate) struct EnvironmentNode {
    pub(crate) environment: HashMap<String, Literal>,
    pub(crate) parent: Option<Rc<RefCell<EnvironmentNode>>>,
}

impl EnvironmentNode {
    pub(crate) fn define(node: Rc<RefCell<EnvironmentNode>>, key: String, value: Literal) {
        (*node).borrow_mut().environment.insert(key, value);
    }
}

pub(crate) struct EnvironmentTree {
    root: Rc<RefCell<EnvironmentNode>>,
    current: Rc<RefCell<EnvironmentNode>>,
    id_steps_map: HashMap<usize, usize>,
}

impl EnvironmentTree {
    pub(crate) fn new() -> Rc<RefCell<Self>> {
        let tmp = Self::new_environment_node(None);
        Rc::new(RefCell::new(Self {
            root: tmp.clone(),
            current: tmp.clone(),
            id_steps_map: HashMap::new(),
        }))
    }

    pub(crate) fn define_current(&self, name: String, value: Literal) {
        self.current.borrow_mut().environment.insert(name, value);
    }

    pub(crate) fn define_global(&mut self, name: String, value: Literal) {
        self.current.borrow_mut().environment.insert(name, value);
    }

    pub(crate) fn get(&self, token: &Token, expr_id: usize) -> ExprVisitorResult {
        if let Some(steps) = self.id_steps_map.get(&expr_id) {
            return self.get_at(&token.lexeme, *steps);
        }

        if let Some(value) = (*self.root).borrow().environment.get(&token.lexeme) {
            return Ok(value.clone());
        }

        return Err(RuntimeErrorOrReturn {
            message: format!("Undefined variable '{}'.", token.lexeme),
            token: token.clone(),
            return_flag: false,
        });
    }

    pub(crate) fn get_at(&self, key: &str, steps: usize) -> ExprVisitorResult {
        let mut current = self.current.clone();
        for _ in 0..steps {
            let tmp = (*current).borrow().parent.clone().unwrap();
            current = tmp;
        }
        return Ok(current
            .as_ref()
            .borrow()
            .environment
            .get(key)
            .unwrap()
            .clone());
    }

    pub(crate) fn assign(
        &mut self,
        token: &Token,
        value: Literal,
        expr_id: usize,
    ) -> Result<(), RuntimeErrorOrReturn> {
        if let Some(steps) = self.id_steps_map.get(&expr_id) {
            let mut current = self.current.clone();
            for _ in 0..*steps {
                let tmp = (*current).borrow().parent.clone().unwrap();
                current = tmp;
            }
            let env = &mut current.borrow_mut().environment;
            env.insert(token.lexeme.clone(), value);
            return Ok(());
        }
        let env = &mut self.root.borrow_mut().environment;
        if !env.contains_key(&token.lexeme) {
            return Err(RuntimeErrorOrReturn {
                message: format!("Undefined variable '{}'.", token.lexeme),
                token: token.clone(),
                return_flag: false,
            });
        } else {
            env.insert(token.lexeme.clone(), value);
            return Ok(());
        }
    }

    pub(crate) fn get_current(&self) -> Rc<RefCell<EnvironmentNode>> {
        return self.current.clone();
    }

    pub(crate) fn set_current(&mut self, current: Rc<RefCell<EnvironmentNode>>) {
        self.current = current;
    }

    // pub(crate) fn get_global(&self) -> Rc<RefCell<EnvironmentNode>> {
    //     return self.root.clone();
    // }

    pub(crate) fn set_step_for_id(&mut self, id: usize, steps: usize) {
        self.id_steps_map.insert(id, steps);
    }

    pub(crate) fn get_step_for_id(&self, id: usize) -> Option<&usize>{
        self.id_steps_map.get(&id)
    }

    pub(crate) fn new_environment_node(
        parent: Option<Rc<RefCell<EnvironmentNode>>>,
    ) -> Rc<RefCell<EnvironmentNode>> {
        Rc::new(RefCell::new(EnvironmentNode {
            environment: HashMap::new(),
            parent,
        }))
    }

    pub(crate) fn assign_cur_or_par(
        &mut self,
        token: &Token,
        value: Literal,
    ) -> Result<(), RuntimeErrorOrReturn> {
        let env = &mut self.current.borrow_mut().environment;
        if env.contains_key(&token.lexeme) {
            env.insert(token.lexeme.clone(), value);
            return Ok(());
        }
        if let Some(par) = &mut (*self.current).borrow_mut().parent {
            let env = &mut par.borrow_mut().environment;
            if env.contains_key(&token.lexeme) {
                env.insert(token.lexeme.clone(), value);
                return Ok(());
            }
        }

        return Err(RuntimeErrorOrReturn {
            message: format!("Undefined variable '{}'.", token.lexeme),
            token: token.clone(),
            return_flag: false,
        });
    }
}
