use std::rc::Rc;
use crate::expr::{Binary, LiteralExpr, Unary, Grouping};
use crate::{token, expr};
use crate::token_type::TokenType::{*, self};
use crate::{token::Token, expr::Expr};
use crate::token::Literal::*;

pub(crate) struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub(crate) fn from(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    pub(crate) fn parse(&mut self) -> Rc<dyn Expr> {
        self.expression()
    }
    fn expression(&mut self) -> Rc<dyn Expr> {
        self.equality()
    }
    fn equality(&mut self) -> Rc<dyn Expr> {
        let mut expr = self.comparison();
        while self.match_next_token_type(vec![BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Rc::new(Binary::new(Rc::clone(&expr), operator.clone(), Rc::clone(&right)));
        }
        expr
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
    fn advance(&mut self)  -> Token{
        if !self.is_at_end() {
            self.current += 1
        }
        return self.previous()
    }
    fn is_at_end(&self) -> bool {
        self.peek().token_type == Eof
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn previous(&self) -> Token {
        self.tokens[self.current-1].clone()
    }
    fn comparison(&mut self) -> Rc<dyn Expr> {
        let mut expr = self.term();
        while self.match_next_token_type(vec![Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let right = self.term();
            expr = Rc::new(Binary::new(Rc::clone(&expr), operator.clone(), Rc::clone(&right)));
        }
        expr
    }
    fn term(&mut self) -> Rc<dyn Expr> {
        let mut expr = self.factor();
        while self.match_next_token_type(vec![Minus, Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Rc::new(Binary::new(Rc::clone(&expr), operator.clone(), Rc::clone(&right)));
        }
        expr
    }
    fn factor(&mut self) -> Rc<dyn Expr> {
        let mut expr = self.unary();
        while self.match_next_token_type(vec![Slash, Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Rc::new(Binary::new(Rc::clone(&expr), operator.clone(), Rc::clone(&right)));
        }
        expr
    }
    fn unary(&mut self) -> Rc<dyn Expr> {
        if self.match_next_token_type(vec![Bang, Minus]) {
            let operator = self.previous();
            let right = self.unary();
            Rc::new(Unary::new(operator.clone(), Rc::clone(&right)))
        } else {
            self.primary()
        }
    }
    fn primary(&mut self) -> Rc<dyn Expr> {
        if self.match_next_token_type(vec![False]) {
            return Rc::new(LiteralExpr::new(BoolLiteral(false)));
        }
        if self.match_next_token_type(vec![True]) {
            return Rc::new(LiteralExpr::new(BoolLiteral(true)));
        }
        if self.match_next_token_type(vec![NilTokenType]) {
            return Rc::new(LiteralExpr::new(NoneLiteral));
        }
        if self.match_next_token_type(vec![NumberLiteralToken, StringLiteralToken]) {
            return Rc::new(LiteralExpr::new(self.previous().literal));
        }
        if self.match_next_token_type(vec![LeftParen]) {
            let expr = self.expression();
            let _ = self.consume(RightParen, "Expect ')' after expression.");
            return Rc::new(Grouping::new(expr));
        }
        panic!()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check_type(token_type) {
            self.advance()
        } else {
            panic!()
        }
    }
        
}
