use core::num::NonZeroUsize;

/// Destination has run out of space
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
#[non_exhaustive]
pub struct FullError(Option<NonZeroUsize>);

impl FullError {
    /// Create a new [`FullError`].
    pub const fn new() -> Self {
        Self(None)
    }

    /// Create a new [`FullError`] from the number of remaining bytes.
    pub const fn from_remaining(remaining: usize) -> Self {
        Self(NonZeroUsize::new(remaining))
    }

    /// Return the number of remaining bytes.
    pub const fn remaining(&self) -> Option<NonZeroUsize> {
        self.0
    }
}
