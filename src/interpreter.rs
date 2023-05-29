use crate::environment_stack::EnvironmentStack;
use crate::expr::{
    self, Assign, Binary, Call, Expr, Grouping, LiteralExpr, Logical, Unary, Variable,
};
use crate::lox_callable::LoxCallable;
use crate::stmt::{Block, Expression, Function, If, Print, Stmt, Var, While};
use crate::token::{Literal, Token};
use crate::token_type::TokenType::*;
use crate::{runtime_error, stmt};
use std::borrow::Borrow;
use std::rc::Rc;

pub(crate) type ExprVisitorResult = Result<Literal, RuntimeError>;
type StmtVisitorResult = Result<(), RuntimeError>;

pub(crate) struct Interpreter {
    pub(crate) env_stack: EnvironmentStack,
}
impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {
            env_stack: EnvironmentStack::new(),
        }
    }

    pub(crate) fn define_global(&mut self, name: String, value: Literal) {
        self.env_stack.define_global(name, value);
    }

    fn evaluate(&mut self, expr: &Expr) -> ExprVisitorResult {
        expr.accept(self)
    }

    pub(crate) fn execute(&mut self, stmt: &Stmt) -> StmtVisitorResult {
        stmt.accept(self)
    }

    pub(crate) fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in &statements {
            if let Err(err) = self.execute(statement) {
                runtime_error(&err);
                return Err(err);
            }
        }
        Ok(())
    }
    fn stringify(&self, literal: &Literal) -> String {
        match literal {
            Literal::NoneLiteral => String::from("nil"),
            Literal::BoolLiteral(b) => format!("{}", b),
            Literal::Float(f) => {
                if f.fract() == 0.0 {
                    format!("{}", *f as i32)
                } else {
                    format!("{}", f)
                }
            }
            Literal::StringLiteral(s) => s.clone(),
            Literal::Callable(callable) => callable.stringify().to_string(),
        }
    }
}

pub(crate) struct RuntimeError {
    pub(crate) message: String,
    pub(crate) token: Token,
}

fn construct_error(message: &str, token: &Token) -> ExprVisitorResult {
    Err(RuntimeError {
        message: String::from(message),
        token: token.clone(),
    })
}

fn construct_number_error(token: &Token) -> ExprVisitorResult {
    construct_error("Operands must be numbers.", token)
}

impl expr::Visitor<ExprVisitorResult> for Interpreter {
    fn visit_literalexpr_expr(&mut self, expr: &LiteralExpr) -> ExprVisitorResult {
        Ok(expr.value.clone())
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> ExprVisitorResult {
        self.evaluate(expr.expression.borrow())
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> ExprVisitorResult {
        let right = self.evaluate(expr.right.borrow())?;
        match expr.operator.token_type {
            Minus => {
                if !right.is_float() {
                    construct_number_error(&expr.operator)
                } else {
                    Ok(Literal::wrap_float(-right.unwrap_float()))
                }
            }
            Bang => Ok(right.negate_and_wrap()),
            _ => panic!(), //TODO:
        }
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> ExprVisitorResult {
        let left = self.evaluate(expr.left.borrow())?;
        let right = self.evaluate(expr.right.borrow())?;
        match expr.operator.token_type {
            BangEqual => return Ok(Literal::wrap_bool(!left.is_equal(&right))),
            EqualEqual => return Ok(Literal::wrap_bool(left.is_equal(&right))),
            _ => (),
        }
        if left.is_string() && right.is_string() && expr.operator.token_type == Plus {
            let mut concat_string = String::from(left.unwrap_str_literal());
            concat_string.push_str(right.unwrap_str_literal());
            return Ok(Literal::wrap_string_literal(concat_string));
        }
        if !right.is_float() || !left.is_float() {
            return construct_number_error(&expr.operator);
        }

        let left = left.unwrap_float();
        let right = right.unwrap_float();
        match expr.operator.token_type {
            Minus => Ok(Literal::wrap_float(left - right)),
            Plus => Ok(Literal::wrap_float(left + right)),
            Slash => Ok(Literal::wrap_float(left / right)),
            Star => Ok(Literal::wrap_float(left * right)),
            Greater => Ok(Literal::wrap_bool(left > right)),
            GreaterEqual => Ok(Literal::wrap_bool(left >= right)),
            Less => Ok(Literal::wrap_bool(left < right)),
            LessEqual => Ok(Literal::wrap_bool(left <= right)),
            _ => panic!(), //TODO:
        }
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> ExprVisitorResult {
        self.env_stack.get(&expr.name)
    }

    fn visit_assign_expr(&mut self, expr: &Assign) -> ExprVisitorResult {
        let value = self.evaluate(&expr.value)?;
        self.env_stack.assign(&expr.name, value.clone())?;
        Ok(value)
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> ExprVisitorResult {
        let left = self.evaluate(&expr.left)?;
        if expr.operator.token_type == Or {
            if left.is_truthy() {
                return Ok(left);
            }
        } else {
            if !left.is_truthy() {
                return Ok(left);
            }
        }
        return self.evaluate(&expr.right);
    }

    fn visit_call_expr(&mut self, expr: &Call) -> ExprVisitorResult {
        let callee = self.evaluate(&expr.callee)?;
        let mut arguments = Vec::new();
        for argument in &expr.arguments {
            arguments.push(self.evaluate(argument)?);
        }
        if let Literal::Callable(calleable) = callee {
            if arguments.len() != calleable.arity() {
                return Err(RuntimeError {
                    message: format!(
                        "Expected {}
                     arguments but got {}.",
                        calleable.arity(),
                        arguments.len()
                    ),
                    token: expr.paren.clone(),
                });
            }
            return calleable.call(self, arguments);
        }
        Err(RuntimeError {
            message: "Can only call functions and classes.".to_string(),
            token: expr.paren.clone(),
        })
    }
}

impl stmt::Visitor<StmtVisitorResult> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> StmtVisitorResult {
        self.evaluate(stmt.expression.borrow())?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> StmtVisitorResult {
        let value = self.evaluate(stmt.expression.borrow())?;
        println!("{}", self.stringify(&value));
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> StmtVisitorResult {
        let literal = self.evaluate(stmt.initializer.borrow())?;
        self.env_stack.define(stmt.name.lexeme.clone(), literal);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Block) -> StmtVisitorResult {
        self.env_stack.push_new();
        for statement in &stmt.statements {
            self.execute(statement)?
        }
        self.env_stack.pop();
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> StmtVisitorResult {
        if self.evaluate(&stmt.condition)?.is_truthy() {
            self.execute(&stmt.then_branch)
        } else if let Some(else_stmt) = &stmt.else_branch {
            self.execute(else_stmt)
        } else {
            Ok(())
        }
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> StmtVisitorResult {
        while self.evaluate(&stmt.condition)?.is_truthy() {
            self.execute(&stmt.body)?
        }
        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: Rc<Function>) -> StmtVisitorResult {
        self.env_stack.define(
            stmt.name.lexeme.clone(),
            Literal::Callable(LoxCallable::UserFunction(stmt)),
        );
        Ok(())
    }
}
