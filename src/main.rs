use clap::{ArgEnum, Parser, Subcommand};

use ast_lowering;
use codegen_llvm::{codegen_and_execute, codegen_string};
use mir::pretty;
use parser::lexer::parse_all_token;
use parser::{self, parse_block_from_source_str};
use thir_lowering;

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
    THIR,
    MIR,
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
                PrintMode::MIR => print_mir(&input),
                PrintMode::THIR => print_thir(&input),
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

fn run_input(input: &str) -> Result<(), Box<dyn Error>> {
    let (ast, map) = parse_block_from_source_str(input);
    let thir = ast_lowering::LoweringContext::new().lower_block(&ast);
    let ir = {
        let mut ctx = thir_lowering::LoweringContext::new(&map);
        ctx.lower_main_block(&thir);
        ctx.build()
    };

    let _ = codegen_and_execute(ir)?;
    Ok(())
}

fn print_token(input: &str) {
    let tokens = parse_all_token(input).tokens;
    for token in tokens {
        println!("{:?}", token);
    }
}

fn print_ast(input: &str) {
    let (ast, _) = parse_block_from_source_str(input);

    println!("{:#?}", ast);
}

fn print_thir(input: &str) {
    let (ast, _) = parse_block_from_source_str(input);
    let thir = ast_lowering::LoweringContext::new().lower_block(&ast);
    println!("{:#?}", thir);
}

fn print_mir(input: &str) {
    let (ast, map) = parse_block_from_source_str(input);
    let thir = ast_lowering::LoweringContext::new().lower_block(&ast);
    let ir = {
        let mut ctx = thir_lowering::LoweringContext::new(&map);
        ctx.lower_main_block(&thir);
        ctx.build()
    };

    let pretty = pretty::ir_to_string(&ir);
    println!("{}", pretty);
}

fn print_llvm(input: &str) {
    let (ast, map) = parse_block_from_source_str(input);
    let thir = ast_lowering::LoweringContext::new().lower_block(&ast);
    let ir = {
        let mut ctx = thir_lowering::LoweringContext::new(&map);
        ctx.lower_main_block(&thir);
        ctx.build()
    };
    print!("{}", codegen_string(ir));
}
