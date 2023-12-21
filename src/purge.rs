use crate::{result::FullResult, Write};

/// Writer that moves data into the void
///
/// Returned by [`purge()`].
#[non_exhaustive]
#[derive(Copy, Clone, Default, Debug)]
pub struct Purge;

/// Create an instance of a writer which will successfully consume all bytes.
///
/// The returned type implements [`Write`](crate::Write).
///
/// This API takes some inspiration from [`std::io::sink()`].
///
/// [`std::io::sink()`]: https://doc.rust-lang.org/stable/std/io/fn.sink.html
#[must_use]
pub fn purge() -> Purge {
    Purge
}

impl Write for Purge {
    fn bytes(&mut self, bytes: impl AsRef<[u8]>) -> FullResult {
        drop(bytes);

        Ok(())
    }
}
