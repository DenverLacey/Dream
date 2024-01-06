use quicksand::Instruction;

use super::block_builder::BlockBuilder;

pub struct ProcedureBuilder<'out> {
    out: &'out mut Vec<u8>,
}

impl<'out> ProcedureBuilder<'out> {
    pub fn new(out: &'out mut Vec<u8>) -> Self {
        Self { out }
    }

    pub fn body(&mut self, f: impl FnOnce(&mut BlockBuilder)) {
        {
            let mut block = BlockBuilder::new(self.out);
            f(&mut block);
        }
        if self
            .out
            .last()
            .filter(|&&l| l == Instruction::Ret as u8)
            .is_none()
        {
            self.out.push(Instruction::Ret as u8);
        }
    }
}
