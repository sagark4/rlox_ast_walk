use crate::expr::{
    Binary, Expr, Grouping, LiteralExpr, Unary, Visitor, VisitorReturnError, VisitorReturnOk,
    VisitorReturnResult,
};
use crate::token::{Literal, Token};
use crate::token_type::TokenType::*;
use std::borrow::Borrow;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Interpreter {}
impl Interpreter {
    fn evaluate(&self, expr: &dyn Expr) -> VisitorReturnResult {
        expr.accept(self)
    }
    pub(crate) fn interpret(&self, expr: &dyn Expr) -> Result<(), RuntimeError> {
        match expr.accept(self) {
            Ok(VisitorReturnOk::VRLiteral(literal)) => {
                println!("{}", self.stringify(&literal));
                Ok(())
            }
            Err(VisitorReturnError::VRRuntimeErr(err)) => {
                crate::runtime_error(&err);
                Err(err)
            }
            _ => panic!(),
        }
    }
    fn stringify(&self, literal: &Literal) -> String {
        match literal {
            Literal::NoneLiteral => String::from("nil"),
            Literal::BoolLiteral(b) => format!("{}", b),
            Literal::Float(f) => {
                if f.fract() == 0.0 {
                    format!("{}", *f as i32)
                } else {
                    format!("{}", f)
                }
            }
            Literal::StringLiteral(s) => s.clone(),
        }
    }
}

pub(crate) struct RuntimeError {
    pub(crate) message: String,
    pub(crate) token: Token,
}

fn construct_error(message: &str, token: &Token) -> VisitorReturnResult {
    Err(VisitorReturnError::VRRuntimeErr(RuntimeError {
        message: String::from(message),
        token: token.clone(),
    }))
}

fn construct_number_error(token: &Token) -> VisitorReturnResult {
    construct_error("Operands must be numbers.", token)
}

impl Visitor for Interpreter {
    fn visit_literalexpr_expr(&self, expr: &LiteralExpr) -> VisitorReturnResult {
        Ok(VisitorReturnOk::VRLiteral(expr.value.clone()))
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> VisitorReturnResult {
        self.evaluate(expr.expression.borrow())
    }

    fn visit_unary_expr(&self, expr: &Unary) -> VisitorReturnResult {
        let right = self.evaluate(expr.right.borrow())?;
        match expr.operator.token_type {
            Minus => {
                if !right.is_float() {
                    construct_number_error(&expr.operator)
                } else {
                    Ok(VisitorReturnOk::wrap_float(-right.unwrap_float()))
                }
            }
            Bang => Ok(right.unwrap_negate_and_wrap_vrl_bool()),
            _ => panic!(), //TODO:
        }
    }

    fn visit_binary_expr(&self, expr: &Binary) -> VisitorReturnResult {
        let left = self.evaluate(expr.left.borrow())?;
        let right = self.evaluate(expr.right.borrow())?;
        match expr.operator.token_type {
            BangEqual => return Ok(VisitorReturnOk::wrap_bool(!left.is_vrl_equal_or_panic(&right))),
            EqualEqual => return Ok(VisitorReturnOk::wrap_bool(left.is_vrl_equal_or_panic(&right))),
            _ => (),
        }
        if left.is_string() && right.is_string() && expr.operator.token_type == Plus {
            let mut concat_string = String::from(left.unwrap_str_literal());
            concat_string.push_str(right.unwrap_str_literal());
            return Ok(VisitorReturnOk::wrap_string_literal(concat_string));
        }
        if !right.is_float() || !left.is_float() {
            return construct_number_error(&expr.operator);
        }

        let left = left.unwrap_float();
        let right = right.unwrap_float();
        match expr.operator.token_type {
            Minus => Ok(VisitorReturnOk::wrap_float(left - right)),
            Plus => Ok(VisitorReturnOk::wrap_float(left + right)),
            Slash => Ok(VisitorReturnOk::wrap_float(left / right)),
            Star => Ok(VisitorReturnOk::wrap_float(left * right)),
            Greater => Ok(VisitorReturnOk::wrap_bool(left > right)),
            GreaterEqual => Ok(VisitorReturnOk::wrap_bool(left >= right)),
            Less => Ok(VisitorReturnOk::wrap_bool(left < right)),
            LessEqual => Ok(VisitorReturnOk::wrap_bool(left <= right)),
            _ => panic!(), //TODO:
        }
    }
}
