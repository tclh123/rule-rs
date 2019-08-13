use std::result;

#[derive(Debug)]
pub enum Error {
    JsonError(serde_json::Error),
    ExprBuildError,
    MatchError,
}

pub type Result<T> = result::Result<T, Error>;

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JsonError(err)
    }
}
