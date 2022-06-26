//! This module is dedicated to the definition and parsing of the [`Token`]s
use std::fmt::Display;

use logos::{Lexer, Logos};

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'source> {
    source: &'source str,
    pub span: logos::Span,
    pub ty: TokenType,
}

impl<'source> Token<'source> {
    pub fn new_from_lexer(lexer: &mut Lexer<'source, TokenType>) -> Self {
        let source = lexer.source();
        if let Some(token_type) = lexer.next() {
            Self {
                source,
                span: lexer.span(),
                ty: token_type,
            }
        } else {
            Self {
                source,
                span: source.len().saturating_sub(1)..source.len(),
                ty: TokenType::EoF,
            }
        }
    }
    pub fn lexeme(&self) -> &str {
        &self.source[self.span.clone()]
    }
}

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // single character token
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[regex(r"[\-−]")]
    Minus,
    #[token("+")]
    Plus,
    #[token("/")]
    Slash,
    #[token("÷")]
    Division,
    #[token("*")]
    Star,
    #[regex("[xX×]")]
    Multiplication,

    // Literals
    #[regex(r#"[0-9]+"#)]
    Number,
    #[regex(r#"[0-9]+\.[0-9]*"#)]
    Float,
    #[regex("[dD]")]
    Dice,

    #[regex(r"[  \r\t\n]+", logos::skip)]
    #[error]
    Error,

    EoF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::Minus => write!(f, "−"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Division => write!(f, "÷"),
            TokenType::Star => write!(f, "*"),
            TokenType::Multiplication => write!(f, "×"),
            TokenType::Number => write!(f, "number"),
            TokenType::Float => write!(f, "float"),
            TokenType::Dice => write!(f, "dice"),
            TokenType::Error => write!(f, "error"),
            TokenType::EoF => write!(f, "EoF"),
        }
    }
}
