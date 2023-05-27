use crate::expr::{Expr, VisitorReturnResult};
pub(crate) trait Stmt {
    fn accept(&self, visitor: &dyn Visitor) -> VisitorReturnResult;
}

pub(crate) trait Visitor {
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> VisitorReturnResult;
    fn visit_print_stmt(&self, expr: &PrintStmt) -> VisitorReturnResult;
}

pub(crate) struct ExpressionStmt {
    pub(crate) expression: Box<dyn Expr>,
}

impl Stmt for ExpressionStmt {
    fn accept(&self, visitor: &dyn Visitor) -> VisitorReturnResult {
        visitor.visit_expression_stmt(&self)
    }
}

impl ExpressionStmt {
    pub(crate) fn new(expression: Box<dyn Expr>) -> Box<Self> {
        Box::new(Self { expression })
    }
}

pub(crate) struct PrintStmt {
    pub(crate) expression: Box<dyn Expr>,
}

impl Stmt for PrintStmt {
    fn accept(&self, visitor: &dyn Visitor) -> VisitorReturnResult {
        visitor.visit_print_stmt(&self)
    }
}

impl PrintStmt {
    pub(crate) fn new(expression: Box<dyn Expr>) -> Box<Self> {
        Box::new(Self { expression })
    }
}
