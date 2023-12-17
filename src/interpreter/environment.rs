use std::{collections::HashMap, mem::swap};

use crate::parser::{error::Error, parser::Value};

#[derive(Debug)]
pub struct Environment<'s> {
    variables: HashMap<&'s str, Value>,
    pub enclosing: Option<Box<Environment<'s>>>,
}

impl<'s> Environment<'s> {
    pub fn new(enclosing: Option<Environment<'s>>) -> Self {
        Self {
            variables: HashMap::new(),
            enclosing: enclosing.map(Box::new),
        }
    }

    pub fn get(&self, name: &str) -> Result<&Value, Error> {
        if self.variables.contains_key(name) {
            return Ok(self.variables.get(name).unwrap());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }

        Err(Error {
            message: format!("Undefined variable '{}'.", name),
        })
    }

    pub fn assign(&mut self, name: &'s str, value: Value) {
        if self.variables.contains_key(name) {
            self.variables.insert(name, value);
            return;
        }

        let mut enclosing_assigned = false;
        if let Some(enclosing) = &mut self.enclosing {
            enclosing_assigned = enclosing.assign_enclosing(name, value);
        }

        if !enclosing_assigned {
            self.variables.insert(name, value);
            return;
        }
    }

    fn assign_enclosing(&mut self, name: &'s str, value: Value) -> bool {
        if self.variables.contains_key(name) {
            self.variables.insert(name, value);
            return true;
        }

        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.assign_enclosing(name, value);
        }

        return false;
    }
}

impl<'s> Default for Environment<'s> {
    fn default() -> Self {
        Self::new(None)
    }
}
