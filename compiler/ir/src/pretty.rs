use crate::{constant::*, stmt::*, *};

use std::{self, fmt::Write};

const INDENT: &str = "    ";

pub fn ir_to_string(body: &Body) -> String {
    let mut w = String::new();
    write_ir_body(body, &mut w).unwrap();
    w
}

pub fn write_ir_body(body: &Body, w: &mut dyn Write) -> fmt::Result {
    writeln!(w, "{{")?;
    write_ir_local_decls(&body.local_decls, w)?;
    writeln!(w)?;
    write_ir_blocks(&body.blocks, w)?;
    writeln!(w, "}}")?;
    Ok(())
}

fn write_ir_local_decls(local_decls: &TiVec<LocalId, LocalDecl>, w: &mut dyn Write) -> fmt::Result {
    let named_locals = local_decls.iter_enumerated().filter_map(|(id, decl)| {
        if let Some(name) = &decl.name {
            Some((id, name))
        } else {
            None
        }
    });

    for (id, name) in named_locals {
        writeln!(w, "{}debug {} => %{};", INDENT, &name, id.index())?;
    }

    for (i, _decl) in local_decls.iter_enumerated() {
        writeln!(w, "{}let %{};", INDENT, i.index())?;
    }

    Ok(())
}

fn write_ir_blocks(blocks: &TiVec<BlockId, Block>, w: &mut dyn Write) -> fmt::Result {
    for (id, block) in blocks.iter_enumerated() {
        writeln!(w, "{}b{} {{", INDENT, id.0)?;
        for stmt in &block.stmts {
            write_ir_stmt(&stmt, w)?;
        }
        writeln!(w, "{}}}", INDENT)?;
    }

    Ok(())
}

fn write_ir_stmt(stmt: &Statement, w: &mut dyn Write) -> fmt::Result {
    write!(w, "{}{}", INDENT, INDENT)?;
    match stmt {
        Statement::Assign(assign) => {
            write_ir_place(&assign.0, w)?;
            write!(w, " = ")?;
            write_ir_rvalue(&assign.1, w)?;
            writeln!(w, ";")?;
        }
    }
    Ok(())
}

fn write_ir_place(place: &Place, w: &mut dyn Write) -> fmt::Result {
    let idx = place.local.index();
    write!(w, "%{}", idx)
}

fn write_ir_operand(op: &Operand, w: &mut dyn Write) -> fmt::Result {
    match op {
        Operand::Copy(place) => write_ir_place(place, w),
        Operand::Constant(constant) => match &constant.literal {
            ConstValue::Scalar(scalar) => {
                write!(w, "{}", &scalar.data)
            }
        },
    }
}

fn write_ir_rvalue(rvalue: &RValue, w: &mut dyn Write) -> fmt::Result {
    match rvalue {
        RValue::Use(op) => write_ir_operand(op, w),

        RValue::BinaryOp(op, operands) => {
            match op {
                BinOp::Add => write!(w, "Add")?,
                BinOp::Sub => write!(w, "Sub")?,
                BinOp::Mul => write!(w, "Mul")?,
                BinOp::Div => write!(w, "Div")?,
            }

            let lhs = &operands.0;
            let rhs = &operands.1;

            write!(w, "(")?;
            write_ir_operand(lhs, w)?;
            write!(w, ", ")?;
            write_ir_operand(rhs, w)?;
            write!(w, ")")?;

            Ok(())
        }

        RValue::UnaryOp(op, operand) => {
            match op {
                UnOp::Neg => write!(w, "Neg")?,
            }

            write!(w, "(")?;
            write_ir_operand(&operand, w)?;
            write!(w, ")")?;
            Ok(())
        }
    }
}
