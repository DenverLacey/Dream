mod block_builder;
mod dream_builder;
mod proc_builder;

pub use block_builder::*;
pub use dream_builder::*;
pub use proc_builder::*;

use crate::errors::{Error, Result};

use quicksand::{OperandType, Register};
use std::io::Write as _;

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

pub struct Operand {
    kind: OperandType,
    value: u64,
}

impl Operand {
    pub const fn reg(reg: Register) -> Self {
        Self {
            kind: OperandType::Register,
            value: reg.to_u64(),
        }
    }

    pub const fn addr(addr: u64) -> Self {
        Self {
            kind: OperandType::Address,
            value: addr,
        }
    }

    pub const fn lit64(lit: u64) -> Self {
        Self {
            kind: OperandType::Lit64,
            value: lit,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputType {
    Bin = 0,
    Lib = 1,
}

impl OutputType {
    pub fn as_bytes(self) -> [u8; 4] {
        (self as u32).to_le_bytes()
    }
}

impl TryFrom<u32> for OutputType {
    type Error = Error;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        let ot = match value {
            0 => Self::Bin,
            1 => Self::Lib,
            _ => return Err(Error::InvalidOutputType),
        };
        Ok(ot)
    }
}

