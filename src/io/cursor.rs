use crate::buf::Buffer;

/// A wrapper around an in-memory [`Buffer`], adding a cursor position
#[derive(Debug)]
pub struct Cursor<'a, const SIZE: usize = 8192> {
    buffer: &'a mut Buffer<SIZE>,
    pos: usize,
}

impl<'a, const SIZE: usize> Cursor<'a, SIZE> {
    /// Create a new [`Cursor`] wrapping the provided in-memory [`Buffer`].
    ///
    /// The initial position of the cursor is `0`.
    pub fn new(buffer: &'a mut Buffer<SIZE>) -> Self {
        Self { buffer, pos: 0 }
    }

    /// Get a reference to this cursor's underlying [`Buffer`].
    pub fn buffer(&self) -> &Buffer<SIZE> {
        self.buffer
    }

    /// Get a mutable reference to this cursor's underlying [`Buffer`].
    pub fn buffer_mut(&mut self) -> &mut Buffer<SIZE> {
        self.buffer
    }

    /// Return the current position of the cursor.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Set the position of the cursor.
    pub fn set_position(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Reset the position of the cursor.
    pub fn reset_position(&mut self) {
        self.set_position(0)
    }

    /// Split underlying [`Buffer`] at the cursor position into two slices.
    ///
    /// # Panics
    ///
    /// Panics if cursor position > `SIZE`.
    pub fn split_at(&self) -> (&[u8], &[u8]) {
        self.buffer().as_ref().split_at(self.position())
    }

    /// Split underlying [`Buffer`] at the cursor position into two mutable
    /// slices.
    ///
    /// # Panics
    ///
    /// Panics if cursor position > `SIZE`.
    pub fn split_at_mut(&mut self) -> (&mut [u8], &mut [u8]) {
        let position = self.position();

        self.buffer_mut().as_mut().split_at_mut(position)
    }
}
