
use core::fmt;

use crate::util::Value;

//use crate::util::Object;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    LParen, RParen, LBrace, RBrace, LSquare, RSquare, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    Arrow, Colon, Questionmark, Exclaimationmark,  

    MinusEqual, PlusEqual,
    TimesEqual, DivEqual,

    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    Identifier, String, Number,

    Mut, Typeof, Sizeof, Nameof, As, Void, Use, With, Out, True, False, If, Elif, Else, While, For, 
    In, Entity, Init, New, This, Set, Enum, Throw, Catch, Pub, Priv, Prot, Unreachable, Trait, Parent, 
    Open, Override, Scene, Import, Todo, Pass, Echo, Try, Await, Thread, Worker, Chan, Select, Pool, Defer, 
    Macro, Vararg, Varargs, Test, Move,

    EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub literal: Option<Value>,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.type_, self.lexeme, self.literal)
    }
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, literal: Option<Value>, line: usize) -> Self {
        Self { type_, lexeme, literal, line }
    }
}