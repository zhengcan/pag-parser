use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("")]
    UnsupportPagVersion(u8),
    #[error("")]
    Incomplete(nom::Needed),
    #[error("")]
    BadFrame(nom::error::ErrorKind),
    #[error("")]
    IoError(#[from] std::io::Error),
    #[error("")]
    Eof,
}

impl<'a> From<nom::Err<nom::error::Error<&'a [u8]>>> for ParseError {
    fn from(error: nom::Err<nom::error::Error<&'a [u8]>>) -> Self {
        match error {
            nom::Err::Incomplete(needed) => ParseError::Incomplete(needed),
            nom::Err::Error(e) => ParseError::BadFrame(e.code),
            nom::Err::Failure(e) => ParseError::BadFrame(e.code),
        }
    }
}
