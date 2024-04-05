use std::str::FromStr;

use crate::errors::{Error, Result};

const VERSION_BASE: usize = 64;
pub const MAX_VERSION_NUMBER: u32 = (VERSION_BASE * VERSION_BASE * VERSION_BASE - 1) as u32;

const DIGITS64: [u8; VERSION_BASE] =
    *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+/";

const CHARS64: [u8; 127] = {
    let mut chars = [0xFFu8; 127];
    let mut i = 0;
    while i < DIGITS64.len() {
        let c = DIGITS64[i];
        chars[c as usize] = i as u8;
        i += 1;
    }
    chars
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(pub(crate) u32);

impl Version {
    pub fn from(number: u32) -> Self {
        if number <= MAX_VERSION_NUMBER {
            Self(number)
        } else {
            panic!("Version out of bounds! {number} is bigger than maximum {MAX_VERSION_NUMBER}.");
        }
    }

    pub const fn try_from(number: u32) -> Result<Self> {
        if number <= MAX_VERSION_NUMBER {
            Ok(Self(number))
        } else {
            Err(Error::VersionOutOfBounds)
        }
    }

    pub fn as_bytes(self) -> [u8; 3] {
        let mut result = [0u8; 3];

        let mut number = self.0 as usize;
        for i in 0..result.len() {
            let digit = number % VERSION_BASE;
            result[result.len() - i - 1] = DIGITS64[digit];
            number /= VERSION_BASE;
        }

        result
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl FromStr for Version {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.as_bytes();

        if s.len() != 3 {
            return Err(Error::VersionFromStrError);
        }

        let units = CHARS64[s[2] as usize] as usize;
        let tens  = CHARS64[s[1] as usize] as usize;
        let hnds  = CHARS64[s[0] as usize] as usize;

        if units == 0xFF || tens == 0xFF || hnds == 0xFF {
            return Err(Error::VersionFromStrError);
        }

        let version = hnds*VERSION_BASE*VERSION_BASE + tens*VERSION_BASE + units;
        if version as u32 > MAX_VERSION_NUMBER {
            return Err(Error::VersionFromStrError);
        }

        Ok(Version(version as u32))
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn encode_v000() {
        let version = Version::from(0);
        let bytes = version.as_bytes();
        assert_eq!(&bytes, b"000");
    }

    #[test]
    fn encode_v001() {
        let version = Version::from(1);
        let bytes = version.as_bytes();
        assert_eq!(&bytes, b"001");
    }

    #[test]
    fn encode_v00A() {
        let version = Version::from(10);
        let bytes = version.as_bytes();
        assert_eq!(&bytes, b"00A");
    }

    #[test]
    fn encode_max() {
        let version = Version::from(MAX_VERSION_NUMBER);
        let bytes = version.as_bytes();
        assert_eq!(&bytes, b"///");
    }
}
