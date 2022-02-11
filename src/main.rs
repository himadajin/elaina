use clap::Parser;
use lexer::Lexer;
use parser;
use std::{
    fs::File,
    io::{self, BufReader, Read},
};
#[derive(Parser, Debug)]
struct Args {
    filename: String,

    #[clap(long)]
    lexer: bool,

    #[clap(long)]
    parser: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.lexer {
        run_lexer(&args.filename)?;
    }

    if args.parser {
        run_parser(&args.filename)?;
    }

    Ok(())
}

fn run_lexer(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();

    buf_reader.read_to_string(&mut input)?;

    let mut lexer = Lexer::new(&input);

    while let Some(token) = lexer.next_token() {
        println!("{:?}", token);
    }

    Ok(())
}

fn run_parser(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();

    buf_reader.read_to_string(&mut input)?;

    let mut lexer = Lexer::new(&input);

    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    let ast = parser::Parser::new(tokens).parse_expr();

    println!("{:?}", ast);


    Ok(())
}
