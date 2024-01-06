use crate::{Error, Operand, Result};
use quicksand::{Instruction, OperandType, Register, INST_ALT_MODE};

pub struct BlockBuilder<'out> {
    out: &'out mut Vec<u8>,
}

impl<'out> BlockBuilder<'out> {
    pub fn new(out: &'out mut Vec<u8>) -> Self {
        Self { out }
    }

    pub fn emit_move(&mut self, dst: Operand, src: Operand, size: Option<u64>) -> Result<()> {
        match dst.kind {
            OperandType::Register => match src.kind {
                OperandType::Register => {
                    self.out.push(Instruction::Move as u8);
                    self.out.push(dst.value as u8);
                    self.out.push(src.value as u8);
                }
                OperandType::Address => {
                    self.out.push(Instruction::MoveAddr as u8);
                    self.out.push(dst.value as u8);
                    self.out.extend(src.value.to_le_bytes());
                }
                OperandType::Lit64 => match src.value {
                    0 => {
                        self.out.push(Instruction::Clear as u8);
                        self.out.push(dst.value as u8);
                    }
                    1 => {
                        self.out.push(Instruction::Set as u8);
                        self.out.push(dst.value as u8);
                    }
                    _ => {
                        self.out.push(Instruction::MoveImm as u8);
                        self.out.push(dst.value as u8);
                        self.out.extend(src.value.to_le_bytes());
                    }
                },
            },
            OperandType::Address => match src.kind {
                OperandType::Register => {
                    self.out.push(Instruction::Move as u8 | INST_ALT_MODE);
                    self.out.extend(dst.value.to_le_bytes());
                    self.out.push(src.value as u8);
                }
                OperandType::Address => {
                    self.out.push(Instruction::MoveAddr as u8 | INST_ALT_MODE);
                    self.out.extend(dst.value.to_le_bytes());
                    self.out.extend(src.value.to_le_bytes());
                    if let Some(size) = size {
                        self.out.extend(size.to_le_bytes());
                    } else {
                        self.out
                            .extend((std::mem::size_of::<u64>() as u64).to_le_bytes());
                    }
                }
                OperandType::Lit64 => match src.value {
                    0 => {
                        self.out.push(Instruction::Clear as u8 | INST_ALT_MODE);
                        self.out.push(dst.value as u8);
                    }
                    1 => {
                        self.out.push(Instruction::Set as u8 | INST_ALT_MODE);
                        self.out.push(dst.value as u8);
                    }
                    _ => {
                        self.out.push(Instruction::MoveImm as u8 | INST_ALT_MODE);
                        self.out.extend(dst.value.to_le_bytes());
                        self.out.extend(src.value.to_le_bytes());
                    }
                },
            },
            _ => return Err(Error::BadOperandType),
        }

        Ok(())
    }

    pub fn emit_clear(&mut self, reg: Register) {
        self.out.push(Instruction::Clear as u8);
        self.out.push(reg.to_u8());
    }

    pub fn emit_set(&mut self, reg: Register) {
        self.out.push(Instruction::Set as u8);
        self.out.push(reg.to_u8());
    }

    pub fn emit_push(&mut self, value: Operand) {
        match value.kind {
            OperandType::Register => {
                self.out.push(Instruction::Push as u8);
                self.out.push(value.value as u8);
            }
            OperandType::Address => {
                self.out.push(Instruction::Push as u8 | INST_ALT_MODE);
                self.out.extend(value.value.to_le_bytes());
            }
            OperandType::Lit64 => {
                self.out.push(Instruction::PushImm as u8);
                self.out.extend(value.value.to_le_bytes());
            }
        }
    }

    pub fn emit_pop(&mut self, reg: Register) {
        self.out.push(Instruction::Pop as u8);
        self.out.push(reg.to_u8());
    }

    pub fn emit_syscall(&mut self, nargs: u8) -> Result<()> {
        match nargs {
            0 => self.out.push(Instruction::Syscall0 as u8),
            1 => self.out.push(Instruction::Syscall1 as u8),
            2 => self.out.push(Instruction::Syscall2 as u8),
            3 => self.out.push(Instruction::Syscall3 as u8),
            4 => self.out.push(Instruction::Syscall4 as u8),
            5 => self.out.push(Instruction::Syscall5 as u8),
            6 => self.out.push(Instruction::Syscall6 as u8),
            _ => return Err(Error::TooManyArgsForSyscall),
        }
        Ok(())
    }

    pub fn emit_ret(&mut self) {
        self.out.push(Instruction::Ret as u8);
    }
}
