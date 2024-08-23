
use crate::{lexer::tokens::Token, /*util::Object*/ util::Value};

pub trait Visitor {
    fn visit_binary(&self, binary: &Binary) -> Value;
    fn visit_grouping(&self, grouping: &Grouping) -> Value;
    fn visit_literal(&self, literal: &Literal) -> Value;
    fn visit_unary(&self, unary: &Unary) -> Value;
}

pub trait Expr {
    fn accept(&self, visitor: &dyn Visitor) -> Value;
}

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
    fn accept(&self, visitor: &dyn Visitor) -> Value {
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
    fn accept(&self, visitor: &dyn Visitor) -> Value {
        visitor.visit_grouping(self)
    }
}

pub struct Literal {
    pub value: Option<Value>,
}

impl Literal {
    pub fn new(value: Option<Value>) -> Self {
        Self { value }
    }
}
impl Expr for Literal {
    fn accept(&self, visitor: &dyn Visitor) -> Value {
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
    fn accept(&self, visitor: &dyn Visitor) -> Value {
        visitor.visit_unary(self)
    }
}