use core::num::NonZeroUsize;

/// Destination has run out of space
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
#[non_exhaustive]
pub struct EndError(Option<NonZeroUsize>);

impl EndError {
    /// Create a new [`EndError`].
    pub const fn new() -> Self {
        Self(None)
    }

    /// Create a new [`EndError`] from the number of remaining bytes.
    pub const fn from_remaining(remaining: usize) -> Self {
        Self(NonZeroUsize::new(remaining))
    }

    /// Return the number of remaining bytes.
    pub const fn remaining(&self) -> Option<NonZeroUsize> {
        self.0
    }
}
