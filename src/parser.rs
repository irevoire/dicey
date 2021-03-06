use logos::{Lexer, Logos};

use crate::{Expr, ParserError, Token, TokenType};

type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a, TokenType>,
    previous: Token<'a>,
    current: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = TokenType::lexer(source);
        let token = Token::new_from_lexer(&mut lexer);

        Self {
            lexer,
            previous: token.clone(),
            current: token,
        }
    }

    pub fn parse(mut self) -> Result<Expr<'a>> {
        let expr = self.expression()?;

        if self.is_at_end() {
            Ok(expr)
        } else {
            Err(ParserError {
                src: self.lexer.source().to_string(),
                message: format!(
                    "Unexpected characters `{:.10}` at the end of file.",
                    self.current.lexeme().to_string() + self.lexer.remainder(),
                ),
                label: format!("Here"),
                span: self.current.span.into(),
            })
        }
    }

    fn expression(&mut self) -> Result<Expr<'a>> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr<'a>> {
        let mut expr = self.factor()?;

        println!("looking for minus, but next token is {:?}", self.current.ty);
        while self.is_followed_by([TokenType::Minus, TokenType::Plus])? {
            let operator = self.previous.clone();
            let right = Box::new(self.factor()?);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr<'a>> {
        let mut expr = self.roll()?;

        while self.is_followed_by([
            TokenType::Star,
            TokenType::Multiplication,
            TokenType::Slash,
            TokenType::Division,
        ])? {
            let operator = self.previous.clone();
            let right = Box::new(self.roll()?);

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right,
            };
        }

        Ok(expr)
    }

    fn roll(&mut self) -> Result<Expr<'a>> {
        let mut expr = self.unary()?;

        while self.is_followed_by([TokenType::Dice])? {
            let dice = self.previous.clone();
            let faces = Box::new(self.unary()?);

            expr = Expr::Roll {
                quantity: Box::new(expr),
                dice,
                faces,
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr<'a>> {
        if self.is_followed_by([TokenType::Minus])? {
            let operator = self.previous.clone();
            let right = Box::new(self.unary()?);

            Ok(Expr::Unary { operator, right })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr<'a>> {
        let token = self.advance()?;
        match token.ty {
            TokenType::Number => self.value(),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen)?;
                Ok(Expr::Grouping {
                    expression: Box::new(expr),
                })
            }
            ty => Err(self.expected_error(TokenType::Number, ty)),
        }
    }

    fn value(&mut self) -> Result<Expr<'a>> {
        match self.previous.lexeme().parse() {
            Ok(value) => Ok(Expr::Literal {
                value: crate::Value::direct(value),
            }),
            Err(e) => Err(ParserError {
                src: self.lexer.source().to_string(),
                message: format!("Could not parse number: {}", e),
                label: format!("{}", e),
                span: self.previous.span.clone().into(),
            }),
        }
    }

    // ------------- After this line are the tools to help build the parser

    fn advance(&mut self) -> Result<&Token<'a>> {
        if self.is_at_end() {
            Ok(&self.current)
        } else {
            self.previous = self.current.clone();
            self.current = Token::new_from_lexer(&mut self.lexer);
            Ok(&self.previous)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current.ty == TokenType::EoF
    }

    fn check(&mut self, ty: TokenType) -> bool {
        self.current.ty == ty
    }

    fn is_followed_by(&mut self, types: impl IntoIterator<Item = TokenType>) -> Result<bool> {
        for ty in types {
            if self.check(ty) {
                self.advance()?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn consume(&mut self, expecting: TokenType) -> Result<&Token<'a>> {
        if self.check(expecting) {
            self.advance()
        } else {
            Err(self.expected_error(expecting, self.previous.ty))
        }
    }

    fn expected_error(&self, expected: TokenType, found: TokenType) -> ParserError {
        let src = if self.lexer.source().is_empty() {
            String::from("\n")
        } else {
            self.lexer.source().to_string()
        };
        ParserError {
            src,
            message: format!("Expected `{expected}`, found `{found}`"),
            label: format!("Expected `{expected}`"),
            span: self.previous.span.clone().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() -> Result<()> {
        let expr = Parser::new("1").parse()?;
        assert!(matches!(expr, Expr::Literal { value } if value == 1));
        let expr = Parser::new("4000").parse()?;
        assert!(matches!(expr, Expr::Literal { value } if value == 4000 ));

        let result = Parser::new("4000.53.10").parse();
        assert!(result.is_err());
        // Here we get the error before even calling parse because the
        // parser needs to call the scanner once to initialize it???s state
        let result = Parser::new("a").parse();
        assert!(matches!(result, Err(ParserError { .. })));
        let result = Parser::new("400a").parse();
        assert!(result.is_err());

        Ok(())
    }
}
