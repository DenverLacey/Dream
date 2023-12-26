use std::fmt::Write;

use crate::version::Version;

#[derive(Default)]
pub struct Builder {
    version: Version,
}

impl Builder {
    pub fn new(version: Version) -> Self {
        Self { version }
    }

    fn write_magic(&self, out: &mut dyn Write) -> std::fmt::Result {
        let version = self.version.as_bytes();
        out.write_str("DREAM")?;
        out.write_char(version[0] as char)?;
        out.write_char(version[1] as char)?;
        out.write_char(version[2] as char)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn write_magic() {
        let builder = Builder::new(Version::new(0).unwrap());

        let mut output = String::new();
        let result = builder.write_magic(&mut output);
        assert!(result.is_ok());
        assert_eq!(output.as_str(), "DREAM000");
    }
}
