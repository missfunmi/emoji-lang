mod ast;
mod environment;
mod error;
mod function;
mod interpreter;
mod parser;
mod return_value;
mod token;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::return_value::Return;
use crate::token::{Token, TokenType};
use clap::Parser as ArgParser;
use logos::Logos;
use std::panic::set_hook;
use std::fs;
use token::Value;

#[derive(ArgParser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    debug: bool,
    #[arg(short, long)]
    file: String,
}

fn main() {
    // Ensure Rust's default panic message is only logged for real errors and not the panics
    // we use to exit early from a function when the interpreter encounters a return statement
    set_hook(Box::new(|info| {
        if let Some(_) = info.payload().downcast_ref::<Return>() {
            return;
        }
        eprintln!("❌ Panic occurred: {}", info);
    }));

    let args = Args::parse();
    let file_path = args.file;
    let contents = fs::read_to_string(&file_path)
        .unwrap_or_else(|_| panic!("❌ Failed to read code from file: {}", file_path));
    if args.debug {
        println!("ℹ️ File {:?} contents are:\n{:?}", file_path, contents);
    }

    let mut tokens: Vec<Token> = Vec::new();
    let mut lexer = TokenType::lexer(contents.as_str());
    while let Some(result) = lexer.next() {
        let span = lexer.span();
        let token_type = result.unwrap();
        let raw_slice = lexer.slice().to_string();
        let mut slice = raw_slice.clone();
        if token_type == TokenType::Text {
            slice = String::from(&slice[4..slice.len() - 4]);
        }
        let value = match token_type {
            TokenType::Text => Some(Value::Text(slice.clone())),
            TokenType::Number => Some(Value::Number(slice.parse().unwrap())),
            TokenType::True => Some(Value::Boolean(true)),
            TokenType::False => Some(Value::Boolean(false)),
            _ => None,
        };
        let token = Token::new(token_type, slice, value, span);
        // println!(">>> [lexer] token: {:?}", token);
        tokens.push(token);
    }

    // let errors: Vec<_> = lexer.filter_map(|result| result.err()).collect();
    // println!(">>> Lexer has errors? {}", !errors.is_empty());

    let mut parser = Parser::new(tokens);
    let statements = parser.parse();
    // println!(">>> [parsed] statements {:?}", statements);

    let mut interpreter = Interpreter::new();
    interpreter.interpret(statements)
}
