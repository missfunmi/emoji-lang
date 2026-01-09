use crate::ast::Stmt;
use crate::environment::{EnvPtr, Environment};
use crate::interpreter::Interpreter;
use crate::return_value::Return;
use crate::token::{Token, Value};
use std::fmt::{Display, Formatter};
use std::panic;
use std::panic::{catch_unwind, AssertUnwindSafe};

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Value;
}

#[derive(Debug, Clone)]
pub struct EmojiFunction {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
    pub closure: EnvPtr,
}

impl EmojiFunction {
    pub fn new_from(name: Token, params: Vec<Token>, body: Vec<Stmt>, closure: EnvPtr) -> Self {
        Self {
            name,
            params,
            body,
            closure,
        }
    }
}

// TODO - hacky, will fix later
unsafe impl Send for EmojiFunction {}

impl Callable for EmojiFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Value {
        let env = Environment::new_enclosed(self.closure.clone());
        for (param, arg) in self.params.iter().zip(args.into_iter()) {
            env.borrow_mut().define(param.lexeme.clone(), arg);
        }
        // println!(">>> calling function: {}", self.name.lexeme);
        let result = catch_unwind(AssertUnwindSafe(|| {
            interpreter.execute_block(self.body.clone(), env);
            // println!(">>> block executed successfully");
            Value::Nil
        }));
        // println!(">>> caught panic? {:?}", result.is_err());
        match result {
            Ok(val) => val,
            Err(payload) => {
                if let Some(ret) = payload.downcast_ref::<Return>() {
                    // println!(">>> caught Return!");
                    return ret.value.clone();
                }
                if let Some(msg) = payload.downcast_ref::<&str>() {
                    println!(">>> Panic msg: {}", msg);
                } else if let Some(msg) = payload.downcast_ref::<String>() {
                    println!(">>> Panic msg: {}", msg);
                } else {
                    println!(">>> Unknown panic type.");
                }

                panic::resume_unwind(payload);
            }
        }
    }
}

impl Display for EmojiFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name.lexeme)
    }
}
