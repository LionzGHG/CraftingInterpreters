use ast::{Expr, Operator};

use crate::{report, scanner::tokens::{Token, TokenType}};


pub mod ast;

#[derive(Debug, Clone)]
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

    pub fn parse(&mut self) -> Expr {
        return self.expression();
    }

    fn expect(&mut self, types: &[TokenType]) -> bool {
        for type_ in types {
            if self.check(*type_) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&mut self, type_: TokenType) -> bool {
        if self.eof() {
            return false;
        }
        self.peek().type_ == type_
    }
    
    fn advance(&mut self) -> Token {
        if !self.eof() {
            self.current += 1;
        }
        self.previous()
    }

    fn eof(&mut self) -> bool {
        self.peek().type_ == TokenType::EOF
    }

    fn peek(&mut self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn as_operator(&self, token: Token) -> Operator {
        match token.type_ {
            TokenType::Plus => Operator::Plus,
            TokenType::Minus => Operator::Minus,
            TokenType::Slash => Operator::Divide,
            TokenType::Star => Operator::Multiply,
            TokenType::EqualEqual => Operator::EqualEqual,
            TokenType::BangEqual => Operator::NotEqual,
            TokenType::Bang => Operator::Not,
            TokenType::Less => Operator::Less,
            TokenType::Greater => Operator::Greater,
            TokenType::LessEqual => Operator::LessEqual,
            TokenType::GreaterEqual => Operator::GreaterEqual,
            _ => panic!("TokenType '{}' is not a valid Operator!", token.type_),
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.expect(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::Binary(Box::new(expr), self.as_operator(operator), Box::new(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.expect(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator: Token = self.previous();
            let right: Expr = self.term();
            expr = Expr::Binary(Box::new(expr), self.as_operator(operator), Box::new(right));
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.expect(&[TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous();
            let right: Expr = self.factor();
            expr = Expr::Binary(Box::new(expr), self.as_operator(operator), Box::new(right));
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.expect(&[TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            expr = Expr::Binary(Box::new(expr), self.as_operator(operator), Box::new(right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.expect(&[TokenType::Minus, TokenType::Bang]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary();
            return Expr::Unary(self.as_operator(operator), Box::new(right)); 
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        
        if self.expect(&[TokenType::False]) {
            return Expr::Literal(Box::new(false));
        }

        if self.expect(&[TokenType::True]) {
            return Expr::Literal(Box::new(true));
        }

        if self.expect(&[TokenType::Null]) {
            return Expr::Literal(Box::new("null"));
        }

        if self.expect(&[TokenType::Number, TokenType::String]) {
            return Expr::Literal(Box::new(self.previous().literal));
        }

        if self.expect(&[TokenType::LParen]) {
            let mut expr: Expr = self.expression();
            self.consume(TokenType::RParen, "Expect ')' after expression.");
            return Expr::Grouping(Box::new(expr));
        }

        error(self.peek(), "Expect Expression.");
    }

    fn consume(&mut self, type_: TokenType, msg: &str) -> Token {
        if self.check(type_) {
            return self.advance();
        }
        error(self.peek(), msg);
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.eof() {
            if self.previous().type_ == TokenType::Semicolon {
                return;
            }

            match self.peek().type_ {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If | TokenType::While |
                TokenType::Print | TokenType::Return => return,
                _ => self.advance(),
            };
        }
    }
}

fn error(token: Token, msg: &str) -> ! {
    if token.type_ == TokenType::EOF {
        report(token.line, " at end", msg);
    } else {
        report(token.line, format!(" at '{}'", token.lexeme).as_str(), msg);
    }
}