use clap::Parser;
use lexer::Lexer;
use std::{
    fs::File,
    io::{self, BufReader, Read},
};
#[derive(Parser, Debug)]
struct Args {
    filename: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let file = File::open(&args.filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    run_lexer(&input);

    Ok(())
}

fn run_lexer(input: &str) {
    let mut lexer = Lexer::new(input);

    while let Some(token) = lexer.next_token() {
        println!("{:?}", token);
    }
}
