use core::str;

use traitful::seal;

use crate::{
    num::UInt,
    result::{EndResult, LenResult, StrResult, Uleb128Result},
    Reader,
};

/// Basic reading methods
#[seal(Reader<'a>)]
pub trait Read<'a> {
    /// Read next number in ULEB128 encoding.
    fn uleb128<T: UInt>(&mut self) -> Uleb128Result<T>;

    /// Read the next byte
    fn u8(&mut self) -> LenResult<u8>;

    /// Read the next signed byte
    fn i8(&mut self) -> LenResult<i8>;

    /// Read a number of raw bytes as a slice.
    fn slice(&mut self, len: usize) -> LenResult<&'a [u8]>;

    /// Read a number of raw bytes as an array.
    fn array<const LEN: usize>(&mut self) -> LenResult<[u8; LEN]>;

    /// Parse a UTF-8 string slice of specified length.
    fn str(&mut self, len: usize) -> StrResult<&'a str>;

    /// Return `Some(())` if end of buffer.
    fn end(&self) -> EndResult;

    /// Return a `Reader` that reads up to the specified length.
    fn reader(&mut self, len: usize) -> LenResult<Self>
    where
        Self: Sized;
}
