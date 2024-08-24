use crate::lexer::tokens::{Token, TokenType};

use super::Value;


pub struct Error;

impl Error {
    pub fn number_operand(line: usize) -> ! {
        panic!("[line {line}] Error: Expected number after Operand in Expression.");
    }

    pub fn unexpected_token(line: usize, token: TokenType) -> ! {
        panic!("[line {line}] Error: Unexpected Token '{token:?}'.");
    }

    pub fn unexpected_type(line: usize, type_: Value) -> ! {
        panic!("[line {line}] Error: Expected type 'float', got value of '{}'", type_);
    }

    pub fn type_mismatch(input: &&String, expected: &&[&str]) -> ! {
        let mut error: String = format!("type mismatch:\n  got: '{input}'\n  expected: ");

        for type_ in *expected {
            if expected.last().unwrap() == type_ {
                error.push_str(format!(" '{type_}'").as_str());
            } else {
                error.push_str(format!("'{type_}' or").as_str());
            }
        }

        panic!("{error}");
    }

    pub fn undefined_var(token: Token) -> ! {
        panic!("Runtime Error: undefined variable '{}'.", token.lexeme);
    }

    pub fn fatal() -> ! {
        panic!("Fatal Error");
    }
}