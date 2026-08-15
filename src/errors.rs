use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("{0}: command not found")]
    CommandNotFound(String),
}
