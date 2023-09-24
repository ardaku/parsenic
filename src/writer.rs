use core::iter::Extend;

use crate::{result::FlushResult, Write};

/// [`Extend`] [`Write`]r
#[derive(Debug)]
pub struct Writer<'a, T: Extend<u8>>(&'a mut T);

impl<'a, T: Extend<u8>> Writer<'a, T> {
    /// Create a new `Writer` to the provided `sink`.
    pub fn new(sink: &'a mut T) -> Self {
        Self(sink)
    }
}

impl<T: Extend<u8>> Write for Writer<'_, T> {
    fn bytes(&mut self, bytes: impl AsRef<[u8]>) -> FlushResult {
        self.0.extend(bytes.as_ref().iter().cloned());

        Ok(())
    }
}
