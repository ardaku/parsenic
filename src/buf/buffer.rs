/// A buffer for buffered reads and writes
#[derive(Debug)]
pub struct Buffer<const SIZE: usize = 8192>([u8; SIZE]);

impl<const SIZE: usize> Buffer<SIZE> {
    /// Create a new buffer.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<const SIZE: usize> Default for Buffer<SIZE> {
    fn default() -> Self {
        Self([0; SIZE])
    }
}

impl<const SIZE: usize> AsRef<[u8]> for Buffer<SIZE> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<const SIZE: usize> AsMut<[u8]> for Buffer<SIZE> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.0.as_mut()
    }
}
