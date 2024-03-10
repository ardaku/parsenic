/// Destination lost (from either corruption or disconnection)
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
#[non_exhaustive]
pub struct LostError(Option<&'static str>);

impl LostError {
    /// Create a new [`LostError`].
    pub fn new() -> Self {
        Self(None)
    }

    /// Create a new [`LostError`] from message payload.
    pub fn from_message(message: &'static str) -> Self {
        Self(Some(message))
    }

    /// Return the message payload.
    pub fn message(&self) -> Option<&'static str> {
        self.0
    }
}
