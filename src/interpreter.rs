use std::borrow::Borrow;
use std::rc::Rc;

use crate::expr::{Binary, Expr, Grouping, LiteralExpr, Unary, Visitor, VisitorReturnType};
use crate::token::Literal;
use crate::token_type::TokenType::*;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Interpreter {}
impl Interpreter {
    fn evaluate(&self, expr: &dyn Expr) -> VisitorReturnType {
        expr.accept(self)
    }
}

impl Visitor for Interpreter {
    fn visit_literalexpr_expr(&self, expr: &LiteralExpr) -> VisitorReturnType {
        VisitorReturnType::VRLiteral(expr.value.clone())
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> VisitorReturnType {
        self.evaluate(expr.expression.borrow())
    }
    fn visit_unary_expr(&self, expr: &Unary) -> VisitorReturnType {
        let right = self.evaluate(expr.right.borrow());
        match expr.operator.token_type {
            Minus => VisitorReturnType::wrap_float(-right.get_float_or_panic()),
            Bang => right.get_vrt_bool_or_panic(),
            _ => panic!(), //TODO:
        }
    }

    fn visit_binary_expr(&self, expr: &Binary) -> VisitorReturnType {
        let left = self.evaluate(expr.left.borrow()).get_float_or_panic();
        let right = self.evaluate(expr.right.borrow()).get_float_or_panic();
        match expr.operator.token_type {
            
            Minus => VisitorReturnType::wrap_float(left - right),
            Slash => VisitorReturnType::wrap_float(left / right),
            Star => VisitorReturnType::wrap_float(left * right),
            _ => panic!(), //TODO:
        }
    }
}
