use crate::error::{LenError, Utf8Error};

/// String parsing error
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StrError {
    /// Ran over the end of the buffer
    Len(LenError),
    /// Invalid UTF-8
    Utf8(Utf8Error),
}

impl From<LenError> for StrError {
    fn from(error: LenError) -> Self {
        Self::Len(error)
    }
}

impl From<Utf8Error> for StrError {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8(error)
    }
}
