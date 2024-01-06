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
        offset + std::mem::size_of::<u64>()
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

    pub fn write_dream(&self, f: &mut dyn Write) -> Result<()> {
        self.write_header(f)?;
        self.write_text_section(f)?;
        self.write_code_section(f)?;
        Ok(())
    }

    fn write_header(&self, f: &mut dyn Write) -> Result<()> {
        f.write_str("DREAM")?;
        f.write_bytes(&self.version.as_bytes())?;
        f.write_str("OUTT")?;
        f.write_bytes(&self.output_type.as_bytes())?;
        Ok(())
    }

    fn write_text_section(&self, f: &mut dyn Write) -> Result<usize> {
        let mut section_size = 0;

        let strings_size: u64 = self
            .strings
            .iter()
            .map(|s| std::mem::size_of::<u64>() + s.len() + Self::PADDING)
            .sum::<usize>() as u64;

        section_size += f.write_str("TEXT")?;
        section_size += f.pad(4)?;
        section_size += f.write_bytes(&strings_size.to_le_bytes())?;

        for s in self.strings.iter() {
            section_size += f.write_bytes(&s.len().to_le_bytes())?;
            section_size += f.write_bytes(s.as_ref())?;
            section_size += f.write_bytes(&[0; Self::PADDING])?;
        }

        Ok(section_size)
    }

    fn write_code_section(&self, f: &mut dyn Write) -> Result<usize> {
        let mut section_size = 0;

        section_size += f.write_str("CODE")?;
        section_size += f.pad(4)?;
        section_size += f.write_bytes(&self.code.len().to_le_bytes())?;
        section_size += f.write_bytes(&self.entry_point.to_le_bytes())?;

        section_size += f.write_bytes(&self.code)?;

        Ok(section_size)
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

    #[test]
    pub fn write_dream() {
        let mut builder = Builder::new(Version::from(0), OutputType::Bin);
        let mut output = File::create("tests/write_dream.bin").unwrap();

        let str_idx = builder.add_string("Hello world!\n");

        let proc_idx = builder.procedure(|proc| {
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
                    .emit_map(Register::new(RegisterType::S, 1).unwrap(), str_idx as u64)
                    .unwrap();
                block
                    .emit_move(
                        Operand::reg(Register::new(RegisterType::S, 2).unwrap()),
                        Operand::lit64(11),
                        None,
                    )
                    .unwrap();
                block.emit_syscall(3).unwrap();
            })
        });

        builder.set_entry(proc_idx);

        let result = builder.write_dream(&mut output);
        assert!(result.is_ok());
    }
}
