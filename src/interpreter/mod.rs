use std::{any::{type_name_of_val, TypeId}, io};

use crate::{lexer::tokens::{Token, TokenType}, parser::ast::{Expr, Visitor}, /*util::{downcast_obj, downcast_to, downcast_to_f64, Number, Object*/};
use crate::util::Value;

pub struct Interpreter;

impl Interpreter {
    fn evaluate(&self, expr: &dyn Expr) -> Value {
        return expr.accept(self);
    }

    fn is_truthy(&self, object: Value) -> bool {
        if let Value::Boolean(b) = object {
            return b;
        }
        return true;
    }

    pub fn interpret(&self, expr: &dyn Expr) {
        let value: Value = self.evaluate(&*expr);
        println!("{}", value);
    }
}


impl Visitor for Interpreter {

    fn visit_literal(&self, literal: &crate::parser::ast::Literal) -> Value {
        return literal.value.clone().expect("no literal expr found");
    }

    fn visit_grouping(&self, grouping: &crate::parser::ast::Grouping) -> Value {
        return self.evaluate(&*grouping.expression);
    }

    fn visit_unary(&self, unary: &crate::parser::ast::Unary) -> Value {
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

    fn visit_binary(&self, binary: &crate::parser::ast::Binary) -> Value {
        let lhs: Value = self.evaluate(&*binary.left);
        let rhs: Value = self.evaluate(&*binary.right);

        let x: f64 = if let Value::Float(f) = lhs { f } else { 0. };
        let y: f64 = if let Value::Float(f) = rhs { f } else { 0. };

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
}

struct Error;

impl Error {
    fn number_operand(line: usize) -> ! {
        panic!("[line {line}] Error: Expected number after Operand in Expression.");
    }

    fn unexpected_token(line: usize, token: TokenType) -> ! {
        panic!("[line {line}] Error: Unexpected Token '{token:?}'.");
    }

    fn unexpected_type(line: usize, type_: Value) -> ! {
        panic!("[line {line}] Error: Expected type 'float', got value of '{}'", type_);
    }

    fn fatal() -> ! {
        panic!("Fatal Error");
    }
}
