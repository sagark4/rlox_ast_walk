use std::borrow::Borrow;

use crate::expr::Visitor;
use crate::expr::VisitorReturnOk::*;
use crate::expr::*;
use crate::token::Literal::NoneLiteral;
#[derive(Copy, Clone, Debug)]
pub(crate) struct AstPrinter {}

impl AstPrinter {
    pub(crate) fn print(&self, expr: &dyn Expr) -> String {
        match expr.accept(self) {
            Ok(VRString(s)) => s,
            _ => panic!(),
        }
    }
    // I cannot put these two expressions in a vector because they are not object safe; so I have to prepare as many functions as there are children expressions!
    fn parenthesize_two(&self, name: &str, lexpr: &dyn Expr, rexpr: &dyn Expr) -> String {
        let mut builder = String::new();
        builder.push('(');
        match lexpr.accept(self) {
            Ok(VRString(s)) => builder.push_str(&s),
            _ => (),
        }
        builder.push(' ');
        builder.push_str(name);
        builder.push(' ');
        match rexpr.accept(self) {
            Ok(VRString(s)) => builder.push_str(&s),
            _ => (),
        }
        builder.push(')');
        builder
    }

    fn parenthesize_one(&self, name: &str, expr: &dyn Expr) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        match expr.accept(self) {
            Ok(VRString(s)) => builder.push_str(&s),
            _ => (),
        }
        builder.push(')');
        builder
    }
}

impl Visitor for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary) -> VisitorReturnResult {
        Ok(VRString(self.parenthesize_two(
            &expr.operator.lexeme,
            expr.left.borrow(),
            expr.right.borrow(),
        )))
    }
    fn visit_grouping_expr(&self, expr: &Grouping) -> VisitorReturnResult {
        Ok(VRString(
            self.parenthesize_one("group ", expr.expression.borrow()),
        ))
    }
    fn visit_literalexpr_expr(&self, expr: &LiteralExpr) -> VisitorReturnResult {
        match expr.value {
            NoneLiteral => Ok(VRString(String::from("nil"))),
            _ => Ok(VRString(format!("{:?}", expr.value))),
        }
    }
    fn visit_unary_expr(&self, expr: &Unary) -> VisitorReturnResult {
        Ok(VRString(self.parenthesize_one(
            &expr.operator.lexeme,
            expr.right.borrow(),
        )))
    }
}
