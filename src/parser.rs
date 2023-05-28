use crate::error_with_token;
use crate::expr::Expr::{
    AssignExpr, BinaryExpr, GroupingExpr, LiteralExprExpr, UnaryExpr, VariableExpr,
};
use crate::expr::{Assign, Binary, Expr, Grouping, LiteralExpr, Unary, Variable};
use crate::stmt::Stmt::{BlockStmt, ExpressionStmt, PrintStmt, VarStmt};
use crate::stmt::{Block, Expression, Print, Stmt, Var};
use crate::token::{
    Literal::{self, *},
    Token,
};
use crate::token_type::TokenType::{self, *};

pub(crate) struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

pub(crate) struct ParseError;
type ExprResult = Result<Expr, ParseError>;
type StmtResult = Result<Stmt, ParseError>;
type ParseResult = Result<Vec<Stmt>, ParseError>;
impl Parser {
    pub(crate) fn from(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub(crate) fn parse(&mut self) -> ParseResult {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn expression(&mut self) -> ExprResult {
        self.assignment()
    }

    fn declaration(&mut self) -> StmtResult {
        if self.match_next_token_type(vec![Var]) {
            return self.var_declaration();
        }
        let stmt_result = self.statement();
        match stmt_result {
            Err(_) => self.synchronize(),
            _ => (),
        }
        stmt_result
    }

    fn statement(&mut self) -> StmtResult {
        if self.match_next_token_type(vec![Print]) {
            self.print_statement()
        } else if self.match_next_token_type(vec![LeftBrace]) {
            self.block()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> StmtResult {
        let value = self.expression()?;
        self.consume(Semicolon, "Expect ';' after value.")?;
        Ok(PrintStmt(Print::new(value)))
    }

    fn var_declaration(&mut self) -> StmtResult {
        let name = self.consume(IdentifierLiteralToken, "Expect variable name.")?;
        let mut initializer: Expr = LiteralExprExpr(LiteralExpr::new(Literal::NoneLiteral));
        if self.match_next_token_type(vec![Equal]) {
            initializer = self.expression()?;
        }
        self.consume(Semicolon, "Expect ';' after declaration.")?;
        Ok(VarStmt(Var::new(name, initializer)))
    }

    fn expression_statement(&mut self) -> StmtResult {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after expression.")?;
        Ok(ExpressionStmt(Expression::new(expr)))
    }

    fn block(&mut self) -> StmtResult {
        let mut statements = Vec::new();
        while !self.check_type(RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?)
        }
        self.consume(RightBrace, "Expect '}' after block.")?;
        Ok(BlockStmt(Block::new(statements)))
    }

    fn assignment(&mut self) -> ExprResult {
        let expr = self.equality()?;
        if self.match_next_token_type(vec![Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;
            if let VariableExpr(var_expr) = expr {
                return Ok(AssignExpr(Assign::new(var_expr.name.clone(), value)));
            } else {
                return Err(self.error(&equals, "Invalid assignment target."));
            }
        } else {
            return Ok(expr);
        }
    }

    fn equality(&mut self) -> ExprResult {
        let mut expr = self.comparison()?;
        while self.match_next_token_type(vec![BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = BinaryExpr(Binary::new(expr, operator.clone(), right));
        }
        Ok(expr)
    }
    fn match_next_token_type(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check_type(token_type) {
                let _ = self.advance();
                return true;
            }
        }
        return false;
    }
    fn check_type(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        }
        return self.previous();
    }
    fn is_at_end(&self) -> bool {
        self.peek().token_type == Eof
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
    fn comparison(&mut self) -> ExprResult {
        let mut expr = self.term()?;
        while self.match_next_token_type(vec![Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = BinaryExpr(Binary::new(expr, operator.clone(), right));
        }
        Ok(expr)
    }
    fn term(&mut self) -> ExprResult {
        let mut expr = self.factor()?;
        while self.match_next_token_type(vec![Minus, Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = BinaryExpr(Binary::new(expr, operator.clone(), right));
        }
        Ok(expr)
    }
    fn factor(&mut self) -> ExprResult {
        let mut expr = self.unary()?;
        while self.match_next_token_type(vec![Slash, Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = BinaryExpr(Binary::new(expr, operator.clone(), right));
        }
        Ok(expr)
    }
    fn unary(&mut self) -> ExprResult {
        if self.match_next_token_type(vec![Bang, Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(UnaryExpr(Unary::new(operator.clone(), right)))
        } else {
            self.primary()
        }
    }
    fn primary(&mut self) -> ExprResult {
        if self.match_next_token_type(vec![False]) {
            return Ok(LiteralExprExpr(LiteralExpr::new(BoolLiteral(false))));
        }
        if self.match_next_token_type(vec![True]) {
            return Ok(LiteralExprExpr(LiteralExpr::new(BoolLiteral(true))));
        }
        if self.match_next_token_type(vec![NilTokenType]) {
            return Ok(LiteralExprExpr(LiteralExpr::new(NoneLiteral)));
        }
        if self.match_next_token_type(vec![NumberLiteralToken, StringLiteralToken]) {
            return Ok(LiteralExprExpr(LiteralExpr::new(self.previous().literal)));
        }
        if self.match_next_token_type(vec![IdentifierLiteralToken]) {
            return Ok(VariableExpr(Variable::new(self.previous())));
        }
        if self.match_next_token_type(vec![LeftParen]) {
            let expr = self.expression()?;
            self.consume(RightParen, "Expect ')' after expression.")?;
            return Ok(GroupingExpr(Grouping::new(expr)));
        }
        Err(ParseError {})
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check_type(token_type) {
            Ok(self.advance())
        } else {
            Err(self.error(&self.peek().clone(), message))
        }
    }

    fn error(&mut self, token: &Token, message: &str) -> ParseError {
        error_with_token(token, message);
        ParseError {}
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }

            match self.peek().token_type {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => _ = self.advance(),
            }
        }
    }
}
