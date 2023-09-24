/// A writer which will move data into the void
///
/// Returned by [`drain()`].
#[non_exhaustive]
#[derive(Copy, Clone, Default, Debug)]
pub struct Drain;

/// Create an instance of a writer which will consume all bytes.
///
/// The returned type implements [`Write`](crate::Write) as an extension trait
/// on [`Extend<u8>`].
///
/// This API takes some inspiration from [`std::io::sink()`].
///
/// The naming comes from [`futures::sink::drain()`].
///
/// [`std::io::sink()`]: https://doc.rust-lang.org/stable/std/io/fn.sink.html
/// [`futures::sink::drain()`]: https://docs.rs/futures/0.3/futures/sink/fn.drain.html
#[must_use]
pub fn drain() -> Drain {
    Drain
}

impl Extend<u8> for Drain {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = u8>,
    {
        iter.into_iter().for_each(drop)
    }
}
