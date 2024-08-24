
use std::collections::HashMap;

use crate::{lexer::tokens::Token, util::{error::Error, Value}};

#[derive(Clone, Debug)]
pub struct VarAttrib(pub Option<Token>, pub Option<Value>);

pub struct Environment(pub HashMap<String, VarAttrib>);

impl Environment {
    pub fn define(&mut self, name: String, var_attrib: VarAttrib) {
        self.0.insert(name, var_attrib);
    }

    pub fn get(&self, name: Token) -> VarAttrib {
        if self.0.contains_key(&name.lexeme) {
            return self.0.get(&name.lexeme).unwrap().clone();
        }
        Error::undefined_var(name);
    } 
}