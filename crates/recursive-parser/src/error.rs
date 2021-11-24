use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostcssError {
    #[error("{0}")]
    /// (error_message, start_offset, end_offset)
    ParseError(String, usize, usize),
    #[error("unknown postcss error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, PostcssError>;