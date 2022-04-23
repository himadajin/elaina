use anyhow::Result;
use clap::{ArgEnum, Parser, Subcommand};

use ast_lowering;
#[allow(unused_imports)]
use codegen_llvm::{codegen_and_execute, codegen_string};
use hir_lowering;
#[allow(unused_imports)]
use mir::pretty;
use parser::lexer::parse_all_token;
use parser::{self, parse_block_from_source_str, parse_items};
use resolve::{resolve_items, ASTNameResolver};
#[allow(unused_imports)]
use thir_lowering;

use std::{
    fs::File,
    io::{BufReader, Read},
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
    HIR,
    THIR,
    MIR,
    LLVM,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Run { filename } => {
            let input = read_file(&filename)?;
            run_input(&input)?;
        }

        Commands::Print { mode, filename } => {
            let input = read_file(&filename)?;
            match mode {
                PrintMode::Token => print_token(&input)?,
                PrintMode::AST => print_ast(&input)?,
                PrintMode::HIR => print_hir(&input)?,
                PrintMode::THIR => print_thir(&input)?,
                PrintMode::MIR => print_mir(&input)?,
                PrintMode::LLVM => print_llvm(&input)?,
            }
        }
    }

    Ok(())
}

fn read_file(filename: &str) -> Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();

    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn run_input(input: &str) -> Result<()> {
    let (ast, map) = parse_block_from_source_str(input)?;
    let res = {
        let mut resolver = ASTNameResolver::new();
        resolver.resolve_block(&ast);
        resolver.finish()
    };
    let hir = ast_lowering::LoweringCtx::new(res).lower_block(&ast);
    let thir = hir_lowering::LoweringCtx::new().lower_block(&hir);
    let mir = {
        let mut ctx = thir_lowering::LoweringCtx::new(&map);
        ctx.lower_main_block(&thir);
        ctx.build()
    };
    let _ = codegen_and_execute(mir)?;
    Ok(())
}

fn print_token(input: &str) -> Result<()> {
    let tokens = parse_all_token(input).tokens;
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

fn print_ast(input: &str) -> Result<()> {
    let (ast, _) = parse_block_from_source_str(input)?;

    println!("{:#?}", ast);
    Ok(())
}

fn print_hir(input: &str) -> Result<()> {
    let (ast, map) = parse_items(input)?;
    let res = resolve_items(ast.as_slice());
    let hir = ast_lowering::LoweringCtx::new(res).lower_items(ast.as_slice());

    let hir_print = hir::pp::print_items(&map, hir.as_slice());
    println!("{}", hir_print);
    Ok(())
}

fn print_thir(input: &str) -> Result<()> {
    let (ast, _) = parse_items(input)?;
    let res = resolve_items(ast.as_slice());
    let hir = ast_lowering::LoweringCtx::new(res).lower_items(ast.as_slice());
    let thir = hir_lowering::LoweringCtx::new().lower_items(&hir);
    println!("{:#?}", thir);
    Ok(())
}

fn print_mir(input: &str) -> Result<()> {
    let (ast, map) = parse_block_from_source_str(input)?;
    let res = {
        let mut resolver = ASTNameResolver::new();
        resolver.resolve_block(&ast);
        resolver.finish()
    };
    let hir = ast_lowering::LoweringCtx::new(res).lower_block(&ast);
    let thir = hir_lowering::LoweringCtx::new().lower_block(&hir);
    let mir = {
        let mut ctx = thir_lowering::LoweringCtx::new(&map);
        ctx.lower_main_block(&thir);
        ctx.build()
    };

    let mir_string = pretty::ir_to_string(&mir);
    println!("{}", mir_string);
    Ok(())
}

fn print_llvm(input: &str) -> Result<()> {
    let (ast, map) = parse_block_from_source_str(input)?;
    let res = {
        let mut resolver = ASTNameResolver::new();
        resolver.resolve_block(&ast);
        resolver.finish()
    };
    let hir = ast_lowering::LoweringCtx::new(res).lower_block(&ast);
    let thir = hir_lowering::LoweringCtx::new().lower_block(&hir);
    let mir = {
        let mut ctx = thir_lowering::LoweringCtx::new(&map);
        ctx.lower_main_block(&thir);
        ctx.build()
    };
    print!("{}", codegen_string(mir));
    Ok(())
}
