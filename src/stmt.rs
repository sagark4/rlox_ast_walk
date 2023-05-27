use crate::{expr::Expr, token::Token};

pub(crate) enum Stmt {
    ExpressionStmt(Box<Expression>),
    PrintStmt(Box<Print>),
    VarStmt(Box<Var>),
}

impl Stmt {
    pub(crate) fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Stmt::ExpressionStmt(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::PrintStmt(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::VarStmt(stmt) => visitor.visit_var_stmt(stmt),
        }
    }
}
pub(crate) trait Visitor<R> {
    fn visit_expression_stmt(&self, stmt: &Expression) -> R;
    fn visit_print_stmt(&self, stmt: &Print) -> R;
    fn visit_var_stmt(&mut self, stmt: &Var) -> R;
}

pub(crate) struct Expression {
    pub(crate) expression: Box<Expr>,
}

impl Expression {
    pub(crate) fn new(expression: Box<Expr>) -> Box<Self> {
        Box::new(Self { expression })
    }
}

pub(crate) struct Print {
    pub(crate) expression: Box<Expr>,
}

impl Print {
    pub(crate) fn new(expression: Box<Expr>) -> Box<Self> {
        Box::new(Self { expression })
    }
}

pub(crate) struct Var {
    pub(crate) name: Token,
    pub(crate) initializer: Box<Expr>,
}

impl Var {
    pub(crate) fn new(token: Token, initializer: Box<Expr>) -> Box<Self> {
        Box::new(Self { name: token, initializer })
    }
}
