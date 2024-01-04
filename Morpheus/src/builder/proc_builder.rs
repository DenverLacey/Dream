use super::block_builder::BlockBuilder;

pub struct ProcedureBuilder<'out> {
    out: &'out mut Vec<u8>,
}

impl<'out> ProcedureBuilder<'out> {
    pub fn new(out: &'out mut Vec<u8>) -> Self {
        Self { out }
    }

    pub fn block(&mut self, mut f: impl FnMut(&mut BlockBuilder)) {
        let mut block = BlockBuilder::new(self.out);
        f(&mut block);
        block.emit_ret();
    }
}
