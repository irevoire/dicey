use crate::{Expr, InterpreterError, Kind, Value};
use rand::{rngs::ThreadRng, Rng};

type Result<T> = std::result::Result<T, InterpreterError>;

pub struct Interpreter<Rng> {
    rng: Rng,
}

impl Interpreter<ThreadRng> {
    pub fn run(source: &str) -> crate::error::Result<Value> {
        let parser = crate::parser::Parser::new(source);
        let expr = parser.parse()?;
        let mut interpreter = Interpreter::<ThreadRng>::default();
        Ok(interpreter.interpret(&expr)?)
    }
}

impl<R: Rng> Interpreter<R> {
    pub fn new(rng: R) -> Self {
        Self { rng }
    }

    pub fn run_with_rng(source: &str, rng: R) -> crate::error::Result<Value> {
        let parser = crate::parser::Parser::new(source);
        let expr = parser.parse()?;
        let mut interpreter = Self::new(rng);
        Ok(interpreter.interpret(&expr)?)
    }

    pub fn interpret(&mut self, expression: &Expr<'_>) -> Result<Value> {
        expression.interpret(self)
    }
}

impl Default for Interpreter<ThreadRng> {
    fn default() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }
}

impl Expr<'_> {
    fn interpret<R: Rng>(&self, interpreter: &mut Interpreter<R>) -> Result<Value> {
        match self {
            Expr::Unary { operator, right } => {
                let right = right.interpret(interpreter)?;
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
                let (left, right) = (left.interpret(interpreter)?, right.interpret(interpreter)?);
                match operator.lexeme() {
                    "+" => Ok(left + right),
                    "-" => Ok(left - right),
                    "*" | "(" => Ok(left * right),
                    "/" => Ok(left / right),
                    _ => unreachable!(),
                }
            }
            Expr::Grouping { expression } => expression.interpret(interpreter),
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Roll {
                quantity,
                dice: _dice,
                faces,
            } => {
                let quantity = quantity.interpret(interpreter)?;
                let faces = faces.interpret(interpreter)?;

                let results: Vec<isize> = (0..*quantity)
                    .map(|_| interpreter.rng.gen_range(1..=*faces))
                    .collect();
                let value = results.iter().sum();

                let all = Kind::Roll(
                    results
                        .into_iter()
                        .map(|i| Kind::Direct(i))
                        .intersperse(Kind::Token("+".to_string()))
                        .collect(),
                );
                Ok(Value::new(value, vec![Kind::Direct(value), all]))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, Interpreter};

    #[test]
    fn test_value() -> Result<(), Error> {
        let res = Interpreter::run("1")?;
        assert_eq!(res, 1);
        let res = Interpreter::run("4000")?;
        assert_eq!(res, 4000);
        Ok(())
    }

    #[test]
    fn test_unary() -> Result<(), Error> {
        let res = Interpreter::run("-1")?;
        assert_eq!(res, -1);
        Ok(())
    }

    #[test]
    fn test_binary() -> Result<(), Error> {
        let test_values = [
            ("1 + 1", 2),
            ("5 - 1", 4),
            ("2 * 3", 6),
            ("6 / 3", 2),
            ("1 / 2", 0),
            ("2 + 3 * 2", 8),
            ("2 + (3 * 2)", 8),
            ("2 * (3 + 2)", 10),
        ];

        for (input, output) in test_values {
            println!("parsing {input}");
            let res = Interpreter::run(input)?;
            assert_eq!(res, output);
        }
        Ok(())
    }
}
