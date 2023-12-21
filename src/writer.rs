use crate::{result::FullResult, Write};

/// [`Extend`] [`Write`]r
#[derive(Debug)]
pub struct Writer<'a, T>(&'a mut T);

impl<'a, T> Writer<'a, T>
where
    T: Extend<u8>,
{
    /// Create a new `Writer` into the provided growable `buffer`.
    pub fn new(buffer: &'a mut T) -> Self {
        Self(buffer)
    }
}

impl<T> Write for Writer<'_, T>
where
    T: Extend<u8>,
{
    fn bytes(&mut self, bytes: impl AsRef<[u8]>) -> FullResult {
        self.0.extend(bytes.as_ref().iter().cloned());

        Ok(())
    }
}
