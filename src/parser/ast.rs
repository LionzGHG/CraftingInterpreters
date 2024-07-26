use crate::scanner::tokens::{Token, TokenType};


#[derive(Clone, Debug)]
pub enum Stmt {
    Expr(Expr)
}

#[derive(Clone, Debug)]
pub enum Operator {
    Plus, Minus, Multiply, Divide,
    EqualEqual, LessEqual, GreaterEqual, Less, Greater, NotEqual, Not
}

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(f64),
    Grouping(Box<Self>),
    Unary(Operator, Box<Self>),
    Binary(Box<Self>, Operator, Box<Self>)
}

#[test]
fn test_expr() {
    let expression: Expr = Expr::Binary(
        Box::new(Expr::Unary(Operator::Minus, Box::new(Expr::Literal(123.)))),
        Operator::Multiply,
        Box::new(Expr::Grouping(Box::new(Expr::Literal(45.67))))
    );

    println!("{expression:?}");
}