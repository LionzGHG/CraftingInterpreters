// use crate::util::Object;
use crate::util::Value;

use super::ast::{Expr, Visitor};


pub struct AstPrinter();

impl AstPrinter {
    pub fn print(&self, expr: &dyn Expr) -> Value {
        return expr.accept(self)
    }

    fn parenthesize(&self, name: String, exprs: &[&dyn Expr]) -> String {
        let mut builder: String = String::new();

        builder.push('(');
        builder.push_str(&name);

        for expr in exprs.to_vec() {
            builder.push(' ');
            builder.push_str(&expr.accept(self).to_string());
        }
        
        builder.push(')');

        builder
    }
}

impl Visitor for AstPrinter {
    fn visit_binary(&self, binary: &super::ast::Binary) -> Value {
        Value::String(self.parenthesize(binary.operator.lexeme.clone(), &[&*binary.left, &*binary.right]))
    }

    fn visit_grouping(&self, grouping: &super::ast::Grouping) -> Value {
        Value::String(self.parenthesize("group".to_string(), &[&*grouping.expression]))
    }

    fn visit_literal(&self, literal: & super::ast::Literal) -> Value {
        return match &literal.value {
            Some(value) => value.clone(),
            None => Value::Null, 
        }
    }

    fn visit_unary(&self, unary: &super::ast::Unary) -> Value {
        Value::String(self.parenthesize(unary.operator.lexeme.clone(), &[&*unary.right]))
    }

    fn visit_echo_stmt(&self, echo: &super::ast::Echo) {
        todo!()
    }
    
    fn visit_expr_stmt(&self, expr: &super::ast::Expression) {
        todo!()
    }
    
    fn visit_var_decl(&mut self, var: &super::ast::Var) {
        todo!()
    }

    fn visit_variable(&self, variable: &super::ast::Variable) -> Value {
        todo!()
    }
}

#[test]
fn test_printer() {
    use super::ast::{Literal, Binary, Unary, Grouping};
    use crate::lexer::tokens::{Token, TokenType};

    // Create the literal expressions.
    let literal_expr_1: Literal = Literal::new(Some(Value::Integer(123)));
    let literal_expr_2: Literal = Literal::new(Some(Value::Float(45.67)));

    // Create the unary expression that references the first literal.
    let unary_expr: Unary = Unary::new(
        Token::new(TokenType::Minus, "-".to_string(), None, 1),
        Box::new(literal_expr_1),
    );

    // Create the grouping expression that references the second literal.
    let grouping_expr: Grouping = Grouping::new(Box::new(literal_expr_2));

    // Create the binary expression that references the unary and grouping expressions.
    let expr: Box<Binary> = Box::new(Binary::new(
        Box::new(unary_expr),
        Token::new(TokenType::Star, "*".to_string(), None, 1),
        Box::new(grouping_expr),
    ));

    // Instantiate the AST printer.
    let printer: AstPrinter = AstPrinter();

    let result: Value = printer.print(&*expr);

    // Print the expression.
    println!("{}", result); // Expected output: (* (- 123) (group 45.67))
}