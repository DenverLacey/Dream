#[derive(Debug)]
pub enum Error {
    InvalidRegisterIndex,
}

pub type Result<T> = std::result::Result<T, Error>;
