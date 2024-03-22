use crate::{
    io::{Destination, Seek, Truncate},
    result::FlushResult,
};

/// [`slice`] buffered sender.
///
/// Sends data to a [`Destination`].
///
/// The `BUF` const generic is used to specify the size of the internal buffer,
/// defaulting to 8kB.
#[derive(Debug)]
pub struct Sender<T, const BUF: usize = 8192> {
    #[allow(dead_code)]
    destination: T,
    #[allow(dead_code)]
    cursor: usize,
    #[allow(dead_code)]
    buffer: [u8; BUF],
}

impl<T, const BUF: usize> Sender<T, BUF> {
    /// Create a new receiver (synchronous source, asynchronous destination).
    pub fn new(destination: T) -> Self {
        Self {
            destination,
            cursor: 0,
            buffer: [0; BUF],
        }
    }
}

impl<T, const BUF: usize> Sender<T, BUF>
where
    T: Destination + Unpin,
{
    /// Send `bytes` to the destination.
    ///
    /// May not send the full amount of bytes until either the buffer is full or
    /// [`flush()`](Self::flush()) is called.
    pub async fn send(&mut self, bytes: &[u8]) -> FlushResult {
        todo!("{bytes:?}") // FIXME
    }

    /// Send buffered data with the destination.
    pub async fn flush(&mut self) -> FlushResult {
        todo!() // FIXME
    }
}

impl<S, const BUF: usize> Seek for Sender<S, BUF>
where
    S: Seek,
{
    fn seek(&mut self, pos: u64) {
        self.destination.seek(pos);
    }

    fn position(&self) -> u64 {
        self.destination.position()
    }

    fn len(&self) -> u64 {
        self.destination.len()
    }
}

impl<T, const BUF: usize> Truncate for Sender<T, BUF>
where
    T: Truncate,
{
    fn truncate(&mut self) {
        self.destination.truncate()
    }
}
