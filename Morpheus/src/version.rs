use crate::errors::{Error, Result};

const VERSION_BASE: usize = 64;
pub const MAX_VERSION_NUMBER: u32 = (VERSION_BASE * VERSION_BASE * VERSION_BASE - 1) as u32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(pub(crate) u32);

impl Version {
    pub const fn new(number: u32) -> Result<Self> {
        if number <= MAX_VERSION_NUMBER {
            Ok(Self(number))
        } else {
            Err(Error::VersionOutOfBounds)
        }
    }

    pub fn as_bytes(self) -> [u8; 3] {
        const DIGITS64: [u8; VERSION_BASE] =
            *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+/";
        let mut result = [0u8; 3];

        let mut number = self.0 as usize;
        for i in 0..result.len() {
            let digit = number % VERSION_BASE;
            result[result.len() - i - 1] = DIGITS64[digit];
            number /= VERSION_BASE;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn encode_v000() {
        let version = Version::new(0).unwrap();
        let bytes = version.as_bytes();
        assert_eq!(&bytes, b"000");
    }

    #[test]
    fn encode_v001() {
        let version = Version::new(1).unwrap();
        let bytes = version.as_bytes();
        assert_eq!(&bytes, b"001");
    }

    #[test]
    fn encode_v00A() {
        let version = Version::new(10).unwrap();
        let bytes = version.as_bytes();
        assert_eq!(&bytes, b"00A");
    }

    #[test]
    fn encode_max() {
        let version = Version::new(MAX_VERSION_NUMBER).unwrap();
        let bytes = version.as_bytes();
        assert_eq!(&bytes, b"///");
    }
}
