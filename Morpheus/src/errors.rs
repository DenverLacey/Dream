#[derive(Debug)]
pub enum Error {
    SRXOutOfBounds,
    VersionOutOfBounds,
    WriteError,
}

pub type Result<T> = std::result::Result<T, Error>;
