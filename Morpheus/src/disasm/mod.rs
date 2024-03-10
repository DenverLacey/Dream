use quicksand::{Instruction, Register};

use crate::{Error, Result, Write};

pub fn disasemble(code: impl IntoIterator<Item = u8>, f: &mut dyn Write) -> Result<()> {
    let mut code = code.into_iter().enumerate();
    while let Some((i, inst)) = code.next() {
        let is_alt = inst & Instruction::ALT_MODE != 0;
        let inst = inst.try_into().or(Err(Error::InvalidInstruction))?;

        let inst_str = format!("{inst:?}");
        f.write_str(&format!("{i:08X}\t{inst_str:<12}"))?;

        match inst {
            Instruction::NoOp => {}
            Instruction::Move => disasemble_move(&mut code, is_alt, f)?,
            Instruction::MoveImm => disasemble_move_imm(&mut code, is_alt, f)?,
            Instruction::MoveAddr => disasemble_move_addr(&mut code, is_alt, f)?,
            Instruction::Clear => disasemble_clear(&mut code, is_alt, f)?,
            Instruction::Set => disasemble_set(&mut code, is_alt, f)?,
            Instruction::Push => disasemble_push(&mut code, is_alt, f)?,
            Instruction::PushImm => disasemble_push_imm(&mut code, is_alt, f)?,
            Instruction::Pop => disasemble_pop(&mut code, is_alt, f)?,
            Instruction::Map => disasemble_map(&mut code, is_alt, f)?,
            Instruction::Syscall0 => {}
            Instruction::Syscall1 => {}
            Instruction::Syscall2 => {}
            Instruction::Syscall3 => {}
            Instruction::Syscall4 => {}
            Instruction::Syscall5 => {}
            Instruction::Syscall6 => {}
            Instruction::Ret => {}
        }
        f.write_chr('\n').expect("Failed to write disasembly.");
    }

    Ok(())
}

fn disasemble_move(
    code: &mut impl Iterator<Item = (usize, u8)>,
    is_alt: bool,
    f: &mut dyn Write,
) -> Result<()> {
    if is_alt {
        let dst = extract_addr(code)?;
        let src = extract_register(code)?;
        f.write_str(&format!("[{dst}], {src}"))?;
    } else {
        let dst = extract_register(code)?;
        let src = extract_register(code)?;
        f.write_str(&format!("{dst}, {src}"))?;
    }

    Ok(())
}

fn disasemble_move_imm(
    code: &mut impl Iterator<Item = (usize, u8)>,
    is_alt: bool,
    f: &mut dyn Write,
) -> Result<()> {
    if is_alt {
        let dst = extract_addr(code)?;
        let value = extract_lit64(code)?;
        f.write_str(&format!("[{dst}], #{value}"))?;
    } else {
        let dst = extract_register(code)?;
        let value = extract_lit64(code)?;
        f.write_str(&format!("{dst}, #{value}"))?;
    }

    Ok(())
}

fn disasemble_move_addr(
    code: &mut impl Iterator<Item = (usize, u8)>,
    is_alt: bool,
    f: &mut dyn Write,
) -> Result<()> {
    if is_alt {
        let dst = extract_addr(code)?;
        let src = extract_addr(code)?;
        let size = extract_lit64(code)?;
        f.write_str(&format!("[{dst}], [{src}], #{size}"))?;
    } else {
        let dst = extract_register(code)?;
        let src = extract_addr(code)?;
        f.write_str(&format!("{dst}, [{src}]"))?;
    }

    Ok(())
}

fn disasemble_clear(
    code: &mut impl Iterator<Item = (usize, u8)>,
    is_alt: bool,
    f: &mut dyn Write,
) -> Result<()> {
    if is_alt {
        return Err(Error::InvalidInstruction);
    }

    let reg = extract_register(code)?;

    f.write_str(&format!("{reg}"))?;

    Ok(())
}

fn disasemble_set(
    code: &mut impl Iterator<Item = (usize, u8)>,
    is_alt: bool,
    f: &mut dyn Write,
) -> Result<()> {
    if is_alt {
        return Err(Error::InvalidInstruction);
    }

    let reg = extract_register(code)?;

    f.write_str(&format!("{reg}"))?;

    Ok(())
}

fn disasemble_push(
    code: &mut impl Iterator<Item = (usize, u8)>,
    is_alt: bool,
    f: &mut dyn Write,
) -> Result<()> {
    if is_alt {
        let src = extract_addr(code)?;
        f.write_str(&format!("[{src}]"))?;
    } else {
        let src = extract_register(code)?;
        f.write_str(&format!("{src}"))?;
    }

    Ok(())
}

fn disasemble_push_imm(
    code: &mut impl Iterator<Item = (usize, u8)>,
    is_alt: bool,
    f: &mut dyn Write,
) -> Result<()> {
    if is_alt {
        return Err(Error::InvalidInstruction);
    }

    let value = extract_lit64(code)?;

    f.write_str(&format!("#{value}"))?;

    Ok(())
}

fn disasemble_pop(
    code: &mut impl Iterator<Item = (usize, u8)>,
    is_alt: bool,
    f: &mut dyn Write,
) -> Result<()> {
    if is_alt {
        return Err(Error::InvalidInstruction);
    }

    let reg = extract_register(code)?;

    f.write_str(&format!("{reg}"))?;

    Ok(())
}

fn disasemble_map(
    code: &mut impl Iterator<Item = (usize, u8)>,
    is_alt: bool,
    f: &mut dyn Write,
) -> Result<()> {
    if is_alt {
        return Err(Error::InvalidInstruction);
    }

    let dst = extract_register(code)?;
    let index = extract_lit64(code)?;

    f.write_str(&format!("{dst}, #{index}"))?;

    Ok(())
}

fn extract_register(code: &mut impl Iterator<Item = (usize, u8)>) -> Result<Register> {
    let reg = code.next().ok_or(Error::NotEnoughOperandsForInstruction)?.1;
    reg.try_into().or(Err(Error::InvalidRegister))
}

fn extract_addr(code: &mut impl Iterator<Item = (usize, u8)>) -> Result<u64> {
    let mut bytes = [0u8; 8];
    for i in 0..bytes.len() {
        bytes[i] = code.next().ok_or(Error::InvalidAddr)?.1;
    }
    Ok(u64::from_le_bytes(bytes))
}

fn extract_lit64(code: &mut impl Iterator<Item = (usize, u8)>) -> Result<u64> {
    let mut bytes = [0u8; 8];
    for i in 0..bytes.len() {
        bytes[i] = code.next().ok_or(Error::InvalidLit64)?.1;
    }
    Ok(u64::from_le_bytes(bytes))
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    #[test]
    pub fn disasemble() {
        let file = BufReader::new(File::open("tests/input/test_disasemble.bin").unwrap());
        let mut output = File::create("tests/test_disasemble.txt").unwrap();
        let result = super::disasemble(file.bytes().map(Result::unwrap), &mut output);
        assert!(result.is_ok());
    }
}

