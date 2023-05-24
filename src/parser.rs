use crate::error_with_token;
use crate::expr::{Binary, Grouping, LiteralExpr, Unary};
use crate::token::Literal::*;
use crate::token_type::TokenType::{self, *};
use crate::{expr::Expr, token::Token};

pub(crate) struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

pub(crate) struct ParseError {}
type ExprResult = Result<Box<dyn Expr>, ParseError>;

impl Parser {
    pub(crate) fn from(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    pub(crate) fn parse(&mut self) -> ExprResult {
        self.expression()
    }
    fn expression(&mut self) -> ExprResult {
        self.equality()
    }
    fn equality(&mut self) -> ExprResult {
        let mut expr = self.comparison()?;
        while self.match_next_token_type(vec![BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Binary::new(expr, operator.clone(), right);
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
            expr = Binary::new(expr, operator.clone(), right);
        }
        Ok(expr)
    }
    fn term(&mut self) -> ExprResult {
        let mut expr = self.factor()?;
        while self.match_next_token_type(vec![Minus, Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Binary::new(expr, operator.clone(), right);
        }
        Ok(expr)
    }
    fn factor(&mut self) -> ExprResult {
        let mut expr = self.unary()?;
        while self.match_next_token_type(vec![Slash, Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Binary::new(expr, operator.clone(), right);
        }
        Ok(expr)
    }
    fn unary(&mut self) -> ExprResult {
        if self.match_next_token_type(vec![Bang, Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Unary::new(operator.clone(), right))
        } else {
            self.primary()
        }
    }
    fn primary(&mut self) -> ExprResult {
        if self.match_next_token_type(vec![False]) {
            return Ok(LiteralExpr::new(BoolLiteral(false)));
        }
        if self.match_next_token_type(vec![True]) {
            return Ok(LiteralExpr::new(BoolLiteral(true)));
        }
        if self.match_next_token_type(vec![NilTokenType]) {
            return Ok(LiteralExpr::new(NoneLiteral));
        }
        if self.match_next_token_type(vec![NumberLiteralToken, StringLiteralToken]) {
            return Ok(LiteralExpr::new(self.previous().literal));
        }
        if self.match_next_token_type(vec![LeftParen]) {
            let expr = self.expression()?;
            self.consume(RightParen, "Expect ')' after expression.")?;
            return Ok(Grouping::new(expr));
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
}
