use crate::errors::{Error, Result};
use crate::version::Version;

use std::io::Write as _;

#[derive(Default)]
pub struct Builder {
    version: Version,
}

impl Builder {
    pub fn new(version: Version) -> Self {
        Self { version }
    }

    fn write_magic(&self, f: &mut dyn Write) -> Result<()> {
        let version = self.version.as_bytes();
        f.write_str("DREAM")?;
        f.write_bytes(&version)?;
        Ok(())
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
        let mut file = File::create("test_write_magic.txt").unwrap();
        let result = builder.write_magic(&mut file);
        assert!(result.is_ok());
    }
}
