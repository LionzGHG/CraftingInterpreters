
use std::{collections::HashMap};

use crate::{lexer::tokens::Token, util::{error::Error, Value}};

#[derive(Clone, Debug)]
//                   datatype         , value            , mutability        
pub struct VarAttrib(pub Option<Token>, pub Option<Value>, pub bool);

#[derive(Clone, Debug)]
pub struct Environment {
    pub map: HashMap<String, VarAttrib>,
    pub enclosing: Option<Box<Self>>,
}

impl Environment {

    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_enclosing(enclosing: Environment) -> Self {
        Self {
            map: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn add_scope(&mut self, enclosing: Environment) {
        self.enclosing = Some(Box::new(enclosing));
    }

    pub fn define(&mut self, name: String, var_attrib: VarAttrib) {
        self.map.insert(name, var_attrib);
    }

    pub fn get(&self, name: Token) -> VarAttrib {
        if self.map.contains_key(&name.lexeme) {
            return self.map.get(&name.lexeme).unwrap().clone();
        }

        if let Some(n) = &self.enclosing {
            return n.get(name);
        }

        Error::undefined_var(name);
    } 

    pub fn assign(&mut self, name: Token, value: &Value) {
        if self.map.contains_key(&name.lexeme) {
            let var_attrib: &VarAttrib = self.map.get(&name.lexeme).unwrap();
            if var_attrib.2 == true {
                self.map.insert(name.lexeme, VarAttrib(var_attrib.0.clone(), Some(value.clone()), var_attrib.2));
                return;
            } else {
                Error::immutable_var(name.lexeme);
            }
        }

        if let Some(mut n) = self.enclosing.clone() {
            n.assign(name, value);
            return;
        }

        Error::undefined_var(name.clone());
    }
}