use std::{
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::interpreter::ExprVisitorResult;
use crate::{interpreter::Interpreter, stmt::Function, token::Literal};
#[derive(Clone)]
pub(crate) enum LoxCallable {
    UserFunction(Rc<Function>),
    Clock,
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
                interpreter.env_stack.push_with_parent_id(0);
                for i in 0..fun.params.len() {
                    interpreter
                        .env_stack
                        .define(fun.params[i].lexeme.clone(), arguments[i].clone());
                }
                for statement in &fun.body {
                    if let Err(err) = interpreter.execute(statement) {
                        if err.return_flag {
                            interpreter.env_stack.pop();
                            return Ok(interpreter.return_value.take().unwrap());
                        }
                    }
                }
                interpreter.env_stack.pop();
                Ok(Literal::NoneLiteral)
            }
        }
    }

    pub(crate) fn stringify(&self) -> String {
        match self {
            LoxCallable::Clock => "<native fn>".to_string(),
            LoxCallable::UserFunction(fun) => format!("<fn {}>", &fun.name.lexeme),
        }
    }

    pub(crate) fn arity(&self) -> usize {
        match self {
            LoxCallable::Clock => 0,
            LoxCallable::UserFunction(fun) => fun.params.len(),
        }
    }
}
