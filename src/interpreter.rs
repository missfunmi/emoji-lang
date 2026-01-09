use crate::ast::{Expr, ExprVisitor, Stmt, StmtVisitor};
use crate::environment::{EnvPtr, Environment};
use crate::function::{Callable, EmojiFunction};
use crate::return_value::Return;
use crate::token::Value::{Boolean, Function, Nil, Number, Text};
use crate::token::{Token, TokenType, Value};
use panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::mem::replace;
use std::panic;
use std::panic::panic_any;
use std::rc::Rc;
use TokenType::{
    Bang, BangEqual, EqualEqual, Greater, GreaterEqual, Less, LessEqual, Minus, Or, Plus, Slash,
    Star, TextConcat,
};

pub struct Interpreter {
    environment: EnvPtr,
}

impl Interpreter {
    pub fn new() -> Self {
        let environment = Environment::new();
        Self { environment }
    }
    pub fn interpret(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.execute(stmt)
        }
    }
    const fn is_truthy(value: &Value) -> bool {
        // Boolean(true) is the only truly “truthy” Boolean;
        // Nil and Boolean(false) are falsy;
        // Everything else is considered truthy by implication.
        matches!(value, Boolean(true)) || !matches!(value, Nil | Boolean(false))
    }
    fn is_equal(a: Value, b: Value) -> bool {
        match (a, b) {
            (Nil, Nil) => true,
            (Boolean(a), Boolean(b)) => a == b,
            (Text(a), Text(b)) => a == b,
            (Number(a), Number(b)) => {
                // Compare for approximate equality by checking if difference is within a small, scale-aware epsilon
                let epsilon = 1e-8 * a.abs().max(b.abs()).max(1.0);
                (a - b).abs() < epsilon
            }
            (_, _) => false,
        }
    }
    pub fn execute_block(&mut self, stmts: Vec<Stmt>, new_env: EnvPtr) {
        let previous = replace(&mut self.environment, new_env);
        let result = catch_unwind(AssertUnwindSafe(|| {
            for stmt in stmts {
                self.execute(stmt);
            }
        }));
        self.environment = previous;
        if let Err(payload) = result {
            resume_unwind(payload);
        }
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: Expr) {
        self.evaluate(stmt);
    }
    fn visit_print_stmt(&mut self, stmt: Expr) {
        let value = self.evaluate(stmt);
        println!("{}", value);
    }
    fn visit_var_stmt(&mut self, name: Token, initializer: Option<Expr>) {
        let value = initializer.map_or_else(|| Nil, |initializer| self.evaluate(initializer));
        self.environment.borrow_mut().define(name.lexeme, value);
    }
    fn visit_block_stmt(&mut self, stmts: Vec<Stmt>) {
        self.execute_block(stmts, Environment::new_enclosed(self.environment.clone()))
    }
    fn visit_if_stmt(
        &mut self,
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Option<Stmt>>,
    ) {
        if Self::is_truthy(&self.evaluate(condition)) {
            self.execute(*then_branch)
        } else if let Some(else_branch) = *else_branch {
            self.execute(else_branch)
        }
    }
    fn visit_while_stmt(&mut self, condition: Expr, body: Box<Stmt>) {
        while Self::is_truthy(&self.evaluate(condition.clone())) {
            self.execute(*body.clone())
        }
    }
    fn visit_func_stmt(&mut self, name: Token, params: Vec<Token>, body: Vec<Stmt>) {
        // Since we're using Rc<RefCell<Environment>> (i.e. EnvPtr),
        // calling .clone() on self.environment doesn’t make a new environment —
        // it returns another pointer to the same shared, mutable environment
        // This is what we want: the function should share the surrounding
        // scope as it was when the function was defined.
        let function =
            EmojiFunction::new_from(name.clone(), params, body, self.environment.clone());
        self.environment
            .borrow_mut()
            .define(name.lexeme, Function(Rc::new(function)));
    }
    fn visit_return_stmt(&mut self, _keyword: Token, value: Option<Expr>) {
        // println!(">>> returning {:?}", value);
        let return_value = value.map(|expr| self.evaluate(expr)).unwrap_or(Nil);
        // println!(">>> throwing return: {}", return_value);
        panic_any(Return::new(return_value));
    }
}

