use crate::error::{FullError, LostError};

/// Flush error
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FlushError {
    /// Destination has run out of space
    Full(FullError),
    /// Destination lost (from either corruption or disconnection)
    Lost(LostError),
}

impl From<FullError> for FlushError {
    fn from(error: FullError) -> Self {
        Self::Full(error)
    }
}

impl From<LostError> for FlushError {
    fn from(error: LostError) -> Self {
        Self::Lost(error)
    }
}
