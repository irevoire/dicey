use crate::{Expr, InterpreterError, Value};

type Result<T> = std::result::Result<T, InterpreterError>;

impl Expr<'_> {
    pub fn interpret(&self) -> Result<Value> {
        match self {
            Expr::Unary { operator, right } => {
                let right = right.interpret()?;
                match operator.lexeme() {
                    "-" => Ok(-right),
                    _ => unreachable!(),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let (left, right) = (left.interpret()?, right.interpret()?);
                match operator.lexeme() {
                    "+" => Ok(left + right),
                    "-" => Ok(left - right),
                    "*" | "(" => Ok(left * right),
                    "/" => Ok(left / right),
                    _ => unreachable!(),
                }
            }
            Expr::Grouping { expression } => expression.interpret(),
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Roll {
                quantity,
                dice,
                faces,
            } => {
                let quantity = quantity.interpret()?;
                let faces = faces.interpret()?;

                Ok(quantity * faces)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, Parser};

    #[test]
    fn test_value() -> Result<(), Error> {
        let parser = Parser::new("1").parse()?;
        let res = parser.interpret()?;
        assert_eq!(res, 1.);
        let parser = Parser::new("4000").parse()?;
        let res = parser.interpret()?;
        assert_eq!(res, 4000.);
        Ok(())
    }

    #[test]
    fn test_unary() -> Result<(), Error> {
        let parser = Parser::new("-1").parse()?;
        let res = parser.interpret()?;
        assert_eq!(res, -1.);
        Ok(())
    }

    #[test]
    fn test_binary() -> Result<(), Error> {
        let test_values = [
            ("1 + 1", 2.),
            ("5 - 1", 4.),
            ("2 * 3", 6.),
            ("6 / 3", 2.),
            ("1 / 2", 0.5),
            ("2 + 3 * 2", 8.),
            ("2 + (3 * 2)", 8.),
            ("2 * (3 + 2)", 10.),
        ];

        for (input, output) in test_values {
            println!("parsing {input}");
            let parser = Parser::new(input).parse()?;
            let res = parser.interpret()?;
            assert_eq!(res, output);
        }
        Ok(())
    }
}
