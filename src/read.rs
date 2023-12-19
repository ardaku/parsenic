use core::str;

use traitful::seal;

use crate::{
    class::UInt,
    error::{EndError, LenError, OverflowError, Uleb128Error, Utf8Error},
    result::{EndResult, LenResult, StrResult, Uleb128Result},
    Empty, Reader,
};

/// Basic reading methods
#[seal(Reader<'_>, Empty)]
pub trait Read {
    /// Return the number of bytes remaining in this reader.
    fn remaining(&self) -> usize;

    /// Read a number of raw bytes as a reader.
    fn bytes(&mut self, len: usize) -> LenResult<Self>
    where
        Self: Sized;

    /// Read a number of raw bytes as a slice.
    fn slice(&mut self, len: usize) -> LenResult<&'_ [u8]>;

    /// Parse a UTF-8 string slice of specified length.
    fn str(&mut self, len: usize) -> StrResult<&'_ str> {
        str::from_utf8(self.slice(len)?).map_err(|_| Utf8Error.into())
    }

    /// Read a number of raw bytes as an array.
    fn array<const LEN: usize>(&mut self) -> LenResult<[u8; LEN]> {
        self.slice(LEN)?.try_into().map_err(|_| LenError)
    }

    /// Read the next byte
    fn u8(&mut self) -> LenResult<u8> {
        self.array().map(|[byte]| byte)
    }

    /// Read the next signed byte
    fn i8(&mut self) -> LenResult<i8> {
        self.array().map(|[byte]| i8::from_ne_bytes([byte]))
    }

    /// Read next number in ULEB128 encoding.
    fn uleb128<T: UInt>(&mut self) -> Uleb128Result<T> {
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

    /// Return [`Ok`] if end of buffer.
    fn end(&self) -> EndResult {
        (self.remaining() == 0).then_some(()).ok_or(EndError)
    }
}
