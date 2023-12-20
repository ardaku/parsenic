use core::{
    pin::Pin,
    task::{Context, Poll},
};

use crate::result::LostResult;

/// [`Sender`] asynchronous destination
///
/// [`Sender`]: crate::io::Sender
pub trait Destination {
    /// Attempt to send `buf` bytes.
    ///
    /// Returns the number of bytes sent when ready, or zero when destination is
    /// full.
    fn poll_send(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<LostResult<usize>>;
}
