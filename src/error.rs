use std::result;

pub struct Error {
}

pub type Result<T> = result::Result<T, Error>;
