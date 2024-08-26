
use std::{any::Any, ffi::FromVecWithNulError};

use crate::{lexer::tokens::Token, /*util::Object*/ util::Value};

pub trait Visitor {
    fn visit_binary(&mut self, binary: &Binary) -> Value;
    fn visit_grouping(&mut self, grouping: &Grouping) -> Value;
    fn visit_literal(&self, literal: &Literal) -> Value;
    fn visit_unary(&mut self, unary: &Unary) -> Value;
    fn visit_variable(&self, variable: &Variable) -> Value;
    fn visit_assign(&mut self, assign: &Assign) -> Value;

    fn visit_expr_stmt(&mut self, expr: &Expression);
    fn visit_echo_stmt(&mut self, echo: &Echo);
    fn visit_block_stmt(&mut self, block: &Block);
    
    fn visit_var_decl(&mut self, var: &Var);
}

pub trait Expr {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value;
    fn as_any(&self) -> &dyn Any;
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
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_binary(self)
    }
    fn as_any(&self) -> &dyn Any {
        self
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
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_grouping(self)
    }
    fn as_any(&self) -> &dyn Any {
        self
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
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_literal(self)
    }
    fn as_any(&self) -> &dyn Any {
        self
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
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_unary(self)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}

impl Expr for Variable {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_variable(self)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub struct Assign {
    pub name: Token,
    pub value: Box<dyn Expr>,
}

impl Assign {
    pub fn new(name: Token, value: Box<dyn Expr>) -> Self {
        Self { name, value }
    }
}

impl Expr for Assign {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_assign(self)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub trait Stmt {
    fn accept(&self, visitor: &mut dyn Visitor);
}

pub struct Expression {
    pub expr: Box<dyn Expr>,
}

impl Expression {
    pub fn new(expr: Box<dyn Expr>) -> Expression {
        Self { expr }
    }
}
impl Stmt for Expression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_expr_stmt(self);
    }
}

pub struct Echo {
    pub expr: Box<dyn Expr>,
}

impl Echo {
    pub fn new(expr: Box<dyn Expr>) -> Echo {
        Self { expr }
    }
}
impl Stmt for Echo {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_echo_stmt(self);
    }
}

pub struct Var {
    pub datatype: Option<Token>,
    pub mutability: bool,
    pub name: Token,
    pub expr: Option<Box<dyn Expr>>,
}

impl Var {
    pub fn inferred(mut_: bool, name: Token, expr: Option<Box<dyn Expr>>) -> Var {
        Self {
            datatype: None,
            mutability: mut_,
            name,
            expr
        }
    }
    pub fn typed(datatype: Token, mut_: bool, name: Token, expr: Option<Box<dyn Expr>>) -> Var {
        Self {
            datatype: Some(datatype),
            mutability: mut_,
            name,
            expr,
        }
    }
}

impl Stmt for Var {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_var_decl(self);
    }
}

pub struct Block {
    pub statements: Vec<Box<dyn Stmt>>,
}

impl Block {
    pub fn new(statements: Vec<Box<dyn Stmt>>) -> Self {
        Self { statements }
    }
}

impl Stmt for Block {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_block_stmt(self);
    }
}