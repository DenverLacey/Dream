use crate::{Error, Operand, Result};
use quicksand::{inst_sig, Instruction, OperandType, INST_ALT_MODE};

pub struct BlockBuilder<'out> {
    out: &'out mut Vec<u8>,
}

impl<'out> BlockBuilder<'out> {
    pub fn new(out: &'out mut Vec<u8>) -> Self {
        Self { out }
    }

    pub fn emit_move(&mut self, dst: Operand, src: Operand, _size: Option<usize>) -> Result<()> {
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
                OperandType::Lit64 => {
                    self.out.push(Instruction::MoveImm as u8);
                    self.out.push(dst.value as u8);
                    self.out.extend(src.value.to_le_bytes());
                }
            },
            OperandType::Address => match src.kind {
                OperandType::Register => todo!(),
                OperandType::Address => todo!(),
                OperandType::Lit64 => todo!(),
            },
            _ => return Err(Error::BadOperandType),
        }

        Ok(())
    }

    pub fn emit_ret(&mut self) {
        self.out.push(Instruction::Ret as u8);
    }
}
