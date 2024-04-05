#[derive(Debug)]
pub enum Error {
    InvalidRegister,
    VersionOutOfBounds,
    VersionFromStrError,
    WriteError,
    BadOperandType,
    BadOperandValue,
    TooManyArgsForSyscall,
    InvalidInstruction,
    NotEnoughOperandsForInstruction,
    InvalidAddr,
    InvalidLit64,
    DisassembleFailure,
    InvalidOutputType,
}

pub type Result<T> = std::result::Result<T, Error>;
