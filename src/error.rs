//! Error types

/// Ran over the end of the buffer
#[derive(Copy, Clone, Debug)]
pub struct LenError;

/// Expected buffer to end, but it didn't
#[derive(Copy, Clone, Debug)]
pub struct EndError;

/// Invalid UTF8
#[derive(Copy, Clone, Debug)]
pub struct Utf8Error;

/// Parsing error
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Ran over the end of the buffer
    Len(LenError),
    /// Expected buffer to end, but it didn't
    End(EndError),
    /// Invalid UTF8
    Utf8(Utf8Error),
}

/// String parsing error
#[derive(Copy, Clone, Debug)]
pub enum StrError {
    /// Ran over the end of the buffer
    Len(LenError),
    /// Invalid UTF8
    Utf8(Utf8Error),
}

impl From<StrError> for Error {
    fn from(error: StrError) -> Self {
        match error {
            StrError::Len(error) => Self::Len(error),
            StrError::Utf8(error) => Self::Utf8(error),
        }
    }
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
