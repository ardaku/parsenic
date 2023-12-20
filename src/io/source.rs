use core::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::result::LostResult;

/// [`Receiver`] asynchronous source
///
/// [`Receiver`]: crate::io::Receiver
pub trait Source {
    /// Attempt to receive bytes into `buf`.
    ///
    /// Returns the number of bytes received when ready, or zero when no more
    /// data is available.
    fn poll_recv(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<LostResult<usize>>;
}
