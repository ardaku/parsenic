use crate::error::{
    EndError, FlushError, FullError, LenError, LostError, OverflowError,
    StrError, Uleb128Error, Utf8Error,
};

/// Parsing error
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Ran over the end of the buffer
    Len(LenError),
    /// Expected buffer to end, but it didn't
    End(EndError),
    /// Invalid UTF8
    Utf8(Utf8Error),
    /// Overflow (variable can't contain parsed value)
    Overflow(OverflowError),
    /// Destination has run out of space
    Full(FullError),
    /// Destination lost (from either corruption or disconnection)
    Lost(LostError),
}

impl From<LenError> for Error {
    fn from(error: LenError) -> Self {
        Self::Len(error)
    }
}

impl From<EndError> for Error {
    fn from(error: EndError) -> Self {
        Self::End(error)
    }
}

impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Self {
        Self::Utf8(error)
    }
}

impl From<OverflowError> for Error {
    fn from(error: OverflowError) -> Self {
        Self::Overflow(error)
    }
}

impl From<FullError> for Error {
    fn from(error: FullError) -> Self {
        Self::Full(error)
    }
}

impl From<LostError> for Error {
    fn from(error: LostError) -> Self {
        Self::Lost(error)
    }
}

impl From<FlushError> for Error {
    fn from(error: FlushError) -> Self {
        match error {
            FlushError::Full(error) => Self::Full(error),
            FlushError::Lost(error) => Self::Lost(error),
        }
    }
}

impl From<Uleb128Error> for Error {
    fn from(error: Uleb128Error) -> Self {
        match error {
            Uleb128Error::Len(error) => Self::Len(error),
            Uleb128Error::Overflow(error) => Self::Overflow(error),
        }
    }
}

impl From<StrError> for Error {
    fn from(error: StrError) -> Self {
        match error {
            StrError::Len(error) => Self::Len(error),
            StrError::Utf8(error) => Self::Utf8(error),
        }
    }
}
