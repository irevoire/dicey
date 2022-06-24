//! This module is dedicated to the definition and parsing of the [`Token`]s
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
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token("/")]
    Slash,
    #[token("*")]
    Star,

    // Literals
    #[regex(r#"[0-9]+"#)]
    Number,
    #[regex(r#"[0-9]+\.[0-9]*"#)]
    Float,
    #[regex("[dD]")]
    Dice,

    #[regex(r"[ \r\t\n]+", logos::skip)]
    #[error]
    Error,

    EoF,
}
