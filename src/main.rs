use clap::{ArgEnum, Parser, Subcommand};
#[allow(unused_imports)]
use codegen_llvm::{codegen_and_execute, codegen_string};
#[allow(unused_imports)]
use ir::pretty;
use lexer::run_lexer;
use parser::{self, parse_block_from_source_str};
#[allow(unused_imports)]
use thir_lowering::LoweringContext;

use std::{
    error::Error,
    fs::File,
    io::{self, BufReader, Read},
};
#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        filename: String,
    },
    Print {
        #[clap(arg_enum)]
        mode: PrintMode,
        filename: String,
    },
}

#[derive(Debug, Copy, Clone, ArgEnum)]
enum PrintMode {
    Token,
    AST,
    IR,
    LLVM,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Run { filename } => {
            let input = read_file(&filename)?;
            run_input(&input)?;
        }

        Commands::Print { mode, filename } => {
            let input = read_file(&filename)?;
            match mode {
                PrintMode::Token => print_token(&input),
                PrintMode::AST => print_ast(&input),
                PrintMode::IR => print_ir(&input),
                PrintMode::LLVM => print_llvm(&input),
            }
        }
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

fn run_input(_input: &str) -> Result<(), Box<dyn Error>> {
    todo!();
    // let ast = parse_block_from_source_str(input);
    // let mut lowering_ctx = LoweringContext::new();
    // lowering_ctx.lower_main_block(&ast);

    // let ir = lowering_ctx.build();
    // let _ = codegen_and_execute(ir)?;
    // Ok(())
}

fn print_token(input: &str) {
    for token in run_lexer(input) {
        println!("{:?}", token);
    }
}

fn print_ast(input: &str) {
    let ast = parse_block_from_source_str(input);

    println!("{:?}", ast);
}

fn print_ir(_input: &str) {
    todo!();
    // let ast = parse_block_from_source_str(input);
    // let mut lowering_ctx = LoweringContext::new();
    // lowering_ctx.lower_main_block(&ast);

    // let ir = lowering_ctx.build();

    // let pretty = pretty::ir_to_string(&ir);
    // println!("{}", pretty);
}

fn print_llvm(_input: &str) {
    todo!();
    // let ast = parse_block_from_source_str(input);
    // let mut lowering_ctx = LoweringContext::new();
    // lowering_ctx.lower_main_block(&ast);

    // let ir = lowering_ctx.build();

    // print!("{}", codegen_string(ir));
}
