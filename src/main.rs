use clap::{ArgEnum, Parser};
use lexer::Lexer;
use parser;
use std::{
    fs::File,
    io::{self, BufReader, Read},
};
#[derive(Parser, Debug)]
struct Args {
    filename: String,

    #[clap(long, arg_enum)]
    pprint: Option<PPrintMode>,
}

#[derive(Debug, Copy, Clone, ArgEnum)]
enum PPrintMode {
    Token,
    AST,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    match args.pprint {
        Some(PPrintMode::Token) => run_lexer(&args.filename)?,
        Some(PPrintMode::AST) => run_parser(&args.filename)?,
        None => (),
    }

    Ok(())
}

fn read_file(filename: &str) -> io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();

    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn run_lexer(filename: &str) -> io::Result<()> {
    let input = read_file(filename)?;

    let mut lexer = Lexer::new(&input);

    while let Some(token) = lexer.next_token() {
        println!("{:?}", token);
    }

    Ok(())
}

fn run_parser(filename: &str) -> io::Result<()> {
    let input = read_file(filename)?;

    let mut lexer = Lexer::new(&input);

    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    let ast = parser::Parser::new(tokens).parse_expr();

    println!("{:?}", ast);

    Ok(())
}
