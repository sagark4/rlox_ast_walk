use std::rc::Rc;

use crate::token::Literal;
use crate::token::Token;

pub(crate) enum VisitorReturnType {
    VRString(String)
}
pub(crate) trait Expr {
    fn accept(&self, visitor: Rc<dyn Visitor>) -> VisitorReturnType;
}

pub(crate) trait Visitor {
    fn visit_binary_expr(&self, expr: &Binary) -> VisitorReturnType;
    fn visit_grouping_expr(&self, expr: &Grouping) -> VisitorReturnType;
    fn visit_literalexpr_expr(&self, expr: &LiteralExpr) -> VisitorReturnType;
    fn visit_unary_expr(&self, expr: &Unary) -> VisitorReturnType;
}

pub(crate) struct Binary {
    pub(crate) left: Rc<dyn Expr>,
    pub(crate) operator: Token,
    pub(crate) right: Rc<dyn Expr>,
}

impl Expr for Binary {
    fn accept(&self, visitor: Rc<dyn Visitor>) -> VisitorReturnType {
        visitor.visit_binary_expr(&self)
    }
}

impl Binary {
    pub(crate) fn new(left: Rc<dyn Expr>, operator: Token, right: Rc<dyn Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

pub(crate) struct Grouping {
    pub(crate) expression: Rc<dyn Expr>,
}

impl Expr for Grouping {
    fn accept(&self, visitor: Rc<dyn Visitor>) ->  VisitorReturnType{
        visitor.visit_grouping_expr(&self)
    }
}

impl Grouping {
    pub(crate) fn new(expression: Rc<dyn Expr>) -> Self {
        Self { expression }
    }
}

pub(crate) struct LiteralExpr {
    pub(crate) value: Literal,
}

impl Expr for LiteralExpr {
    fn accept(&self, visitor: Rc<dyn Visitor>) -> VisitorReturnType {
        visitor.visit_literalexpr_expr(&self)
    }
}

impl LiteralExpr {
    pub(crate) fn new(value: Literal) -> Self {
        Self { value }
    }
}

pub(crate) struct Unary {
    pub(crate) operator: Token,
    pub(crate) right: Rc<dyn Expr>,
}

impl Expr for Unary {
    fn accept(&self, visitor: Rc<dyn Visitor>) -> VisitorReturnType {
        visitor.visit_unary_expr(&self)
    }
}

impl Unary {
    pub(crate) fn new(operator: Token, right: Rc<dyn Expr>) -> Self {
        Self { operator, right }
    }
}

