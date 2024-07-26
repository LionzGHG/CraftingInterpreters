
pub mod tokens;

use std::collections::HashMap;
use tokens::{Token, TokenType};
use tokens::CloneAny;

use crate::error;

macro_rules! static_keywords_map {
    ($($key:expr => $value:expr ),* $(,)?) => {
        {
            let mut hash: HashMap<&'static str, TokenType> = HashMap::new();
            $(
                hash.insert($key, $value);
            )*
            hash
        }
    };
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
            keywords: static_keywords_map!(
                "and" => TokenType::And,
                "class" => TokenType::Class,
                "else" => TokenType::Else,
                "false" => TokenType::False,
                "for" => TokenType::For,
                "fun" => TokenType::Fun,
                "if" => TokenType::If,
                "null" => TokenType::Null,
                "or" => TokenType::Or,
                "print" => TokenType::Print,
                "return" => TokenType::Return,
                "super" => TokenType::Super,
                "this" => TokenType::This,
                "true" => TokenType::True,
                "var" => TokenType::Var,
                "while" => TokenType::While
            ),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.eof() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, "", self.line));
        self.tokens.clone()
    }

    fn eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LParen),
            ')' => self.add_token(TokenType::RParen),
            '{' => self.add_token(TokenType::LBrace),
            '}' => self.add_token(TokenType::RBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => match self.expect('=') {
                true => self.add_token(TokenType::BangEqual),
                false => self.add_token(TokenType::Bang)
            },
            '=' => match self.expect('=') {
                true => self.add_token(TokenType::EqualEqual),
                false => self.add_token(TokenType::Equal)
            },
            '<' => match self.expect('=') {
                true => self.add_token(TokenType::LessEqual),
                false => self.add_token(TokenType::Less)
            },
            '>' => match self.expect('=') {
                true => self.add_token(TokenType::GreaterEqual),
                false => self.add_token(TokenType::Greater)
            },
            '/' => {
                if self.expect('/') {
                    while self.peek() != '\n' && !self.eof() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            },
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            '"' => self.string(),
            _ if is_digit(c) => self.number(),
            _ if is_alpha(c) => self.identifier(),
            _ => error(self.line, "Unexpected Character"),
        }
    }

    fn identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].to_string();
        let type_: Option<&TokenType> = self.keywords.get(text.as_str());

        match type_ {
            Some(keyword) => self.add_token(*keyword),
            None => self.add_token(TokenType::Identifier)
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

        self.add_token_lit(
            TokenType::Number, 
            Some(self.source[self.start..self.current].to_string().parse::<f64>()
                .expect("Failed to parse number to f64."))
        );
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.eof() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.eof() {
            error(self.line, "Unterminated String");
        }
        self.advance();

        let value: String = self.source[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token_lit(TokenType::String, Some(value));
    }

    fn peek(&self) -> char {
        match self.eof() {
            true => '\0',
            false => self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        match self.current + 1 >= self.source.len() {
            true => '\0',
            false => self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn expect(&mut self, expected: char) -> bool {
        if self.eof() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }
    
    fn advance(&mut self) -> char {
        let res: char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        res
    }

    fn add_token(&mut self, type_: TokenType) {
        self.add_token_lit(type_, None::<Box<dyn CloneAny>>);
    }

    fn add_token_lit(&mut self, type_: TokenType, literal: Option<impl CloneAny>) {
        let text: String = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new_lit(type_, &text, literal, self.line));
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
} 

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}