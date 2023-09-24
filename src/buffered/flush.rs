use core::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::result::FlushResult;

/// [`Future`] returned from [`Write::flush()`](super::Write::flush)
#[allow(missing_debug_implementations)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Flush<'a>(
    pub(super) &'a mut (dyn Future<Output = crate::result::FlushResult> + Unpin),
);

impl Future for Flush<'_> {
    type Output = FlushResult;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<FlushResult> {
        let mut this = self;

        Pin::new(&mut this.0).poll(cx)
    }
}
