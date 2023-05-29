use std::time::{SystemTime, UNIX_EPOCH};

use crate::{interpreter::Interpreter, token::Literal};

#[derive(Clone, Debug)]
pub(crate) enum LoxCallable {
    //UserDefFunction(Rc<TODO: define when function declaration is defined
    Clock,
}

impl LoxCallable {
    pub(crate) fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Literal>) -> Literal {
        match self {
            LoxCallable::Clock => Literal::Float(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            ),
        }
    }

    pub(crate) fn stringify(&self) -> &str {
        match self {
            LoxCallable::Clock => "<native fn>",
        }
    }

    pub(crate) fn arity(&self) -> usize {
        0
    }
}
