
pub mod tokens;

use std::{collections::HashMap, fmt::Debug};

use tokens::{Token, TokenType};

fn error(line: usize, msg: &str) -> ! {
    crate::report(line, "", msg)
}

macro_rules! map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut hash: HashMap<&'static str, TokenType> = HashMap::new();
            $(
                hash.insert($key, $value);
            )*
            hash
        }
    };
}

#[derive(Debug, Clone)]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: map! {
                "mut" => TokenType::Mut,
                "typeof" => TokenType::Typeof,
                "sizeof" => TokenType::Sizeof,
                "nameof" => TokenType::Nameof,
                "as" => TokenType::As,
                "void" => TokenType::Void,
                "use" => TokenType::Use,
                "with" => TokenType::With,
                "out" => TokenType::Out,
                "true" => TokenType::True,
                "false" => TokenType::False,
                "if" => TokenType::If,
                "elif" => TokenType::Elif,
                "else" => TokenType::Else,
                "while" => TokenType::While,
                "for" => TokenType::For,
                "in" => TokenType::In,
                "entity" => TokenType::Entity,
                "init" => TokenType::Init,
                "new" => TokenType::New,
                "this" => TokenType::This,
                "set" => TokenType::Set,
                "enum" => TokenType::Enum,
                "throw" => TokenType::Throw,
                "catch" => TokenType::Catch,
                "pub" => TokenType::Pub,
                "priv" => TokenType::Priv,
                "prot" => TokenType::Prot,
                "unreachable" => TokenType::Unreachable,
                "todo" => TokenType::Todo,
                "pass" => TokenType::Pass,
                "test" => TokenType::Test,
                "trait" => TokenType::Trait,
                "parent" => TokenType::Parent,
                "open" => TokenType::Open,
                "override" => TokenType::Override,
                "scene" => TokenType::Scene,
                "import" => TokenType::Import,
                "echo" => TokenType::Echo,
                "try" => TokenType::Try
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        while !self.eof() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
        return self.tokens.clone();
    }

    fn eof(&self) -> bool {
        self.current >= self.source.len() 
    }
    
    fn next(&mut self) -> char {
        let c: char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, type_: TokenType) {
        self.add_token_lit(type_, None);
    }

    fn add_token_lit(&mut self, type_: TokenType, literal: Option<String>) {
        let text: String = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(type_, text, literal, self.line));
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

    fn peek(&self) -> char {
        if self.eof() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn scan_token(&mut self) {
        println!("Checking: {}", self.source.chars().nth(self.current).unwrap());

        let c: char = self.next();
        match c {
            '(' => self.add_token(TokenType::LParen),
            ')' => self.add_token(TokenType::RParen),
            '{' => self.add_token(TokenType::LBrace),
            '}' => self.add_token(TokenType::RBrace),
            '[' => self.add_token(TokenType::LSquare),
            ']' => self.add_token(TokenType::RSquare),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => {
                if self.expect('=') {
                    self.add_token(TokenType::MinusEqual)
                } else if self.expect('>') {
                    self.add_token(TokenType::Arrow)
                } else if self.expect('-') { // <-- comment
                    while self.peek() != '\n' && !self.eof() {
                        self.next();
                    }
                }
                else {
                    self.add_token(TokenType::Minus)
                }
            }
            '+' => match self.expect('=') {
                true => self.add_token(TokenType::PlusEqual),
                _ => self.add_token(TokenType::Plus)
            },
            '/' => match self.expect('=') {
                true => self.add_token(TokenType::DivEqual),
                _ => self.add_token(TokenType::Slash)
            },
            '*' => match self.expect('=') {
                true => self.add_token(TokenType::TimesEqual),
                _ => self.add_token(TokenType::Star)
            },
            ';' => self.add_token(TokenType::Semicolon),
            ':' => self.add_token(TokenType::Colon),
            '?' => self.add_token(TokenType::Questionmark),
            '!' => match self.expect('=') {
                true => self.add_token(TokenType::BangEqual),
                _ => self.add_token(TokenType::Bang)
            }
            '=' => match self.expect('=') {
                true => self.add_token(TokenType::EqualEqual),
                _ => self.add_token(TokenType::Equal)
            }
            '>' => match self.expect('=') {
                true => self.add_token(TokenType::GreaterEqual),
                _ => self.add_token(TokenType::Greater)
            }
            '<' => match self.expect('=') {
                true => self.add_token(TokenType::LessEqual),
                _ => self.add_token(TokenType::Less)
            }
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            '"' => self.string(),
            _ if is_digit(c) => self.number(),
            _ if is_alpha(c) => self.identifier(),
            _ => {
                error(self.line, "Unexpected Character");
            }
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.eof() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.next();
        }

        if self.eof() {
            error(self.line, "Unterminated String");
        }
        self.next();

        let value: String = self.source[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token_lit(TokenType::String, Some(value));
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            println!("current = {}", self.source.chars().nth(self.current).unwrap());
            self.next();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.next();

            while is_digit(self.peek()) {
                self.next();
            }
        }

        let value: String = self.source[self.start..self.current].to_string();
        self.add_token_lit(TokenType::Number, Some(value));
    }

    fn identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.next();
        }

        let text: String = self.source[self.start..self.current].to_string().to_string();
        let type_: Option<&TokenType> = self.keywords.get(text.as_str());

        match type_ {
            Some(keyword) => self.add_token(*keyword),
            None => self.add_token(TokenType::Identifier)
        }
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}