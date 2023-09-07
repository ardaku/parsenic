use core::str;

use crate::{
    error::{EndError, LenError, OverflowError, Uleb128Error, Utf8Error},
    num::UInt,
    result::{EndResult, LenResult, StrResult, Uleb128Result},
};

/// Reads from a buffer.
#[derive(Debug)]
pub struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    /// Create a new `Reader` on the provided `buffer`.
    pub fn new(buffer: &'a [u8]) -> Self {
        Self(buffer)
    }

    /// Read next number in ULEB128 encoding.
    pub fn uleb128<T: UInt>(&mut self) -> Uleb128Result<T> {
        let mut value = T::ZERO;
        let mut shift = 0;

        while {
            let byte = self.u8()?;
            let next = byte & !0x80;
            let more = byte != next;

            if shift > T::BITS - 7 {
                return Err(Uleb128Error::Overflow(OverflowError));
            }

            value |= T::from(next) << shift;
            shift += 7;

            more
        } {}

        Ok(value)
    }

    /// Read the next byte
    pub fn u8(&mut self) -> LenResult<u8> {
        self.array().map(|[byte]| byte)
    }

    /// Read the next signed byte
    pub fn i8(&mut self) -> LenResult<i8> {
        self.array().map(|[byte]| i8::from_ne_bytes([byte]))
    }

    /// Read a number of raw bytes as a slice.
    pub fn slice(&mut self, len: usize) -> LenResult<&'a [u8]> {
        self.subslice(len)?.get(..len).ok_or(LenError)
    }

    /// Read a number of raw bytes as an array.
    pub fn array<const LEN: usize>(&mut self) -> LenResult<[u8; LEN]> {
        self.subslice(LEN)?.try_into().map_err(|_| LenError)
    }

    /// Parse a UTF-8 string slice of specified length.
    pub fn str(&mut self, len: usize) -> StrResult<&'a str> {
        str::from_utf8(self.slice(len)?).map_err(|_| Utf8Error.into())
    }

    /// Return a `Reader` that reads up to the specified length.
    pub fn reader(&mut self, len: usize) -> LenResult<Self> {
        Ok(Self(self.subslice(len)?))
    }

    /// Return `Some(())` if end of buffer.
    pub fn end(&self) -> EndResult {
        self.0.is_empty().then_some(()).ok_or(EndError)
    }

    pub(crate) fn subslice(&mut self, size: usize) -> LenResult<&'a [u8]> {
        if size > self.0.len() {
            return Err(LenError);
        }

        let (slice, data) = self.0.split_at(size);

        self.0 = data;

        Ok(slice)
    }
}
