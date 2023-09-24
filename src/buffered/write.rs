use core::future::Future;

use traitful::extend;

use super::{Destination, Flush};

/// Buffered writer extension trait
#[extend]
pub trait Write: Future<Output = crate::result::FlushResult> + Unpin {
    /// Ensure all intermediately buffered contents reach their destination.
    fn flush(&mut self) -> Flush<'_> {
        Flush(self)
    }
}
