#[derive(Debug)]
pub enum Error {
    InvalidRegisterIndex,
    VersionOutOfBounds,
    WriteError,
    BadOperandType,
    BadOperandValue,
}

pub type Result<T> = std::result::Result<T, Error>;
