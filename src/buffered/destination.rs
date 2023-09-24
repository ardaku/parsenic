use core::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::result::{FlushResult, FullResult};

/// Trait for implementing custom buffered writer destinations
pub trait Destination {
    /// Write bytes to the internal buffer.
    fn write(&mut self, bytes: &[u8]) -> FullResult;

    /// Return `Ready` if flush is either complete or failed.
    ///
    /// Should return `Pending` if not ready yet.  Should restart flush
    /// operation if called after returning `Ready`.
    ///
    /// Will be called if [`Self::write()`] fails.
    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<FlushResult>;
}

impl<T: Extend<u8>> Destination for T {
    fn write(&mut self, bytes: &[u8]) -> FullResult {
        self.extend(bytes.into_iter().cloned());

        Ok(())
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<FlushResult> {
        Poll::Ready(Ok(()))
    }
}
