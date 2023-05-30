use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    environment_tree::{EnvironmentNode, EnvironmentTree},
    interpreter::ExprVisitorResult,
    lox_instance::LoxInstance,
    stmt,
};
use crate::{interpreter::Interpreter, stmt::Function, token::Literal};
#[derive(Clone)]
pub(crate) enum LoxCallable {
    UserFunction(LoxFunction),
    UserClass(Rc<RefCell<LoxClass>>),
    Clock,
}

#[derive(Clone)]
pub(crate) struct LoxFunction {
    pub(crate) declaration: Rc<Function>,
    pub(crate) closure: Rc<RefCell<EnvironmentNode>>,
    pub(crate) is_initializer: bool,
}
impl LoxFunction {
    pub(crate) fn bind(&self, instance: Rc<RefCell<LoxInstance>>) -> Literal {
        let environment = EnvironmentTree::new_environment_node(Some(self.closure.clone()));
        EnvironmentNode::define(
            environment.clone(),
            "this".to_string(),
            Literal::Instance(instance),
        );
        Literal::Callable(LoxCallable::UserFunction(LoxFunction {
            declaration: self.declaration.clone(),
            closure: environment,
            is_initializer: self.is_initializer,
        }))
    }
}
#[derive(Clone)]
pub(crate) struct LoxClass {
    pub(crate) class: Rc<stmt::Class>,
    pub(crate) superclass: Option<Rc<RefCell<LoxClass>>>,
    pub(crate) methods: HashMap<String, Literal>,
}

impl LoxClass {
    pub(crate) fn new(
        class: Rc<stmt::Class>,
        superclass: Option<Rc<RefCell<LoxClass>>>,
        methods: HashMap<String, Literal>,
    ) -> LoxCallable {
        LoxCallable::UserClass(Rc::new(RefCell::new(Self {
            class,
            superclass,
            methods,
        })))
    }
    pub(crate) fn find_method(&self, name: &str) -> Option<LoxFunction> {
        if let Some(Literal::Callable(LoxCallable::UserFunction(method))) = self.find_method_wrapped_in_literal(name) {
            Some(method.clone())
        } else {
            None
        }
    }

    pub(crate) fn find_method_wrapped_in_literal(&self, name: &str) -> Option<Literal> {
        if let Some(literal) = self.methods.get(name) {
            Some(literal.clone())
        } else if let Some(superclass) = &self.superclass {
            if let Some(literal) = superclass.borrow().find_method_wrapped_in_literal(name) {
                Some(literal.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl LoxCallable {
    pub(crate) fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Literal>,
    ) -> ExprVisitorResult {
        match self {
            LoxCallable::Clock => Ok(Literal::Float(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            )),
            LoxCallable::UserFunction(fun) => {
                let new_env_node = EnvironmentTree::new_environment_node(Some(fun.closure.clone()));
                for i in 0..fun.declaration.params.len() {
                    (*new_env_node).borrow_mut().environment.insert(
                        fun.declaration.params[i].lexeme.clone(),
                        arguments[i].clone(),
                    );
                }
                if let Err(err) =
                    interpreter.execute_block(&fun.declaration.body, new_env_node.clone())
                {
                    if err.return_flag {
                        if fun.is_initializer {
                            Ok(fun
                                .closure
                                .borrow()
                                .environment
                                .get("this")
                                .unwrap()
                                .clone())
                        } else {
                            Ok(interpreter.return_value.take().unwrap())
                        }
                    } else {
                        Err(err)
                    }
                } else if fun.is_initializer {
                    Ok(fun
                        .closure
                        .borrow()
                        .environment
                        .get("this")
                        .unwrap()
                        .clone())
                } else {
                    Ok(Literal::NoneLiteral)
                }
            }
            LoxCallable::UserClass(class) => {
                let lox_inst = LoxInstance::new(class.clone());
                let initializer = class.borrow().find_method("init");
                if let Some(lox_function) = initializer {
                    let bounded = lox_function.bind(lox_inst.clone());
                    if let Literal::Callable(calleable) = bounded {
                        return calleable.call(interpreter, arguments);
                    }
                }
                Ok(Literal::Instance(lox_inst))
            }
        }
    }

    pub(crate) fn stringify(&self) -> String {
        match self {
            LoxCallable::Clock => "<native fn>".to_string(),
            LoxCallable::UserFunction(fun) => format!("<fn {}>", &fun.declaration.name.lexeme),
            LoxCallable::UserClass(class) => {
                let mut cur = format!("<cls {}>", &class.borrow().class.name.lexeme);
                if let Some(superclass) = &class.borrow().superclass {
                    cur.push_str(" extends ");
                    cur.push_str(&LoxCallable::UserClass(superclass.clone()).stringify());
                }
                cur
            }
        }
    }

    pub(crate) fn arity(&self) -> usize {
        match self {
            LoxCallable::Clock => 0,
            LoxCallable::UserFunction(fun) => fun.declaration.params.len(),
            LoxCallable::UserClass(class) => {
                let initializer = class.borrow().find_method("init");
                if let Some(lox_function) = initializer {
                    lox_function.declaration.params.len()
                } else {
                    0
                }
            }
        }
    }
}