impl ExprVisitor<Value> for Interpreter {
    fn visit_binary_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Value {
        let left = self.evaluate(*left);
        let right = self.evaluate(*right);
        // println!(">>> [binary] {:?} {:?} {:?}", left, operator.token_type, right);
        match operator.token_type {
            TextConcat => match (left, right) {
                (Text(l), Text(r)) => Text(l + &r),
                (l, r) => panic!("Type mismatch for concatenation: {:?} 🪡 {:?}", l, r),
            },
            EqualEqual => Boolean(Self::is_equal(left, right)),
            BangEqual => Boolean(!Self::is_equal(left, right)),
            Minus | Star | Slash | Plus | Greater | GreaterEqual | Less | LessEqual => {
                let (l, r) = match (left, right) {
                    (Number(l), Number(r)) => (l, r),
                    (l, r) => panic!("Operands must be numbers: {:?} and {:?}", l, r),
                };
                match operator.token_type {
                    Minus => Number(l - r),
                    Star => Number(l * r),
                    Slash => Number(l / r),
                    Plus => Number(l + r),
                    Greater => Boolean(l > r),
                    GreaterEqual => Boolean(l >= r),
                    Less => Boolean(l < r),
                    LessEqual => Boolean(l <= r),
                    _ => unreachable!(),
                }
            }
            _ => Nil,
        }
    }
    fn visit_unary_expr(&mut self, operator: Token, right: Box<Expr>) -> Value {
        let right = self.evaluate(*right);
        match operator.token_type {
            Minus => match right {
                Number(number) => Number(-number),
                _ => panic!("{:?} is not a number", right),
            },
            Bang => match right {
                Boolean(bool) => Boolean(bool),
                Nil => Boolean(false),
                _ => Boolean(true),
            },
            _ => Nil,
        }
    }
    fn visit_literal_expr(&mut self, value: Value) -> Value {
        value.into()
    }
    fn visit_grouping_expr(&mut self, expr: Box<Expr>) -> Value {
        self.evaluate(*expr)
    }
    fn visit_variable_expr(&mut self, variable: Token) -> Value {
        // println!(">>> [variable lookup] {}", variable.lexeme);
        self.environment.borrow().get(&variable).unwrap()
    }
    fn visit_assignment_expr(&mut self, name: Token, value: Box<Expr>) -> Value {
        let value = self.evaluate(*value);
        // println!(">>> [assign] {} = {:?}", name.lexeme.clone(), value);
        self.environment
            .borrow_mut()
            .assign(name, value.clone())
            .unwrap();
        value
    }
    fn visit_logical_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Value {
        let left = self.evaluate(*left);
        let left_is_truthy = Self::is_truthy(&left);
        let is_or = operator.token_type == Or;
        if (is_or && left_is_truthy) || (!is_or && !left_is_truthy) {
            left
        } else {
            self.evaluate(*right)
        }
    }
    fn visit_call_expr(&mut self, callee: Box<Expr>, _paren: Token, args: Vec<Expr>) -> Value {
        // println!(">>> calling {:?} with {:?}", callee, args);
        let callee = self.evaluate(*callee);
        let mut arguments = Vec::with_capacity(args.len());
        for arg in args {
            arguments.push(self.evaluate(arg));
        }
        match callee {
            Function(f) => {
                if arguments.len() != f.arity() {
                    panic!(
                        "Expected {} arguments but got {} instead.",
                        f.arity(),
                        arguments.len()
                    );
                }
                f.call(self, arguments)
            }
            _ => panic!("Only functions are callable."),
        }
    }
}
