/// Writer that moves data into the void
///
/// Returned by [`purge()`].
#[non_exhaustive]
#[derive(Copy, Clone, Default, Debug)]
pub struct Purge;

/// Create an instance of a writer which will consume all bytes.
///
/// The returned type implements [`Write`](crate::Write) as an extension trait
/// on [`Extend<u8>`].
///
/// This API takes some inspiration from [`std::io::sink()`].
///
/// [`std::io::sink()`]: https://doc.rust-lang.org/stable/std/io/fn.sink.html
#[must_use]
pub fn purge() -> Purge {
    Purge
}

impl Extend<u8> for Purge {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = u8>,
    {
        iter.into_iter().for_each(drop)
    }
}
