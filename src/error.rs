//! Error types

/// Ran over the end of the buffer
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LenError;

/// Expected buffer to end, but it didn't
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct EndError;

/// Invalid UTF8
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Utf8Error;

/// Overflow (variable can't contain parsed value)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OverflowError;

/// Parsing error
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
}

/// ULEB128 parsing error
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Uleb128Error {
    /// Ran over the end of the buffer
    Len(LenError),
    /// Overflow (variable can't contain parsed value)
    Overflow(OverflowError),
}

impl From<Uleb128Error> for Error {
    fn from(error: Uleb128Error) -> Self {
        match error {
            Uleb128Error::Len(error) => Self::Len(error),
            Uleb128Error::Overflow(error) => Self::Overflow(error),
        }
    }
}

/// String parsing error
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl From<LenError> for Uleb128Error {
    fn from(error: LenError) -> Self {
        Self::Len(error)
    }
}

impl From<OverflowError> for Uleb128Error {
    fn from(error: OverflowError) -> Self {
        Self::Overflow(error)
    }
}
