use core::{future, pin::Pin};

use crate::{error::LenError, io::Source, result::ReceiveResult, Reader};

/// [`Extend`] buffered receiver.
///
/// Receives data from the operating system.
///
/// The `BUF` const generic is used to specify the size of the internal buffer,
/// defaulting to 8kB.
#[derive(Copy, Clone, Debug)]
pub struct Receiver<S, T, const BUF: usize = 8192> {
    source: S,
    destination: T,
    cursor: usize,
    buffer: [u8; BUF],
}

impl<S, T, const BUF: usize> Receiver<S, T, BUF>
where
    S: Source + Unpin,
    T: Extend<u8> + AsRef<[u8]>,
{
    /// Create a new receiver (asynchronous source, synchronous destination).
    pub fn new(source: S, destination: T) -> Self {
        Self {
            source,
            destination,
            cursor: BUF,
            buffer: [0; BUF],
        }
    }

    /// Receive exactly `count` bytes from the operating system to be read.
    pub async fn recv(&mut self, count: usize) -> ReceiveResult<Reader<'_>> {
        let mut count = count;

        // While not all bytes are written
        while count != 0 {
            // If cursor has reached the buffer length, overwrite
            if BUF == self.cursor {
                let mut written = 0;

                while written < BUF {
                    // FIXME: If error is end of file, allow failure for partial
                    // read
                    written += future::poll_fn(|cx| {
                        Pin::new(&mut self.source).poll_recv(
                            cx,
                            self.buffer.get_mut(written..).unwrap_or(&mut []),
                        )
                    })
                    .await?;
                }

                self.cursor = 0;
            }

            // Extend destination with buffer contents
            let leftovers = self.buffer.get(self.cursor..).ok_or(LenError)?;
            let leftovers = leftovers.get(..count).unwrap_or(leftovers);

            count = count.saturating_sub(leftovers.len());
            self.cursor = self.cursor.saturating_add(leftovers.len());
            self.destination.extend(leftovers.iter().cloned());
        }

        Ok(Reader::new(self.destination.as_ref()))
    }
}
