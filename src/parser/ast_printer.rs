use super::ast::{Expr, Visitor};


pub struct AstPrinter();

impl AstPrinter {
    pub fn print(&self, expr: &dyn Expr) -> String {
        return expr.accept(self)    
    }

    fn parenthesize(&self, name: String, exprs: &[&dyn Expr]) -> String {
        let mut builder: String = String::new();

        builder.push('(');
        builder.push_str(&name);

        for expr in exprs.to_vec() {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        
        builder.push(')');

        builder
    }
}

impl Visitor for AstPrinter {
    fn visit_binary(&self, binary: &super::ast::Binary) -> String {
        self.parenthesize(binary.operator.lexeme.clone(), &[&*binary.left, &*binary.right])
    }

    fn visit_grouping(&self, grouping: &super::ast::Grouping) -> String {
        self.parenthesize("group".to_string(), &[&*grouping.expression])
    }

    fn visit_literal(&self, literal: &super::ast::Literal) -> String {
        return match &literal.value {
            Some(value) => value.to_string(),
            None => "null".to_string() 
        }
    }

    fn visit_unary(&self, unary: &super::ast::Unary) -> String {
        self.parenthesize(unary.operator.lexeme.clone(), &[&*unary.right])
    }
}

#[test]
fn test_printer() {
    // Create the literal expressions.
    let literal_expr_1: crate::parser::ast::Literal = super::ast::Literal::new(Some("123".to_string()));
    let literal_expr_2: crate::parser::ast::Literal = super::ast::Literal::new(Some("45.67".to_string()));

    // Create the unary expression that references the first literal.
    let unary_expr: crate::parser::ast::Unary = super::ast::Unary::new(
        crate::lexer::tokens::Token::new(crate::lexer::tokens::TokenType::Minus, "-".to_string(), None, 1),
        Box::new(literal_expr_1),
    );

    // Create the grouping expression that references the second literal.
    let grouping_expr: crate::parser::ast::Grouping = super::ast::Grouping::new(Box::new(literal_expr_2));

    // Create the binary expression that references the unary and grouping expressions.
    let expr: Box<crate::parser::ast::Binary> = Box::new(super::ast::Binary::new(
        Box::new(unary_expr),
        crate::lexer::tokens::Token::new(crate::lexer::tokens::TokenType::Star, "*".to_string(), None, 1),
        Box::new(grouping_expr),
    ));

    // Instantiate the AST printer.
    let printer: AstPrinter = AstPrinter();

    // Print the expression.
    println!("{}", printer.print(&*expr)); // Expected output: (* (- 123) (group 45.67))
}