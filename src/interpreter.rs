use crate::environment::Environment;
use crate::expr::{self, Assign, Binary, Expr, Grouping, LiteralExpr, Unary, Variable};
use crate::stmt::{Block, Expression, Print, Stmt, Var};
use crate::token::{Literal, Token};
use crate::token_type::TokenType::*;
use crate::{runtime_error, stmt};
use std::borrow::Borrow;
use std::ptr;

type ExprVisitorResult = Result<Literal, RuntimeError>;
type StmtVisitorResult = Result<(), RuntimeError>;

pub(crate) struct Interpreter {
    environment: Box<Environment>,
}
impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {
            environment: Environment::new(None),
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> ExprVisitorResult {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> StmtVisitorResult {
        stmt.accept(self)
    }

    fn execute_block(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        unsafe {
            let tmp_env = ptr::read(&self.environment);
            ptr::write(&mut self.environment, Environment::new(Some(tmp_env)));
            for statement in statements {
                self.execute(statement)?
            }
            let tmp_env = ptr::read(&self.environment);
            // Always be true
            if let Some(parent) = tmp_env.enclosing {
                ptr::write(&mut self.environment, parent);
            }
        }
        Ok(())
    }

    pub(crate) fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements {
            if let Err(err) = self.execute(statement.borrow()) {
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
        self.environment.get(&expr.name)
    }

    fn visit_assign_expr(&mut self, expr: &Assign) -> ExprVisitorResult {
        let value = self.evaluate(&expr.value)?;
        self.environment.assign(&expr.name, value.clone())?;
        Ok(value)
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
        self.environment.define(stmt.name.lexeme.clone(), literal);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Block) -> StmtVisitorResult {
        self.execute_block(&stmt.statements)?;
        Ok(())
    }
}
