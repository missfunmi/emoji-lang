use crate::token::Token;
use crate::token::Value;
use anyhow::{Result, anyhow};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type EnvPtr = Rc<RefCell<Environment>>;

#[derive(Clone, Debug, Default)]
pub struct Environment {
    enclosing: Option<EnvPtr>,
    values: RefCell<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> EnvPtr {
        Rc::new(RefCell::new(Environment {
            enclosing: None,
            values: RefCell::new(HashMap::new()),
        }))
    }
    pub fn new_enclosed(enclosing: EnvPtr) -> EnvPtr {
        Rc::new(RefCell::new(Environment {
            enclosing: Some(enclosing),
            values: RefCell::new(HashMap::new()),
        }))
    }
    pub fn define(&mut self, name: String, value: Value) {
        self.values.borrow_mut().insert(name, value);
    }
    pub fn get(&self, name: &Token) -> Result<Value> {
        if let Some(v) = self.values.borrow().get(&name.lexeme) {
            Ok(v.clone())
        } else if let Some(env) = &self.enclosing {
            env.borrow().get(name)
        } else {
            Err(anyhow!("Undefined variable '{}'", name.lexeme))
        }
    }
    pub fn assign(&mut self, name: Token, value: Value) -> Result<()> {
        if self.values.borrow().contains_key(&name.lexeme) {
            self.define(name.lexeme, value);
            Ok(())
        } else if let Some(env) = &mut self.enclosing {
            env.borrow_mut().assign(name, value)
        } else {
            Err(anyhow!("Undefined variable '{}'", name.lexeme))
        }
    }
}
