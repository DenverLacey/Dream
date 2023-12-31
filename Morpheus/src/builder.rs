use crate::errors::{Error, Result};
use crate::version::Version;

use std::io::Write as _;
pub struct Builder {
    version: Version,
    strings: Vec<Box<[u8]>>,
}

impl Builder {
    pub fn new(version: Version) -> Self {
        Self {
            version,
            strings: vec![],
        }
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

pub trait Write {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize>;

    fn write_str(&mut self, s: &str) -> Result<usize> {
        self.write_bytes(s.as_bytes())
    }

    fn write_chr(&mut self, c: char) -> Result<usize> {
        let mut buf = [0u8; 4];
        let c_str = c.encode_utf8(&mut buf);
        self.write_str(c_str)
    }

    fn pad(&mut self, nbytes: usize) -> Result<usize> {
        for _ in 0..nbytes {
            self.write_bytes(&[0])?;
        }
        Ok(nbytes)
    }
}

macro_rules! impl_io_write {
    ($t:ty) => {
        impl $crate::builder::Write for $t {
            fn write_bytes(&mut self, bytes: &[u8]) -> $crate::errors::Result<usize> {
                self.write_all(bytes)
                    .map_err(|_| $crate::errors::Error::WriteError)?;
                Ok(bytes.len())
            }
        }
    };
}

impl_io_write!(std::fs::File);
impl_io_write!(std::io::Cursor<&mut Vec<u8>>);
impl_io_write!(std::io::Stdout);
impl_io_write!(std::io::Stderr);

impl Write for String {
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize> {
        let s = std::str::from_utf8(bytes).map_err(|_| Error::WriteError)?;
        self.push_str(s);
        Ok(bytes.len())
    }
}

trait IteratorExt: Iterator {
    fn consume(&mut self) {
        loop {
            let n = self.next();
            if n.is_none() {
                break;
            }
        }
    }
}

impl<I: Iterator> IteratorExt for I {}

#[cfg(test)]
mod tests {
    use std::fs::File;

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
}
