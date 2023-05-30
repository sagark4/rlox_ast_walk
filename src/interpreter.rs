use crate::environment_tree::{EnvironmentNode, EnvironmentTree};
use crate::expr::{
    self, Assign, Binary, Call, Expr, Expr::VariableExpr, Grouping, LiteralExpr, Logical, Unary,
    Variable,
};
use crate::lox_callable::{LoxCallable, LoxClass, LoxFunction};
use crate::stmt::{Block, Expression, Function, If, Print, Return, Stmt, Var, While};
use crate::token::{Literal, Token};
use crate::token_type::TokenType::*;
use crate::{runtime_error, stmt};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) type ExprVisitorResult = Result<Literal, RuntimeErrorOrReturn>;
type StmtVisitorResult = Result<(), RuntimeErrorOrReturn>;

pub(crate) struct Interpreter {
    pub(crate) env: Rc<RefCell<EnvironmentTree>>,
    pub(crate) return_value: Option<Literal>,
}
impl Interpreter {
    fn evaluate(&mut self, expr: &Expr) -> ExprVisitorResult {
        expr.accept(self)
    }

    pub(crate) fn execute(&mut self, stmt: &Stmt) -> StmtVisitorResult {
        stmt.accept(self)
    }

    pub(crate) fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeErrorOrReturn> {
        for statement in &statements {
            if let Err(err) = self.execute(statement) {
                runtime_error(&err);
                return Err(err);
            }
        }
        Ok(())
    }
    fn stringify(&self, literal: &Literal) -> String {
        literal.stringify()
    }

    pub(crate) fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: Rc<RefCell<EnvironmentNode>>,
    ) -> StmtVisitorResult {
        let previous = (*self.env).borrow().get_current();
        (*self.env).borrow_mut().set_current(environment);
        for statement in statements {
            if let Err(err) = self.execute(statement) {
                (*self.env).borrow_mut().set_current(previous);
                return Err(err);
            }
        }
        (*self.env).borrow_mut().set_current(previous);
        Ok(())
    }
}

pub(crate) struct RuntimeErrorOrReturn {
    pub(crate) message: String,
    pub(crate) token: Token,
    pub(crate) return_flag: bool,
}

fn construct_error(message: &str, token: &Token) -> ExprVisitorResult {
    Err(RuntimeErrorOrReturn {
        message: String::from(message),
        token: token.clone(),
        return_flag: false,
    })
}

fn construct_number_error(token: &Token) -> ExprVisitorResult {
    construct_error("Operand must be a number.", token)
}

