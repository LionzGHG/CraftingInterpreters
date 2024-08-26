use std::{any::{type_name_of_val, TypeId}, collections::HashMap, io};

use environment::{Environment, VarAttrib};

use crate::{lexer::tokens::{Token, TokenType}, parser::ast::{Expr, Stmt, Visitor}, /*util::{downcast_obj, downcast_to, downcast_to_f64, Number, Object*/};
use crate::util::{Value, error::Error};

pub mod environment;

pub struct Interpreter {
    environment: Environment
}

impl Interpreter {

    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    fn evaluate(&mut self, expr: &dyn Expr) -> Value {
        return expr.accept(self);
    }

    fn is_truthy(&self, object: Value) -> bool {
        if let Value::Boolean(b) = object {
            return b;
        }
        return true;
    }

    pub fn interpret(&mut self, stmts: Vec<Box<dyn Stmt>>) {
        for stmt in stmts {
            self.execute(stmt);
        }
    }

    fn execute(&mut self, stmt: Box<dyn Stmt>) {
        stmt.accept(self);
    }

    fn execute_block(&mut self, statements: &Vec<Box<dyn Stmt>>, environment: Environment) {
        let previous: Environment = self.environment.clone();
        self.environment = environment;

        for statement in statements {
            statement.accept(self);
        }

        self.environment = previous;
    }
}


impl Visitor for Interpreter {

    fn visit_literal(&self, literal: &crate::parser::ast::Literal) -> Value {
        return literal.value.clone().expect("no literal expr found");
    }

    fn visit_grouping(&mut self, grouping: &crate::parser::ast::Grouping) -> Value {
        return self.evaluate(&*grouping.expression);
    }

    fn visit_unary(&mut self, unary: &crate::parser::ast::Unary) -> Value {
        let right: Value = self.evaluate(&*unary.right);

        match unary.operator.type_ {
            TokenType::Minus => {
                if let Value::Float(value) = right {
                    return Value::Float(-value);
                }
                Error::number_operand(unary.operator.line);
            },
            TokenType::Bang => {
                return Value::Boolean(!self.is_truthy(right));
            },
            _ => Error::unexpected_token(unary.operator.line, unary.operator.type_),
        }
    }

    fn visit_binary(&mut self, binary: &crate::parser::ast::Binary) -> Value {
        let lhs: Value = self.evaluate(&*binary.left);
        let rhs: Value = self.evaluate(&*binary.right);

        let x: f64 = match lhs {
            Value::Float(n) => n,
            Value::Integer(n) => n as f64,
            _ => panic!()
        };
        let y: f64 = match rhs {
            Value::Float(n) => n,
            Value::Integer(n) => n as f64,
            _ => panic!()
        };

        return match binary.operator.type_ {
            // arithmetic
            TokenType::Minus => Value::Float(x-y),
            TokenType::Plus => Value::Float(x+y),
            TokenType::Slash => Value::Float(x/y),
            TokenType::Star => Value::Float(x*y),
            // comparison
            TokenType::Greater => Value::Boolean(x>y),
            TokenType::GreaterEqual => Value::Boolean(x>=y),
            TokenType::Less => Value::Boolean(x<y),
            TokenType::LessEqual => Value::Boolean(x<=y),
            TokenType::EqualEqual => Value::Boolean(x==y),
            TokenType::BangEqual => Value::Boolean(x!=y),
            // else
            _ => Error::unexpected_token(binary.operator.line, binary.operator.type_),
        }
    }

    fn visit_variable(&self, variable: &crate::parser::ast::Variable) -> Value {
        return self.environment.get(variable.name.clone()).1.unwrap();
    }

    
    fn visit_assign(&mut self, assign: &crate::parser::ast::Assign) -> Value {
        let value: Value = self.evaluate(&*assign.value);
        self.environment.assign(assign.name.clone(), &value);
        return value;
    }

    fn visit_expr_stmt(&mut self, expr: &crate::parser::ast::Expression) {
        self.evaluate(&*expr.expr);
    }

    fn visit_echo_stmt(&mut self, echo: &crate::parser::ast::Echo) {
        let value: Value = self.evaluate(&*echo.expr);
        println!("{value}");
    }

    fn visit_block_stmt(&mut self, block: &crate::parser::ast::Block) {
        self.execute_block(&block.statements, Environment::with_enclosing(self.environment.clone()));
    }

    fn visit_var_decl(&mut self, var: &crate::parser::ast::Var) {
        let mut value: Option<Value> = None;
        if let Some(n) = &var.expr {
            value = Some(self.evaluate(&**n));
        }

        // type checking
        self.match_types(var, value.clone());

        self.environment.define(var.name.lexeme.clone(), VarAttrib(var.datatype.clone(), value, var.mutability));
        println!("{:?}", self.environment.map);
    }
}

impl Interpreter {
    fn match_types(&mut self, var: &crate::parser::ast::Var, value: Option<Value>) {
        if let Some(type_) = &var.datatype {
            if let TokenType::Identifier = type_.type_ {
                if let Some(value) = value {
                    if value.is_i32() {
                        self.assert_type(&type_.lexeme, &["i32", "u32"]);
                    }
                    if value.is_f64() {
                        self.assert_type(&type_.lexeme, &["f64", "f32"]);
                    }
                    if value.is_boolean() {
                        self.assert_type(&type_.lexeme, &["boolean"]);
                    }
                    if value.is_string() {
                        self.assert_type(&type_.lexeme, &["String", "string"]);
                    }
                }
            }
        }
    }

    fn assert_type(&self, input: &String, expected: &[&str]) {
        if !expected.contains(&input.as_str()) {
            Error::type_mismatch(&input, &expected);
        }
    }
}
