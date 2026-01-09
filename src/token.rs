use crate::function::{EmojiFunction};
use logos::{Logos, Span};
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;

#[allow(dead_code, unused, unused_variables)]
#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub value: Option<Value>,
    pub span: Span,
}

#[allow(dead_code, unused, unused_variables)]
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, value: Option<Value>, span: Span) -> Self {
        Self {
            token_type,
            lexeme,
            value,
            span,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?} {} {:?}", self.token_type, self.lexeme, self.value)
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum TokenType {
    // Operators - tokens
    #[token("🫱")]
    LeftParen,
    #[token("🫲")]
    RightParen,
    #[token("🫸")]
    LeftCurlyBrace,
    #[token("🫷")]
    RightCurlyBrace,
    #[token("🥂")]
    Plus,
    #[token("🪡")]
    TextConcat,
    #[token("💔")]
    Minus,
    #[token("🔪")]
    Slash,
    #[token("⚡️")]
    Percent,
    #[token("✨")]
    Star,
    #[token("👏")]
    Equal,
    #[token("👏👏")]
    EqualEqual,
    #[token("🙅‍♀️")]
    Bang,
    #[token("🙅‍♀️👏")]
    BangEqual,
    #[token("📈")]
    Greater,
    #[token("📈👏")]
    GreaterEqual,
    #[token("📉")]
    Less,
    #[token("📉👏")]
    LessEqual,
    #[token("🔸")]
    Comma,
    // TODO: Add "Dot"?

    // Literals - regexes
    #[regex("🧵[^🧵\n\r]*🧵")]
    Text,
    #[regex(r"[0-9]*\.?[0-9]+")]
    Number,
    #[regex("[a-zA-Z_]*")]
    Identifier,
    #[regex("🗣[^\n\r]*")]
    Comment,

    // Keywords - tokens
    #[token("🤝")]
    And,
    #[token("🤌")]
    Or,
    #[token("👍")]
    True,
    #[token("👎")]
    False,
    #[token("🪄")]
    Var,
    #[token("🔒")]
    Const,
    #[token("🖨")]
    Print,
    #[token("🔙")]
    Return,
    #[token("🤔")]
    If,
    #[token("🤷‍♀️")]
    Else,
    #[token("🫥")]
    Nil,
    #[token("🌀")]
    While,
    #[token("⏳")]
    For,
    #[token("🤖")]
    Function,

    // Delimiters - tokens
    #[token("✊")]
    EndOfExpression,
    #[token("🔚")]
    EndOfFile,
}

#[derive(Debug, Clone)]
pub enum Value {
    Text(String),
    Number(f64),
    Boolean(bool),
    Nil,
    Function(Rc<EmojiFunction>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Text(s) => write!(f, "{}", s),
            Self::Number(n) => write!(f, "{}", n),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Nil => write!(f, "nil"),
            Self::Function(func) => write!(f, "{}", func),
        }
    }
}

// TODO - hacky, will fix later
unsafe impl Send for Value {}