fn construct_numbers_error(token: &Token) -> ExprVisitorResult {
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
        if expr.operator.token_type == Plus {
            if left.is_string() && right.is_string() {
                let mut concat_string = String::from(left.unwrap_str_literal());
                concat_string.push_str(right.unwrap_str_literal());
                return Ok(Literal::wrap_string_literal(concat_string));
            } else if !right.is_float() || !left.is_float() {
                return construct_error("Operands must be two numbers or two strings.", &expr.operator)
            }
        }

        if !right.is_float() || !left.is_float() {
            return construct_numbers_error(&expr.operator);
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
        (*self.env).borrow().get(&expr.name, expr.id)
    }

    fn visit_assign_expr(&mut self, expr: &Assign) -> ExprVisitorResult {
        let value = self.evaluate(&expr.value)?;
        self.env
            .borrow_mut()
            .assign(&expr.name, value.clone(), expr.id)?;
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
                return Err(RuntimeErrorOrReturn {
                    message: format!(
                        "Expected {} arguments but got {}.",
                        calleable.arity(),
                        arguments.len()
                    ),
                    token: expr.paren.clone(),
                    return_flag: false,
                });
            }
            return calleable.call(self, arguments);
        }
        Err(RuntimeErrorOrReturn {
            message: "Can only call functions and classes.".to_string(),
            token: expr.paren.clone(),
            return_flag: false,
        })
    }

    fn visit_get_expr(&mut self, expr: &expr::Get) -> ExprVisitorResult {
        let object = self.evaluate(&expr.object)?;
        if let Literal::Instance(inst) = object {
            Ok((*inst).borrow().get(inst.clone(), &expr.name)?)
        } else {
            Err(RuntimeErrorOrReturn {
                message: "Only instances have properties.".to_string(),
                token: expr.name.clone(),
                return_flag: false,
            })
        }
    }

    fn visit_set_expr(&mut self, expr: &expr::Set) -> ExprVisitorResult {
        let object = self.evaluate(&expr.object)?;
        if let Literal::Instance(inst) = object {
            let value = self.evaluate(&expr.value)?;
            (*inst).borrow_mut().set(&expr.name, value.clone());
            Ok(value)
        } else {
            Err(RuntimeErrorOrReturn {
                message: "Only instances have fields.".to_string(),
                token: expr.name.clone(),
                return_flag: false,
            })
        }
    }

    fn visit_this_expr(&mut self, expr: &expr::This) -> ExprVisitorResult {
        (*self.env).borrow().get(&expr.keyword, expr.id)
    }

    fn visit_super_expr(&mut self, expr: &expr::Super) -> ExprVisitorResult {
        let steps = *(*self.env).borrow().get_step_for_id(expr.id).unwrap();
        let superclass = (*self.env).borrow().get_at(&expr.keyword.lexeme, steps)?;
        let object = (*self.env).borrow().get_at("this", steps - 1)?;
        if let Literal::Callable(LoxCallable::UserClass(class)) = superclass {
            let method_option = (*class).borrow().find_method(&expr.method.lexeme);
            if let Some(method) = method_option {
                if let Literal::Instance(inst) = object {
                    return Ok(method.bind(inst));
                } else {
                    panic!() //Should not reach here.
                }
            } else {
                return Err(RuntimeErrorOrReturn {
                    message: format!("Undefined property '{}'.", expr.method.lexeme).to_string(),
                    token: expr.method.clone(),
                    return_flag: false,
                });
            }
        } else {
            panic!() //Should not reach here.
        }
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
        (*self.env)
            .borrow()
            .define_current(stmt.name.lexeme.clone(), literal);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Block) -> StmtVisitorResult {
        let curr_env = (*self.env).borrow().get_current();
        self.execute_block(
            &stmt.statements,
            EnvironmentTree::new_environment_node(Some(curr_env)),
        )?;
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
        (*self.env).borrow().define_current(
            stmt.name.lexeme.clone(),
            Literal::Callable(LoxCallable::UserFunction(LoxFunction {
                declaration: stmt,
                closure: (*self.env).borrow().get_current(),
                is_initializer: false,
            })),
        );
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &Return) -> StmtVisitorResult {
        self.return_value = Some(self.evaluate(&stmt.value)?);
        Err(RuntimeErrorOrReturn {
            message: String::new(),
            token: stmt.keyword.clone(),
            return_flag: true,
        })
    }

    fn visit_class_stmt(&mut self, stmt: Rc<stmt::Class>) -> StmtVisitorResult {
        let mut superclass = None;
        let mut superclass_literal = Literal::NoneLiteral;
        if let Some(variable) = &stmt.superclass {
            let literal = self.evaluate(&VariableExpr(variable.clone()))?;
            if let Literal::Callable(LoxCallable::UserClass(class)) = literal.clone() {
                superclass = Some(class);
                superclass_literal = literal;
            } else {
                return Err(RuntimeErrorOrReturn {
                    message: "Superclass must be a class.".to_string(),
                    token: variable.name.clone(),
                    return_flag: false,
                });
            }
        }
        (*self.env)
            .borrow()
            .define_current((*stmt).name.lexeme.clone(), Literal::NoneLiteral);
        if let Some(_) = superclass {
            let new_env =
                EnvironmentTree::new_environment_node(Some((*self.env).borrow().get_current()));
            new_env
                .borrow_mut()
                .environment
                .insert("super".to_string(), superclass_literal);
            (*self.env).borrow_mut().set_current(new_env);
        }
        let mut methods = HashMap::new();
        for method in &stmt.methods {
            let function = Literal::Callable(LoxCallable::UserFunction(LoxFunction {
                declaration: method.clone(),
                closure: (*self.env).borrow().get_current(),
                is_initializer: method.name.lexeme == "init",
            }));
            methods.insert(method.name.lexeme.clone(), function);
        }
        let class = LoxClass::new(stmt.clone(), superclass.clone(), methods);
        if let Some(_) = superclass {
            let cur_env = (*self.env).borrow().get_current();
            let par_env = (*cur_env).borrow().parent.clone().unwrap();
            (*self.env).borrow_mut().set_current(par_env);
        }
        (*self.env)
            .borrow_mut()
            .assign_cur_or_par(&stmt.name, Literal::Callable(class))?;
        Ok(())
    }
}
