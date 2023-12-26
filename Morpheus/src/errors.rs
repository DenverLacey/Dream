#[derive(Debug)]
pub enum Error {
    SRXOutOfBounds,
    VersionOutOfBounds,
}

pub type Result<T> = std::result::Result<T, Error>;
