use crate::{expr::Expr, token::Token};

pub(crate) enum Stmt {
    ExpressionStmt(Box<Expression>),
    PrintStmt(Box<Print>),
    VarStmt(Box<Var>),
    BlockStmt(Box<Block>),
}

impl Stmt {
    pub(crate) fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Stmt::ExpressionStmt(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::PrintStmt(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::VarStmt(stmt) => visitor.visit_var_stmt(stmt),
            Stmt::BlockStmt(stmt) => visitor.visit_block_stmt(stmt),
        }
    }
}
pub(crate) trait Visitor<R> {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> R;
    fn visit_print_stmt(&mut self, stmt: &Print) -> R;
    fn visit_var_stmt(&mut self, stmt: &Var) -> R;
    fn visit_block_stmt(&mut self, stmt: &Block) -> R;
}

pub(crate) struct Expression {
    pub(crate) expression: Expr,
}

impl Expression {
    pub(crate) fn new(expression: Expr) -> Box<Self> {
        Box::new(Self { expression })
    }
}

pub(crate) struct Print {
    pub(crate) expression: Expr,
}

impl Print {
    pub(crate) fn new(expression: Expr) -> Box<Self> {
        Box::new(Self { expression })
    }
}

pub(crate) struct Var {
    pub(crate) name: Token,
    pub(crate) initializer: Expr,
}

impl Var {
    pub(crate) fn new(token: Token, initializer: Expr) -> Box<Self> {
        Box::new(Self {
            name: token,
            initializer,
        })
    }
}

pub(crate) struct Block {
    pub(crate) statements: Vec<Stmt>,
}

impl Block {
    pub(crate) fn new(statements: Vec<Stmt>) -> Box<Self> {
        Box::new(Self { statements })
    }
}
