use core::str;

use crate::{error, result};

/// Reads from a buffer.
#[derive(Debug)]
pub struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    /// Create a new `Reader` on the provided `buffer`.
    pub fn new(buffer: &'a [u8]) -> Self {
        Self(buffer)
    }

    /// Read the next byte
    pub fn u8(&mut self) -> result::Len<u8> {
        self.take().map(|[byte]| byte)
    }

    /// Read a number of raw bytes.
    pub fn bytes(&mut self, len: usize) -> result::Len<&'a [u8]> {
        self.subslice(len)?.get(..len).ok_or(error::Len)
    }

    /// Parse a UTF-8 `String` of specified length.
    pub fn str(&mut self, len: usize) -> result::Str<&'a str> {
        str::from_utf8(self.bytes(len)?).map_err(|_| error::Utf8.into())
    }

    /// Return a `Reader` that reads up to the specified length.
    pub fn reader(&mut self, len: usize) -> result::Len<Self> {
        Ok(Self(self.subslice(len)?))
    }

    /// Return `Some(())` if end of buffer.
    pub fn end(&self) -> result::End {
        self.0.is_empty().then_some(()).ok_or(error::End)
    }

    pub(crate) fn subslice(&mut self, size: usize) -> result::Len<&'a [u8]> {
        if size > self.0.len() {
            return Err(error::Len);
        }

        let (slice, data) = self.0.split_at(size);

        self.0 = data;

        Ok(slice)
    }

    pub(crate) fn take<const SIZE: usize>(&mut self) -> result::Len<[u8; SIZE]> {
        self.subslice(SIZE)?.try_into().map_err(|_| error::Len)
    }
}
