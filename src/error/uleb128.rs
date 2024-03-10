use crate::error::{LenError, OverflowError};

/// ULEB128 parsing error
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Uleb128Error {
    /// Ran over the end of the buffer
    Len(LenError),
    /// Overflow (variable can't contain parsed value)
    Overflow(OverflowError),
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
