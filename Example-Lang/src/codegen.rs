use std::{collections::HashMap, fs::File};

use morpheus::{
    BlockBuilder, Builder, Operand, OutputType, Register, RegisterAllocator, RegisterArena,
    RegisterType, Version,
};

use crate::ir::{Expr, Operator};

// WARN: These have be the same as what is specified by the DreamVM.
// TODO: Refactor so that we can use the DreamVM definitions directly.
const STDOUT: u64 = 2;
const SYS_WRITE: u64 = 1;

#[derive(Debug, Default)]
struct Generator {
    errored: bool,
    variables: HashMap<String, u64>,
    stack_pointer: u64,
}

pub fn compile(out: &mut File, exprs: &[Expr]) {
    let mut builder = Builder::new(Version::from(0), OutputType::Bin);
    let mut generator = Generator::default();
    let mut reg_ator = RegisterAllocator::new();
    let func_id = builder.procedure(|proc| {
        proc.body(|body| {
            let mut registers = reg_ator.start_arena();
            for expr in exprs {
                if let Err(err) =
                    compile_expression(body, &mut generator, registers.new_arena(), expr)
                {
                    println!("ERROR: {err}");
                    generator.errored = true;
                    break;
                }
            }
        });
    });

    if generator.errored {
        return;
    }

    builder.set_entry(func_id);
    builder
        .write_dream(out)
        .expect("INTERNAL ERROR: failed to write dream file.");
}

fn compile_expression(
    b: &mut BlockBuilder,
    gen: &mut Generator,
    mut registers: RegisterArena,
    expr: &Expr,
) -> Result<Register, &'static str> {
    match expr {
        Expr::Int(value) => {
            let result = registers.next(RegisterType::Q);
            b.emit_move(Operand::reg(result), Operand::lit64(*value as u64), None)
                .expect("INTERNAL ERROR: failed to emit move instruction.");
            Ok(result)
        }
        Expr::Ident(ident) => {
            if let Some(&var_offset) = gen.variables.get(ident) {
                let result = registers.next(RegisterType::Q);
                b.emit_stack_load(result, var_offset);
                Ok(result)
            } else {
                Err("Unknown identifier")
            }
        }
        Expr::Operation(Operator::Dollar, operands) => {
            if operands.len() < 1 {
                return Err("Not enough operands for operation");
            }

            b.emit_move(Operand::reg(Register::RSI), Operand::lit64(SYS_WRITE), None)
                .expect("INTERNAL ERROR: failed to emit move instruction for dollar opeartor.");

            b.emit_move(Operand::reg(Register::RS0), Operand::lit64(STDOUT), None)
                .expect("INTERNAL ERROR: failed to emit move instruction for dollar operator.");

            // TODO: Implement this for multiple operands.
            let value = compile_expression(b, gen, registers.new_arena(), &operands[0])?;
            b.emit_move(Operand::reg(Register::RS1), Operand::reg(value), None)
                .expect("INTERNAL ERROR: failed to emit move instruction.");

            b.emit_syscall(2)
                .expect("INTERNAL ERROR: failed to emit syscall2.");

            Ok(Register::RSR)
        }
        Expr::Operation(_op, operands) => {
            if operands.len() < 2 {
                return Err("Not enough operands for operation");
            }

            let _result = registers.next(RegisterType::Q);
            todo!()
        }
        Expr::Let(ident, init) => {
            let value = compile_expression(b, gen, registers.new_arena(), init)?;
            b.emit_push(Operand::reg(value));
            gen.variables.insert(ident.clone(), gen.stack_pointer);
            gen.stack_pointer += 8;
            Ok(Register::RXZ) // This is a bogus value just to avoid having to return Option<Register>
        }
    }
}

