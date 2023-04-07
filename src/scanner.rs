use std::collections::HashMap;

use crate::Lox;

use super::token::*;
use LiteralVal::Nil;

fn is_digit(c: char) -> bool {
    let u = c as u8;
    (u >= b'0') && (u <= b'9')
}

fn is_alpha(c: char) -> bool {
    let u = c as u8;
    (u >= b'a' && u <= b'z') || (u >= b'A' && u <= b'Z') || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("if", TokenType::If),
                ("nil", TokenType::Nil),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
            ]),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self, lox: &mut Lox) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, Nil),
            ')' => self.add_token(TokenType::RightParen, Nil),
            '{' => self.add_token(TokenType::LeftBrace, Nil),
            '}' => self.add_token(TokenType::RightBrace, Nil),
            ',' => self.add_token(TokenType::Comma, Nil),
            '.' => self.add_token(TokenType::Dot, Nil),
            '-' => self.add_token(TokenType::Minus, Nil),
            '+' => self.add_token(TokenType::Plus, Nil),
            ';' => self.add_token(TokenType::Semicolon, Nil),
            '*' => self.add_token(TokenType::Star, Nil),
            '!' => {
                let token = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };

                self.add_token(token, Nil)
            }
            '=' => {
                let token = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token, Nil)
            }
            '<' => {
                let token = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token, Nil)
            }
            '>' => {
                let token = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token, Nil)
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    self.block_comment(lox);
                } else {
                    self.add_token(TokenType::Slash, Nil)
                }
            }
            ' ' | '\t' | '\r' => (),
            '\n' => self.line += 1,
            '"' => self.string(lox),
            _ => {
                if is_digit(c) {
                    self.number()
                } else if is_alpha(c) {
                    self.identifier()
                } else {
                    lox.error(self.line, "Unexpected Character")
                }
            }
        }
    }

    fn block_comment(&mut self, lox: &mut Lox) {
        let mut depth = 1;
        while depth > 0 {
            if self.is_at_end() {
                lox.error(self.line, "Unterminated block comment.");
                break;
            }

            if self.peek() == '*' && self.peek_next() == '/' {
                depth -= 1;
                self.advance();
            } else if self.peek() == '/' && self.peek_next() == '*' {
                depth += 1;
                self.advance();
            } else if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        if let Some(keyword) = self.keywords.get(text) {
            self.add_token(keyword.clone(), Nil)
        } else {
            self.add_token(TokenType::Identifier, Nil);
        }
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let substr = String::from(&self.source[self.start..self.current]);
        self.add_token(
            TokenType::Number,
            LiteralVal::Number(substr.parse::<f32>().unwrap()),
        )
    }

    fn string(&mut self, lox: &mut Lox) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            lox.error(self.line, "Unterminated string.")
        }

        self.advance(); // The closing "

        let value = String::from(&self.source[self.start + 1..self.current - 1]);
        self.add_token(TokenType::String, LiteralVal::Str(value));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.char_at(self.current) != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.char_at(self.current)
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.char_at(self.current + 1)
    }

    fn char_at(&self, i: usize) -> char {
        self.source.chars().nth(i).unwrap()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.char_at(self.current - 1)
    }

    fn add_token(&mut self, token_type: TokenType, literal: LiteralVal) {
        let text = self.source.get(self.start..self.current).unwrap();
        self.tokens.push(Token {
            token_type,
            lexeme: text.to_string(),
            literal,
            line: self.line,
        });
    }

    pub fn scan_tokens(&mut self, lox: &mut Lox) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(lox)
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            literal: Nil,
            line: self.line,
        });

        self.tokens.clone()
    }
}
