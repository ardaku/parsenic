use core::num::NonZeroUsize;

/// Source ran over the end of the buffer
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
#[non_exhaustive]
pub struct LenError(Option<NonZeroUsize>);

impl LenError {
    /// Create a new [`LenError`].
    pub const fn new() -> Self {
        Self(None)
    }

    /// Create a new [`LenError`] from the number of remaining bytes.
    pub const fn from_remaining(remaining: usize) -> Self {
        Self(NonZeroUsize::new(remaining))
    }

    /// Return the number of remaining bytes.
    pub const fn remaining(&self) -> Option<NonZeroUsize> {
        self.0
    }
}
