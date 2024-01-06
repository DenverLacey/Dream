#[derive(Debug)]
pub enum Error {
    InvalidArgument,
    InvalidRegisterIndex,
}

pub type Result<T> = std::result::Result<T, Error>;
