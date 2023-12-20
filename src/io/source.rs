use core::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::result::ReceiveResult;

/// [`Receiver`] asynchronous source
///
/// [`Receiver`]: crate::io::Receiver
pub trait Source {
    /// Attempt to receive bytes into `buf`.
    ///
    /// Returns the number of bytes received when ready.
    fn poll_recv(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<ReceiveResult<usize>>; // FIXME: LenError is unnecessary here
}
