
use crate::{lexer::tokens::Token, util::Object};

pub trait Visitor {
    fn visit_binary(&self, binary: &Binary) -> Box<dyn Object>;
    fn visit_grouping(&self, grouping: &Grouping) -> Box<dyn Object>;
    fn visit_literal(&self, literal: &Literal) -> Box<dyn Object>;
    fn visit_unary(&self, unary: &Unary) -> Box<dyn Object>;
}

pub trait Expr {
    fn accept(&self, visitor: &dyn Visitor) -> Box<dyn Object>;
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
    fn accept(&self, visitor: &dyn Visitor) -> Box<dyn Object> {
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
    fn accept(&self, visitor: &dyn Visitor) -> Box<dyn Object> {
        visitor.visit_grouping(self)
    }
}

pub struct Literal {
    pub value: Option<Box<dyn Object>>,
}

impl Literal {
    pub fn new(value: Option<Box<dyn Object>>) -> Self {
        Self { value }
    }
}
impl Expr for Literal {
    fn accept(&self, visitor: &dyn Visitor) -> Box<dyn Object> {
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
    fn accept(&self, visitor: &dyn Visitor) -> Box<dyn Object> {
        visitor.visit_unary(self)
    }
}