use crate::expr::Expr;
use crate::interpreter::RuntimeError;
use crate::token::Literal;
use crate::token::Token;

pub(crate) enum VisitorReturnOk {
    NoResult,
}

pub(crate) enum VisitorReturnError {
    VRRuntimeErr(RuntimeError),
}

pub(crate) type VisitorReturnResult = Result<VisitorReturnOk, VisitorReturnError>;

pub(crate) trait Stmt {
    fn accept(&self, visitor: &dyn Visitor) -> VisitorReturnResult;
}

pub(crate) trait Visitor {
    fn visit_expression_stmt(&self, expr: &Expression) -> VisitorReturnResult;
    fn visit_print_stmt(&self, expr: &Print) -> VisitorReturnResult;
}

pub(crate) struct Expression {
    pub(crate) expression: Box<dyn Expr>,
}

impl Stmt for Expression {
    fn accept(&self, visitor: &dyn Visitor) -> VisitorReturnResult {
        visitor.visit_expression_stmt(&self)
    }
}

impl Expression {
    pub(crate) fn new(expression: Box<dyn Expr>) -> Box<Self> {
        Box::new(Self { expression })
    }
}

pub(crate) struct Print {
    pub(crate) expression: Box<dyn Expr>,
}

impl Stmt for Print {
    fn accept(&self, visitor: &dyn Visitor) -> VisitorReturnResult {
        visitor.visit_print_stmt(&self)
    }
}

impl Print {
    pub(crate) fn new(expression: Box<dyn Expr>) -> Box<Self> {
        Box::new(Self { expression })
    }
}
