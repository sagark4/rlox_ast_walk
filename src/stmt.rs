use crate::{
    expr::{Expr, LiteralExpr, VisitorReturnResult},
    token::{Literal, Token},
};
pub(crate) trait Stmt {
    fn accept(&self, visitor: &mut dyn Visitor) -> VisitorReturnResult;
}

pub(crate) trait Visitor {
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> VisitorReturnResult;
    fn visit_print_stmt(&self, expr: &PrintStmt) -> VisitorReturnResult;
    fn visit_var_stmt(&mut self, expr: &VarStmt) -> VisitorReturnResult;
}

pub(crate) struct ExpressionStmt {
    pub(crate) expression: Box<dyn Expr>,
}

impl Stmt for ExpressionStmt {
    fn accept(&self, visitor: &mut dyn Visitor) -> VisitorReturnResult {
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
    fn accept(&self, visitor: &mut dyn Visitor) -> VisitorReturnResult {
        visitor.visit_print_stmt(&self)
    }
}

impl PrintStmt {
    pub(crate) fn new(expression: Box<dyn Expr>) -> Box<Self> {
        Box::new(Self { expression })
    }
}

pub(crate) struct VarStmt {
    pub(crate) name: Token,
    pub(crate) initializer: Box<dyn Expr>,
}

impl Stmt for VarStmt {
    fn accept(&self, visitor: &mut dyn Visitor) -> VisitorReturnResult {
        visitor.visit_var_stmt(&self)
    }
}

impl VarStmt {
    pub(crate) fn new(token: Token, initializer: Box<dyn Expr>) -> Box<Self> {
        Box::new(Self { name: token, initializer })
    }
    pub(crate) fn new_nil_initialized(token: Token) -> Box<Self> {
        Box::new(Self {
            name: token,
            initializer: LiteralExpr::new(Literal::NoneLiteral),
        })
    }
}
