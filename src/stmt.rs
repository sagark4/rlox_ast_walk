use std::rc::Rc;

use crate::{expr::Expr, token::Token};

pub(crate) enum Stmt {
    ExpressionStmt(Box<Expression>),
    PrintStmt(Box<Print>),
    VarStmt(Box<Var>),
    BlockStmt(Box<Block>),
    IfStmt(Box<If>),
    WhileStmt(Box<While>),
    FunctionStmt(Rc<Function>),
}

impl Stmt {
    pub(crate) fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Stmt::ExpressionStmt(stmt) => visitor.visit_expression_stmt(stmt),
            Stmt::PrintStmt(stmt) => visitor.visit_print_stmt(stmt),
            Stmt::VarStmt(stmt) => visitor.visit_var_stmt(stmt),
            Stmt::BlockStmt(stmt) => visitor.visit_block_stmt(stmt),
            Stmt::IfStmt(stmt) => visitor.visit_if_stmt(stmt),
            Stmt::WhileStmt(stmt) => visitor.visit_while_stmt(stmt),
            Stmt::FunctionStmt(stmt) => visitor.visit_function_stmt(stmt.clone()),
        }
    }
}
pub(crate) trait Visitor<R> {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> R;
    fn visit_print_stmt(&mut self, stmt: &Print) -> R;
    fn visit_var_stmt(&mut self, stmt: &Var) -> R;
    fn visit_block_stmt(&mut self, stmt: &Block) -> R;
    fn visit_if_stmt(&mut self, stmt: &If) -> R;
    fn visit_while_stmt(&mut self, stmt: &While) -> R;
    fn visit_function_stmt(&mut self, stmt: Rc<Function>) -> R;
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

pub(crate) struct If {
    pub(crate) condition: Expr,
    pub(crate) then_branch: Stmt,
    pub(crate) else_branch: Option<Stmt>,
}

impl If {
    pub(crate) fn new(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Box<Self> {
        Box::new(Self {
            condition,
            then_branch,
            else_branch,
        })
    }
}

pub(crate) struct While {
    pub(crate) condition: Expr,
    pub(crate) body: Stmt,
}

impl While {
    pub(crate) fn new(condition: Expr, body: Stmt) -> Box<Self> {
        Box::new(Self { condition, body })
    }
}

pub(crate) struct Function {
    pub(crate) name: Token,
    pub(crate) params: Vec<Token>,
    pub(crate) body: Vec<Stmt>,
}

impl Function {
    pub(crate) fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Rc<Self> {
        Rc::new(Self { name, params, body })
    }
}
