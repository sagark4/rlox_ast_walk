use std::rc::Rc;

use crate::token::Literal;
use crate::token::Token;

pub(crate) enum Expr {
    BinaryExpr(Box<Binary>),
    GroupingExpr(Box<Grouping>),
    LiteralExprExpr(Box<LiteralExpr>),
    UnaryExpr(Box<Unary>),
    VariableExpr(Rc<Variable>),
    AssignExpr(Box<Assign>),
    LogicalExpr(Box<Logical>),
    CallExpr(Box<Call>),
    GetExpr(Box<Get>),
    SetExpr(Box<Set>),
    ThisExpr(Box<This>),
    SuperExpr(Box<Super>),
}

impl Expr {
    pub(crate) fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Expr::BinaryExpr(expr) => visitor.visit_binary_expr(expr),
            Expr::GroupingExpr(expr) => visitor.visit_grouping_expr(expr),
            Expr::LiteralExprExpr(expr) => visitor.visit_literalexpr_expr(expr),
            Expr::UnaryExpr(expr) => visitor.visit_unary_expr(expr),
            Expr::VariableExpr(expr) => visitor.visit_variable_expr(expr),
            Expr::AssignExpr(expr) => visitor.visit_assign_expr(expr),
            Expr::LogicalExpr(expr) => visitor.visit_logical_expr(expr),
            Expr::CallExpr(expr) => visitor.visit_call_expr(expr),
            Expr::GetExpr(expr) => visitor.visit_get_expr(expr),
            Expr::SetExpr(expr) => visitor.visit_set_expr(expr),
            Expr::ThisExpr(expr) => visitor.visit_this_expr(expr),
            Expr::SuperExpr(expr) => visitor.visit_super_expr(expr),
        }
    }
}
pub(crate) trait Visitor<R> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> R;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> R;
    fn visit_literalexpr_expr(&mut self, expr: &LiteralExpr) -> R;
    fn visit_unary_expr(&mut self, expr: &Unary) -> R;
    fn visit_variable_expr(&mut self, expr: &Variable) -> R;
    fn visit_assign_expr(&mut self, expr: &Assign) -> R;
    fn visit_logical_expr(&mut self, expr: &Logical) -> R;
    fn visit_call_expr(&mut self, expr: &Call) -> R;
    fn visit_get_expr(&mut self, expr: &Get) -> R;
    fn visit_set_expr(&mut self, expr: &Set) -> R;
    fn visit_this_expr(&mut self, expr: &This) -> R;
    fn visit_super_expr(&mut self, expr: &Super) -> R;
}

pub(crate) struct Binary {
    pub(crate) left: Expr,
    pub(crate) operator: Token,
    pub(crate) right: Expr,
}

impl Binary {
    pub(crate) fn new(left: Expr, operator: Token, right: Expr) -> Box<Self> {
        Box::new(Self {
            left,
            operator,
            right,
        })
    }
}

pub(crate) struct Grouping {
    pub(crate) expression: Expr,
}

impl Grouping {
    pub(crate) fn new(expression: Expr) -> Box<Self> {
        Box::new(Self { expression })
    }
}

pub(crate) struct LiteralExpr {
    pub(crate) value: Literal,
}

impl LiteralExpr {
    pub(crate) fn new(value: Literal) -> Box<Self> {
        Box::new(Self { value })
    }
}

pub(crate) struct Unary {
    pub(crate) operator: Token,
    pub(crate) right: Expr,
}

impl Unary {
    pub(crate) fn new(operator: Token, right: Expr) -> Box<Self> {
        Box::new(Self { operator, right })
    }
}

pub(crate) struct Variable {
    pub(crate) name: Token,
    pub(crate) id: usize,
}

impl Variable {
    pub(crate) fn new(name: Token, id: usize) -> Rc<Self> {
        Rc::new(Self { name, id })
    }
}

pub(crate) struct Assign {
    pub(crate) name: Token,
    pub(crate) value: Expr,
    pub(crate) id: usize,
}

impl Assign {
    pub(crate) fn new(name: Token, value: Expr, id: usize) -> Box<Self> {
        Box::new(Self { name, value, id })
    }
}

pub(crate) struct Logical {
    pub(crate) left: Expr,
    pub(crate) operator: Token,
    pub(crate) right: Expr,
}

impl Logical {
    pub(crate) fn new(left: Expr, operator: Token, right: Expr) -> Box<Self> {
        Box::new(Self {
            left,
            operator,
            right,
        })
    }
}

pub(crate) struct Call {
    pub(crate) callee: Expr,
    pub(crate) paren: Token,
    pub(crate) arguments: Vec<Expr>,
}

impl Call {
    pub(crate) fn new(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Box<Self> {
        Box::new(Self {
            callee,
            paren,
            arguments,
        })
    }
}

pub(crate) struct Get {
    pub(crate) object: Expr,
    pub(crate) name: Token,
}

impl Get {
    pub(crate) fn new(object: Expr, name: Token) -> Box<Self> {
        Box::new(Self { object, name })
    }
}

pub(crate) struct Set {
    pub(crate) object: Expr,
    pub(crate) name: Token,
    pub(crate) value: Expr,
}

impl Set {
    pub(crate) fn new(object: Expr, name: Token, value: Expr) -> Box<Self> {
        Box::new(Self {
            object,
            name,
            value,
        })
    }
}

pub(crate) struct This {
    pub(crate) keyword: Token,
    pub(crate) id: usize,
}

impl This {
    pub(crate) fn new(keyword: Token, id: usize) -> Box<Self> {
        Box::new(Self { keyword, id })
    }
}

pub(crate) struct Super {
    pub(crate) keyword: Token,
    pub(crate) method: Token,
    pub(crate) id: usize,
}

impl Super {
    pub(crate) fn new(keyword: Token, method: Token, id: usize) -> Box<Self> {
        Box::new(Self {
            keyword,
            method,
            id,
        })
    }
}
