use crate::token::Value;

#[derive(Debug)]
pub struct Return {
    pub value: Value,
}

impl Return {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

// TODO - hacky, will fix later
unsafe impl Send for Return {}