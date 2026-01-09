use crate::ast::{Expr, Stmt};
use crate::error::{error_at, error_at_token};
use crate::token::TokenType::{
    And, Bang, BangEqual, Comma, TextConcat, Else, EndOfExpression, Equal, EqualEqual, False, For,
    Function, Greater, GreaterEqual, Identifier, If, LeftCurlyBrace, LeftParen, Less, LessEqual,
    Minus, Nil, Number, Or, Plus, Print, Return, RightCurlyBrace, RightParen, Slash, Star, Text,
    True, Var, While,
};
use crate::token::Value;
use crate::token::{Token, TokenType};
use anyhow::Result;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let current = 0;
        Self { tokens, current }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }
        statements
    }

    fn declaration(&mut self) -> Option<Stmt> {
        let result = if self.matches(&[Function]) {
            self.function("function")
        } else if self.matches(&[Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        match result {
            Ok(stmt) => Some(stmt),
            Err(_) => {
                self.synchronize();
                None
            }
        }
    }

    fn function(&mut self, kind: &str) -> Result<Stmt> {
        let name = self.consume(&Identifier, format!("Expected {} name", kind).as_str())?;
        self.consume(
            &LeftParen,
            format!("Expected '🫱' after {} name", kind).as_str(),
        )?;
        let mut params = Vec::new();
        if !self.check(&RightParen) {
            // first parameter
            params.push(self.consume(&Identifier, "Expected parameter name")?);

            // remaining parameters
            while self.matches(&[Comma]) {
                if params.len() >= 255 {
                    error_at_token(&self.peek(), "Can't have more than 255 params");
                }
                params.push(self.consume(&Identifier, "Expected parameter name")?);
            }
        }
        self.consume(&RightParen, "Expected '🫲' after params")?;
        self.consume(
            &LeftCurlyBrace,
            format!("Expected '🫸' before {} body", kind).as_str(),
        )?;
        let body = self.block()?;
        Ok(Stmt::func(name, params, body))
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume(&Identifier, "Expected a variable name")?;
        let initializer = self
            .matches(&[Equal])
            .then(|| self.expression().ok())
            .flatten();

        self.consume(
            &EndOfExpression,
            "Expected '✊' at the end of variable declaration",
        )?;
        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt> {
        if self.matches(&[For]) {
            self.for_statement()
        } else if self.matches(&[If]) {
            self.if_statement()
        } else if self.matches(&[Print]) {
            self.print_statement()
        } else if self.matches(&[Return]) {
            self.return_statement()
        } else if self.matches(&[While]) {
            self.while_statement()
        } else if self.matches(&[LeftCurlyBrace]) {
            self.block_statement()
        } else {
            self.expression_statement()
        }
    }

    fn for_statement(&mut self) -> Result<Stmt> {
        self.consume(&LeftParen, "Expected '🫱' after '⏳'")?;

        let initializer = match () {
            _ if self.matches(&[EndOfExpression]) => None,
            _ if self.matches(&[Var]) => self.var_declaration().ok(),
            _ => self.expression_statement().ok(),
        };

        let condition = if self.check(&EndOfExpression) {
            Expr::literal(Value::Boolean(true))
        } else {
            self.expression()?
        };
        self.consume(&EndOfExpression, "Expected '✊' after '⏳' loop condition")?;

        let increment = (!self.check(&RightParen))
            .then(|| self.expression().ok())
            .flatten();
        self.consume(&RightParen, "Expected '🫲' after '⏳' clause")?;

        let mut body = self.statement()?;
        if let Some(inc) = increment {
            body = Stmt::block(vec![body, Stmt::expression(inc)])
        }
        body = Stmt::while_(condition, body);
        if let Some(init) = initializer {
            body = Stmt::block(vec![init, body])
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        self.consume(&LeftParen, "Expected '🫱' after '🤔'")?;

        let condition = self.expression()?;
        self.consume(&RightParen, "Expected '🫲' after '🤔' condition")?;

        let then_branch = self.statement()?;
        let else_branch = self
            .matches(&[Else])
            .then(|| self.statement().ok())
            .flatten();
        Ok(Stmt::if_(condition, then_branch, else_branch))
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(&EndOfExpression, "Expected '✊' after value")?;
        Ok(Stmt::print(expr))
    }

    fn return_statement(&mut self) -> Result<Stmt> {
        let keyword = self.previous();
        let value = (!self.check(&EndOfExpression)).then(|| self.expression().ok()).flatten();
        self.consume(&EndOfExpression, "Expected '✊' after return value")?;
        Ok(Stmt::return_(keyword, value))
    }

    fn while_statement(&mut self) -> Result<Stmt> {
        self.consume(&LeftParen, "Expected '🫱' after '🌀'")?;
        let condition = self.expression()?;
        self.consume(&RightParen, "Expected '🫲' after '🌀' condition")?;
        let body = self.statement()?;
        Ok(Stmt::while_(condition, body))
    }

    fn block_statement(&mut self) -> Result<Stmt> {
        Ok(Stmt::block(self.block()?))
    }

    fn block(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts = Vec::new();
        while !self.check(&RightCurlyBrace) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                stmts.push(stmt);
            }
        }
        self.consume(&RightCurlyBrace, "Expected '🫷' after block")?;
        Ok(stmts)
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(&EndOfExpression, "Expected '✊' after value")?;
        Ok(Stmt::expression(expr))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.or()?;
        if self.matches(&[Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;
            if let Expr::Variable(name) = expr {
                Ok(Expr::assign(name, value))
            } else {
                error_at(equals, "Invalid assignment target")
            }
        } else {
            Ok(expr)
        }
    }

    // Binary operators
    fn or(&mut self) -> Result<Expr> {
        let mut expr = self.and()?;
        while self.matches(&[Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::logical(expr, operator, right);
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;
        while self.matches(&[And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::logical(expr, operator, right);
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut left = self.comparison()?;
        while self.matches(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            left = Expr::binary(left, operator, right);
        }
        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut left = self.term()?;
        while self.matches(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            left = Expr::binary(left, operator, right);
        }
        Ok(left)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut left = self.factor()?;
        while self.matches(&[Minus, Plus, TextConcat]) {
            let operator = self.previous();
            let right = self.factor()?;
            left = Expr::binary(left, operator, right);
        }
        Ok(left)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut left = self.unary()?;
        while self.matches(&[Slash, Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            left = Expr::binary(left, operator, right);
        }
        Ok(left)
    }

    // Unary operators
    fn unary(&mut self) -> Result<Expr> {
        if self.matches(&[Bang, Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            let expr = Expr::unary(operator, right);
            Ok(expr)
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;
        loop {
            if self.matches(&[LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr> {
        let mut arguments = Vec::new();

        if !self.check(&RightParen) {
            loop {
                if arguments.len() >= 255 {
                    error_at_token(&self.peek(), "Can't have more than 255 arguments");
                }
                arguments.push(self.expression()?);
                if !self.matches(&[Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(&RightParen, "Expected a right hand `🫲` after arguments")?;
        Ok(Expr::call(callee, paren, arguments))
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.matches(&[False]) {
            let value = Value::Boolean(false);
            return Ok(Expr::literal(value));
        }
        if self.matches(&[True]) {
            let value = Value::Boolean(true);
            return Ok(Expr::literal(value));
        }
        if self.matches(&[Nil]) {
            let value = Value::Nil;
            return Ok(Expr::literal(value));
        }
        if self.matches(&[Number, Text]) {
            let value = self.previous().value.unwrap_or_else(|| Value::Nil);
            return Ok(Expr::literal(value));
        }
        if self.matches(&[Identifier]) {
            return Ok(Expr::variable(self.previous()));
        }
        if self.matches(&[LeftParen]) {
            let expr = self.expression()?;
            self.consume(&RightParen, "Expected a right hand `🫲` after expression")?;
            return Ok(Expr::grouping(expr));
        }

        // TODO: panic?
        error_at(self.peek(), "Parse error in primary")
    }

    // Helper functions
    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        error_at(self.peek(), message)
    }

    // fn error(&self, token: Token, message: &str) -> Result<Token> {
    //     Err(anyhow!(
    //         "Parse error: \"{}\", got `{}`",
    //         message,
    //         token.lexeme
    //     ))
    // }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == EndOfExpression {
                return;
            }
            match self.peek().token_type {
                // TODO: add class, for
                Function | Var | If | While | Print | Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    /*
    TODO Implement this error logic and call it from fn error(...) above
      static void error(Token token, String message) {
        if (token.type == TokenType.EOF) {
          report(token.line, " at end", message);
        } else {
          report(token.line, " at '" + token.lexeme + "'", message);
        }
      }
    */

    fn matches(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EndOfFile
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
