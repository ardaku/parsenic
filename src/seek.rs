/// Trait providing a cursor which can be moved within a fixed-size buffer of
/// bytes.
///
/// Unlike [`std::io::Seek`], the methods are infallible, as they should not do
/// any I/O.
///
/// If a file is being concurrently accessed by another process, don't expect
/// things to work at all.  Files should have either a locking or ownership
/// mechanism to prevent concurrent access from other processes.
///
/// [`std::io::Seek`]: https://doc.rust-lang.org/stable/std/io/trait.Seek.html
pub trait Seek {
    /// Seek to an offset, in bytes, in a buffer.
    ///
    /// A seek beyond the end of a buffer should fill in skipped bytes with
    /// zeroes upon the next write, or cause a [`LenError`] on the next read.
    ///
    /// [`LenError`]: crate::error::LenError
    fn seek(&mut self, pos: u64);

    /// Return the size of the buffer.
    ///
    /// This value should change after either writing past the end of the buffer
    /// or truncating the buffer.
    fn size(&self, pos: u64) -> u64;

    /// Return the current seek position from the start of the buffer.
    ///
    /// This value should change on each read or write larger than zero, as well
    /// as each call to [`Self::seek()`].
    fn position(&self) -> u64;
}
