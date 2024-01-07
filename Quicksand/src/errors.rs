#[derive(Debug)]
pub enum Error {
    InvalidArgument,
    InvalidRegister,
    InvalidInstruction,
}

pub type Result<T> = std::result::Result<T, Error>;
