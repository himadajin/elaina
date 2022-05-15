use anyhow::Result;
use clap::{ArgEnum, Parser, Subcommand};

use ast_lowering;
use codegen_llvm::{codegen_and_execute, codegen_string};
use hir_lowering;
use parser::lexer::parse_all_token;
use parser::{self, parse_block_from_source_str, parse_items};
use resolve::resolve_items;
#[allow(unused_imports)]
use thir_lowering;
use ty::{TyArena, TyCtx};

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
    let (ast, map) = parse_items(input)?;
    let res = resolve_items(ast.as_slice(), &map)?;
    let hir = ast_lowering::LoweringCtx::new(res).lower_items(ast.as_slice());

    let arena = TyArena::new();
    let context = TyCtx::new(&arena, &map);

    let mut hir_lowering_ctx = hir_lowering::HIRLoweringCtx::new(context);
    let thir = hir_lowering_ctx.lower_items(&hir);
    let context = hir_lowering_ctx.finish();
    let mir = {
        let mut mir = Vec::new();
        for item in thir {
            let mir_item = match item.kind {
                thir::ItemKind::Fn(fun) => {
                    let mut ctx =
                        thir_lowering::LoweringCtx::new(fun.header.def, fun.header.name, &context);
                    ctx.lower_item_fun(&fun.header.inputs, &fun.header.output, &fun.body);
                    ctx.build()
                }
            };
            mir.push(mir_item);
        }

        mir
    };
    codegen_and_execute(mir.as_slice(), &map)?;
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
    let res = resolve_items(ast.as_slice(), &map)?;
    let hir = ast_lowering::LoweringCtx::new(res).lower_items(ast.as_slice());

    let hir_print = hir::pp::print_items(&map, hir.as_slice());
    println!("{}", hir_print);
    Ok(())
}

fn print_thir(input: &str) -> Result<()> {
    let (ast, map) = parse_items(input)?;
    let res = resolve_items(ast.as_slice(), &map)?;
    let hir = ast_lowering::LoweringCtx::new(res).lower_items(ast.as_slice());

    let arena = TyArena::new();
    let context = TyCtx::new(&arena, &map);

    let thir = hir_lowering::HIRLoweringCtx::new(context).lower_items(&hir);

    let thir_print = thir::pp::print_items(&map, thir.as_slice());
    println!("{}", thir_print);

    Ok(())
}

fn print_mir(input: &str) -> Result<()> {
    let (ast, map) = parse_items(input)?;
    let res = resolve_items(ast.as_slice(), &map)?;
    let hir = ast_lowering::LoweringCtx::new(res).lower_items(ast.as_slice());

    let arena = TyArena::new();
    let context = TyCtx::new(&arena, &map);

    let mut hir_lowering_ctx = hir_lowering::HIRLoweringCtx::new(context);
    let thir = hir_lowering_ctx.lower_items(&hir);
    let mut context = hir_lowering_ctx.finish();

    let mir = {
        let mut mir = Vec::new();
        for item in thir {
            let mir_item = match item.kind {
                thir::ItemKind::Fn(fun) => {
                    let mut ctx = thir_lowering::LoweringCtx::new(
                        fun.header.def,
                        fun.header.name,
                        &mut context,
                    );
                    ctx.lower_item_fun(&fun.header.inputs, &fun.header.output, &fun.body);
                    ctx.build()
                }
            };
            mir.push(mir_item);
        }

        mir
    };

    let mir_print = mir::pp::print_bodies(&map, mir.as_slice());
    println!("{}", mir_print);

    Ok(())
}

fn print_llvm(input: &str) -> Result<()> {
    let (ast, map) = parse_items(input)?;
    let res = resolve_items(ast.as_slice(), &map)?;
    let hir = ast_lowering::LoweringCtx::new(res).lower_items(ast.as_slice());

    let arena = TyArena::new();
    let context = TyCtx::new(&arena, &map);

    let mut hir_lowering_ctx = hir_lowering::HIRLoweringCtx::new(context);
    let thir = hir_lowering_ctx.lower_items(&hir);
    let context = hir_lowering_ctx.finish();

    let mir = {
        let mut mir = Vec::new();
        for item in thir {
            let mir_item = match item.kind {
                thir::ItemKind::Fn(fun) => {
                    let mut ctx =
                        thir_lowering::LoweringCtx::new(fun.header.def, fun.header.name, &context);
                    ctx.lower_item_fun(&fun.header.inputs, &fun.header.output, &fun.body);
                    ctx.build()
                }
            };
            mir.push(mir_item);
        }

        mir
    };
    let llvm_ir = codegen_string(mir.as_slice(), &map);
    println!("{}", llvm_ir);

    Ok(())
}
