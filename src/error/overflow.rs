/// Overflow (variable can't contain parsed value)
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
#[non_exhaustive]
pub struct OverflowError();

impl OverflowError {
    /// Create a new [`OverflowError`].
    pub fn new() -> Self {
        Self()
    }
}
