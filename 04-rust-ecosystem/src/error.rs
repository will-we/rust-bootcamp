use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("An IO error occurred: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Unknown error occurred")]
    Unknown,
}
