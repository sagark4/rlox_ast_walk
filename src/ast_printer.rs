use std::rc::Rc;

use crate::expr::Visitor;
use crate::expr::VisitorReturnType::*;
use crate::expr::*;
use crate::token::Literal::NoneLiteral;
#[derive(Copy, Clone, Debug)]
pub(crate) struct AstPrinter {}

impl AstPrinter {
    pub(crate) fn print(&self, expr: Rc<dyn Expr>) -> String {
        match expr.accept(Rc::new(*self)) {
            VRString(s) => s,
            _ => panic!(),
        }
    }
    // I cannot put these two expressions in a vector because they are not object safe; so I have to prepare as many functions as there are children expressions!
    fn parenthesize_two(&self, name: &str, lexpr: Rc<dyn Expr>, rexpr: Rc<dyn Expr>) -> String {
        let mut builder = String::new();
        builder.push('(');
        match lexpr.accept(Rc::new(*self)) {
            VRString(s) => builder.push_str(&s),
            _ => (),
        }
        builder.push(' ');
        builder.push_str(name);
        builder.push(' ');
        match rexpr.accept(Rc::new(*self)) {
            VRString(s) => builder.push_str(&s),
            _ => (),
        }
        builder.push(')');
        builder
    }

    fn parenthesize_one(&self, name: &str, expr: Rc<dyn Expr>) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        match expr.accept(Rc::new(*self)) {
            VRString(s) => builder.push_str(&s),
            _ => (),
        }
        builder.push(')');
        builder
    }
}

impl Visitor for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary) -> VisitorReturnType {
        VisitorReturnType::VRString(self.parenthesize_two(
            &expr.operator.lexeme,
            Rc::clone(&expr.left),
            Rc::clone(&expr.right),
        ))
    }
    fn visit_grouping_expr(&self, expr: &Grouping) -> VisitorReturnType {
        VisitorReturnType::VRString(self.parenthesize_one("group ", Rc::clone(&expr.expression)))
    }
    fn visit_literalexpr_expr(&self, expr: &LiteralExpr) -> VisitorReturnType {
        match expr.value {
            NoneLiteral => VisitorReturnType::VRString(String::from("nil")),
            _ => VisitorReturnType::VRString(format!("{:?}", expr.value)),
        }
    }
    fn visit_unary_expr(&self, expr: &Unary) -> VisitorReturnType {
        VisitorReturnType::VRString(self.parenthesize_one(&expr.operator.lexeme, Rc::clone(&expr.right)))
    }
}
