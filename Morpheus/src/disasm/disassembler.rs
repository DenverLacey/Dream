use std::iter::Peekable;

use quicksand::{Instruction, Register};

use crate::{Error, OutputType, Result, Version, Write};

pub struct Disassembler<'out, I: IntoIterator<Item = u8>> {
    bytes: Peekable<I::IntoIter>,
    offset: usize,
    out: &'out mut dyn Write,
    version: u32,
    output_type: OutputType,
}

impl<'out, I> Disassembler<'out, I>
where
    I: IntoIterator<Item = u8>,
{
    pub fn new(bytes: I, out: &'out mut dyn Write) -> Self {
        Self {
            bytes: bytes.into_iter().peekable(),
            offset: 0,
            out,
            version: 0,
            output_type: OutputType::Bin,
        }
    }
}

impl<'out, I> Disassembler<'out, I> 
where
    I: IntoIterator<Item = u8>,
{
    pub fn disassemble(&mut self) -> Result<()> {
        self.disassemble_header()?;

        let mut disassembled_text = false;
        let mut disassembled_code = false;

        while self.peek().is_some() {
            if self.matches_all(b"TEXT") {
                if disassembled_text {
                    eprintln!("ERROR: Cannot have more than one TEXT section in a dream file.");
                    return Err(Error::DisassembleFailure);
                }
                disassembled_text = true;
                self.disassemble_text_section()?;
            } else if self.matches_all(b"CODE") {
                if disassembled_code {
                    eprintln!("ERROR: Cannot have more than one CODE section in a dream file.");
                    return Err(Error::DisassembleFailure);
                }
                disassembled_code = true;
                self.disassemble_code_section()?;
            } else {
                eprintln!("ERROR: Didn't encounter valid section header while parsing dream file.");
                return Err(Error::DisassembleFailure);
            }
        }

        Ok(())
    }

    fn peek(&mut self) -> Option<u8> {
        self.bytes.peek().copied()
    }

    fn next(&mut self) -> Option<u8> {
        self.offset += 1;
        self.bytes.next()
    }

    fn matches(&mut self, byte: u8) -> bool {
        if self.bytes.next_if_eq(&byte).is_some() {
            self.offset += 1;
            true
        } else {
            false
        }
    }

    fn matches_all(&mut self, bytes: &[u8]) -> bool {
        for &b in bytes {
            if !self.matches(b) {
                return false;
            }
        }
        return true;
    }

    fn extract(&mut self, dst: &mut [u8]) -> Result<()> {
        for i in 0..dst.len() {
            dst[i] = self.next().ok_or_else(|| {
                eprintln!("ERROR: Unexpected end of dream file.");
                Error::DisassembleFailure
            })?;
        }
        Ok(())
    }

    fn extract_u32(&mut self) -> Result<u32> {
        let mut bytes = [0u8; std::mem::size_of::<u32>()];
        self.extract(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn extract_u64(&mut self) -> Result<u64> {
        let mut bytes = [0u8; std::mem::size_of::<u64>()];
        self.extract(&mut bytes)?;
        Ok(u64::from_le_bytes(bytes))
    }

    fn extract_reg(&mut self) -> Result<Register> {
        let reg = self.next().ok_or(Error::DisassembleFailure)?;
        reg.try_into().or(Err(Error::DisassembleFailure))
    }

    fn disassemble_header(&mut self) -> Result<()> {
        if !self.matches_all(b"DREAM") {
            eprintln!("ERROR: Not a dream file.");
            return Err(Error::DisassembleFailure);
        }

        let mut version_bytes = [0u8; 3];
        self.extract(&mut version_bytes)?;

        let version: Version = std::str::from_utf8(&version_bytes)
            .map_err(|err| {
                eprintln!("ERROR: Version bytes not valid UTF-8: {err}");
                Error::DisassembleFailure
            })?
            .parse().map_err(|_| {
                eprintln!("ERROR: Failed to parse version number.");
                Error::DisassembleFailure
            })?;

        self.version = version.as_u32();

        self.out.write_str("#Version ")?;
        self.out.write_bytes(&version_bytes).map_err(|err| {
            eprintln!("ERROR: Failed to write version to disassembly file.");
            err
        })?;
        self.out.write_chr('\n')?;

        if !self.matches_all(b"OUTT") {
            eprintln!("ERROR: No output type found for dream file.");
            return Err(Error::DisassembleFailure);
        }

        self.output_type = self.extract_u32().and_then(|ot| OutputType::try_from(ot)).map_err(|_| {
            eprintln!("ERROR: Failed to parse output type of dream file.");
            Error::DisassembleFailure
        })?;

        let output_type_line = format!("#OutputType {:?}\n", self.output_type);
        self.out.write_str(&output_type_line)?;

        self.out.write_chr('\n')?;

        Ok(())
    }

    fn disassemble_text_section(&mut self) -> Result<()> {
        if !self.matches_all(&[0u8; 4]) {
            eprintln!("ERROR: No padding bytes after 'TEXT' header.");
            return Err(Error::DisassembleFailure);
        }

        self.out.write_str(&format!("{:08X}  TEXT:\n", self.offset - 8))?;

        let data_size = self.extract_u64().map_err(|_| {
            eprintln!("ERROR: Failed to parse data size of TEXT section.");
            Error::DisassembleFailure
        })? as usize;

        let mut data_remaining = data_size;
        while data_remaining > 0 {
            let string_offset = self.offset;

            let string_size = self.extract_u64().map_err(|_| {
                eprintln!("ERROR: Failed to parse length of string in TEXT section.");
                Error::DisassembleFailure
            })?;

            data_remaining -= std::mem::size_of::<u64>();

            self.out.write_str(&format!("{string_offset:08X}      \""))?;

            for _ in 0..string_size {
                let c = self.next().ok_or_else(|| {
                    eprintln!("ERROR: Unexpected end of dream file while parsing TEXT section.");
                    Error::DisassembleFailure
                })?;

                data_remaining -= 1;

                let escaped = std::ascii::escape_default(c);
                for e in escaped {
                    self.out.write_bytes(&[e])?;
                }
            }

            if !self.matches_all(&[0u8; 8]) {
                eprintln!("ERROR: Did not encounter padding bytes after string data in TEXT section.");
                return Err(Error::DisassembleFailure);
            }

            data_remaining -= 8;

            self.out.write_str("\"\n")?;
        }

        self.out.write_chr('\n')?;

        Ok(())
    }

    fn disassemble_code_section(&mut self) -> Result<()> {
        if !self.matches_all(&[0u8; 4]) {
            eprintln!("ERROR: No padding bytes after 'CODE' header.");
            return Err(Error::DisassembleFailure);
        }

        self.out.write_str(&format!("{:08X}  CODE:\n", self.offset - 8))?;

        let code_size = self.extract_u64().map_err(|_| {
            eprintln!("ERROR: Failed to parse length of code section.");
            Error::DisassembleFailure
        })?;

        let entry_point = self.extract_u64().map_err(|_| {
            eprintln!("ERROR: Failed to parse entry point in code section.");
            Error::DisassembleFailure
        })?;

        let code_begin = self.offset;

        let mut code_remaining = code_size;
        while code_remaining > 0 {
            let inst_offset = self.offset;

            let inst = self.next().ok_or_else(|| {
                eprintln!("ERROR: Unexpected end of dream file in CODE section.");
                Error::DisassembleFailure
            })?;

            code_remaining -= 1;

            let is_alt = inst & Instruction::ALT_MODE != 0;
            let inst: Instruction = inst.try_into().map_err(|_| {
                eprintln!("ERROR: Invalid instruction in CODE section.");
                Error::DisassembleFailure
            })?;

            if inst_offset - code_begin == entry_point as usize {
                self.out.write_str("ENTRY:\n")?;
            }

            let inst_str = format!("{inst:?}");
            self.out.write_str(&format!("{inst_offset:08X}      {inst_str:<12}"))?;

            match inst {
                Instruction::NoOp => {}
                Instruction::Move => {
                    if is_alt {
                        let dst = self.extract_u64()?;
                        code_remaining -= 8;
                        let src = self.extract_reg()?;
                        code_remaining -= 1;
                        self.out.write_str(&format!("[{dst}], {src}"))?;
                    } else {
                        let dst = self.extract_reg()?;
                        code_remaining -= 1;
                        let src = self.extract_reg()?;
                        code_remaining -= 1;
                        self.out.write_str(&format!("{dst}, {src}"))?;
                    }
                }
                Instruction::MoveImm => {
                    if is_alt {
                        let dst = self.extract_u64()?;
                        code_remaining -= 8;
                        let value = self.extract_u64()?;
                        code_remaining -= 8;
                        self.out.write_str(&format!("[{dst}], ${value}"))?;
                    } else {
                        let dst = self.extract_reg()?;
                        code_remaining -= 1;
                        let value = self.extract_u64()?;
                        code_remaining -= 8;
                        self.out.write_str(&format!("{dst}, ${value}"))?;
                    }
                }
                Instruction::MoveAddr => {
                    if is_alt {
                        let dst = self.extract_u64()?;
                        code_remaining -= 8;
                        let src = self.extract_u64()?;
                        code_remaining -= 8;
                        let size = self.extract_u64()?;
                        code_remaining -= 8;
                        self.out.write_str(&format!("[{dst}], [{src}], ${size}"))?;
                    } else {
                        let dst = self.extract_reg()?;
                        code_remaining -= 1;
                        let src = self.extract_u64()?;
                        code_remaining -= 8;
                        self.out.write_str(&format!("{dst}, [{src}]"))?;
                    }
                }
                Instruction::Clear => {
                    if is_alt {
                        return Err(Error::InvalidInstruction);
                    }
                    let reg = self.extract_reg()?;
                    code_remaining -= 1;
                    self.out.write_str(&format!("{reg}"))?;
                }
                Instruction::Set => {
                    if is_alt {
                        return Err(Error::InvalidInstruction);
                    }
                    let reg = self.extract_reg()?;
                    code_remaining -= 1;
                    self.out.write_str(&format!("{reg}"))?;
                }
                Instruction::Push => {
                    if is_alt {
                        let src = self.extract_u64()?;
                        code_remaining -= 8;
                        self.out.write_str(&format!("[{src}]"))?;
                    } else {
                        let src = self.extract_reg()?;
                        code_remaining -= 1;
                        self.out.write_str(&format!("{src}"))?;
                    }
                }
                Instruction::PushImm => {
                    if is_alt {
                        return Err(Error::InvalidInstruction);
                    }
                    let value = self.extract_u64()?;
                    code_remaining -= 8;
                    self.out.write_str(&format!("${value}"))?;
                }
                Instruction::Pop => {
                    if is_alt {
                        return Err(Error::InvalidInstruction);
                    }
                    let reg = self.extract_reg()?;
                    code_remaining -= 1;
                    self.out.write_str(&format!("{reg}"))?;
                }
                Instruction::StackLoad => {
                    if is_alt {
                        return Err(Error::InvalidInstruction);
                    }
                    let dst = self.extract_reg()?;
                    code_remaining -= 1;
                    let src = self.extract_u64()?;
                    code_remaining -= 8;
                    self.out.write_str(&format!("{dst}, [stk+{src}]"))?;
                }
                Instruction::Map => {
                    if is_alt {
                        return Err(Error::InvalidInstruction);
                    }
                    let dst = self.extract_reg()?;
                    code_remaining -= 1;
                    let index = self.extract_u64()?;
                    code_remaining -= 8;
                    self.out.write_str(&format!("{dst}, ${index}"))?;
                }
                Instruction::Syscall0 => {}
                Instruction::Syscall1 => {}
                Instruction::Syscall2 => {}
                Instruction::Syscall3 => {}
                Instruction::Syscall4 => {}
                Instruction::Syscall5 => {}
                Instruction::Syscall6 => {}
                Instruction::Ret => {}
            }

            self.out.write_chr('\n')?;
        }

        Ok(())
    }
}

