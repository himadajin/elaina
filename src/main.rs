use ast_lowering::LoweringContext;
use clap::{ArgEnum, Parser};
use codegen_llvm::codegen_ir_body;
use lexer::run_lexer;
use parser::{self, parse_block_from_source_str};

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
    LLVM,
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
                PPrintMode::LLVM => pprint_llvm(&input),
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
    for token in run_lexer(input) {
        println!("{:?}", token);
    }
}

fn pprint_ast(input: &str) {
    let ast = parse_block_from_source_str(input);

    println!("{:?}", ast);
}

fn pprint_ir(input: &str) {
    let ast = parse_block_from_source_str(input);
    let mut lowering_ctx = LoweringContext::new();
    lowering_ctx.lower_main_block(&ast);

    let ir = lowering_ctx.build();

    for local in ir.local_decls {
        println!("{}", &local);
    }

    for stmt in &ir.blocks.first().unwrap().stmts {
        println!("{}", &stmt);
    }
}

fn pprint_llvm(input: &str) {
    let ast = parse_block_from_source_str(input);
    let mut lowering_ctx = LoweringContext::new();
    lowering_ctx.lower_main_block(&ast);

    let ir = lowering_ctx.build();

    print!("{}", codegen_ir_body(ir));
}
