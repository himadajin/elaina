use ast_lowering::LoweringContext;
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
    IR,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    match args.pprint {
        Some(mode) => {
            let input = read_file(&args.filename)?;
            match mode {
                PPrintMode::Token => pprint_token(&input),
                PPrintMode::AST => pprint_ast(&input),
                PPrintMode::IR => pprint_ir(&input),
            }
        }
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

fn pprint_token(input: &str) {
    let mut lexer = Lexer::new(&input);

    while let Some(token) = lexer.next_token() {
        println!("{:?}", token);
    }
}

fn pprint_ast(input: &str) {
    let mut lexer = Lexer::new(&input);

    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    let ast = parser::Parser::new(tokens).parse_expr();

    println!("{:?}", ast);
}

fn pprint_ir(input: &str) {
    let mut lexer = Lexer::new(&input);

    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    let ast = parser::Parser::new(tokens).parse_stmt();
    let mut lowering_ctx = LoweringContext::new();
    lowering_ctx.lower_stmt(&ast);

    let ir = lowering_ctx.build();

    for local in ir.local_decls {
        println!("{}", &local);
    }

    for stmt in ir.stmts {
        println!("{}", &stmt);
    }
}
