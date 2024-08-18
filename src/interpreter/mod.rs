
use crate::{lexer::tokens::{Token, TokenType}, parser::ast::{Expr, Visitor}};

pub struct Interpreter;

impl Interpreter {
    fn evaluate(&self, expr: &dyn Expr) -> String {
        return expr.accept(self);
    }

    fn is_truthy(&self, literal: String) -> String {
        if literal == "null".to_string() {
            return "false".to_string();
        }
        if literal == "true".to_string() {
            return "true".to_string();
        }
        if literal == "false".to_string() {
            return "false".to_string();
        }
        return "true".to_string();
    }

    fn is_not_truthy(&self, lit: String) -> String {
        match lit.as_str() {
            "null" => true.to_string(),
            "true" => false.to_string(),
            "false" => true.to_string(),
            _ => false.to_string()
        }
    }

    fn is_equal(&self, lhs: String, rhs: String) -> String {
        if lhs == "null".to_string() && rhs == "null".to_string() {
            return true.to_string();
        }
        if lhs == "null".to_string() {
            return false.to_string();
        }
        return lhs.eq(&rhs).to_string();
    }

    fn is_not_equal(&self, lhs: String, rhs: String) -> String {
        if lhs == "null".to_string() && rhs == "null".to_string() {
            return false.to_string();
        }
        if lhs == "null".to_string() {
            return true.to_string();
        }
        return lhs.ne(&rhs).to_string();
    }

    fn check_number_operand(&self, operand: Token, expr: &dyn Expr) {
        
    }
}

impl Visitor for Interpreter {
    
    fn visit_literal(&self, literal: &crate::parser::ast::Literal) -> String {
        return literal.value.as_ref().unwrap().to_string();
    }
    
    fn visit_grouping(&self, grouping: &crate::parser::ast::Grouping) -> String {
        return self.evaluate(&*grouping);
    }
    
    fn visit_unary(&self, unary: &crate::parser::ast::Unary) -> String {
        let rhs: String = unary.accept(self);

        return match unary.operator.type_ {
            TokenType::Minus => {
                self.check_number_operand(unary.operator.clone(), &*unary.right);
                format!("-{rhs}")
            },
            TokenType::Bang => self.is_not_truthy(rhs),
            _ => panic!()
        }
    }

    fn visit_binary(&self, binary: &crate::parser::ast::Binary) -> String {
        let lhs: String = self.evaluate(&*binary.left);
        let rhs: String = self.evaluate(&*binary.right);

        return match binary.operator.type_ {
            TokenType::Minus => format!("{lhs}-{rhs}"),
            TokenType::Plus => format!("{lhs}+{rhs}"),
            TokenType::Slash => format!("{lhs}/{rhs}"),
            TokenType::Star => format!("{lhs}*{rhs}"),
            TokenType::Greater => format!("{lhs}>{rhs}"),
            TokenType::GreaterEqual => format!("{lhs}>={rhs}"),
            TokenType::Less => format!("{lhs}<{rhs}"),
            TokenType::LessEqual => format!("{lhs}<={rhs}"),
            TokenType::BangEqual => self.is_not_equal(lhs, rhs),
            TokenType::EqualEqual => self.is_equal(lhs, rhs),
            _ => panic!()
        }
    }
}