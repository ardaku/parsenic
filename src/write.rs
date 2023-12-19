use core::iter::Extend;

use traitful::seal;

use crate::{class::UInt, result::FlushResult, Purge, Writer};

/// Basic writing methods
#[seal(
    for<T: Extend<u8>> Writer<'_, T>,
    Purge,
)]
// for<S: Save, const SIZE: usize> BufWriter<'_, S, SIZE>,
pub trait Write {
    /// Write out raw bytes.
    fn bytes(&mut self, bytes: impl AsRef<[u8]>) -> FlushResult;

    /// Write out a UTF-8 string slice (does not include length).
    fn str(&mut self, string: impl AsRef<str>) -> FlushResult {
        self.bytes(string.as_ref().as_bytes())
    }

    /// Write out a byte
    fn u8(&mut self, byte: u8) -> FlushResult {
        self.bytes([byte])
    }

    /// Write out a signed byte
    fn i8(&mut self, byte: i8) -> FlushResult {
        let [byte] = byte.to_ne_bytes();

        self.u8(byte)
    }

    /// Write out `value` in ULEB128 encoding.
    fn uleb128<T: UInt>(&mut self, value: T) -> FlushResult {
        let mut remaining = value;

        while {
            let byte = remaining.little();

            remaining >>= 7;

            let more = remaining != T::ZERO;

            self.u8(if more { byte | 0x80 } else { byte & !0x80 })?;

            more
        } {}

        Ok(())
    }
}
