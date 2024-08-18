use std::ops::Deref;

use ast::{Binary, Expr, Grouping, Literal, Unary};

use crate::lexer::tokens::{Token, TokenType};

fn error(token: Token, msg: &str) -> ! {
    if token.type_ == TokenType::EOF {
        crate::report(token.line, &" at end", &msg);
    } else {
        crate::report(token.line, format!(" at '{}'", token.lexeme).as_str(), &msg);
    }
}

pub mod ast;
pub mod ast_printer;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Box<dyn Expr> {
        return self.expression();
    }

    fn expect(&mut self, types: &[TokenType]) -> bool {
        for type_ in types {
            if self.check(*type_) {
                self.next();
                return true;
            }
        }

        return false;
    }

    fn check(&self, type_: TokenType) -> bool {
        if self.eof() {
            return false;
        }
        return self.peek().type_ == type_;
    }

    fn next(&mut self) -> Token {
        if !self.eof() {
            self.current += 1;
        }
        return self.back();
    }

    fn eof(&self) -> bool {
        self.peek().type_ == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn back(&mut self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn consume(&mut self, type_: TokenType, msg: &str) -> Token {
        if self.check(type_) {
            return self.next();
        }
        error(self.peek(), msg);
    }
    
    fn expression(&mut self) -> Box<dyn Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Box<dyn Expr> {
        let mut expr: Box<dyn Expr> = self.comparison();
    
        while self.expect(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.back();
            let right: Box<dyn Expr> = self.comparison();
            expr = Box::new(Binary::new(expr, operator, right));
        }
    
        expr
    }

    fn comparison(&mut self) -> Box<dyn Expr> {
        let mut expr: Box<dyn Expr> = self.term();

        while self.expect(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator: Token = self.back();
            let right: Box<dyn Expr> = self.term();
            expr = Box::new(Binary::new(expr, operator, right));
        }

        expr
    }

    fn term(&mut self) -> Box<dyn Expr> {
        let mut expr: Box<dyn Expr> = self.factor();

        while self.expect(&[TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.back();
            let right: Box<dyn Expr> = self.factor();
            expr = Box::new(Binary::new(expr, operator, right));
        }

        expr
    }

    fn factor(&mut self) -> Box<dyn Expr> {
        let mut expr: Box<dyn Expr> = self.unary();

        while self.expect(&[TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.back();
            let right: Box<dyn Expr> = self.unary();
            expr = Box::new(Binary::new(expr, operator, right));
        }

        expr
    }

    fn unary(&mut self) -> Box<dyn Expr> {
        if self.expect(&[TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.back();
            let right: Box<dyn Expr> = self.unary();
            return Box::new(Unary::new(operator, right));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Box<dyn Expr> {
        if self.expect(&[TokenType::False]) {
            return Box::new(Literal::new(Some(false.to_string())));
        }

        if self.expect(&[TokenType::True]) {
            return Box::new(Literal::new(Some(true.to_string())));
        }

        if self.expect(&[TokenType::Number, TokenType::String]) {
            return Box::new(Literal::new(self.back().literal));
        }

        if self.expect(&[TokenType::LParen]) {
            let expr: Box<dyn Expr> = self.expression();
            self.consume(TokenType::RParen, "Expect ')' after expression");
            return Box::new(Grouping::new(expr));
        }

        error(self.peek(), "Expect Expression");
    }

    fn synchronize(&mut self) {
        self.next();

        while !self.eof() {
            if self.back().type_ == TokenType::Semicolon {
                return;
            }
        }



        match self.peek().type_ {
            TokenType::Entity | TokenType::Trait | TokenType::Set | TokenType::Catch | TokenType::If |
            TokenType::Else | TokenType::Elif | TokenType::While | TokenType::Unreachable | TokenType::Void |
            TokenType::Typeof | TokenType::Nameof | TokenType::Sizeof | TokenType::Echo |
            TokenType::Todo | TokenType::Test | TokenType::Override | TokenType::Open | TokenType::Scene => {
                return;
            },
            _ => {}
        }

        self.next();
    }
}