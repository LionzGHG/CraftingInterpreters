use crate::{lexer::tokens::{Token, TokenType}, parser::ast::{Expr, Visitor}, util::{downcast_obj, Object}};


pub struct Interpreter;

impl Interpreter {
    fn evaluate(&self, expr: &dyn Expr) -> Box<dyn Object> {
        return expr.accept(self);
    }

    fn is_truthy(&self, object: &dyn Object) -> bool {
        if let Some(value) = downcast_obj::<bool>(object).cloned() {
            return value;
        }
        return true;
    }

    pub fn interpret(&self, expr: &dyn Expr) {
        let value: Box<dyn Object> = self.evaluate(&*expr);
        println!("{}", value.as_string());
    }
}


impl Visitor for Interpreter {

    fn visit_literal(&self, literal: &crate::parser::ast::Literal) -> Box<dyn Object> {
        return literal.value.clone_box();
    }

    fn visit_grouping(&self, grouping: &crate::parser::ast::Grouping) -> Box<dyn Object> {
        return self.evaluate(&*grouping.expression);
    }

    fn visit_unary(&self, unary: &crate::parser::ast::Unary) -> Box<dyn Object> {
        let right: Box<dyn Object> = self.evaluate(&*unary.right);

        match unary.operator.type_ {
            TokenType::Minus => {
                if let Some(value) = downcast_obj::<f64>(&*right).cloned() {
                    return Box::new(-value);
                }
                Error::number_operand(unary.operator.line);
            },
            TokenType::Bang => {
                return Box::new(!self.is_truthy(&*right));
            },
            _ => Error::unexpected_token(unary.operator.line, unary.operator.type_),
        }
    }

    fn visit_binary(&self, binary: &crate::parser::ast::Binary) -> Box<dyn Object> {
        let lhs: Box<dyn Object> = self.evaluate(&*binary.left);
        let rhs: Box<dyn Object> = self.evaluate(&*binary.right);

        let x: f64 = lhs.as_f64().unwrap_or_else(|| {
            Error::unexpected_type(binary.operator.line, &*lhs);
        });
        let y: f64 = rhs.as_f64().unwrap_or_else(|| {
            Error::unexpected_type(binary.operator.line, &*rhs);
        });

        return match binary.operator.type_ {
            // arithmetic
            TokenType::Minus => Box::new(x-y),
            TokenType::Plus => Box::new(x+y),
            TokenType::Slash => Box::new(x/y),
            TokenType::Star => Box::new(x*y),
            // comparison
            TokenType::Greater => Box::new(x>y),
            TokenType::GreaterEqual => Box::new(x>=y),
            TokenType::Less => Box::new(x<y),
            TokenType::LessEqual => Box::new(x<=y),
            TokenType::EqualEqual => Box::new(x==y),
            TokenType::BangEqual => Box::new(x!=y),
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

    fn unexpected_type(line: usize, type_: &dyn Object) -> ! {
        panic!("[line {line}] Error: Expected type 'float', got value of '{}'", type_.as_string());
    }
}