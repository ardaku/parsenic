use core::str;

/// Reads from a buffer.
#[derive(Debug)]
pub struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    /// Create a new `Reader` on the provided `buffer`.
    pub fn new(buffer: &'a [u8]) -> Self {
        Self(buffer)
    }

    /// Read the next byte
    pub fn u8(&mut self) -> Option<u8> {
        self.0.first().copied()
    }

    /// Read a number of raw bytes.
    pub fn bytes(&mut self, len: usize) -> Option<&'a [u8]> {
        self.subslice(len)?.get(..len)
    }

    /// Parse a UTF-8 `String` of specified length.
    pub fn str(&mut self, len: usize) -> Option<&'a str> {
        str::from_utf8(self.bytes(len)?).ok()
    }

    /// Return a `Reader` that reads up to the specified length.
    pub fn reader(&mut self, len: usize) -> Option<Self> {
        Some(Self(self.subslice(len)?))
    }

    /// Return `Some(())` if end of buffer.
    pub fn end(&self) -> Option<()> {
        self.0.is_empty().then_some(())
    }

    pub(crate) fn subslice(&mut self, size: usize) -> Option<&'a [u8]> {
        if size > self.0.len() {
            return None;
        }

        let (slice, data) = self.0.split_at(size);

        self.0 = data;

        Some(slice)
    }
}
