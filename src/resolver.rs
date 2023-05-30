use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::environment_tree::EnvironmentTree;
use crate::error_with_token;
use crate::expr::Expr::VariableExpr;
use crate::expr::{
    self, Assign, Binary, Call, Expr, Get, Grouping, LiteralExpr, Logical, Set, Unary, Variable,
};
use crate::stmt::{self, Block, Class, Expression, Function, If, Print, Return, Stmt, Var, While};
use crate::token::{Literal, Token};
#[derive(Copy, Clone)]
enum FunctionType {
    NotFun,
    Fun,
    Method,
    Initializer,
}
#[derive(Copy, Clone)]
enum ClassType {
    NotClass,
    Class,
    Subclass,
}
pub(crate) struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
    env: Rc<RefCell<EnvironmentTree>>,
    current_function: FunctionType,
    current_class: ClassType,
}

impl Resolver {
    pub(crate) fn new(env: Rc<RefCell<EnvironmentTree>>) -> Self {
        Self {
            scopes: Vec::new(),
            env,
            current_function: FunctionType::NotFun,
            current_class: ClassType::NotClass,
        }
    }

    pub(crate) fn resolve(&mut self, statements: &[Stmt]) {
        for statement in statements {
            self.resolve_statement(statement);
        }
    }

    fn insert_step_for_id(&self, id: usize, steps: usize) {
        self.env.borrow_mut().set_step_for_id(id, steps);
    }

    fn resolve_statement(&mut self, statement: &Stmt) {
        statement.accept(self);
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        expr.accept(self);
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if !self.scopes.is_empty() {
            let map = self.scopes.last_mut().unwrap();
            if map.contains_key(&name.lexeme) {
                error_with_token(name, "Already a variable with this name in this scope.");
            } else {
                map.insert(name.lexeme.to_string(), false);
            }
        }
    }

    fn define(&mut self, name: &Token) {
        if !self.scopes.is_empty() {
            self.scopes
                .last_mut()
                .unwrap()
                .insert(name.lexeme.to_string(), true);
        }
    }

    fn resolve_local(&mut self, name: &Token) -> Option<usize> {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name.lexeme) {
                return Some(self.scopes.len() - 1 - i);
            }
        }
        None
    }

    fn resolve_function(&mut self, function: &Function, function_type: FunctionType) {
        let enclosing_function = self.current_function;
        self.current_function = function_type;
        self.begin_scope();
        for param in &function.params {
            self.declare(param);
            self.define(param);
        }
        self.resolve(&function.body);
        self.end_scope();
        self.current_function = enclosing_function;
    }
}

impl expr::Visitor<()> for Resolver {
    fn visit_binary_expr(&mut self, expr: &Binary) {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
    }
    fn visit_grouping_expr(&mut self, expr: &Grouping) {
        self.resolve_expr(&expr.expression);
    }
    fn visit_literalexpr_expr(&mut self, _expr: &LiteralExpr) {
        ()
    }
    fn visit_unary_expr(&mut self, expr: &Unary) {
        self.resolve_expr(&expr.right);
    }
    fn visit_variable_expr(&mut self, expr: &Variable) {
        if !self.scopes.is_empty() {
            if let Some(value) = self.scopes.last().unwrap().get(&expr.name.lexeme) {
                if *value == false {
                    crate::error_with_token(
                        &expr.name,
                        "Can't read local variable in its own initializer.",
                    );
                }
            }
        }
        let index = self.resolve_local(&expr.name);
        if let Some(step) = index {
            self.insert_step_for_id(expr.id, step);
        }
    }
    fn visit_assign_expr(&mut self, expr: &Assign) {
        self.resolve_expr(&expr.value);
        let index = self.resolve_local(&expr.name);
        if let Some(step) = index {
            self.insert_step_for_id(expr.id, step);
        }
    }
    fn visit_logical_expr(&mut self, expr: &Logical) {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
    }
    fn visit_call_expr(&mut self, expr: &Call) {
        self.resolve_expr(&expr.callee);
        for argument in &expr.arguments {
            self.resolve_expr(argument);
        }
    }
    fn visit_get_expr(&mut self, expr: &Get) {
        self.resolve_expr(&expr.object);
    }
    fn visit_set_expr(&mut self, expr: &Set) {
        self.resolve_expr(&expr.value);
        self.resolve_expr(&expr.object);
    }
    fn visit_this_expr(&mut self, expr: &expr::This) -> () {
        if let ClassType::NotClass = self.current_class {
            error_with_token(&expr.keyword, "Can't use 'this' outside of a class.");
            return;
        }
        let index = self.resolve_local(&expr.keyword);
        if let Some(step) = index {
            self.insert_step_for_id(expr.id, step);
        }
    }
    fn visit_super_expr(&mut self, expr: &expr::Super) -> () {
        match self.current_class {
            ClassType::NotClass => {
                error_with_token(&expr.keyword, "Can't use 'super' outside of a class.")
            }
            ClassType::Class => error_with_token(
                &expr.keyword,
                "Can't use 'super' in a class with no superclass.",
            ),
            ClassType::Subclass => (),
        }
        let index = self.resolve_local(&expr.keyword);
        if let Some(step) = index {
            self.insert_step_for_id(expr.id, step);
        }
    }
}

