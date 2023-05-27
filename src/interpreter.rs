use crate::environment::Environment;
use crate::expr::{Binary, Expr, Grouping, LiteralExpr, Unary, Variable, self};
use crate::stmt::{Expression, Print, Stmt, Var};
use crate::token::{Literal, Token};
use crate::token_type::TokenType::*;
use crate::{runtime_error, stmt};
use std::borrow::Borrow;

type ExprVisitorResult = Result<Literal, RuntimeError>;
type StmtVisitorResult = Result<(), RuntimeError>;

pub(crate) struct Interpreter {
    environment: Environment,
}
impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }
    fn evaluate(&self, expr: &Expr) -> ExprVisitorResult {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> StmtVisitorResult {
        stmt.accept(self)
    }

    // fn evaluate_literal(&self, expr: &Expr) -> Result<Literal, RuntimeError> {
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
    pub(crate) fn interpret(&mut self, statements: Vec<Box<Stmt>>) -> Result<(), RuntimeError> {
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
    fn visit_literalexpr_expr(&self, expr: &LiteralExpr) -> ExprVisitorResult {
        Ok(expr.value.clone())
    }

    fn visit_grouping_expr(&self, expr: &Grouping) -> ExprVisitorResult {
        self.evaluate(expr.expression.borrow())
    }

    fn visit_unary_expr(&self, expr: &Unary) -> ExprVisitorResult {
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

    fn visit_binary_expr(&self, expr: &Binary) -> ExprVisitorResult {
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

    fn visit_variable_expr(&self, expr: &Variable) -> ExprVisitorResult {
        match self.environment.get(&expr.name) {
            Ok(literal) => Ok(literal.clone()),
            Err(runtime_error) => Err(runtime_error),
        }
    }
}

impl stmt::Visitor<StmtVisitorResult> for Interpreter {
    fn visit_expression_stmt(&self, stmt: &Expression) -> StmtVisitorResult {
        self.evaluate(stmt.expression.borrow())?;
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &Print) -> StmtVisitorResult {
        println!(
            "{}",
            self.stringify(&self.evaluate(stmt.expression.borrow())?)
        );
        Ok(())
    }
    fn visit_var_stmt(&mut self, stmt: &Var) -> StmtVisitorResult {
        let literal = self.evaluate(stmt.initializer.borrow())?;
        self.environment.define(stmt.name.lexeme.clone(), literal);
        Ok(())
    }
}
