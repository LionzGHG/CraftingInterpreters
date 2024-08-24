
use ast::{Binary, Echo, Expr, Expression, Grouping, Literal, Stmt, Unary, Var, Variable};

use crate::lexer::tokens::{Token, TokenType};
use crate::util::Value;

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

    pub fn parse(&mut self) -> Vec<Box<dyn Stmt>> {
        let mut stmts: Vec<Box<dyn Stmt>> = Vec::new();
        while !self.eof() {
            stmts.push(self.declaration());
        }

        return stmts;
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

    fn go_back(&mut self) -> Token {
        self.current -= 1;
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

    fn declaration(&mut self) -> Box<dyn Stmt> {
        if self.expect(&[TokenType::Set]) {
            if self.peek().type_ == TokenType::Mut {
                return self.var_declaration(true, true);
            }
            return self.var_declaration(false, true);
        }
        if self.expect(&[TokenType::Identifier]) && (self.peek().type_ == TokenType::Identifier || self.peek().type_ == TokenType::Mut) {
            if self.peek().type_ == TokenType::Mut {
                return self.var_declaration(true, false);
            }
            return self.var_declaration(false, false);
        }

        return self.statement();
    }

    fn var_declaration(&mut self, mutable: bool, inferred: bool) -> Box<dyn Stmt> {
        if mutable {
            self.next(); // skip the "mut" keyword
        }
    
        let (type_, name) = if inferred {
            (None, self.consume(TokenType::Identifier, "Expect variable name."))
        } else {
            let type_: Option<Token> = Some(self.tokens.iter().nth(self.current - 1).expect("Expect variable type").clone());
            let name: Token = self.consume(TokenType::Identifier, "Expect variable name.");
            (type_, name)
        };
    
        let mut initializer: Option<Box<dyn Expr>> = None;
        if self.expect(&[TokenType::Equal]) {
            initializer = Some(self.expression());
        }
    
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.");
    
        if let Some(type_) = type_ {
            return Box::new(Var::typed(type_, mutable, name, initializer));
        }
    
        Box::new(Var::inferred(mutable, name, initializer))
    }
    

    fn statement(&mut self) -> Box<dyn Stmt> {
        if self.expect(&[TokenType::Echo]) {
            return self.echo_statement();
        }

        return self.expression_statement();
    }

    fn echo_statement(&mut self) -> Box<dyn Stmt> {
        let value: Box<dyn Expr> = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        return Box::new(Echo::new(value));
    }

    fn expression_statement(&mut self) -> Box<dyn Stmt> {
        let expr: Box<dyn Expr> = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        return Box::new(Expression::new(expr));
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
            return Box::new(Literal::new(Some(Value::Boolean(false))));
        }

        if self.expect(&[TokenType::True]) {
            return Box::new(Literal::new(Some(Value::Boolean(true))));
        }

        if self.expect(&[TokenType::Number, TokenType::String]) {
            return Box::new(Literal::new(self.back().literal));
        }

        if self.expect(&[TokenType::Identifier]) {
            return Box::new(Variable::new(self.back()));
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