impl stmt::Visitor<()> for Resolver {
    fn visit_expression_stmt(&mut self, stmt: &Expression) {
        self.resolve_expr(&stmt.expression);
    }
    fn visit_print_stmt(&mut self, stmt: &Print) {
        self.resolve_expr(&stmt.expression)
    }
    fn visit_var_stmt(&mut self, stmt: &Var) {
        self.declare(&stmt.name);
        self.resolve_expr(&stmt.initializer);
        self.define(&stmt.name);
    }
    fn visit_block_stmt(&mut self, stmt: &Block) {
        self.begin_scope();
        self.resolve(&stmt.statements);
        self.end_scope();
    }
    fn visit_if_stmt(&mut self, stmt: &If) {
        self.resolve_expr(&stmt.condition);
        self.resolve_statement(&stmt.then_branch);
        if let Some(else_branch) = &stmt.else_branch {
            self.resolve_statement(else_branch);
        }
    }
    fn visit_while_stmt(&mut self, stmt: &While) {
        self.resolve_expr(&stmt.condition);
        self.resolve_statement(&stmt.body);
    }
    fn visit_function_stmt(&mut self, stmt: Rc<Function>) {
        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.resolve_function(&stmt, FunctionType::Fun);
    }
    fn visit_return_stmt(&mut self, stmt: &Return) {
        match self.current_function {
            FunctionType::NotFun => {
                error_with_token(&stmt.keyword, "Can't return from top-level code.")
            }
            FunctionType::Initializer => {
                if let Expr::LiteralExprExpr(lee) = &stmt.value {
                    if let Literal::NoneLiteral = lee.value {
                    } else {
                        error_with_token(
                            &stmt.keyword,
                            "Can't return a value from an initializer.",
                        );
                    }
                }
            }
            _ => (),
        }

        self.resolve_expr(&stmt.value);
    }
    fn visit_class_stmt(&mut self, stmt: Rc<Class>) {
        let enclosing_class = self.current_class;
        self.current_class = ClassType::Class;
        self.declare(&stmt.name);
        self.define(&stmt.name);
        if let Some(superclass) = stmt.superclass.clone() {
            self.current_class = ClassType::Subclass;
            if stmt.name.lexeme == superclass.name.lexeme {
                error_with_token(&superclass.name, "A class can't inherit from itself.");
            }
            self.resolve_expr(&VariableExpr(superclass));
        }
        if let Some(_) = stmt.superclass {
            self.begin_scope();
            self.scopes
                .last_mut()
                .unwrap()
                .insert("super".to_string(), true);
        }
        self.begin_scope();
        self.scopes
            .last_mut()
            .unwrap()
            .insert("this".to_string(), true);
        for method in &stmt.methods {
            if method.name.lexeme == "init" {
                self.resolve_function(method, FunctionType::Initializer);
            } else {
                self.resolve_function(method, FunctionType::Method);
            }
        }
        self.end_scope();
        if let Some(_) = stmt.superclass {
            self.end_scope();
        }

        self.current_class = enclosing_class;
    }
}
