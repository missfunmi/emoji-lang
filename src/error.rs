use crate::token::{Token, TokenType};
use anyhow::{Result, anyhow};
use logos::Span;
use std::sync::atomic::{AtomicBool, Ordering};

static HAD_ERROR: AtomicBool = AtomicBool::new(false);

pub fn error_at<T>(token: Token, message: &str) -> Result<T> {
    error(token.span, message);
    Err(anyhow!(message.to_string()))
}

pub fn error(span: Span, message: &str) {
    report(span, "", message);
}

pub fn error_at_token(token: &Token, message: &str) {
    if token.token_type == TokenType::EndOfFile {
        report(token.span.clone(), " at end", message);
    } else {
        report(
            token.span.clone(),
            &format!(" at '{}'", token.lexeme),
            message,
        );
    }
}

fn report(span: Span, where_: &str, message: &str) {
    println!("[line {:?}] Error{}: {}", span, where_, message);
    set_had_error(true);
}

fn had_error() -> bool {
    HAD_ERROR.load(Ordering::Relaxed)
}

fn set_had_error(b: bool) {
    HAD_ERROR.store(b, Ordering::Relaxed)
}
