#[derive(Debug)]
pub enum Error {
    InvalidRegisterIndex,
    VersionOutOfBounds,
    WriteError,
}

pub type Result<T> = std::result::Result<T, Error>;
