use std::fmt::Debug;

use crate::lexer::tokens::Token;

pub trait Visitor {
    fn visit_binary(&self, binary: &Binary) -> String;
    fn visit_grouping(&self, grouping: &Grouping) -> String;
    fn visit_literal(&self, literal: &Literal) -> String;
    fn visit_unary(&self, unary: &Unary) -> String;
}

pub trait Expr {
    fn accept(&self, visitor: &dyn Visitor) -> String;
}

#[derive()]
pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}

impl Binary {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
        Self { left, operator, right }
    }
}

impl Expr for Binary {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_binary(self)
    }
}

pub struct Grouping {
    pub expression: Box<dyn Expr>
}

impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Self { expression }
    }
}
impl Expr for Grouping {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_grouping(self)
    }
}

pub struct Literal {
    pub value: Option<String>,
}

impl Literal {
    pub fn new(value: Option<String>) -> Self {
        Self { value }
    }
}
impl Expr for Literal {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_literal(self)
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>
}

impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Self {
        Self { operator, right }
    }
}
impl Expr for Unary {
    fn accept(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_unary(self)
    }
}