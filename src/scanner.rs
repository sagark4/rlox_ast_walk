use crate::token::Literal;
use crate::token::Literal::*;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::token_type::TokenType::*;
use std::collections::HashMap;
pub(crate) struct Scanner {
    pub(crate) tokens: Vec<Token>,
    source_chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}
impl Scanner {
    pub(crate) fn new(source: &str) -> Self {
        let source_chars = source.chars().collect();
        let mut keywords = HashMap::new();
        keywords.insert(String::from("and"), And);
        keywords.insert(String::from("class"), Class);
        keywords.insert(String::from("else"), Else);
        keywords.insert(String::from("false"), False);
        keywords.insert(String::from("for"), For);
        keywords.insert(String::from("fun"), Fun);
        keywords.insert(String::from("if"), If);
        keywords.insert(String::from("nil"), NilTokenType);
        keywords.insert(String::from("or"), Or);
        keywords.insert(String::from("print"), Print);
        keywords.insert(String::from("return"), Return);
        keywords.insert(String::from("super"), Super);
        keywords.insert(String::from("this"), This);
        keywords.insert(String::from("true"), True);
        keywords.insert(String::from("var"), Var);
        keywords.insert(String::from("while"), While);

        Self {
            tokens: Vec::<Token>::new(),
            source_chars,
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub(crate) fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::from(Eof, String::from(""), NoneLiteral, self.line));
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
            '\n' => (),
            '"' => self.handle_string(),
            r => {
                if is_numeric(r) {
                    self.handle_number();
                } else if is_alpha(r) {
                    self.handle_identifier();
                } else {
                    crate::error(self.line, &format!("Unexpected character: {r}."));
                }
            }
        }
    }

    fn handle_identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let token_slice = &self.source_chars[self.start..self.current];
        let text: String = token_slice.into_iter().collect();
        match self.keywords.get(&text) {
            None => self.add_token_null_literal(IdentifierLiteralToken),
            Some(id) => self.add_token_null_literal(*id),
        }
    }

    fn handle_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let token_slice = &self.source_chars[self.start..self.current];
        let number_text: String = token_slice.into_iter().collect();

        self.add_token(NumberLiteralToken, Float(number_text.parse().unwrap()));
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

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source_chars.len() {
            '\0'
        } else {
            self.source_chars[self.current + 1]
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source_chars[self.current - 1]
    }

    fn add_token_null_literal(&mut self, token_type: TokenType) {
        self.add_token(token_type, NoneLiteral)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let token_slice = &self.source_chars[self.start..self.current];
        let text: String = token_slice.into_iter().collect();

        self.tokens
            .push(Token::from(token_type, text, literal, self.line));
    }
}

fn is_numeric(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_numeric(c)
}
