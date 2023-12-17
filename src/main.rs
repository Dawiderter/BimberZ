use bimberz::{parser::{lexer::Lexer, token::Token, error::Error}, interpreter::interpreter::{interpret}};

fn main() -> Result<(), Error> {
    let args = std::env::args().collect::<Vec<_>>();

    let mut program = String::new();
    let mut file = std::fs::File::open(&args[1]).unwrap();
    std::io::Read::read_to_string(&mut file, &mut program).unwrap();

    let program = program.chars().collect::<Vec<_>>();

    let tokens: Vec<_> = Lexer::new(&program).collect();
    let tokens: Result<Vec<Token>, Error> = tokens.into_iter().collect();

    let statements = bimberz::parser::parser::parse(&tokens.unwrap()).unwrap();

    interpret(statements)
}
