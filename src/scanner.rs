use crate::token::Literal::*;
use crate::token::Token;
use crate::token_type::TokenType::*;
pub(crate) struct Scanner<'a> {
    pub tokens: Vec<Token>,
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    had_error: &'a mut bool,
}
impl<'a> Scanner<'a> {
    pub(crate) fn new(source: &'a str, had_error: &'a mut bool) -> Self {
        Self {
            tokens: Vec::<Token>::new(),
            source,
            start: 0,
            current: 0,
            line: 1,
            had_error,
        }
    }

    pub(crate) fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();

            break; // currently it goes in infinite loop otherwise
        }

        self.tokens.push(Token {
            token_type: Eof,
            lexeme: String::from(""),
            literal: Null,
            line: self.line,
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&self) {
        ()
    }
}
