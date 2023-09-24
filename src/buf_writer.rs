use core::iter::Extend;

use crate::{result::FlushResult, Buffer, Cursor, Save, UInt, Write};

/// [`Buffer`]ed persistent [`Write`]r
pub struct BufWriter<'a, S: Save, const SIZE: usize = 8192> {
    cursor: Cursor<'a, SIZE>,
    persistence: S,
    position: u64,
}

impl<'a, S: Save, const SIZE: usize> BufWriter<'a, S, SIZE> {
    /// Create a new [`Write`]r to the provided persistence and in-memory
    /// buffer.
    ///
    /// Default "seek" position is `0`.
    pub fn new(buffer: &'a mut Buffer<SIZE>, persistence: S) -> Self {
        Self {
            cursor: Cursor::new(buffer),
            persistence,
            position: 0,
        }
    }

    /// Seek to a new position in the file.
    ///
    /// If the position is past the end of the file, the file should be
    /// automatically expanded on write, and filled in with zeros.
    pub fn seek(&mut self, position: u64) {
        self.position = position;
    }

    /// Write the remaining in-memory buffer to persistent storage.
    pub fn flush(&mut self) -> FlushResult {
        let buffer = self.cursor.split_at().0;

        self.persistence.save(self.position, buffer)
    }
}

impl<const SIZE: usize, S: Save> Write for BufWriter<'_, S, SIZE> {
    fn bytes(&mut self, bytes: impl AsRef<[u8]>) -> FlushResult {
        let bytes = bytes.as_ref();
        let (left, right) = self.cursor.split_at_mut();

        while bytes.len() > right.len() {}

        Ok(())
    }
}
