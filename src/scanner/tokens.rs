use core::fmt;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TokenType {
    LParen, RParen, LBrace, RBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    Identifier, String, Number,

    And, Class, Else, False, Fun, For, If, Null, Or,
    Print, Return, Super, This, True, Var, While,

    EOF, Unexpected,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub trait CloneAny: std::any::Any + Debug {
    fn clone_any(&self) -> Box<dyn CloneAny>;
}

impl<T> CloneAny for T
where
    T: std::any::Any + Clone + Debug,
{
    fn clone_any(&self) -> Box<dyn CloneAny> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CloneAny> {
    fn clone(&self) -> Self {
        self.as_ref().clone_any()
    }
}

#[derive(Clone)]
pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub literal: Option<Box<dyn CloneAny>>,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {:?}]", self.type_, self.lexeme, self.literal)
    }
}

impl Token {
    pub fn new(type_: TokenType, lexeme: &str, line: usize) -> Self {
        Self {
            type_,
            lexeme: lexeme.to_string(),
            literal: None::<Box<dyn CloneAny>>,
            line,
        }
    }

    pub fn new_lit(type_: TokenType, lexeme: &str, literal: Option<impl CloneAny>, line: usize) -> Self {
        Self { 
            type_, 
            lexeme: lexeme.to_string(), 
            literal: {
                if let Some(n) = literal {
                    Some(Box::new(n))
                } else {
                    None
                }
            }, 
            line 
        }
    }
}

