use std::collections::HashMap;

use crate::parser::{error::Error, parser::Value};

#[derive(Debug)]
pub struct Environment<'a> {
    variables: HashMap<&'a str, Value>,
    pub enclosing: Option<&'a mut Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new(enclosing: Option<&'a mut Environment<'a>>) -> Self {
        Self {
            variables: HashMap::new(),
            enclosing,
        }
    }

    pub fn get(&self, name: &'a str) -> Result<&Value, Error> {
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

    pub fn assign(&mut self, name: &'a str, value: Value) {
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

    fn assign_enclosing(&mut self, name: &'a str, value: Value) -> bool {
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

impl<'a> Default for Environment<'a> {
    fn default() -> Self {
        Self::new(None)
    }
}
