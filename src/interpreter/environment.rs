
use std::collections::HashMap;

use crate::{lexer::tokens::Token, util::Value};

pub struct Environment(pub HashMap<String, Option<Value>>);

impl Environment {
    pub fn define(&mut self, name: String, value: Option<Value>) {
        self.0.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Option<Value> {
        if self.0.contains_key(&name.lexeme) {
            return self.0.get(&name.lexeme).unwrap().clone();
        }
        panic!("Runtime Error: undefined variable '{}'.", name.lexeme);
    } 
}