/// Overflow (variable can't contain parsed value)
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
#[non_exhaustive]
pub struct OverflowError(Option<&'static str>);

impl OverflowError {
    /// Create a new [`OverflowError`].
    pub fn new() -> Self {
        Self(None)
    }

    /// Create a new [`OverflowError`] from message payload.
    pub fn from_message(message: &'static str) -> Self {
        Self(Some(message))
    }

    /// Return the message payload.
    pub fn message(&self) -> Option<&'static str> {
        self.0
    }
}
