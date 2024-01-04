use super::{proc_builder::ProcedureBuilder, Write};
use crate::errors::Result;
use crate::version::Version;

pub struct Builder {
    version: Version,
    entry_point: usize,
    strings: Vec<Box<[u8]>>,
    code: Vec<u8>,
}

impl Builder {
    pub fn new(version: Version) -> Self {
        Self {
            version,
            entry_point: 0,
            strings: vec![],
            code: vec![],
        }
    }

    pub fn set_entry(&mut self, entry: usize) {
        self.entry_point = entry;
    }

    pub fn add_string(&mut self, new: impl AsRef<[u8]>) -> usize {
        let mut offset = 0;
        for s in self.strings.iter() {
            if s.as_ref() == new.as_ref() {
                return offset;
            }
            offset += std::mem::size_of::<usize>() + s.len() + Self::PADDING;
        }
        self.strings.push(Box::from(new.as_ref()));
        offset
    }

    pub fn procedure(&mut self, mut f: impl FnMut(&mut ProcedureBuilder)) -> usize {
        let proc_begin = self.code.len();

        let mut proc = ProcedureBuilder::new(&mut self.code);
        f(&mut proc);

        proc_begin
    }
}

impl Builder {
    const PADDING: usize = 8;

    fn write_magic(&self, f: &mut dyn Write) -> Result<()> {
        f.write_str("DREAM")?;
        f.write_bytes(&self.version.as_bytes())?;
        Ok(())
    }

    fn write_text_section(&self, f: &mut dyn Write) -> Result<usize> {
        let mut text_size = 0;

        let strings_size: usize = self
            .strings
            .iter()
            .map(|s| std::mem::size_of::<usize>() + s.len() + Self::PADDING)
            .sum();

        text_size += f.write_str("TEXT")?;
        text_size += f.write_bytes(&strings_size.to_le_bytes())?;
        text_size += f.pad(4)?;

        for s in self.strings.iter() {
            text_size += f.write_bytes(&s.len().to_le_bytes())?;
            text_size += f.write_bytes(s.as_ref())?;
            text_size += f.write_bytes(&[0; Self::PADDING])?;
        }

        Ok(text_size)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use quicksand::RegisterType;

    use crate::Operand;

    use super::*;

    #[test]
    fn write_magic() {
        let builder = Builder::new(Version::from(0));
        let mut output = String::new();
        let result = builder.write_magic(&mut output);
        assert!(result.is_ok());
        assert_eq!(output.as_str(), "DREAM000");
    }

    #[test]
    fn write_magic_to_file() {
        let builder = Builder::new(Version::from(87));
        let mut file = File::create("tests/test_write_magic.bin").unwrap();
        let result = builder.write_magic(&mut file);
        assert!(result.is_ok());
    }

    #[test]
    fn write_text_section() {
        let mut builder = Builder::new(Version::from(0));
        let mut output = File::create("tests/test_write_text_section.bin").unwrap();

        builder.add_string("hello");
        builder.add_string("world!");
        builder.add_string("");

        let result = builder.write_text_section(&mut output);
        assert!(result.is_ok());
    }

    #[test]
    fn write_procedure() {
        let mut builder = Builder::new(Version::from(0));
        let mut output = File::create("tests/test_write_procedure.bin").unwrap();

        builder.procedure(|proc| {
            proc.block(|block| {
                block
                    .emit_move(
                        Operand::gpr(RegisterType::Q, 0).unwrap(),
                        Operand::lit64(69),
                        None,
                    )
                    .unwrap();
            })
        });

        output.write_bytes(&builder.code).unwrap();
    }
}
