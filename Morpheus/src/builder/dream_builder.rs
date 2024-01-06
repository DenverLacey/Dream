use super::{proc_builder::ProcedureBuilder, Write};
use crate::{errors::Result, version::Version, OutputType};

pub struct Builder {
    version: Version,
    output_type: OutputType,
    entry_point: usize,
    strings: Vec<Box<[u8]>>,
    code: Vec<u8>,
}

impl Builder {
    pub fn new(version: Version, output: OutputType) -> Self {
        Self {
            version,
            output_type: output,
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

    pub fn procedure(&mut self, f: impl FnOnce(&mut ProcedureBuilder)) -> usize {
        let proc_begin = self.code.len();

        let mut proc = ProcedureBuilder::new(&mut self.code);
        f(&mut proc);

        proc_begin
    }
}

impl Builder {
    const PADDING: usize = 8;

    fn write_header(&self, f: &mut dyn Write) -> Result<()> {
        f.write_str("DREAM")?;
        f.write_bytes(&self.version.as_bytes())?;
        f.write_str("OUTT")?;
        f.write_bytes(&self.output_type.as_bytes())?;
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
    use std::fs::File;

    use quicksand::{Register, RegisterType};

    use crate::Operand;

    use super::*;

    #[test]
    fn write_header_bin() {
        let builder = Builder::new(Version::from(0), OutputType::Bin);
        let mut output = String::new();
        let result = builder.write_header(&mut output);
        assert!(result.is_ok());
        assert_eq!(output.as_str(), "DREAM000OUTT\x00\x00\x00\x00");
    }

    #[test]
    fn write_header_lib() {
        let builder = Builder::new(Version::from(0), OutputType::Lib);
        let mut output = String::new();
        let result = builder.write_header(&mut output);
        assert!(result.is_ok());
        assert_eq!(output.as_str(), "DREAM000OUTT\x01\x00\x00\x00");
    }

    #[test]
    fn write_header_to_file() {
        let builder = Builder::new(Version::from(87), OutputType::Bin);
        let mut file = File::create("tests/test_write_header.bin").unwrap();
        let result = builder.write_header(&mut file);
        assert!(result.is_ok());
    }

    #[test]
    fn write_text_section() {
        let mut builder = Builder::new(Version::from(0), OutputType::Bin);
        let mut output = File::create("tests/test_write_text_section.bin").unwrap();

        builder.add_string("hello");
        builder.add_string("world!");
        builder.add_string("");

        let result = builder.write_text_section(&mut output);
        assert!(result.is_ok());
    }

    #[test]
    fn write_procedure() {
        let mut builder = Builder::new(Version::from(0), OutputType::Bin);
        let mut output = File::create("tests/test_write_procedure.bin").unwrap();

        builder.procedure(|proc| {
            proc.body(|block| {
                block
                    .emit_move(
                        Operand::reg(Register::new(RegisterType::Q, 0).unwrap()),
                        Operand::lit64(69),
                        None,
                    )
                    .unwrap();

                block
                    .emit_move(Operand::reg(Register::RSI), Operand::lit64(1), None)
                    .unwrap();
            })
        });

        output.write_bytes(&builder.code).unwrap();
    }

    #[test]
    fn write_hello_world_procedure() {
        let mut builder = Builder::new(Version::from(0), OutputType::Bin);
        let mut output = File::create("tests/test_write_hello_world_procedure.bin").unwrap();

        builder.procedure(|proc| {
            proc.body(|block| {
                block
                    .emit_move(Operand::reg(Register::RSI), Operand::lit64(1), None)
                    .unwrap();
                block
                    .emit_move(
                        Operand::reg(Register::new(RegisterType::S, 0).unwrap()),
                        Operand::lit64(2),
                        None,
                    )
                    .unwrap();
                block
                    .emit_map(Register::new(RegisterType::S, 1).unwrap(), 0)
                    .unwrap();
                block
                    .emit_move(
                        Operand::reg(Register::new(RegisterType::S, 2).unwrap()),
                        Operand::lit64(11),
                        None,
                    )
                    .unwrap();
            })
        });

        output.write_bytes(&builder.code).unwrap();
    }
}
