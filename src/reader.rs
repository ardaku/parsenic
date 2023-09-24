use core::str;

use crate::{
    error::{EndError, LenError, OverflowError, Uleb128Error, Utf8Error},
    num::UInt,
    result::{EndResult, LenResult, StrResult, Uleb128Result},
    Read,
};

/// [`slice`] [`Read`]er
#[derive(Debug)]
pub struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    /// Create a new `Reader` on the provided `stream`.
    pub fn new(stream: &'a [u8]) -> Self {
        Self(stream)
    }

    fn subslice(&mut self, size: usize) -> LenResult<&'a [u8]> {
        if size > self.0.len() {
            return Err(LenError);
        }

        let (slice, data) = self.0.split_at(size);

        self.0 = data;

        Ok(slice)
    }
}

impl Read for Reader<'_> {
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

    fn u8(&mut self) -> LenResult<u8> {
        self.array().map(|[byte]| byte)
    }

    fn i8(&mut self) -> LenResult<i8> {
        self.array().map(|[byte]| i8::from_ne_bytes([byte]))
    }

    fn slice(&mut self, len: usize) -> LenResult<&'_ [u8]> {
        self.subslice(len)?.get(..len).ok_or(LenError)
    }

    fn array<const LEN: usize>(&mut self) -> LenResult<[u8; LEN]> {
        self.subslice(LEN)?.try_into().map_err(|_| LenError)
    }

    fn str(&mut self, len: usize) -> StrResult<&'_ str> {
        str::from_utf8(self.slice(len)?).map_err(|_| Utf8Error.into())
    }

    fn end(&self) -> EndResult {
        self.0.is_empty().then_some(()).ok_or(EndError)
    }

    fn reader(&mut self, len: usize) -> LenResult<Self>
    where
        Self: Sized,
    {
        Ok(Self(self.subslice(len)?))
    }
}
