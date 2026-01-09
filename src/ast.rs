use crate::ast::Expr::{Assign, Binary, Call, Grouping, Literal, Logical, Unary, Variable};
use crate::ast::Stmt::{Block, Expression, Func, If, Print, Return, Var, While};
use crate::token::{Token, Value};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Box<Option<Stmt>>),
    While(Expr, Box<Stmt>),
    Func(Token, Vec<Token>, Vec<Stmt>),
    Return(Token, Option<Expr>)
}

impl Stmt {
    pub fn expression(expr: Expr) -> Self {
        Expression(expr)
    }
    pub fn print(expr: Expr) -> Self {
        Print(expr)
    }
    pub fn block(stmts: Vec<Stmt>) -> Self {
        Block(stmts)
    }
    pub fn if_(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Self {
        If(condition, Box::new(then_branch), Box::new(else_branch))
    }
    pub fn while_(condition: Expr, body: Stmt) -> Self {
        While(condition, Box::new(body))
    }
    pub fn func(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Func(name, params, body)
    }
    pub fn return_(keyword: Token, value: Option<Expr>) -> Self {
        Return(keyword, value)
    }
}

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, stmt: Expr) -> T;
    fn visit_print_stmt(&mut self, stmt: Expr) -> T;
    fn visit_var_stmt(&mut self, name: Token, initializer: Option<Expr>) -> T;
    fn visit_block_stmt(&mut self, stmts: Vec<Stmt>) -> T;
    fn visit_if_stmt(
        &mut self,
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Option<Stmt>>,
    ) -> T;
    fn visit_while_stmt(&mut self, condition: Expr, body: Box<Stmt>) -> T;
    fn visit_func_stmt(&mut self, name: Token, params: Vec<Token>, body: Vec<Stmt>) -> T;
    fn visit_return_stmt(&mut self, keyword: Token, value: Option<Expr>) -> T;
    fn execute(&mut self, stmt: Stmt) -> T {
        // println!(">>> [execute] {:?}", stmt);
        match stmt {
            Expression(expr) => self.visit_expression_stmt(expr),
            Print(expr) => self.visit_print_stmt(expr),
            Var(name, initializer) => self.visit_var_stmt(name, initializer),
            Block(stmts) => self.visit_block_stmt(stmts),
            If(condition, then_branch, else_branch) => {
                self.visit_if_stmt(condition, then_branch, else_branch)
            }
            While(condition, body) => self.visit_while_stmt(condition, body),
            Func(name, params, body) => self.visit_func_stmt(name, params, body),
            Return(keyword, value) => self.visit_return_stmt(keyword, value)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Literal(Value),
    Grouping(Box<Expr>),
    Variable(Token),
    Assign(Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
}

impl Expr {
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Binary(Box::new(left), operator, Box::new(right))
    }
    pub fn unary(operator: Token, right: Expr) -> Self {
        Unary(operator, Box::new(right))
    }
    pub fn grouping(expr: Expr) -> Self {
        Grouping(Box::new(expr))
    }
    pub fn literal(value: Value) -> Self {
        Literal(value)
    }
    pub fn variable(variable: Token) -> Self {
        Variable(variable)
    }
    pub fn assign(name: Token, value: Expr) -> Self {
        Assign(name, Box::new(value))
    }
    pub fn logical(left: Expr, operator: Token, right: Expr) -> Self {
        Logical(Box::new(left), operator, Box::new(right))
    }
    pub fn call(callee: Expr, paren: Token, args: Vec<Expr>) -> Self {
        Call(Box::new(callee), paren, args)
    }
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> T;
    fn visit_unary_expr(&mut self, operator: Token, right: Box<Expr>) -> T;
    fn visit_literal_expr(&mut self, value: Value) -> T;
    fn visit_grouping_expr(&mut self, expr: Box<Expr>) -> T;
    fn visit_variable_expr(&mut self, variable: Token) -> T;
    fn visit_assignment_expr(&mut self, name: Token, expr: Box<Expr>) -> T;
    fn visit_logical_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> T;
    fn visit_call_expr(&mut self, callee: Box<Expr>, paren: Token, args: Vec<Expr>) -> T;
    fn evaluate(&mut self, expr: Expr) -> T {
        match expr {
            Binary(left, op, right) => {
                // println!(">>> [evaluate] Binary: {:?} {:?} {:?}", left, op.token_type, right);
                self.visit_binary_expr(left, op, right)
            }
            Unary(op, right) => {
                // println!(">>> [evaluate] Unary: {:?} {:?}", op.token_type, right);
                self.visit_unary_expr(op, right)
            }
            Literal(value) => {
                // println!(">>> [evaluate] Literal: {:?}", value);
                self.visit_literal_expr(value)
            }
            Grouping(inner) => {
                // println!(">>> [evaluate] Grouping: {:?}", inner);
                self.visit_grouping_expr(inner)
            }
            Variable(name) => {
                // println!(">>> [evaluate] Variable: {:?}", name.lexeme);
                self.visit_variable_expr(name)
            }
            Assign(name, rhs) => {
                // println!(">>> [evaluate] Assign: {:?} = {:?}", name.lexeme, rhs);
                self.visit_assignment_expr(name, rhs)
            }
            Logical(left, op, right) => {
                // println!(">>> [evaluate] Logical: {:?} {:?} {:?}", left, op.token_type, right);
                self.visit_logical_expr(left, op, right)
            }
            Call(callee, paren, args) => {
                // println!(">>> [evaluate] Call: {:?} {:?} {:?}", callee, paren.token_type, args);
                self.visit_call_expr(callee, paren, args)
            }
        }
    }
}
