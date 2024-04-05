pub mod disassembler;

use crate::{Result, Write};

use disassembler::Disassembler;

pub fn disassemble(dream: impl IntoIterator<Item = u8>, f: &mut dyn Write) -> Result<()> {
    let mut dismblr = Disassembler::new(dream, f);
    dismblr.disassemble()
}

