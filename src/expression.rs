use crate::{Token, Value};

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Unary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Binary {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Grouping {
        expression: Box<Expr<'a>>,
    },
    Literal {
        value: Value,
    },
    Roll {
        quantity: Box<Expr<'a>>,
        dice: Token<'a>,
        faces: Box<Expr<'a>>,
    },
}
