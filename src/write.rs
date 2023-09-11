use core::iter::Extend;

use traitful::seal;

use crate::{UInt, Writer};

/// Basic writing methods
#[seal(for<T: Extend<u8>> Writer<'_, T>)]
pub trait Write {
    /// Write out `value` in ULEB128 encoding.
    fn uleb128<T: UInt>(&mut self, value: T);

    /// Write out a byte
    fn u8(&mut self, byte: u8);

    /// Write out a signed byte
    fn i8(&mut self, byte: i8);

    /// Write out a UTF-8 string slice (does not include length).
    fn str(&mut self, string: impl AsRef<str>);

    /// Write out raw bytes.
    fn bytes(&mut self, bytes: impl AsRef<[u8]>);
}
