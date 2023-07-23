//! Error types

/// Ran over the end of the buffer
#[derive(Copy, Clone, Debug)]
pub struct Len;

/// Expected buffer to end, but it didn't
#[derive(Copy, Clone, Debug)]
pub struct End;

/// Invalid UTF8
#[derive(Copy, Clone, Debug)]
pub struct Utf8;

/// Parsing error
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Ran over the end of the buffer
    Len(Len),
    /// Expected buffer to end, but it didn't
    End(End),
    /// Invalid UTF8
    Utf8(Utf8),
}

/// String parsing error
#[derive(Copy, Clone, Debug)]
pub enum Str {
    /// Ran over the end of the buffer
    Len(Len),
    /// Invalid UTF8
    Utf8(Utf8),
}

impl From<Str> for Error {
    fn from(error: Str) -> Self {
        match error {
            Str::Len(error) => Error::Len(error),
            Str::Utf8(error) => Error::Utf8(error),
        }
    }
}

impl From<Len> for Error {
    fn from(error: Len) -> Self {
        Self::Len(error)
    }
}

impl From<End> for Error {
    fn from(error: End) -> Self {
        Self::End(error)
    }
}

impl From<Utf8> for Error {
    fn from(error: Utf8) -> Self {
        Self::Utf8(error)
    }
}

impl From<Len> for Str {
    fn from(error: Len) -> Self {
        Self::Len(error)
    }
}

impl From<Utf8> for Str {
    fn from(error: Utf8) -> Self {
        Self::Utf8(error)
    }
}
