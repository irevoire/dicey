use std::{
    io::{BufRead, Write},
    path::Path,
};

use dicey::*;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() > 3 {
        return Err(SetupError::Usage)?;
    }

    if let Some(filename) = args.get(1) {
        run_file(filename)
    } else if atty::is(atty::Stream::Stdin) {
        run_prompt()
    } else {
        run_file("/dev/stdin")
    }
}

fn run_file(filename: impl AsRef<Path>) -> Result<()> {
    let source = std::fs::read_to_string(filename).map_err(SetupError::from)?;
    run(&source)
}

fn run_prompt() -> Result<()> {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut stdout = std::io::stdout();

    print!("> ");
    stdout.flush().map_err(SetupError::from)?;

    for line in stdin.lines() {
        let line = line.map_err(SetupError::from)?;
        match run(&line) {
            Ok(_) => (),
            Err(Error::Parser(error)) => println!("{:?}", miette::Report::from(error)),
            Err(error) => println!("{:?}", error),
        }
        print!("> ");
        stdout.flush().map_err(SetupError::from)?;
    }

    Ok(())
}

fn run(source: &str) -> Result<()> {
    let parser = Parser::new(source);
    let expr = parser.parse()?;
    let result = expr.interpret().unwrap();
    println!("{}", result);

    Ok(())
}
