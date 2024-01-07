#[derive(Debug)]
pub enum Error {
    InvalidRegister,
    VersionOutOfBounds,
    WriteError,
    BadOperandType,
    BadOperandValue,
    TooManyArgsForSyscall,
    InvalidInstruction,
    NotEnoughOperandsForInstruction,
    InvalidAddr,
    InvalidLit64,
}

pub type Result<T> = std::result::Result<T, Error>;
