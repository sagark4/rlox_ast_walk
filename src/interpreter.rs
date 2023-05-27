use crate::environment::Environment;
use crate::expr::{self, Variable, VisitorReturnError, VisitorReturnOk};
use crate::expr::{
    Binary, Expr, Grouping, LiteralExpr, Unary, VisitorReturnError::VRRuntimeErr,
    VisitorReturnOk::NoResult, VisitorReturnOk::VRLiteral, VisitorReturnResult,
};
use crate::runtime_error;
use crate::stmt::{self, ExpressionStmt, PrintStmt, Stmt, VarStmt};
use crate::token::{Literal, Token};
use crate::token_type::TokenType::*;
use std::borrow::Borrow;

pub(crate) struct Interpreter {
    environment: Environment,
}
impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }
    fn evaluate(&self, expr: &dyn Expr) -> VisitorReturnResult {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &dyn Stmt) -> VisitorReturnResult {
        stmt.accept(self)
    }

    // fn evaluate_literal(&self, expr: &dyn Expr) -> Result<Literal, RuntimeError> {
    //     match expr.accept(self) {
    //         Ok(VRLiteral(literal)) => {
    //             println!("{}", self.stringify(&literal));
    //             Ok(literal)
    //         }
    //         Err(VRRuntimeErr(err)) => {
    //             crate::runtime_error(&err);
    //             Err(err)
    //         }
    //         _ => panic!(),
    //     }
    // }
    pub(crate) fn interpret(&mut self, statements: Vec<Box<dyn Stmt>>) -> Result<(), RuntimeError> {
        for statement in statements {
            if let Err(VRRuntimeErr(err)) = self.execute(statement.borrow()) {
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

fn construct_error(message: &str, token: &Token) -> VisitorReturnResult {
    Err(VRRuntimeErr(RuntimeError {
        message: String::from(message),
        token: token.clone(),
    }))
}

fn construct_number_error(token: &Token) -> VisitorReturnResult {
    construct_error("Operands must be numbers.", token)
}

impl expr::Visitor for Interpreter {
    fn visit_literalexpr_expr(&self, expr: &LiteralExpr) -> VisitorReturnResult {
        Ok(VRLiteral(expr.value.clone()))
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> VisitorReturnResult {
        self.evaluate(expr.expression.borrow())
    }

    fn visit_unary_expr(&self, expr: &Unary) -> VisitorReturnResult {
        let right = self.evaluate(expr.right.borrow())?;
        match expr.operator.token_type {
            Minus => {
                if !right.is_float() {
                    construct_number_error(&expr.operator)
                } else {
                    Ok(VisitorReturnOk::wrap_float(-right.unwrap_float()))
                }
            }
            Bang => Ok(right.unwrap_negate_and_wrap_vrl_bool()),
            _ => panic!(), //TODO:
        }
    }

    fn visit_binary_expr(&self, expr: &Binary) -> VisitorReturnResult {
        let left = self.evaluate(expr.left.borrow())?;
        let right = self.evaluate(expr.right.borrow())?;
        match expr.operator.token_type {
            BangEqual => {
                return Ok(VisitorReturnOk::wrap_bool(
                    !left.is_vrl_equal_or_panic(&right),
                ))
            }
            EqualEqual => {
                return Ok(VisitorReturnOk::wrap_bool(
                    left.is_vrl_equal_or_panic(&right),
                ))
            }
            _ => (),
        }
        if left.is_string() && right.is_string() && expr.operator.token_type == Plus {
            let mut concat_string = String::from(left.unwrap_str_literal());
            concat_string.push_str(right.unwrap_str_literal());
            return Ok(VisitorReturnOk::wrap_string_literal(concat_string));
        }
        if !right.is_float() || !left.is_float() {
            return construct_number_error(&expr.operator);
        }

        let left = left.unwrap_float();
        let right = right.unwrap_float();
        match expr.operator.token_type {
            Minus => Ok(VisitorReturnOk::wrap_float(left - right)),
            Plus => Ok(VisitorReturnOk::wrap_float(left + right)),
            Slash => Ok(VisitorReturnOk::wrap_float(left / right)),
            Star => Ok(VisitorReturnOk::wrap_float(left * right)),
            Greater => Ok(VisitorReturnOk::wrap_bool(left > right)),
            GreaterEqual => Ok(VisitorReturnOk::wrap_bool(left >= right)),
            Less => Ok(VisitorReturnOk::wrap_bool(left < right)),
            LessEqual => Ok(VisitorReturnOk::wrap_bool(left <= right)),
            _ => panic!(), //TODO:
        }
    }

    fn visit_variable_expr(&self, expr: &Variable) -> VisitorReturnResult {
        match self.environment.get(&expr.name) {
            Ok(literal) => Ok(VRLiteral(literal.clone())),
            Err(runtime_error) => Err(VisitorReturnError::VRRuntimeErr(runtime_error)),
        }
    }
}

impl stmt::Visitor for Interpreter {
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> VisitorReturnResult {
        self.evaluate(expr.expression.borrow())?;
        Ok(NoResult)
    }

    fn visit_print_stmt(&self, expr: &PrintStmt) -> VisitorReturnResult {
        match self.evaluate(expr.expression.borrow())? {
            VRLiteral(value) => {
                println!("{}", self.stringify(&value));
                Ok(NoResult)
            }
            _ => panic!(), // Should not reach here.
        }
    }

    fn visit_var_stmt(&mut self, expr: &VarStmt) -> VisitorReturnResult {
        let eval_result = self.evaluate(expr.initializer.borrow())?;
        match eval_result {
            VRLiteral(literal) => {
                self.environment.define(expr.name.lexeme.clone(), literal);
            }
            _ => (),
        }

        Ok(VisitorReturnOk::NoResult)
    }
}
