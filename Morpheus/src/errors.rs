#[derive(Debug)]
pub enum Error {
    InvalidRegisterIndex,
    VersionOutOfBounds,
    WriteError,
    BadOperandType,
    BadOperandValue,
    TooManyArgsForSyscall,
}

pub type Result<T> = std::result::Result<T, Error>;
