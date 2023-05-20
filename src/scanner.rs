use crate::token::Literal;
use crate::token::Literal::*;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::token_type::TokenType::*;
pub(crate) struct Scanner {
    pub tokens: Vec<Token>,
    source_chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}
impl Scanner {
    pub(crate) fn new(source: &str) -> Self {
        let source_chars = source.chars().collect();
        Self {
            tokens: Vec::<Token>::new(),
            source_chars,
            start: 0,
            current: 0,
            line: 1,
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
        self.current >= self.source_chars.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token_null_literal(LeftParen),
            ')' => self.add_token_null_literal(RightParen),
            '{' => self.add_token_null_literal(LeftBrace),
            '}' => self.add_token_null_literal(RightBrace),
            ',' => self.add_token_null_literal(Comma),
            '.' => self.add_token_null_literal(Dot),
            '-' => self.add_token_null_literal(Minus),
            '+' => self.add_token_null_literal(Plus),
            ';' => self.add_token_null_literal(Semicolon),
            '*' => self.add_token_null_literal(Star),
            '!' => {
                let token_type = if self.match_cur('=') { BangEqual } else { Bang };
                self.add_token_null_literal(token_type);
            }
            '=' => {
                let token_type = if self.match_cur('=') {
                    EqualEqual
                } else {
                    Equal
                };
                self.add_token_null_literal(token_type);
            }
            '<' => {
                let token_type = if self.match_cur('=') { LessEqual } else { Less };
                self.add_token_null_literal(token_type);
            }
            '>' => {
                let token_type = if self.match_cur('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token_null_literal(token_type);
            }
            '/' => {
                if self.match_cur('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_null_literal(Slash)
                }
            }
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '"' => self.handle_string(),
            _ => crate::error(self.line, "Unexpected character."),
        }
    }
    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            crate::error(self.line, "Unterminated string.");
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let token_slice = &self.source_chars[self.start + 1..self.current - 1];
        let value: String = token_slice.into_iter().collect();
        self.add_token(StringLiteralToken, StringLiteral(value));
    }

    fn match_cur(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source_chars[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source_chars[self.current]
        }
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source_chars[self.current - 1]
    }

    fn add_token_null_literal(&mut self, token_type: TokenType) {
        self.add_token(token_type, Null)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let token_slice = &self.source_chars[self.start..self.current];
        let text: String = token_slice.into_iter().collect();

        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
        });
    }
}
